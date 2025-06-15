// FILE: docs/scripts/src/conclude.rs

use crate::model::TaskGraph;
// Removed direct import of create_project_snapshot to use fully qualified path later
use crate::versioning::{ScannedFileInfo, SnapshotRequest}; 
use crate::{utils, AppState, versioning}; // Ensure versioning module is available
use anyhow::{anyhow, Context, Result};
use std::fs;
// use std::path::Path; // This import was unused, as flagged by the compiler warning.
use walkdir::WalkDir;

/// Automates marking a task as 'completed' in the tasks.yaml file
/// and creates a version snapshot in the database.
pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let tasks_path = state.project_root.join("docs/state/tasks.yaml");

    // --- Part 1: Update tasks.yaml ---
    let content = fs::read_to_string(&tasks_path)
        .with_context(|| format!("Failed to read task graph at {:?}", tasks_path))?;
    let mut graph: TaskGraph = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML from {:?}", tasks_path))?;

    let task_to_conclude = graph
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found in tasks.yaml", task_id))?;

    task_to_conclude.status = "completed".to_string();
    println!(
        "Updated status for task ['{}'] to 'completed'.",
        task_to_conclude.id
    );

    let updated_content = serde_yaml::to_string(&graph)
        .with_context(|| "Failed to serialize updated task graph back to YAML")?;
    fs::write(&tasks_path, updated_content)
        .with_context(|| format!("Failed to write updated content to {:?}", tasks_path))?;
    println!("Successfully saved changes to tasks.yaml.");

    // --- Part 2: Create Version Snapshot ---
    println!("Creating version snapshot for completed task '{}'...", task_id);

    let mut scanned_files: Vec<ScannedFileInfo> = Vec::new();
    let ignore_dirs = [".git", "target", "docs/scripts/target"]; 

    for entry in WalkDir::new(&state.project_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            !e.path().components().any(|component| {
                ignore_dirs.contains(&component.as_os_str().to_str().unwrap_or(""))
            })
        })
    {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path
                .strip_prefix(&state.project_root)
                .unwrap_or(path); 

            match utils::calculate_file_hash(path) {
                Ok(hash) => {
                    let metadata = fs::metadata(path)?;
                    scanned_files.push(ScannedFileInfo {
                        path: relative_path.to_string_lossy().into_owned(),
                        hash,
                        size: metadata.len() as i64,
                    });
                }
                Err(e) => {
                    eprintln!("Warning: Could not hash file {:?}: {}", path, e);
                }
            }
        }
    }

    let snapshot_request = SnapshotRequest {
        parent_version_id: None, 
        task_id_completed: Some(task_id.to_string()),
        description: format!("Snapshot after completing task: {}", task_id),
        files: scanned_files,
    };

    let mut conn = state
        .db_conn
        .lock()
        .map_err(|_| anyhow!("Failed to acquire database lock for snapshot"))?;
    
    // THE FIX: Call create_project_snapshot with its fully qualified path.
    match versioning::create_project_snapshot(&mut conn, snapshot_request) {
        Ok(version_id) => {
            println!(
                "Successfully created version snapshot with ID: {}",
                version_id
            );
        }
        Err(e) => {
            eprintln!("Error creating version snapshot: {}", e);
        }
    }

    Ok(())
}