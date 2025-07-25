use ratatui::{
    crossterm::event::{self, Event, KeyEvent},
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
        self, TasksState,
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


pub fn run(terminal: &mut DefaultTerminal, data: &mut Data, mut state: TasksState) -> tui::TuiState
{
    let mut adding_state = AddingState::default();

    'tasks_render_loop: loop
    {
        // rendering
        match state
        {
            TasksState::Exit => 
            {
                break 'tasks_render_loop;
            }
            TasksState::Main => 
            {
                terminal.draw(|frame| render_main(frame, data)).unwrap();
            }
            TasksState::Adding => 
            {
                terminal.draw(|frame| render_adding(frame, data, &adding_state)).unwrap();
            }
            TasksState::Editing => 
            {
                terminal.draw(|frame| render_editing(frame, data, &adding_state)).unwrap();
            }
        };
        // input handeling
        if let Event::Key(key) = event::read().unwrap()
        {
            state = match state
            {
                TasksState::Exit => 
                {
                    break 'tasks_render_loop;
                }
                TasksState::Main => 
                {
                    handle_keys_main(key, data, &mut adding_state)
                }
                TasksState::Adding => 
                {
                    handle_keys_adding(key, data, &mut adding_state)
                }
                TasksState::Editing => 
                {
                    handle_keys_editing(key, data, &mut adding_state, data.tasks.as_ref().unwrap().list_state.selected().unwrap())
                }
            }
        }
    }
    commands::tasks::write_tasks(data.tasks.as_ref().unwrap());
    tui::TuiState::Exit
}

fn handle_keys_main(key: KeyEvent, data: &mut Data, adding_state: &mut AddingState) -> tui::TasksState
{
    match key.code
    {
        event::KeyCode::Esc => { return tui::TasksState::Exit; }
        event::KeyCode::Char(char) =>
        {
            if let Some(tasks) = data.tasks.as_mut()
            {
                match char
                {
                    'A' => 
                    {
                        *adding_state = Default::default();
                        return tui::TasksState::Adding
                    },
                    'E' => return 
                    { 
                        if let Some(index) = tasks.list_state.selected()
                        {
                            *adding_state = AddingState{
                                current_field: AddingField::Task,
                                input_task: tasks.tasks[index].task.clone(),
                                selected_priority: tasks.tasks[index].priority.clone(),
                                input_description: tasks.tasks[index].description.clone(),
                            }
                        };
                        tui::TasksState::Editing
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
    tui::TasksState::Main
}

fn handle_keys_adding(key: KeyEvent, data: &mut Data, adding_state: &mut AddingState) -> TasksState
{
    match key.code
    {
        event::KeyCode::Esc =>
        { 
            // Clear inputs and return to main
            *adding_state = AddingState::default();
            return tui::TasksState::Main; 
        }
        event::KeyCode::Tab =>
        {
            // Check if Shift is held for reverse cycling
            if key.modifiers.contains(event::KeyModifiers::SHIFT)
            {
                // Cycle backwards through fields
                adding_state.current_field = match adding_state.current_field
                {
                    AddingField::Task => AddingField::Description,
                    AddingField::Priority => AddingField::Task,
                    AddingField::Description => AddingField::Priority,
                };
            } else {
                // Cycle forward through fields
                adding_state.current_field = match adding_state.current_field
                {
                    AddingField::Task => AddingField::Priority,
                    AddingField::Priority => AddingField::Description,
                    AddingField::Description => AddingField::Task,
                };
            }
        }
        event::KeyCode::BackTab =>
        {
            // BackTab is specifically Shift+Tab on some terminals
            adding_state.current_field = match adding_state.current_field
            {
                AddingField::Task => AddingField::Description,
                AddingField::Priority => AddingField::Task,
                AddingField::Description => AddingField::Priority,
            };
        }
        event::KeyCode::Enter =>
        {
            // Add the task if all fields have content
            if !adding_state.input_task.trim().is_empty()
            {
                if let Some(tasks) = data.tasks.as_mut()
                {
                    let new_task = commands::tasks::Task
                    {
                        task: adding_state.input_task.clone(),
                        priority: adding_state.selected_priority.clone(),
                        description: adding_state.input_description.clone(),
                    };
                    tasks.tasks.push(new_task);
                }
                return tui::TasksState::Main;
            }
        }
        event::KeyCode::Char(c) =>
        {
            // Add character to current field
            match adding_state.current_field
            {
                AddingField::Task => adding_state.input_task.push(c),
                AddingField::Priority =>
                {
                    // Cycle through priority options with h/m/l keys
                    match c.to_ascii_lowercase() {
                        'h' => adding_state.selected_priority = Priority::High,
                        'm' => adding_state.selected_priority = Priority::Medium,
                        'l' => adding_state.selected_priority = Priority::Low,
                        _ => {} // Ignore other characters for priority field
                    }
                }
                AddingField::Description => adding_state.input_description.push(c),
            }
        }
        event::KeyCode::Backspace =>
        {
            // Remove character from current field
            match adding_state.current_field
            {
                AddingField::Task => { adding_state.input_task.pop(); }
                AddingField::Priority =>
                {
                    // For priority, cycle backwards through options
                    adding_state.selected_priority = match adding_state.selected_priority
                    {
                        Priority::High => Priority::Low,
                        Priority::Medium => Priority::High,
                        Priority::Low => Priority::Medium,
                    };
                }
                AddingField::Description => { adding_state.input_description.pop(); }
            }
        }
        event::KeyCode::Up | event::KeyCode::Down =>
        {
            // Arrow keys for priority selection
            if adding_state.current_field == AddingField::Priority
            {
                adding_state.selected_priority = match adding_state.selected_priority
                {
                    Priority::High => Priority::Medium,
                    Priority::Medium => Priority::Low,
                    Priority::Low => Priority::High,
                };
            }
        }
        _ => {},
    }
    tui::TasksState::Adding
}

fn handle_keys_editing(key: KeyEvent, data: &mut Data, adding_state: &mut AddingState, index: usize) -> TasksState
{
    match key.code
    {
        event::KeyCode::Esc =>
        { 
            // Clear inputs and return to main
            *adding_state = AddingState::default();
            return tui::TasksState::Main; 
        }
        event::KeyCode::Tab =>
        {
            // Check if Shift is held for reverse cycling
            if key.modifiers.contains(event::KeyModifiers::SHIFT)
            {
                // Cycle backwards through fields
                adding_state.current_field = match adding_state.current_field
                {
                    AddingField::Task => AddingField::Description,
                    AddingField::Priority => AddingField::Task,
                    AddingField::Description => AddingField::Priority,
                };
            } else {
                // Cycle forward through fields
                adding_state.current_field = match adding_state.current_field
                {
                    AddingField::Task => AddingField::Priority,
                    AddingField::Priority => AddingField::Description,
                    AddingField::Description => AddingField::Task,
                };
            }
        }
        event::KeyCode::BackTab =>
        {
            // BackTab is specifically Shift+Tab on some terminals
            adding_state.current_field = match adding_state.current_field
            {
                AddingField::Task => AddingField::Description,
                AddingField::Priority => AddingField::Task,
                AddingField::Description => AddingField::Priority,
            };
        }
        event::KeyCode::Enter =>
        {
            // Add the task if all fields have content
            if !adding_state.input_task.trim().is_empty()
            {
                if let Some(tasks) = data.tasks.as_mut()
                {
                    let new_task = commands::tasks::Task
                    {
                        task: adding_state.input_task.clone(),
                        priority: adding_state.selected_priority.clone(),
                        description: adding_state.input_description.clone(),
                    };
                    tasks.tasks[index] = new_task;
                }
                return tui::TasksState::Main;
            }
        }
        event::KeyCode::Char(c) =>
        {
            // Add character to current field
            match adding_state.current_field
            {
                AddingField::Task => adding_state.input_task.push(c),
                AddingField::Priority =>
                {
                    // Cycle through priority options with h/m/l keys
                    match c.to_ascii_lowercase() {
                        'h' => adding_state.selected_priority = Priority::High,
                        'm' => adding_state.selected_priority = Priority::Medium,
                        'l' => adding_state.selected_priority = Priority::Low,
                        _ => {} // Ignore other characters for priority field
                    }
                }
                AddingField::Description => adding_state.input_description.push(c),
            }
        }
        event::KeyCode::Backspace =>
        {
            // Remove character from current field
            match adding_state.current_field
            {
                AddingField::Task => { adding_state.input_task.pop(); }
                AddingField::Priority =>
                {
                    // For priority, cycle backwards through options
                    adding_state.selected_priority = match adding_state.selected_priority
                    {
                        Priority::High => Priority::Low,
                        Priority::Medium => Priority::High,
                        Priority::Low => Priority::Medium,
                    };
                }
                AddingField::Description => { adding_state.input_description.pop(); }
            }
        }
        event::KeyCode::Up | event::KeyCode::Down =>
        {
            // Arrow keys for priority selection
            if adding_state.current_field == AddingField::Priority
            {
                adding_state.selected_priority = match adding_state.selected_priority
                {
                    Priority::High => Priority::Medium,
                    Priority::Medium => Priority::Low,
                    Priority::Low => Priority::High,
                };
            }
        }
        _ => {},
    }
    tui::TasksState::Editing
}

fn render_main(frame: &mut Frame, data: &mut Data)
{
    Paragraph::new("Main").render(frame.area(), frame.buffer_mut());
    let chunks: [Rect; 1] = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Fill(1)].as_ref())
        .areas(frame.area());

    let border_area: Rect = chunks[0];

    let chunks_inner: [Rect; 3] = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(20), Constraint::Percentage(50)].as_ref())
        .areas(border_area);

    let inner_name_area: Rect = chunks_inner[0];
    let inner_priority_area: Rect = chunks_inner[1];
    let inner_description_area: Rect = chunks_inner[2];

    Block::bordered().border_type(widgets::BorderType::Rounded)
        .fg(data.settings.colors.default_text)
        .render(border_area, frame.buffer_mut());


    let list_name: List<'_> = List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.task.clone()))
        )
        .highlight_symbol(">")
        .highlight_style(Style::default()
            .fg(data.settings.colors.selected)
        );

    let list_priority: List<'_> = List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.priority.to_string()))
        )
        .highlight_style(Style::default()
            .fg(data.settings.colors.selected)
        );

    let list_description: List<'_> = List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.description.clone()))
        )
        .highlight_style(Style::default()
            .fg(data.settings.colors.selected)
        );

    let tasks: &mut commands::tasks::Tasks = data.tasks.as_mut().unwrap();
    frame.render_stateful_widget(list_name, inner_name_area, &mut tasks.list_state);
    frame.render_stateful_widget(list_priority, inner_priority_area, &mut tasks.list_state);
    frame.render_stateful_widget(list_description, inner_description_area, &mut tasks.list_state);

}

