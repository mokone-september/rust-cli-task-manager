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