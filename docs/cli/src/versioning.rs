// ===== FILE: brain/docs/cli/src/versioning.rs  ===== //
// Adapted from diranalyze/backend/src/version_control.rs
// Handles the logic for creating and managing project state snapshots.
// !!! WARNING: THIS IS STUBBED OUT AND WILL NOT WORK. !!!

use serde::{Deserialize, Serialize};
use crate::db::Connection; // Use our dummy connection type

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScannedFileInfo {
    pub path: String,
    pub hash: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotRequest {
    pub parent_version_id: Option<i64>,
    pub task_id_completed: Option<String>,
    pub description: String,
    pub files: Vec<ScannedFileInfo>,
}

// THE FIX: The stub doesn't mutate anything, so accept an immutable reference `&Connection`.
pub fn create_project_snapshot(
    _conn: &Connection, // Changed from &mut Connection
    _payload: SnapshotRequest,
) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    eprintln!("[SNAPSHOT] WARNING: Database is disabled. Snapshot creation skipped.");
    Ok(0) // Return a dummy version ID
}
// ===== END brain/docs/cli/src/versioning.rs ===== //