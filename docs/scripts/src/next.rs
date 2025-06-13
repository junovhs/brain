// CORRECTED FILE: docs/scripts/src/next.rs

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

    // Find tasks that are 'pending' and whose dependencies are all met.
    let available_tasks: Vec<Task> = graph
        .tasks
        .iter() // <-- THE FIX: Use .iter() to borrow, not .into_iter() to move.
        .filter(|task| {
            if task.status != "pending" {
                return false;
            }
            task.needs.iter().all(|dep_id| completed_ids.contains(dep_id.as_str()))
        })
        .cloned() // Since we are borrowing, we need to .clone() the tasks to create a new Vec.
        .collect();

    Ok(available_tasks)
}