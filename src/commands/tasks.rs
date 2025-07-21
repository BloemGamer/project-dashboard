use std::{fmt, io::{self, Write}, str::FromStr};
use bitflags::bitflags;
use tabled;
use clap;
use toml;

use crate::{files, structs::Priority};


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Tasks
{
    pub tasks: Vec<Task>,   
}

fn priority_default() -> Priority { Priority::Low }
fn explanation_default() -> String { String::new() }

#[derive(Debug, serde::Deserialize, serde::Serialize, tabled::Tabled, clap::Parser, Clone)]
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

pub fn run(tasks: &mut Tasks, tasks_cli: &TasksCli)
{
    if tasks_cli.intersects(TasksCli::HIGH | TasksCli::MEDIUM | TasksCli::LOW | TasksCli::NONE)
    {
        show(tasks, tasks_cli);
    }
    if tasks_cli.intersects(TasksCli::ADD)
    {
        add_task_cli(tasks);
    }
    if tasks_cli.intersects(TasksCli::REMOVE)
    {
        remove_task_cli(tasks);
    }
    if tasks_cli.intersects(TasksCli::REMOVE | TasksCli::ADD)
    {
        write_tasks(&tasks);
    }
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

    // For only showing the asked priority
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
    print_task(&all_sorted);
}

fn print_task(tasks: &Vec<&Task>)
{
    let mut table: tabled::Table = tabled::Table::new(tasks);
    table.with(tabled::settings::Style::rounded());

    use tabled::settings::{Modify, object::Cell};

    // Adding colors for the prority
    for (i, task) in tasks.iter().enumerate()
    {
        table.with(Modify::new(Cell::new(i + 1, 1)).with(task.priority.color()));
    }

    println!("{}", table);
}

// Writing the new tasks to the file, and replacing the whole file
fn write_tasks(tasks: &Tasks)
{
    let path = generate_path!(files::base_path(), tasks);

    let toml_str = toml::to_string_pretty(&tasks)
        .expect("Failed to serialize tasks");

    std::fs::write(&path, toml_str).expect("Failed to write TOML file");
}

#[derive(clap::Parser, Debug)]
pub struct InputTaskArgs
{
    #[arg(short = 't', long)]
    pub task: String,

    #[arg(short = 'p', long, default_value = "Low")]
    pub priority: Priority,

    #[arg(short = 'e', long, default_value = "")]
    pub explanation: String,
}

impl From<InputTaskArgs> for Task
{
    fn from(args: InputTaskArgs) -> Self
    {
        Task
        {
            task: args.task,
            priority: args.priority,
            explanation: args.explanation,
        }
    }
}

fn add_task_cli(tasks_file: &mut Tasks)
{
    use clap::Parser;
    let stdin = io::stdin();
    loop
    {
        println!("{}", "Enter new task");

        // Read one line of input
        let mut input: String = String::new();
        if stdin.read_line(&mut input).is_err()
        {
            eprintln!("Failed to read input.");
            continue;
        }
        let input = input.trim();
        if input.is_empty()
        {
            eprintln!("Input was empty. Try again.");
            continue;
        }

        // Parse args
        let parsed_args = match shell_words::split(input)
        {
            Ok(args) => args,
            Err(e) => {
                eprintln!("Parsing error: {}", e);
                continue;
            }
        };

        let args = std::iter::once("stdin-app")
            .chain(parsed_args.iter().map(String::as_str));

        match InputTaskArgs::try_parse_from(args)
        {
            Ok(cli_args) =>
            {
                let task: Task = cli_args.into();
                tasks_file.tasks.push(task);
                println!("Task added!");
            }
            Err(e) =>
            {
                eprintln!("Failed to parse task input: {}", e);
                continue;
            }
        }

        // Ask if they want to add another
        println!("Do you want to add another task? (Y/n): ");

        let mut response = String::new();
        if stdin.read_line(&mut response).is_err()
        {
            eprintln!("Error reading response.");
            break;
        }

        let response = response.trim().to_lowercase();
        if response == "n" || response == "no"
        {
            println!("Finished adding tasks.");
            break;
        }
    }
    //return output_tasks
}

fn remove_task_cli(tasks: &mut Tasks)
{
    let stdin = io::stdin();

    loop
    {
        if tasks.tasks.is_empty()
        {
            println!("No tasks left to remove.");
            break;
        }

        show(&tasks, &TasksCli::NONE);

        // Ask for task name
        println!("Enter the exact name of the task you want to remove:");

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }
        let task_name = input.trim();

        // Find task by name
        let position = tasks.tasks.iter().position(|t| t.task == task_name);
        match position
        {
            Some(index) =>
            {
                // Confirm deletion
                println!("Are you sure you want to remove \"{}\"? (Y/n):", task_name);

                let mut confirm = String::new();
                if stdin.read_line(&mut confirm).is_err()
                {
                    eprintln!("Failed to read confirmation.");
                    continue;
                }

                if confirm.trim().to_lowercase().starts_with('n')
                {
                    println!("Task not removed.");
                } else {
                    println!("Task \"{}\" removed.", task_name);
                    tasks.tasks.remove(index);
                }
            }
            None =>
            {
                println!("No task found with name \"{}\".", task_name);
            }
        }

        // Ask if they want to remove another
        println!("Remove another task? (Y/n):");

        let mut again = String::new();
        if stdin.read_line(&mut again).is_err()
        {
            break;
        }

        let again = again.trim().to_lowercase();
        if again == "n" || again == "no"
        {
            println!("Finished removing tasks.");
            break;
        }
    }
}
