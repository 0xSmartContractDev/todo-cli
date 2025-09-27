use crate::task::Task;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn default_path() -> PathBuf {
    let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    p.push("todo-cli");
    std::fs::create_dir_all(&p).ok();
    p.push("todos.json");
    p
}

pub fn load(path: &Path) -> Result<Vec<Task>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save(path: &Path, tasks: &[Task]) -> Result<()> {
    let tmp = path.with_extension("tmp");
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write(&tmp, json)?;
    fs::rename(tmp, path)?;
    Ok(())
}
