use ratatui::{
    crossterm::event::{self, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Margin},
    prelude::Rect,
    style::{Style, Stylize},
    widgets::{self, Block, Clear, List, ListItem, Paragraph, Widget, Wrap},
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
    pub description_scroll_offset: u16,
    pub form_dimensions: FormDimensions,
}

impl Default for AddingState
{
    fn default() -> Self
    {
        Self::new()
    }
}


impl AddingState {
    pub fn new() -> Self
    {
        Self
        {
            input_task: String::new(),
            selected_priority: Priority::Medium,
            input_description: String::new(),
            current_field: AddingField::Task,
            description_scroll_offset: 0,
            form_dimensions: FormDimensions::new(),
        }
    }
    
    // Auto-scroll to keep cursor visible
    pub fn calculate_max_scroll(&self, field_width: u16, field_height: u16) -> u16
    {
        if field_width == 0 || field_height == 0
        {
            return 0;
        }
        
        let text_lines = if self.input_description.is_empty()
        {
            1
        } else {
            (self.input_description.len() as u16 + field_width - 1) / field_width
        };
        
        text_lines.saturating_sub(field_height)
    }
    
    pub fn auto_scroll_to_cursor(&mut self, form_dimensions: &FormDimensions)
    {
        if form_dimensions.field_width == 0 || form_dimensions.field_height == 0
        {
            return;
        }
        
        let cursor_line = if self.input_description.is_empty()
        {
            0
        } else {
            (self.input_description.len() as u16) / form_dimensions.field_width
        };
        
        let max_scroll = self.calculate_max_scroll(form_dimensions.field_width, form_dimensions.field_height);
        
        // Scroll down if cursor is below visible area
        if cursor_line >= self.description_scroll_offset + form_dimensions.field_height
        {
            self.description_scroll_offset = cursor_line.saturating_sub(form_dimensions.field_height - 1);
        }
        // Scroll up if cursor is above visible area
        else if cursor_line < self.description_scroll_offset
        {
            self.description_scroll_offset = cursor_line;
        }
        
        // Clamp to max scroll
        self.description_scroll_offset = self.description_scroll_offset.min(max_scroll);
    }
    
    pub fn scroll_up(&mut self, amount: u16)
    {
        self.description_scroll_offset = self.description_scroll_offset.saturating_sub(amount);
    }
    
    pub fn scroll_down(&mut self, amount: u16, field_width: u16, field_height: u16)
    {
        let max_scroll = self.calculate_max_scroll(field_width, field_height);
        self.description_scroll_offset = (self.description_scroll_offset + amount).min(max_scroll);
    }

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
    
