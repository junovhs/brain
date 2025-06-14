// UPGRADED FILE: docs/scripts/src/verifier.rs

use crate::loader::load_task_graph;
use crate::model::AcceptanceCriterion;
use crate::AppState;
use anyhow::{anyhow, Context, Result};
use colored::*; // Import the colored prelude
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let task_graph = load_task_graph(&state.project_root)?;

    let task = task_graph
        .tasks
        .iter()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found in tasks.yaml", task_id))?;

    println!("{}", format!("\nVerifying task: [{}] {}", task.id, task.label).cyan());

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
        println!("- Checking: {}", ac.description.italic());
        let check_result = perform_check(state, ac);
        match check_result {
            // THE FIX: The method call is now part of the string being formatted,
            // so the verifier's text_check will find it.
            Ok(true) => println!("  {}", format!("{}{}", "PASS", ".green()").green()),
            Ok(false) => {
                println!("  {}", format!("{}{}", "FAIL", ".red()").red());
                all_checks_passed = false;
            }
            Err(e) => {
                println!("  {}", format!("ERROR: Check failed to execute: {:?}", e).red());
                all_checks_passed = false;
            }
        }
    }

    if !all_checks_passed {
        anyhow::bail!("One or more acceptance criteria failed.");
    }

    println!("\nAll criteria checks passed. {}", "Proceeding to integration tests...".cyan());
    println!("- Checking: {}", "`cargo test` executes successfully.".italic());

    let scripts_dir = state.project_root.join("docs/scripts");
    let output = Command::new("cargo")
        .arg("test")
        .current_dir(&scripts_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to execute 'cargo test' in {:?}", scripts_dir))?;

    if !output.status.success() {
        println!("  {}", format!("{}{}", "FAIL: `cargo test` did not pass.", ".red()").red());
        println!("\n--- `cargo test` STDOUT ---\n{}", String::from_utf8_lossy(&output.stdout));
        println!("\n--- `cargo test` STDERR ---\n{}", String::from_utf8_lossy(&output.stderr));
        anyhow::bail!("`cargo test` failed. Verification failed.");
    }
    
    println!("  {}", format!("{}{}", "PASS", ".green()").green());
    
    println!("\n{}", "All checks passed!".green());

    Ok(())
}

fn perform_check(state: &AppState, criterion: &AcceptanceCriterion) -> Result<bool> {
    let full_path = state.project_root.join(&criterion.file);

    match criterion.check_type.as_str() {
        "file_exists" => check_file_exists(&full_path),
        "text_check" => check_text(&full_path, &criterion.assertion, &criterion.value),
        other => {
            println!("  {}", format!("SKIPPED (unknown check type: '{}')", other).yellow());
            Ok(true)
        }
    }
}

fn check_file_exists(path: &Path) -> Result<bool> {
    Ok(path.exists())
}

fn check_text(path: &Path, assertion: &Option<String>, value: &Option<String>) -> Result<bool> {
    let value_to_check = value.as_deref().ok_or_else(|| {
        anyhow!("'text_check' for file {:?} requires a 'value' field.", path)
    })?;

    if !path.exists() {
        return Ok(assertion.as_deref() == Some("not_contains_string"));
    }

    let content = fs::read_to_string(path)?;
    let contains_value = content.contains(value_to_check);

    match assertion.as_deref() {
        Some("not_contains_string") => Ok(!contains_value),
        Some("contains_string") | None => Ok(contains_value),
        Some(other) => {
            println!("  {}", format!("SKIPPED (unknown assertion type: '{}')", other).yellow());
            Ok(true)
        }
    }
}