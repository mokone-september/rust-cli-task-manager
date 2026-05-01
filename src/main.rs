// src/main.rs

mod cli;
mod storage;
mod task;

use chrono::Local;
use clap::Parser;
use cli::{Cli, Commands};
use storage::{load_tasks, save_tasks};
use task::Task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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