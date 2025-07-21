use std::{path::PathBuf};
use clap::Parser;
// https://docs.rs/clap/latest/clap/

#[macro_use]
mod r#macro;
mod files;
mod structs;
mod commands;

use structs::{
    Data,
    Cli,
};

use commands::{
    tasks,
};




fn main()
{
    let cli: Cli = Cli::parse();
    let path: PathBuf = files::check_dir_valid().expect("failed in checking the dirs");
    println!("Hello, world! {}", path.display());

    let data: Data = files::read_data(&cli);

    if cli.tasks.is_some()
    {
        tasks::run(&mut data.tasks.unwrap(), &cli.tasks.unwrap());
    }
}

