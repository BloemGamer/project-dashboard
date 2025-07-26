use ratatui::widgets::ListState;
use clap;
use toml;

use crate::{
    files,
    structs::Priority,
};


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Tasks
{
    pub tasks: Vec<Task>,   

    #[serde(skip)]
    pub list_state: ListState,
}

fn priority_default() -> Priority { Priority::Low }
fn description_default() -> String { String::new() }

#[derive(Debug, serde::Deserialize, serde::Serialize, clap::Parser, Clone)]
pub struct Task
{
    pub task: String,

    #[serde(default = "priority_default")]
    pub priority: Priority,

    #[serde(default = "description_default")]
    pub description: String,
}

// Writing the new tasks to the file, and replacing the whole file
pub fn write_tasks(tasks: &Tasks)
{
    let path = generate_path!(files::base_path(), tasks);

    let toml_str = toml::to_string_pretty(&tasks)
        .expect("Failed to serialize tasks");

    std::fs::write(&path, toml_str).expect("Failed to write TOML file");
}
