use ratatui::{
    crossterm::event::{self, KeyEvent},
    layout::{Constraint, Direction, Layout, Margin},
    prelude::Rect,
    style::{Style, Stylize},
    widgets::{self, Block, List, ListItem, Paragraph, Widget, Clear},
    DefaultTerminal,
    Frame,
};

use crate::{
    commands::{
        self
    },
    structs::{
        Data,
        Priority,
    },
    tui::{
        self, tui::TuiColor, AppState, TasksState
    }
};

#[derive(Debug, Clone, PartialEq)]
pub enum AddingField
{
    Task,
    Priority, 
    Description,
}

pub struct AddingState
{
    pub input_task: String,
    pub selected_priority: Priority,
    pub input_description: String,
    pub current_field: AddingField,
}

impl Default for AddingState
{
    fn default() -> Self
    {
        Self
        {
            input_task: String::new(),
            selected_priority: Priority::Medium,
            input_description: String::new(),
            current_field: AddingField::Task,
        }
    }
}

impl AddingState
{
    fn handle_field_navigation(&mut self, key: KeyEvent) -> bool
    {
        match key.code
        {
            event::KeyCode::Tab =>
            {
                if key.modifiers.contains(event::KeyModifiers::SHIFT)
                {
                    self.cycle_field_backward();
                } else {
                    self.cycle_field_forward();
                }
                true
            }
            event::KeyCode::BackTab =>
            {
                self.cycle_field_backward();
                true
            }
            _ => false
        }
    }
    
    fn cycle_field_forward(&mut self)
    {
        self.current_field = match self.current_field
        {
            AddingField::Task => AddingField::Priority,
            AddingField::Priority => AddingField::Description,
            AddingField::Description => AddingField::Task,
        };
    }
    
    fn cycle_field_backward(&mut self)
    {
        self.current_field = match self.current_field
        {
            AddingField::Task => AddingField::Description,
            AddingField::Priority => AddingField::Task,
            AddingField::Description => AddingField::Priority,
        };
    }
    
    fn handle_character_input(&mut self, c: char)
    {
        match self.current_field
        {
            AddingField::Task => self.input_task.push(c),
            AddingField::Priority =>
            {
                match c.to_ascii_lowercase()
                {
                    'h' => self.selected_priority = Priority::High,
                    'm' => self.selected_priority = Priority::Medium,
                    'l' => self.selected_priority = Priority::Low,
                    _ => {}
                }
            }
            AddingField::Description => self.input_description.push(c),
        }
    }
    
    fn handle_backspace(&mut self)
    {
        match self.current_field
        {
            AddingField::Task => { self.input_task.pop(); }
            AddingField::Priority =>
            {
                self.selected_priority = match self.selected_priority
                {
                    Priority::High => Priority::Low,
                    Priority::Medium => Priority::High,
                    Priority::Low => Priority::Medium,
                };
            }
            AddingField::Description => { self.input_description.pop(); }
        }
    }
    
    fn handle_priority_arrows(&mut self)
    {
        if self.current_field == AddingField::Priority
        {
            self.selected_priority = match self.selected_priority
            {
                Priority::High => Priority::Medium,
                Priority::Medium => Priority::Low,
                Priority::Low => Priority::High,
            };
        }
    }
    
    fn to_task(&self) -> commands::tasks::Task
    {
        commands::tasks::Task
        {
            task: self.input_task.clone(),
            priority: self.selected_priority.clone(),
            description: self.input_description.clone(),
        }
    }
    
    fn is_valid(&self) -> bool
    {
        !self.input_task.trim().is_empty()
    }
}

macro_rules! draw_terminal
{
    ($terminal:expr => $render_function:ident ( $($args:expr),*): $app_state:expr, $data:expr) =>
    {{
        if !$app_state.has_error()
        {
            $terminal.draw(|frame| $render_function(frame $(, $args)*)).unwrap();
        } else {
            $terminal.draw(|frame|
            {
                $render_function(frame $(, $args)*);
                crate::tui::render_popup(frame, $app_state.error_state.as_ref().unwrap(), &$data.settings.colors)
            }).unwrap();
        }
    }};
}

