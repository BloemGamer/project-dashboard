use std::str::FromStr;
use bitflags::bitflags;

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

bitflags!
{
    #[derive(Debug, Clone)]
    pub struct TasksCli: u8
    {
        const LOW    = 0b0001;
        const MEDIUM = 0b0010;
        const HIGH   = 0b0100;
        const NONE   = 0b1000;
    }
}

impl FromStr for TasksCli
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut flags = TasksCli::empty();
        for part in s.split(',')
        {
            match part.trim().to_lowercase().as_str()
            {
                "low" => flags |= TasksCli::LOW,
                "medium" => flags |= TasksCli::MEDIUM,
                "high" => flags |= TasksCli::HIGH,
                "none" => flags |= TasksCli::NONE,
                unknown => return Err(format!("Invalid level: {}", unknown)),
            }
        }
        Ok(flags)
    }
}
