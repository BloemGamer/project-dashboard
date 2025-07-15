use clap::Parser;
use serde;


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
        pub tasks: Option<Tasks> => Tasks,
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


#[derive(Debug, serde::Deserialize)]
pub struct Tasks
{
    pub tasks: Vec<Task>,   
}

fn priority_default() -> Priority { Priority::Low }
fn explanation_default() -> String { String::new() }

#[derive(Debug, serde::Deserialize)]
pub struct Task
{
    pub task: String,

    #[serde(default = "priority_default")]
    pub priority: Priority,
    #[serde(default = "explanation_default")]
    pub explanation: String,
}
