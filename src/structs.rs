use clap::Parser;
use serde;

use crate::{commands::{
    tasks,
}};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "A tool for checking and keeping track of your project")]
pub struct Cli
{
    #[arg(short, long, default_value_t = false)]
    pub tasks: bool,

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


