use std::{path::PathBuf, panic};

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
    
    let mut data: Data = files::read_data();

    let terminal: DefaultTerminal = ratatui::init();
    tui::start();
    tui::run(terminal, &mut data);
    ratatui::restore();
}

fn set_panic_function()
{
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info|
        {
            // Restore the terminal
            let _ = ratatui::restore();
            // Call the original panic hook to preserve default panic behavior
            original_hook(panic_info);
        }));
}
