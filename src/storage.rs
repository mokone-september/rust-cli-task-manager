// src/storage.rs

use crate::task::Task;
use std::error::Error;
use std::fs;

const FILE: &str = "tasks.json";

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let data = match fs::read_to_string(FILE) {
        Ok(content) => content,
        Err(_) => return Ok(vec![]),
    };

    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE, data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_tasks() {
        let tasks = vec![
            Task {
                id: 1,
                title: String::from("Learn Rust"),
                completed: false,
                created_at: String::from("2026-04-26 10:00:00"),
            }
        ];

        let save_result = save_tasks(&tasks);

        assert!(save_result.is_ok());

        let loaded_tasks = load_tasks().unwrap();

        assert_eq!(loaded_tasks.len(), 1);
        assert_eq!(loaded_tasks[0].title, "Learn Rust");
    }

    #[test]
    fn test_empty_tasks() {
        let tasks: Vec<Task> = vec![];

        let save_result = save_tasks(&tasks);

        assert!(save_result.is_ok());

        let loaded_tasks = load_tasks().unwrap();

        assert_eq!(loaded_tasks.len(), 0);
    }
}