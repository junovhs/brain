use crate::loader::load_task_graph; // Use the shared loader
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use std::fs;

pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let task_graph = load_task_graph(&state.project_root)?;
    let task = task_graph.tasks.iter().find(|t| t.id == task_id).ok_or_else(|| anyhow!("Task ID '{}' not found", task_id))?;
    // ... (rest of the logic is correct and remains the same)
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
    println!("// FILE: BRAIN.md");
    let brain_md_path = state.project_root.join("BRAIN.md");
    let brain_content = fs::read_to_string(&brain_md_path)
        .with_context(|| format!("Failed to read BRAIN.md at {:?}", brain_md_path))?;
    println!("{}", brain_content);
    println!("\n---\n");
    if let Some(files) = &task.context_files {
        for file_path in files {
            if file_path == "BRAIN.md" { continue; }
            println!("// FILE: {}", file_path);
            let full_path = state.project_root.join(file_path);
            match fs::read_to_string(&full_path) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("// Error reading file {:?}: {}", full_path, e),
            }
            println!("\n---\n");
        }
    }
    Ok(())
}