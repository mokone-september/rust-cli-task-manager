// src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "task",
    version = "1.0",
    about = "A simple command-line task manager built with Rust"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { title: String },
    List {
        #[arg(long)]
        completed: bool,
    },
    Done { id: u32 },
    Delete { id: u32 },
}