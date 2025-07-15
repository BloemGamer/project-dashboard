use std::{str::FromStr, fmt};
use bitflags::bitflags;
use tabled;
use clap;
use toml_edit::{Document, DocumentMut};

use crate::{files, structs::Priority};


#[derive(Debug, serde::Deserialize)]
pub struct Tasks
{
    pub tasks: Vec<Task>,   
}

fn priority_default() -> Priority { Priority::Low }
fn explanation_default() -> String { String::new() }

#[derive(Debug, serde::Deserialize, tabled::Tabled, clap::Parser)]
pub struct Task
{
    #[tabled{rename = "Task"}]
    pub task: String,

    #[serde(default = "priority_default")]
    #[tabled{rename = "Priority"}]
    pub priority: Priority,
    #[serde(default = "explanation_default")]
    #[tabled{rename = "Explanation"}]
    pub explanation: String,
}

pub fn run(tasks: &Tasks, tasks_cli: &TasksCli)
{
    show(tasks, tasks_cli);
    if tasks_cli.intersects(TasksCli::ADD)
    {
        add(tasks, tasks_cli)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };
        write!(f, "{}", s)
    }
}

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

bitflags!
{
    #[derive(Debug, Clone)]
    pub struct TasksCli: u8
    {
        const LOW       = 0b000001;
        const MEDIUM    = 0b000010;
        const HIGH      = 0b000100;
        const NONE      = 0b001000;
        const ADD       = 0b010000;
        const REMOVE    = 0b100000;
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
                "mid" => flags |= TasksCli::MEDIUM,
                "high" => flags |= TasksCli::HIGH,
                "none" => flags |= TasksCli::NONE,
                "add" => flags |= TasksCli::ADD,
                "remove" => flags |= TasksCli::REMOVE,
                unknown => return Err(format!("Invalid level: {}", unknown)),
            }
        }
        Ok(flags)
    }
}

fn show(tasks: &Tasks, tasks_cli: &TasksCli)
{
    let mut high: Vec<&Task> = Vec::new();
    let mut medium: Vec<&Task> = Vec::new();
    let mut low: Vec<&Task> = Vec::new();

    for task in &tasks.tasks
    {
        match task.priority
        {
            Priority::High => 
            {
                if tasks_cli.intersects(TasksCli::HIGH | TasksCli::NONE)
                {
                    high.push(task);
                }
            }
            Priority::Medium => 
            {
                if tasks_cli.intersects(TasksCli::MEDIUM | TasksCli::NONE)
                {
                    medium.push(task);
                }
            }
            Priority::Low => 
            {
                if tasks_cli.intersects(TasksCli::LOW | TasksCli::NONE)
                {
                    low.push(task);
                }
            }
        }
    }

    let mut all_sorted: Vec<&Task> = Vec::new();
    all_sorted.extend(high);
    all_sorted.extend(medium);
    all_sorted.extend(low);
    if !all_sorted.is_empty()
    {
        print_task(&all_sorted);
    }
}

fn print_task(tasks: &Vec<&Task>)
{
    let mut table: tabled::Table = tabled::Table::new(tasks);
    table.with(tabled::settings::Style::rounded());

    use tabled::settings::{Modify, object::Cell};
    for (i, task) in tasks.iter().enumerate()
    {
        table.with(Modify::new(Cell::new(i + 1, 1)).with(task.priority.color()));
    }

    println!("{}", table);
}

fn add(tasks: &Tasks, tasks_cli: &TasksCli)
{
    let path = generate_path!(files::base_path(), tasks);

    let toml_str: String = std::fs::read_to_string(&path).unwrap();
    let mut doc: DocumentMut = toml_str.parse().unwrap();

    let tasks = doc["tasks"].as_array_of_tables_mut()
        .expect("`tasks` should be an array of tables");

    let mut new_task = toml_edit::Table::new();
    new_task["task"] = toml_edit::value("test8");
    new_task["priority"] = toml_edit::value("Medium");

    tasks.push(new_task);

    // Save back to file
    std::fs::write(&path, doc.to_string()).unwrap();

}
