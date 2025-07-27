use std::fmt;
use clap::{self, Parser};
use serde;

use crate::{commands::tasks, tui::TuiColor};

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
    pub settings: Settings,
    pub tasks: Option<tasks::Tasks>,
    //overview: Option<Overview>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct Settings
{
    pub colors: TuiColor,
}

// for the priority for the tasks
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, clap::ValueEnum)]
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

impl Priority
{
    pub fn next(&mut self)
    {
        *self = match self
        {
            Priority::High => Priority::Low,
            Priority::Medium => Priority::High,
            Priority::Low => Priority::Medium,
        }
    }
    
    pub fn previous(&mut self)
    {
        *self = match self
        {
            Priority::Low => Priority::High,
            Priority::High => Priority::Medium,
            Priority::Medium => Priority::Low,
        }
    }
}

impl Data 
{
    pub fn new() -> Self 
    {
        Self { 
            tasks: None,
            settings: Settings::new(),
        }
    }
}

impl Settings
{
    pub fn new() -> Self
    {
        Self { colors: TuiColor { selected: TuiColor::SELECTED, default_text: TuiColor::DEFAULT_TEXT }, }
    }
}

