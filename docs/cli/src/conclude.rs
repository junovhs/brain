// ===== FILE: brain/docs/cli/src/conclude.rs  ===== //
use crate::model::TaskGraph;
use crate::versioning::{self, ScannedFileInfo, SnapshotRequest};
// THE FIX: Add manifest module to imports
use crate::{utils, AppState, manifest};
use anyhow::{anyhow, Context, Result};
use std::fs;
use walkdir::WalkDir;
use sha2::{Digest, Sha256}; // For hashing the new tasks.yaml content

pub fn run(state: &AppState, task_id: &str) -> Result<()> {
    let tasks_path = state.project_root.join("docs/state/tasks.yaml");

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

    // --- Atomic Write Implementation (tempfile + rename) ---
    println!("Atomically writing changes to tasks.yaml...");
    let temp_path = tasks_path.with_extension("yaml.tmp");
    fs::write(&temp_path, &updated_content)
        .with_context(|| format!("Failed to write to temporary file {:?}", temp_path))?;
    fs::rename(&temp_path, &tasks_path)
        .with_context(|| format!("Failed to rename temporary file to {:?}", tasks_path))?;
    println!("Successfully saved changes to tasks.yaml.");

    // --- Manifest Update Implementation ---
    println!("Updating .brain/manifest.json...");
    let mut hasher = Sha256::new();
    hasher.update(updated_content.as_bytes());
    let hash_result = hasher.finalize();
    let new_hash = format!("{:x}", hash_result);

    let mut current_manifest = manifest::read_manifest(&state.project_root)?;
    current_manifest.tasks_yaml_sha256 = new_hash;
    manifest::write_manifest(&state.project_root, &current_manifest)?;
    println!("Successfully updated manifest with new tasks.yaml hash.");
    
    // --- Stubbed Snapshot Creation ---
    println!("(Stubbed) Creating version snapshot for completed task '{}'...", task_id);
    // This part remains stubbed and will just print warnings.
    let snapshot_request = SnapshotRequest {
        parent_version_id: None, 
        task_id_completed: Some(task_id.to_string()),
        description: format!("Snapshot after completing task: {}", task_id),
        files: Vec::new(), // Pass empty vec for now, was not fully implemented.
    };
    let conn = &state.db_conn;
    match versioning::create_project_snapshot(conn, snapshot_request) {
        Ok(version_id) => println!("[SNAPSHOT] Successfully created dummy version snapshot with ID: {}", version_id),
        Err(e) => eprintln!("Error creating dummy version snapshot: {}", e),
    }

    Ok(())
}
// ===== END brain/docs/cli/src/conclude.rs ===== //