use std::{env, fs::{self, File}, path::{PathBuf}};
use toml;

use crate::{
    Data,
    structs,
};


pub fn check_dir_valid() -> Result<PathBuf, ()>
{
    let path: PathBuf = env::current_dir().expect("couldn't open path");

    if !path.exists()
    {
        eprintln!("Couldn't find or open path");
        todo!();
    }
    let mut dashboard_path: PathBuf = path.clone();
    dashboard_path.push(".dashboard");
    if !dashboard_path.exists()
    {
        match fs::create_dir(".dashboard")
        {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Couldn't make dashboard directory");
                todo!();
            }
        }
    }
    return Ok(path)
}



pub fn read_data() -> Data
{
    let dashboard_path: PathBuf = base_path();
    let mut data: Data = Data::new();

    get_files!(
        dashboard_path,
        data,
        tasks => commands::tasks::Tasks => None,
        settings => structs::Settings => structs::Settings::new(),
    );
    return data
}



pub fn base_path() -> PathBuf
{
    let mut dashboard_path: PathBuf = std::env::current_dir().unwrap();
    dashboard_path.push(".dashboard");
    dashboard_path
}

pub fn ensure_file_exists(path: &PathBuf) -> Result<(), ()>
{
    if !path.exists()
    {
        match File::create(path)
        {
            Ok(_) => return Ok(()),
            Err(_) => return Err(()),
        }
    }
    Ok(())
}
