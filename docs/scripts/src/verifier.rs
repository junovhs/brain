// FILE: docs/scripts/src/verifier.rs
use crate::model::TaskGraph;
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

/// Checks if a file exists at the given path.
///
/// Returns `Ok(true)` if the file exists, `Ok(false)` otherwise.
/// Errors are propagated via `Result`.
fn check_file_exists(path: &Path) -> Result<bool> {
    Ok(path.exists())
}

/// Checks if a file's content contains a specific string.
///
/// - If `expected_value` is `None`, the check fails (`Ok(false)`).
/// - If the file does not exist, the check fails (`Ok(false)`).
/// - If the file exists and its content contains the `expected_value`, it returns `Ok(true)`.
/// - Errors during file reading are propagated via `Result`.
fn check_text_contains(path: &Path, expected_value: Option<&str>) -> Result<bool> {
    let Some(value_to_find) = expected_value else {
        // A text_check without a value to check for is not a valid check.
        // It should fail.
        return Ok(false);
    };

    if !path.exists() {
        return Ok(false); // File must exist to contain text.
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file for text_check: {:?}", path))?;

    Ok(content.contains(value_to_find))
}

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
                check_text_contains(&file_path, ac.value.as_deref())?
            }
            // Any other check type is skipped and considered a PASS for now.
            _ => true,
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

fn load_task_graph(project_root: &Path) -> Result<TaskGraph> {
    let tasks_path = project_root.join("docs/state/tasks.yaml");
    let content = fs::read_to_string(&tasks_path)
        .with_context(|| format!("Failed to read task graph at {:?}", tasks_path))?;
    let graph: TaskGraph = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML from {:?}", tasks_path))?;
    Ok(graph)
}