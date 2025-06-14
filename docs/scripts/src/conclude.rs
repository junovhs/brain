// FILE: docs/scripts/src/conclude.rs

use crate::model::TaskGraph;
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use std::fs;

/// Automates marking a task as 'completed' in the tasks.yaml file.
pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let tasks_path = state.project_root.join("docs/state/tasks.yaml");

    // 1. Read the YAML file
    let content = fs::read_to_string(&tasks_path)
        .with_context(|| format!("Failed to read task graph at {:?}", tasks_path))?;

    // 2. Deserialize into the TaskGraph struct
    let mut graph: TaskGraph = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML from {:?}", tasks_path))?;

    // 3. Find the task and update its status
    let task = graph
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found in tasks.yaml", task_id))?;

    task.status = "completed".to_string();
    println!(
        "Updated status for task ['{}'] to 'completed'.",
        task.id
    );

    // 4. Serialize the updated TaskGraph back to a YAML string
    let updated_content = serde_yaml::to_string(&graph)
        .with_context(|| "Failed to serialize updated task graph back to YAML")?;

    // 5. Write the new content back to the file
    fs::write(&tasks_path, updated_content)
        .with_context(|| format!("Failed to write updated content to {:?}", tasks_path))?;

    println!("Successfully saved changes to tasks.yaml.");

    Ok(())
}