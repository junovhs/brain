// FILE: docs/scripts/src/versioning.rs
// Adapted from diranalyze/backend/src/version_control.rs
// Handles the logic for creating and managing project state snapshots.

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

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

pub fn create_project_snapshot(
    conn: &mut Connection,
    payload: SnapshotRequest,
) -> Result<i64> {
    let tx = conn.transaction()?;
    let version_id;
    {
        let timestamp = chrono::Utc::now().to_rfc3339();

        tx.execute(
            "INSERT INTO ProjectVersions (parent_version_id, task_id_completed, timestamp, description) VALUES (?1, ?2, ?3, ?4)",
            params![payload.parent_version_id, payload.task_id_completed, timestamp, payload.description],
        )?;
        version_id = tx.last_insert_rowid();

        let mut stmt = tx.prepare(
            "INSERT INTO VersionFiles (project_version_id, file_path, content_hash, file_size) VALUES (?1, ?2, ?3, ?4)",
        )?;
        for file in payload.files {
            stmt.execute(params![version_id, file.path, file.hash, file.size])?;
        }
    }

    tx.commit()?;
    Ok(version_id)
}