
use crate::structs::{
    Priority,
};


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
