use std::{env, fs, path::PathBuf};
use tokio;
use toml;

use crate::{
    Data,
    DataEnum,
    Cli,
    Tasks,
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



pub fn read_data(cli: &Cli) -> Data
{
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(load_data_from_cli(cli))
}


async fn load_data_from_cli(cli: &Cli) -> Data
{
    let mut dashboard_path = std::env::current_dir().unwrap();
    dashboard_path.push(".dashboard");

    let futures = generate_load_futures!(
        cli,
        dashboard_path,
        tasks => Tasks => Tasks,
    );

    let results = futures::future::join_all(futures).await;

    let mut data = Data::default();
    for result in results {
        match result {
            Ok(DataEnum::Tasks(t)) => data.tasks = Some(t),
            Err(e) => eprintln!("⚠️ {}", e),
        }
    }

    data
}
