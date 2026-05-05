// src/storage.rs

use crate::task::Task;
use std::error::Error;
use std::fs;

const FILE: &str = "tasks.json";

// App functions
pub fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    load_tasks_from(FILE)
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    save_tasks_to(FILE, tasks)
}

// Testable versions
pub fn load_tasks_from(path: &str) -> Result<Vec<Task>, Box<dyn Error>> {
    let data = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Ok(vec![]),
    };

    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save_tasks_to(path: &str, tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(path, data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FILE: &str = "test_tasks.json";

    fn cleanup() {
        let _ = fs::remove_file(TEST_FILE);
    }

    #[test]
    fn test_save_and_load_tasks() {
        cleanup();

        let tasks = vec![
            Task {
                id: 1,
                title: String::from("Learn Rust"),
                completed: false,
                created_at: String::from("2026-04-26 10:00:00"),
            }
        ];

        save_tasks_to(TEST_FILE, &tasks).unwrap();

        let loaded = load_tasks_from(TEST_FILE).unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].title, "Learn Rust");

        cleanup();
    }

    #[test]
    fn test_empty_tasks() {
        cleanup();

        let tasks: Vec<Task> = vec![];

        save_tasks_to(TEST_FILE, &tasks).unwrap();

        let loaded = load_tasks_from(TEST_FILE).unwrap();

        assert_eq!(loaded.len(), 0);

        cleanup();
    }
}