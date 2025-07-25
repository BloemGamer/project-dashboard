use ratatui::widgets::ListState;
use tabled;
use clap;
use toml;

use crate::{files, structs::Priority};


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Tasks
{
    pub tasks: Vec<Task>,   

    #[serde(skip)]
    pub list_state: ListState,
}

fn priority_default() -> Priority { Priority::Low }
fn description_default() -> String { String::new() }

#[derive(Debug, serde::Deserialize, serde::Serialize, tabled::Tabled, clap::Parser, Clone)]
pub struct Task
{
    #[tabled{rename = "Task"}]
    pub task: String,

    #[serde(default = "priority_default")]
    #[tabled{rename = "Priority"}]
    pub priority: Priority,

    #[serde(default = "description_default")]
    #[tabled{rename = "Description"}]
    pub description: String,
}

#[allow(dead_code)]
impl Priority
{
    fn color(&self) -> tabled::settings::Color
    {
        use tabled::settings::Color;
        match self
        {
            Priority::Low => Color::FG_GREEN,
            Priority::Medium => Color::FG_YELLOW,
            Priority::High => Color::FG_RED,
        }
    }
}

// Writing the new tasks to the file, and replacing the whole file
pub fn write_tasks(tasks: &Tasks)
{
    let path = generate_path!(files::base_path(), tasks);

    let toml_str = toml::to_string_pretty(&tasks)
        .expect("Failed to serialize tasks");

    std::fs::write(&path, toml_str).expect("Failed to write TOML file");
}