pub fn run(terminal: &mut DefaultTerminal, data: &mut Data, app_state: &mut AppState) -> tui::TuiState
{
    let mut adding_state = AddingState::default();

    'tasks_render_loop: loop
    {
        if let tui::TuiState::Tasks(ref task_state) = app_state.current_state
        {
            // rendering
            match task_state
            {
                TasksState::Exit => 
                {
                    break 'tasks_render_loop;
                }
                TasksState::Main => 
                {
                    draw_terminal!(terminal => render_main(data): app_state, data);
                }
                TasksState::Adding => 
                {
                    draw_terminal!(terminal => render_adding(data, &adding_state): app_state, data);
                }
                TasksState::Editing => 
                {
                    draw_terminal!(terminal => render_editing(data, &adding_state): app_state, data);
                }
            };
            // input handeling
            if let event::Event::Key(key) = event::read().unwrap()
            {
                if app_state.has_error()
                {
                    match key.code
                    {
                        event::KeyCode::Enter | event::KeyCode::Esc | event::KeyCode::Char(' ') =>
                            app_state.clear_error(),
                        _ => {},
                    }
                } else {
                    match task_state
                    {
                        TasksState::Exit => 
                        {
                            break 'tasks_render_loop;
                        }
                        TasksState::Main => 
                        {
                            handle_keys_main(app_state, key, data, &mut adding_state);
                        }
                        TasksState::Adding => 
                        {
                            handle_keys_adding(app_state, key, data, &mut adding_state);
                        }
                        TasksState::Editing => 
                        {
                            handle_keys_editing(app_state, key, data, &mut adding_state, data.tasks.as_ref().unwrap().list_state.selected().unwrap());
                        }
                    }
                }
            }
        }
    }
    commands::tasks::write_tasks(data.tasks.as_ref().unwrap());
    tui::TuiState::Exit
}

fn handle_keys_main(app_state: &mut AppState, key: KeyEvent, data: &mut Data, adding_state: &mut AddingState)
{
    match key.code
    {
        event::KeyCode::Esc => { app_state.current_state = tui::TuiState::Tasks(TasksState::Exit); return; }
        event::KeyCode::Char(char) =>
        {
            if let Some(tasks) = data.tasks.as_mut()
            {
                match char
                {
                    'A' => 
                    {
                        *adding_state = Default::default();
                        app_state.current_state = tui::TuiState::Tasks(TasksState::Exit);
                        return;
                    },
                    'E' =>
                    { 
                        if let Some(index) = tasks.list_state.selected()
                        {
                            *adding_state = AddingState {
                                current_field: AddingField::Task,
                                input_task: tasks.tasks[index].task.clone(),
                                selected_priority: tasks.tasks[index].priority.clone(),
                                input_description: tasks.tasks[index].description.clone(),
                            };
                            app_state.current_state = tui::TuiState::Tasks(tui::TasksState::Editing);
                            return;
                        } else {
                            app_state.set_error("Nothing selected".to_string(), "No task has been selected".to_string(), tui::ErrorType::Warning);
                            return;
                        };
                    },
                    'X' =>
                    {
                        if let Some(index) = tasks.list_state.selected()
                        {
                            tasks.tasks.remove(index);
                        };
                    }
                    'k' =>
                    {
                        tasks.list_state.select_previous();
                    }
                    'j' =>
                    {
                        tasks.list_state.select_next();
                    }
                    _ => {},
                }
            }
        }
        _ => {},
    }
    return;
}

fn handle_keys_form(
    app_state: &mut AppState,
    key: KeyEvent, 
    data: &mut Data, 
    adding_state: &mut AddingState, 
    index: Option<usize>
)
{
    match key.code
    {
        event::KeyCode::Esc =>
        {
            *adding_state = AddingState::default();
            app_state.current_state = tui::TuiState::Tasks(TasksState::Main);
            return;
        }
        
        event::KeyCode::Enter =>
        {
            if adding_state.is_valid()
            {
                if let Some(tasks) = data.tasks.as_mut()
                {
                    let new_task = adding_state.to_task();
                    
                    match index
                    {
                        Some(idx) => tasks.tasks[idx] = new_task, // Edit mode
                        None => tasks.tasks.push(new_task),       // Add mode
                    }
                }
                app_state.current_state = tui::TuiState::Tasks(TasksState::Main);
                return;
            }
        }
        
        event::KeyCode::Char(c) =>
        {
            adding_state.handle_character_input(c);
        }
        
        event::KeyCode::Backspace =>
        {
            adding_state.handle_backspace();
        }
        
        event::KeyCode::Up | event::KeyCode::Down =>
        {
            adding_state.handle_priority_arrows();
        }
        
        _ =>
        {
            if adding_state.handle_field_navigation(key)
            {
                // Field navigation was handled
            }
        }
    }
    
    // Return appropriate state based on mode
    app_state.current_state = match index
    {
        Some(_) => tui::TuiState::Tasks(TasksState::Editing),
        None => tui::TuiState::Tasks(TasksState::Adding),
    }
}

fn handle_keys_adding(app_state: &mut AppState, key: KeyEvent, data: &mut Data, adding_state: &mut AddingState)
{
    handle_keys_form(app_state, key, data, adding_state, None)
}

fn handle_keys_editing(app_state: &mut AppState, key: KeyEvent, data: &mut Data, adding_state: &mut AddingState, index: usize)
{
    handle_keys_form(app_state, key, data, adding_state, Some(index))
}

