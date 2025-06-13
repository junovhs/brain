// docs/scripts/src/verifier.rs (Final, Instrumented Version)

use crate::loader::load_task_graph;
use crate::model::AcceptanceCriterion;
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

/// Executes the verification process for a given task.
pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let task_graph = load_task_graph(&state.project_root)?;

    let task = task_graph
        .tasks
        .iter()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found in tasks.yaml", task_id))?;

    // =================================================================
    // DEBUG LOGGING
    // =================================================================
    println!("\n--- DEBUG: Parsed Task Data ---\n{:#?}\n-----------------------------\n", task);

    println!("Verifying task: [{}] {}", task.id, task.label);

    let criteria = match &task.acceptance_criteria {
        Some(c) if !c.is_empty() => c,
        _ => {
            if task.status == "pending" {
                 anyhow::bail!(
                    "FATAL: Task '{}' is pending but has no acceptance criteria. The plan is incomplete or a parsing error occurred.",
                    task.id
                );
            }
            println!("\nNo acceptance criteria to verify for this task (status: {}). Verification skipped.", task.status);
            return Ok(());
        }
    };

    let mut all_checks_passed = true;
    for ac in criteria {
        println!("- Checking: {}", ac.description);
        let check_result = perform_check(state, ac);

        match check_result {
            Ok(true) => {
                println!("  \x1b[32mPASS\x1b[0m");
            }
            Ok(false) => {
                println!("  \x1b[31mFAIL\x1b[0m");
                all_checks_passed = false;
            }
            Err(e) => {
                println!("  \x1b[31mERROR: Check failed to execute: {:?}\x1b[0m", e);
                all_checks_passed = false;
            }
        }
    }

    if !all_checks_passed {
        anyhow::bail!("One or more acceptance criteria failed.");
    } else {
        println!("\nAll checks passed!");
    }

    Ok(())
}

/// Dispatches a check based on the AcceptanceCriterion type.
fn perform_check(state: &AppState, criterion: &AcceptanceCriterion) -> Result<bool> {
    let full_path = state.project_root.join(&criterion.file);

    match criterion.check_type.as_str() {
        "file_exists" => check_file_exists(&full_path),
        "text_check" => check_text_contains(&full_path, criterion.value.as_deref()),
        other => {
            println!("  \x1b[33mSKIPPED (unknown check type: '{}')\x1b[0m", other);
            Ok(true)
        }
    }
}

/// Checks if a file exists at the given path.
fn check_file_exists(path: &Path) -> Result<bool> {
    Ok(path.exists())
}

/// Checks if a file's content contains a specific string.
fn check_text_contains(path: &Path, expected_value: Option<&str>) -> Result<bool> {
    let value_to_find = expected_value.ok_or_else(|| {
        anyhow!("'text_check' for file {:?} requires a 'value' field.", path)
    })?;

    if !path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file for text_check: {:?}", path))?;

    Ok(content.contains(value_to_find))
}