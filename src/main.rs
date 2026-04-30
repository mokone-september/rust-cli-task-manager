// src/main.rs

mod task;

use chrono::Local;
use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use task::Task;

const FILE: &str = "tasks.json";

#[derive(Parser)]
#[command(
    name = "task",
    version = "1.0",
    about = "A simple command-line task manager built with Rust",
    long_about = "Manage your tasks from the terminal using Rust."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        title: String,
    },

    /// List tasks
    List {
        #[arg(long)]
        completed: bool,
    },

    /// Mark task as completed
    Done {
        id: u32,
    },

    /// Delete a task
    Delete {
        id: u32,
    },
}

fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let data = match fs::read_to_string(FILE) {
        Ok(content) => content,
        Err(_) => return Ok(vec![]),
    };

    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE, data)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut tasks = load_tasks()?;

    match cli.command {
        Commands::Add { title } => {
            let id = tasks.last().map_or(1, |task| task.id + 1);

            let task = Task {
                id,
                title,
                completed: false,
                created_at: Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            };

            tasks.push(task);

            save_tasks(&tasks)?;

            println!("✅ Task added successfully.");
        }

        Commands::List { completed } => {
            if tasks.is_empty() {
                println!("📭 No tasks found.");
            } else {
                println!("\n📋 Task List\n");

                for task in tasks {
                    if completed && !task.completed {
                        continue;
                    }

                    let status = if task.completed { "✔" } else { "✘" };

                    println!(
                        "{} [{}] {} ({})",
                        task.id,
                        status,
                        task.title,
                        task.created_at
                    );
                }
            }
        }

        Commands::Done { id } => {
            let mut found = false;

            for task in &mut tasks {
                if task.id == id {
                    task.completed = true;
                    found = true;
                }
            }

            if found {
                save_tasks(&tasks)?;
                println!("✅ Task marked as completed.");
            } else {
                println!("❌ Task not found.");
            }
        }

        Commands::Delete { id } => {
            let original_length = tasks.len();

            tasks.retain(|task| task.id != id);

            if tasks.len() < original_length {
                save_tasks(&tasks)?;
                println!("🗑️ Task deleted successfully.");
            } else {
                println!("❌ Task not found.");
            }
        }
    }

    Ok(())
}