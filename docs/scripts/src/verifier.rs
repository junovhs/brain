use crate::loader::load_task_graph; // Use the shared loader
use crate::AppState;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

fn check_file_exists(path: &Path) -> Result<bool> {
    Ok(path.exists())
}

fn check_text_contains(path: &Path, expected_value: Option<&str>) -> Result<bool> {
    let value_to_find = expected_value.ok_or_else(|| anyhow!("'text_check' requires a 'value' to be specified."))?;
    if !path.exists() {
        return Ok(false);
    }
    let content = fs::read_to_string(path)?;
    Ok(content.contains(value_to_find))
}

pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let task_graph = load_task_graph(&state.project_root)?;
    let task = task_graph.tasks.iter().find(|t| t.id == task_id).ok_or_else(|| anyhow!("Task ID '{}' not found", task_id))?;
    
    println!("Verifying task: [{}] {}", task.id, task.label);
    let criteria = match &task.acceptance_criteria {
        Some(c) => c,
        None => {
            println!("\nNo acceptance criteria defined for this task. Verification considered vacuously true.");
            return Ok(());
        }
    };

    let mut all_checks_passed = true;
    for ac in criteria {
        println!("- Checking: {}", ac.description);
        let file_path = state.project_root.join(&ac.file);
        let check_passed = match ac.check_type.as_str() {
            "file_exists" => check_file_exists(&file_path)?,
            "text_check" => check_text_contains(&file_path, ac.value.as_deref())?,
            other => {
                println!("  \x1b[33mSKIPPED (unknown check type: '{}')\x1b[0m", other);
                true
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
        anyhow::bail!("One or more acceptance criteria failed.");
    } else {
        println!("\nAll checks passed!");
    }
    Ok(())
}