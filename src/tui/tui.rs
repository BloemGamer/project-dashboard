use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect, 
    style::Color, 
    DefaultTerminal, 
    Frame,
};

use crate::{
    commands::{
        self
    },
    structs::Data,
    tui::{
        tasks,
    }
};


#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct TuiColor
{
    pub default_text: Color,
    pub selected: Color,
}

impl TuiColor
{
    pub const DEFAULT_TEXT: Color = Color::Blue;
    pub const SELECTED: Color = Color::Gray;
}

pub struct AppState
{
    pub current_state: TuiState,
    pub error_state: Option<ErrorInfo>,
}


impl AppState
{
    pub fn new() -> Self
    {
        Self
        {
            current_state: TuiState::Tasks(TasksState::Main),
            error_state: None,
        }
    }
    
    pub fn set_error(&mut self, title: String, message: String, error_type: ErrorType)
    {
        self.error_state = Some(ErrorInfo { title, message, error_type });
    }
    
    pub fn clear_error(&mut self)
    {
        self.error_state = None;
    }
    
    pub fn has_error(&self) -> bool
    {
        self.error_state.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct ErrorInfo
{
    pub title: String,
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone)]
pub enum ErrorType
{
    Warning,
    Error,
    Info,
}

pub enum TuiState
{
    Tasks(TasksState),
    Exit,
}

pub enum TasksState
{
    Main,
    Adding,
    Editing,
    Exit,
}

pub fn start()
{
    color_eyre::install().unwrap();
}

pub fn run(mut terminal: DefaultTerminal, data: &mut Data)
{
    let mut app_state: AppState = AppState::new();
    'main_render_loop: loop
    {
        app_state.current_state = match app_state.current_state
        {
            TuiState::Tasks(_) => tasks::run(&mut terminal, data, &mut app_state),

            TuiState::Exit => break 'main_render_loop,
        }
    }
    commands::tasks::write_tasks(data.tasks.as_ref().unwrap());
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect
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

pub fn render_log_popup(frame: &mut Frame, error_info: &ErrorInfo, colors: &TuiColor)
{
    use ratatui::{
        widgets::{Clear, Block, Paragraph, Wrap},
        layout::{Constraint, Direction, Layout, Margin},
        style::{Style, Stylize},
    };
    
    // Create popup area (smaller than form popups)
    let popup_area = centered_rect(60, 30, frame.area());
    
    // Clear the popup area
    frame.render_widget(Clear, popup_area);
    
    // Choose color based on error type
    let border_color = match error_info.error_type
    {
        ErrorType::Error => ratatui::prelude::Color::Red,
        ErrorType::Warning => ratatui::prelude::Color::Yellow,
        ErrorType::Info => colors.default_text,
    };
    
    // Create the popup block
    let popup_block = Block::bordered()
        .title(error_info.title.clone())
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(border_color);
    
    frame.render_widget(popup_block, popup_area);
    
    // Create inner layout
    let inner_area = popup_area.inner(Margin::new(1, 1));
    let chunks: [Rect; 2] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),   // Message area
            Constraint::Length(1), // Help text
        ])
        .areas(inner_area);
    
    // Render error message
    let message = Paragraph::new(error_info.message.clone())
        .style(Style::default().fg(colors.default_text))
        .wrap(Wrap { trim: true });
    frame.render_widget(message, chunks[0]);
    
    // Render help text
    let help_text = Paragraph::new("Press Enter, Esc, or Space to close")
        .style(Style::default().fg(colors.default_text));
    frame.render_widget(help_text, chunks[1]);
}
