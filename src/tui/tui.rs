use ratatui::{
    DefaultTerminal,
    style::Color,
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

pub enum TuiState
{
    Tasks(TasksState),
    Exit,
}

pub enum TasksState
{
    Main,
    Adding,
    Exit,
}

pub fn start()
{
    color_eyre::install().unwrap();
}

pub fn run(mut terminal: DefaultTerminal, data: &mut Data)
{
    let mut state: TuiState = TuiState::Tasks(TasksState::Main);
    'main_render_loop: loop
    {
        state = match state 
        {
            TuiState::Tasks(state) => tasks::run(&mut terminal, data, state),

            TuiState::Exit => break 'main_render_loop,
        }
    }
    commands::tasks::write_tasks(data.tasks.as_ref().unwrap());
}

