// CORRECTED FILE: docs/scripts/src/context.rs

use crate::loader::load_task_graph;
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use std::fs;

pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let task_graph = load_task_graph(&state.project_root)?;
    let task = task_graph.tasks.iter().find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found", task_id))?;

    // Print the task header
    println!("// Current Task: [{}] {}", task.id, task.label);
    if let Some(goal) = &task.goal {
        println!("// Goal: {}", goal);
    }
    if let Some(criteria) = &task.acceptance_criteria {
        println!("\n// Acceptance Criteria:");
        for ac in criteria {
            println!("// - {}", ac.description);
        }
    }
    
    // --- START: Refactored File-Printing Logic ---
    if let Some(files) = &task.context_files {
        for file_path in files {
            // Print the separator and header *before* each file.
            println!("\n---\n");
            println!("// FILE: {}", file_path);
            let full_path = state.project_root.join(file_path);
            match fs::read_to_string(&full_path) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("// Error reading file {:?}: {}", full_path, e),
            }
        }
    }
    // The final separator is printed only if there were files.
    if task.context_files.is_some() && !task.context_files.as_ref().unwrap().is_empty() {
        println!("\n---\n");
    }
    // --- END: Refactored File-Printing Logic ---

    Ok(())
}