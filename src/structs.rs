use clap::Parser;
use serde;

use crate::{commands::{
    tasks,
}};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "A tool for checking and keeping track of your project")]
pub struct Cli
{
    #[arg(short, long, num_args(0..=1), default_missing_value = "none")]
    pub tasks: Option<tasks::TasksCli>,

}

generate_data_enum!
{
    #[derive(Debug, serde::Deserialize, Default)]
    pub struct Data
    {
        pub tasks: Option<tasks::Tasks> => Tasks,
        //overview: Option<Overview>,
    }
}

#[derive(Debug, serde::Deserialize)]
pub enum Priority
{
    High,
    Medium,
    Low,
}