fn render_adding(frame: &mut Frame, data: &mut Data, adding_state: &AddingState)
{
    // First render the main view as background
    render_main(frame, data);
    
    // Create a centered popup area
    let popup_area: Rect = centered_rect(70, 40, frame.area());
    
    // Clear the popup area
    frame.render_widget(Clear, popup_area);
    
    // Create the main popup block
    let popup_block: Block<'_> = Block::bordered()
        .title("Add New Task")
        .border_type(widgets::BorderType::Rounded)
        .fg(data.settings.colors.default_text);
    
    frame.render_widget(popup_block, popup_area);
    
    // Create inner layout for the form fields
    let inner_area: Rect = popup_area.inner(Margin::new(1, 1));
    let chunks: [Rect; 4] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Task input
            Constraint::Length(3), // Priority input  
            Constraint::Length(3), // Explanation input
            Constraint::Length(2), // Help text
        ])
        .areas(inner_area);
    
    // Task input field
    let task_style: Style = if adding_state.current_field == AddingField::Task
    {
        Style::default().fg(data.settings.colors.selected)
    } else {
        Style::default().fg(data.settings.colors.default_text)
    };
    
    let task_input: Paragraph<'_> = Paragraph::new(adding_state.input_task.as_str())
        .style(task_style)
        .block(Block::bordered().title("Task Name"));
    frame.render_widget(task_input, chunks[0]);
    
    // Priority input field
    let priority_style: Style = if adding_state.current_field == AddingField::Priority
    {
        Style::default().fg(data.settings.colors.selected)
    } else {
        Style::default().fg(data.settings.colors.default_text)
    };
    
    let priority_display: String = format!("{} (h/m/l or ↑↓)", adding_state.selected_priority);
    let priority_input: Paragraph<'_> = Paragraph::new(priority_display)
        .style(priority_style)
        .block(Block::bordered().title("Priority"));
    frame.render_widget(priority_input, chunks[1]);
    
    // Explanation input field
    let description_style: Style = if adding_state.current_field == AddingField::Description
    {
        Style::default().fg(data.settings.colors.selected)
    } else {
        Style::default().fg(data.settings.colors.default_text)
    };
    
    let description_input: Paragraph<'_> = Paragraph::new(adding_state.input_description.as_str())
        .style(description_style)
        .block(Block::bordered().title("Description"));
    frame.render_widget(description_input, chunks[2]);
    
    // Help text
    let help_text: Paragraph<'_> = Paragraph::new("Tab: Next field | h/m/l or ↑↓: Priority | Enter: Add task | Esc: Cancel")
        .style(Style::default().fg(data.settings.colors.default_text));
    frame.render_widget(help_text, chunks[3]);
    
    // Set cursor position for the active field
    let cursor_pos: (u16, u16) = match adding_state.current_field {
        AddingField::Task => (chunks[0].x + adding_state.input_task.len() as u16 + 1, chunks[0].y + 1),
        AddingField::Priority => (chunks[1].x + 1, chunks[1].y + 1), // Fixed position for priority
        AddingField::Description => (chunks[2].x + adding_state.input_description.len() as u16 + 1, chunks[2].y + 1),
    };
    frame.set_cursor_position((cursor_pos.0, cursor_pos.1));
}

