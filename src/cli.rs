// src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "task",
    version = "1.0",
    about = "A simple command-line task manager built with Rust",
    long_about = "Manage your tasks from the terminal using Rust."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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