    fn handle_arrows(&mut self, form_dimensions: &FormDimensions, key: KeyEvent)
    {
        if self.current_field == AddingField::Priority
        {
            match key.code
            {
                event::KeyCode::Up => self.selected_priority.next(),
                event::KeyCode::Down => self.selected_priority.previous(),
                _ => {},
            }
        }
        if self.current_field == AddingField::Description
        {
            match key.code
            {
                event::KeyCode::Up => self.scroll_up(1),
                event::KeyCode::Down => 
                {
                    self.scroll_down(1, form_dimensions.field_width, form_dimensions.field_height)
                },
                _ => {},
            }
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

#[derive(Clone)]
pub struct FormDimensions
{
    pub field_width: u16,
    pub field_height: u16,
}

impl FormDimensions
{
    pub fn new() -> Self
    {
        Self { field_width: 0, field_height: 0 }
    }

    pub fn calculate(frame_area: Rect) -> Self
    {
        Self
        {
            field_width: frame_area.width.saturating_sub(2),
            field_height: frame_area.height.saturating_sub(2),
        }
    }
}



pub fn run(terminal: &mut DefaultTerminal, data: &mut Data, app_state: &mut AppState) -> tui::TuiState
{
    let mut adding_state = AddingState::default();

    'tasks_render_loop: loop
    {
        let tui::TuiState::Tasks(ref task_state) = app_state.current_state else
        {
            continue 'tasks_render_loop;
        };

        if matches!(task_state, TasksState::Exit)
        {
            break 'tasks_render_loop;
        }

        // rendering
        match task_state
        {
            TasksState::Main =>
            {
                draw_terminal!(terminal => render_main(data): app_state, data);
            }
            TasksState::Adding =>
            {
                draw_terminal!(terminal => render_adding(data, &mut adding_state): app_state, data);
            }
            TasksState::Editing =>
            {
                draw_terminal!(terminal => render_editing(data, &mut adding_state): app_state, data);
            }
            TasksState::Exit => unreachable!(),
        }

        // input handling
        let event::Event::Key(key) = event::read().unwrap() else
        {
            continue 'tasks_render_loop;
        };

        if key.kind != KeyEventKind::Press
        {
            continue 'tasks_render_loop;
        }

        if app_state.has_error()
        {
            match key.code
            {
                event::KeyCode::Enter | event::KeyCode::Esc | event::KeyCode::Char(' ') =>
                {
                    app_state.clear_error();
                }
                _ => {}
            }
            continue 'tasks_render_loop;
        }

        match task_state
        {
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
                let selected_index = data.tasks.as_ref().unwrap().list_state.selected().unwrap();
                handle_keys_editing(app_state, key, data, &mut adding_state, selected_index);
            }
            TasksState::Exit =>
            {
                break 'tasks_render_loop;
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
                        app_state.current_state = tui::TuiState::Tasks(TasksState::Adding);
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
                                description_scroll_offset: 0,
                                form_dimensions: FormDimensions::new(),
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
            let form_dimensions: FormDimensions = adding_state.form_dimensions.clone();
            adding_state.handle_arrows(&form_dimensions, key);
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

fn render_form(
    frame: &mut Frame, 
    data: &mut Data, 
    adding_state: &mut AddingState, 
    title: &str,
    help_text: &str
)
{
    render_main(frame, data);
    
    
    // Make popup bigger to accommodate more content
    let popup_area = tui::centered_rect(70, 60, frame.area());
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
            Constraint::Length(3),      // Task name - fixed
            Constraint::Length(3),      // Priority - fixed  
            Constraint::Min(5),         // Description - grows as needed
            Constraint::Length(2),      // Help text - fixed
        ])
        .areas(inner_area);
    
    adding_state.form_dimensions = FormDimensions::calculate(chunks[2]);

    // Render all fields as paragraphs
    let field_data = [
        ("Task Name", &adding_state.input_task, adding_state.current_field == AddingField::Task, false),
        ("Priority (h/m/l or ↑↓)", &adding_state.selected_priority.to_string(), adding_state.current_field == AddingField::Priority, false),
        ("Description", &adding_state.input_description, adding_state.current_field == AddingField::Description, true),
    ];
    
    for (i, (label, value, is_selected, wrap)) in field_data.iter().enumerate() {
        let style = if *is_selected {
            Style::default().fg(data.settings.colors.selected)
        } else {
            Style::default().fg(data.settings.colors.default_text)
        };
        
        let mut paragraph = if *wrap {
            Paragraph::new(value.as_str())
                .block(Block::bordered().title(*label))
                .wrap(Wrap { trim: true })
                .style(style)
        } else {
            Paragraph::new(value.as_str())
                .block(Block::bordered().title(*label))
                .style(style)
        };
        
        // Add scrolling for description field
        if i == 2 && *wrap {
            paragraph = paragraph.scroll((adding_state.description_scroll_offset, 0));
        }
        
        frame.render_widget(paragraph, chunks[i]);
    }
    
    // Help text with scroll instructions
    let help_with_scroll = match adding_state.current_field
    {
        AddingField::Task => format!("{}", help_text),
        AddingField::Priority => format!("{} | h/m/l or ↑↓", help_text),
        AddingField::Description => format!("{} | ↑↓ to scroll", help_text),
    };
    
    let help = Paragraph::new(help_with_scroll)
        .style(Style::default().fg(data.settings.colors.default_text));
    frame.render_widget(help, chunks[3]);
    
    // Set cursor position with wrapping consideration for description
    let cursor_pos = match adding_state.current_field
    {
        AddingField::Task => (chunks[0].x + adding_state.input_task.len() as u16 + 1, chunks[0].y + 1),
        AddingField::Priority => (chunks[1].x + 1, chunks[1].y + 1),
        AddingField::Description => {
            // Calculate wrapped position considering scroll
            let field_width = chunks[2].width.saturating_sub(2);
            let field_height = chunks[2].height.saturating_sub(2);
            let text_len = adding_state.input_description.len() as u16;
            let line = text_len / field_width;
            let col = text_len % field_width;
            
            // Adjust for scroll offset
            let visible_line = line.saturating_sub(adding_state.description_scroll_offset);
            
            // Keep cursor within visible area
            let cursor_y = if visible_line < field_height {
                chunks[2].y + visible_line + 1
            } else {
                chunks[2].y + field_height // Bottom of visible area
            };
            
            (chunks[2].x + col + 1, cursor_y)
        },
    };
    frame.set_cursor_position(cursor_pos);
    let form_dimensions: FormDimensions = adding_state.form_dimensions.clone();
    adding_state.auto_scroll_to_cursor(&form_dimensions);
}

fn render_adding(frame: &mut Frame, data: &mut Data, adding_state: &mut AddingState)
{
    render_form(frame, data, adding_state, "Add New Task", 
                "Tab: Next field | Enter: Add task | Esc: Cancel");
}

fn render_editing(frame: &mut Frame, data: &mut Data, adding_state: &mut AddingState)
{
    render_form(frame, data, adding_state, "Edit Task", 
                "Tab: Next field | Enter: Save task | Esc: Cancel");
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