fn render_editing(frame: &mut Frame, data: &mut Data, adding_state: &AddingState)
{
    // First render the main view as background
    render_main(frame, data);
    
    // Create a centered popup area
    let popup_area: Rect = centered_rect(70, 40, frame.area());
    
    // Clear the popup area
    frame.render_widget(Clear, popup_area);
    
    // Create the main popup block
    let popup_block: Block<'_> = Block::bordered()
        .title("Edit Task")
        .border_type(widgets::BorderType::Rounded)
        .fg(data.settings.colors.default_text);
    
    frame.render_widget(popup_block, popup_area);
    
    // Create inner layout for the form fields
    let inner_area: Rect = popup_area.inner(Margin::new(1, 1));
    let chunks: [Rect; 4] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Task input
            Constraint::Length(3), // Priority input  
            Constraint::Length(3), // Explanation input
            Constraint::Length(2), // Help text
        ])
        .areas(inner_area);
    
    // Task input field
    let task_style: Style = if adding_state.current_field == AddingField::Task
    {
        Style::default().fg(data.settings.colors.selected)
    } else {
        Style::default().fg(data.settings.colors.default_text)
    };
    
    let task_input: Paragraph<'_> = Paragraph::new(adding_state.input_task.as_str())
        .style(task_style)
        .block(Block::bordered().title("Task Name"));
    frame.render_widget(task_input, chunks[0]);
    
    // Priority input field
    let priority_style: Style = if adding_state.current_field == AddingField::Priority
    {
        Style::default().fg(data.settings.colors.selected)
    } else {
        Style::default().fg(data.settings.colors.default_text)
    };
    
    let priority_display: String = format!("{} (h/m/l or ↑↓)", adding_state.selected_priority);
    let priority_input: Paragraph<'_> = Paragraph::new(priority_display)
        .style(priority_style)
        .block(Block::bordered().title("Priority"));
    frame.render_widget(priority_input, chunks[1]);
    
    // Explanation input field
    let description_style: Style = if adding_state.current_field == AddingField::Description
    {
        Style::default().fg(data.settings.colors.selected)
    } else {
        Style::default().fg(data.settings.colors.default_text)
    };
    
    let description_input: Paragraph<'_> = Paragraph::new(adding_state.input_description.as_str())
        .style(description_style)
        .block(Block::bordered().title("Description"));
    frame.render_widget(description_input, chunks[2]);
    
    // Help text
    let help_text: Paragraph<'_> = Paragraph::new("Tab: Next field | h/m/l or ↑↓: Priority | Enter: Save task | Esc: Cancel")
        .style(Style::default().fg(data.settings.colors.default_text));
    frame.render_widget(help_text, chunks[3]);
    
    // Set cursor position for the active field
    let cursor_pos: (u16, u16) = match adding_state.current_field {
        AddingField::Task => (chunks[0].x + adding_state.input_task.len() as u16 + 1, chunks[0].y + 1),
        AddingField::Priority => (chunks[1].x + 1, chunks[1].y + 1), // Fixed position for priority
        AddingField::Description => (chunks[2].x + adding_state.input_description.len() as u16 + 1, chunks[2].y + 1),
    };
    frame.set_cursor_position((cursor_pos.0, cursor_pos.1));
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect
{
    let popup_layout: [Rect; 3] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .areas(r);

    let horizontal_layout: [Rect; 3] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .areas(popup_layout[1]);
        
    horizontal_layout[1]
}
