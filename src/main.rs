use std::{path::PathBuf};

#[macro_use]
mod r#macro;
mod files;
mod structs;
mod commands;
mod tui;

use ratatui::DefaultTerminal;
use structs::{
    Data,
};

fn main()
{
    //let mut cli: Cli = Cli::parse();
    let path: PathBuf = files::check_dir_valid().expect("failed in checking the dirs");
    println!("Hello, world! {}", path.display());

    //cli.tasks = Some(commands::tasks::TasksCli::NONE);
    
    let mut data: Data = files::read_data();


    //tasks::run(&mut data.tasks.as_mut().unwrap(), &cli.tasks.as_ref().unwrap());

    let terminal: DefaultTerminal = ratatui::init();
    tui::start();
    tui::run(terminal, &mut data);
    ratatui::restore();
}

