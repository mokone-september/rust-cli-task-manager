mod task;

use clap::{Parser, Subcommand};
use task::Task;
use std::fs;

const FILE: &str = "tasks.json";

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string(FILE).unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or(vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE, data).unwrap();
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { title } => {
            let id = tasks.len() as u32 + 1;
            let task = Task {
                id,
                title,
                completed: false,
            };
            tasks.push(task);
            save_tasks(&tasks);
            println!("Task added.");
        }

        Commands::List => {
            for task in tasks {
                let status = if task.completed { "✔" } else { "✘" };
                println!("{} [{}] {}", task.id, status, task.title);
            }
        }

        Commands::Done { id } => {
            for task in &mut tasks {
                if task.id == id {
                    task.completed = true;
                }
            }
            save_tasks(&tasks);
            println!("Task marked as done.");
        }

        Commands::Delete { id } => {
            tasks.retain(|t| t.id != id);
            save_tasks(&tasks);
            println!("Task deleted.");
        }
    }
}