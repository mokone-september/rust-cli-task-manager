# README.md

# Rust CLI Task Manager 🦀

A simple command-line task manager built with Rust.

---

## Features

- Add tasks
- List tasks
- Mark tasks as completed
- Delete tasks
- Store tasks using JSON
- CLI argument parsing with clap

---

## Installation

Clone the repository:

git clone https://github.com/mokone-september/rust-cli-task-manager.git

Go into the project folder:

cd rust-cli-task-manager

Build the project:

cargo build

---

## Usage

Add a task:

cargo run -- add "Learn Rust"

List tasks:

cargo run -- list

Mark a task as completed:

cargo run -- done 1

Delete a task:

cargo run -- delete 1

Show help:

cargo run -- --help

---

## Screenshot

Add your screenshot inside:

screenshots/demo.png

Then include:

![CLI Demo](screenshots/demo.png)

---

## Concepts Learned

This project helped me learn:

- Structs
- Enums
- Pattern matching
- File I/O
- JSON persistence
- Serialization with serde
- CLI argument parsing

---

## Tech Stack

- Rust
- clap
- serde
- serde_json

---

## Future Improvements

- Better error handling
- Task timestamps
- Filtering completed tasks
- SQLite support
- Colored terminal output
