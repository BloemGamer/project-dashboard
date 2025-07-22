use ratatui::{
    DefaultTerminal,
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

pub enum TuiState
{
    Tasks,
    Exit,
}

pub fn start()
{
    color_eyre::install().unwrap();
}

pub fn run(mut terminal: DefaultTerminal, data: &mut Data)
{
    let mut state: TuiState = TuiState::Tasks;
    'main_render_loop: loop
    {
        state = match state 
        {
            TuiState::Tasks => tasks::run(&mut terminal, data),

            TuiState::Exit => break 'main_render_loop,
        }
    }
    commands::tasks::write_tasks(data.tasks.as_ref().unwrap());
}

