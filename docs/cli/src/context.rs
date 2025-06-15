// CORRECTED FILE: docs/scripts/src/context.rs

use crate::loader::load_task_graph;
use crate::sketch;
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use std::fs;

pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let task_graph = load_task_graph(&state.project_root)?;
    let task = task_graph
        .tasks
        .iter()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found", task_id))?;

    // --- Task Header ---
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
    println!("\n---\n");

    // --- Intelligent Context Generation (The New Way) ---
    if let Some(query) = &task.context_query {
        // Pass the AppState to the sketch generator
        let context_package = sketch::generate_context_package(state, query)
            .with_context(|| "Failed to generate intelligent context package")?;
        println!("{}", context_package);
    
    // --- Legacy File-Based Context (The Old Way) ---
    } else if let Some(files) = &task.context_files {
        println!("// LEGACY CONTEXT (static file list)\n");
        for file_path in files {
            println!("// FILE: {}", file_path);
            let full_path = state.project_root.join(file_path);
            match fs::read_to_string(&full_path) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("// Error reading file {:?}: {}", full_path, e),
            }
            println!("\n---\n");
        }
    } else {
        println!("// No context query or context files specified for this task.");
    }

    Ok(())
}