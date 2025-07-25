use std::fmt;
use clap::{self, Parser};
use serde;
use tabled;

use crate::{commands::{
    tasks,
}};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "A tool for checking and keeping track of your project")]
#[allow(dead_code)]
pub struct Cli
{
    //#[arg(short, long, num_args(0..=1), default_missing_value = "none")]
    //pub tasks: Option<tasks::TasksCli>,

}

// All data that should be found in the TOML files
#[derive(Debug, serde::Deserialize, Default)]
pub struct Data
{
    pub tasks: Option<tasks::Tasks>,
    //overview: Option<Overview>,
}

// for the priority for the tasks
#[derive(Debug, serde::Deserialize, serde::Serialize, tabled::Tabled, Clone, clap::ValueEnum)]
pub enum Priority
{
    High,
    Medium,
    Low,
}

// For printing Priority
impl fmt::Display for Priority
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let s = match self
        {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };
        write!(f, "{}", s)
    }
}


impl Data 
{
    pub fn new() -> Self 
    {
        Self { tasks: None }
    }
}

