// File: docs/cli/src/next.rs
use crate::loader::load_task_graph;
use crate::model::Task;
use crate::AppState;
use anyhow::Result;
use std::collections::HashSet;

pub fn run(state: &AppState) -> Result<()> {
    let available_tasks = get_next_tasks(state)?;

    if available_tasks.is_empty() {
        println!("No available tasks. All tasks are either completed or blocked.");
    } else {
        println!("Available tasks:");
        for task in available_tasks {
            println!("- [{}] {}", task.id, task.label);
        }
    }

    Ok(())
}

pub fn get_next_tasks(state: &AppState) -> Result<Vec<Task>> {
    let graph = load_task_graph(&state.project_root)?;

    let completed_ids: HashSet<_> = graph
        .tasks
        .iter()
        .filter(|t| t.status == "completed")
        .map(|t| t.id.as_str())
        .collect();

    // Find tasks that are 'pending' or 'todo' and whose dependencies are all met.
    let available_tasks: Vec<Task> = graph
        .tasks
        .iter()
        .filter(|task| {
            // THE FIX: Accept both "pending" and "todo" as valid startable statuses.
            if task.status != "pending" && task.status != "todo" {
                return false;
            }
            task.needs.iter().all(|dep_id| completed_ids.contains(dep_id.as_str()))
        })
        .cloned()
        .collect();

    Ok(available_tasks)
}