struct FormField<'a>
{
    title: &'a str,
    content: String,
    is_active: bool,
    help_text: Option<&'a str>,
}

impl<'a> FormField<'a>
{
    fn new(title: &'a str, content: String, is_active: bool) -> Self
    {
        Self { title, content, is_active, help_text: None }
    }
    
    fn with_help(mut self, help: &'a str) -> Self
    {
        self.help_text = Some(help);
        self
    }
    
    fn render(&self, frame: &mut Frame, area: Rect, colors: &TuiColor)
    {
        let style = if self.is_active
        {
            Style::default().fg(colors.selected)
        } else {
            Style::default().fg(colors.default_text)
        };
        
        let display_content = match self.help_text
        {
            Some(help) => format!("{} {}", self.content, help),
            None => self.content.clone(),
        };
        
        let paragraph = Paragraph::new(display_content)
            .style(style)
            .block(Block::bordered().title(self.title));
        
        frame.render_widget(paragraph, area);
    }
}

fn render_form(
    frame: &mut Frame, 
    data: &mut Data, 
    adding_state: &AddingState, 
    title: &str,
    help_text: &str
)
{
    render_main(frame, data);
    
    let popup_area = tui::centered_rect(70, 40, frame.area());
    frame.render_widget(Clear, popup_area);
    
    let popup_block = Block::bordered()
        .title(title)
        .border_type(widgets::BorderType::Rounded)
        .fg(data.settings.colors.default_text);
    
    frame.render_widget(popup_block, popup_area);
    
    let inner_area = popup_area.inner(Margin::new(1, 1));
    let chunks: [Rect; 4] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .areas(inner_area);
    
    // Render form fields
    let fields = [
        FormField::new("Task Name", adding_state.input_task.clone(), 
                      adding_state.current_field == AddingField::Task),
        FormField::new("Priority", adding_state.selected_priority.to_string(), 
                      adding_state.current_field == AddingField::Priority)
            .with_help("(h/m/l or ↑↓)"),
        FormField::new("Description", adding_state.input_description.clone(), 
                      adding_state.current_field == AddingField::Description),
    ];
    
    for (i, field) in fields.iter().enumerate()
    {
        field.render(frame, chunks[i], &data.settings.colors);
    }
    
    // Help text
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(data.settings.colors.default_text));
    frame.render_widget(help, chunks[3]);
    
    // Set cursor position
    let cursor_pos = match adding_state.current_field
    {
        AddingField::Task => (chunks[0].x + adding_state.input_task.len() as u16 + 1, chunks[0].y + 1),
        AddingField::Priority => (chunks[1].x + 1, chunks[1].y + 1),
        AddingField::Description => (chunks[2].x + adding_state.input_description.len() as u16 + 1, chunks[2].y + 1),
    };
    frame.set_cursor_position(cursor_pos);
}

fn render_adding(frame: &mut Frame, data: &mut Data, adding_state: &AddingState)
{
    render_form(frame, data, adding_state, "Add New Task", 
                "Tab: Next field | h/m/l or ↑↓: Priority | Enter: Add task | Esc: Cancel");
}

fn render_editing(frame: &mut Frame, data: &mut Data, adding_state: &AddingState)
{
    render_form(frame, data, adding_state, "Edit Task", 
                "Tab: Next field | h/m/l or ↑↓: Priority | Enter: Save task | Esc: Cancel");
}

fn create_task_list<'a, F>(tasks: &'a [commands::tasks::Task], extractor: F, colors: &TuiColor) -> List<'a>
where 
    F: Fn(&commands::tasks::Task) -> String,
{
    List::new(
        tasks.iter()
            .map(|task| ListItem::from(extractor(task)))
            .collect::<Vec<_>>()
    )
    .highlight_style(Style::default().fg(colors.selected))
}

fn render_main(frame: &mut Frame, data: &mut Data)
{
    let chunks: [Rect; 1] = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Fill(1)])
        .areas(frame.area());

    let chunks_inner: [Rect; 3] = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(20), Constraint::Percentage(50)])
        .areas(chunks[0]);

    Block::bordered()
        .border_type(widgets::BorderType::Rounded)
        .fg(data.settings.colors.default_text)
        .render(chunks[0], frame.buffer_mut());

    if let Some(tasks_data) = data.tasks.as_mut()
    {
        let lists = [
            (create_task_list(&tasks_data.tasks, |t| t.task.clone(), &data.settings.colors).highlight_symbol(">"), chunks_inner[0]),
            (create_task_list(&tasks_data.tasks, |t| t.priority.to_string(), &data.settings.colors), chunks_inner[1]),
            (create_task_list(&tasks_data.tasks, |t| t.description.clone(), &data.settings.colors), chunks_inner[2]),
        ];

        for (list, area) in lists
        {
            frame.render_stateful_widget(list, area, &mut tasks_data.list_state);
        }
    }
}

