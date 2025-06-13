use crate::model::TaskGraph;
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let task_graph = load_task_graph(&state.project_root)?;

    let task = task_graph
        .tasks
        .iter()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found in tasks.yaml", task_id))?;

    println!("Verifying task: [{}] {}", task.id, task.label);

    let criteria = match &task.acceptance_criteria {
        Some(c) => c,
        None => {
            println!("\nNo acceptance criteria defined for this task. Verification skipped.");
            return Ok(());
        }
    };

    let mut all_checks_passed = true;

    for ac in criteria {
        println!("- Checking: {}", ac.description);

        let check_passed = match ac.check_type.as_str() {
            "file_exists" => {
                let file_path = state.project_root.join(&ac.file);
                check_file_exists(&file_path)?
            }
            "text_check" => {
                let file_path = state.project_root.join(&ac.file);
                if let Some(value) = &ac.value {
                    check_text_contains(&file_path, value)?
                } else {
                    println!("  \x1b[33mWARN: 'text_check' is missing the 'value' field. Check fails.\x1b[0m");
                    false
                }
            }
            _ => {
                println!("  \x1b[33mSKIPPED (unknown check type: '{}')\x1b[0m", ac.check_type);
                true // Skipped checks are considered passing as per requirements.
            }
        };

        if check_passed {
            println!("  \x1b[32mPASS\x1b[0m");
        } else {
            println!("  \x1b[31mFAIL\x1b[0m");
            all_checks_passed = false;
        }
    }

    if !all_checks_passed {
        anyhow::bail!("Some acceptance criteria were not met.");
    } else {
        println!("\nAll checks passed!");
    }

    Ok(())
}

/// Checks if a file exists at the given path.
/// Returns Ok(true) if it exists, Ok(false) otherwise.
fn check_file_exists(file_path: &Path) -> Result<bool> {
    Ok(file_path.exists())
}

/// Checks if a file contains the given text.
/// Returns Ok(true) if the file exists and contains the text, Ok(false) otherwise.
/// If the file doesn't exist, it's considered not to contain the text.
fn check_text_contains(file_path: &Path, text_to_find: &str) -> Result<bool> {
    if !file_path.exists() {
        return Ok(false);
    }
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file for text check: {:?}", file_path))?;
    Ok(content.contains(text_to_find))
}

// This function is duplicated from context.rs.
// A future refactoring task would be to move this to a shared 'loader' module.
fn load_task_graph(project_root: &Path) -> Result<TaskGraph> {
    let tasks_path = project_root.join("docs/state/tasks.yaml");
    let content = fs::read_to_string(&tasks_path)
        .with_context(|| format!("Failed to read task graph at {:?}", tasks_path))?;
    let graph: TaskGraph = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML from {:?}", tasks_path))?;
    Ok(graph)
}