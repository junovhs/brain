// File: docs/cli/src/db.rs
// !!! WARNING: DATABASE FUNCTIONALITY IS TEMPORARILY DISABLED !!!
use std::path::Path;

// THE FIX: Define an error type that is Send + Sync, compatible with anyhow.
// Using `thiserror` would be ideal, but for now a simple string is fine.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct DummyDbError(String);

// A dummy Connection struct.
pub struct Connection;

// THE FIX: The Result type now uses our anyhow-compatible dummy error.
pub type RusqliteResult<T> = std::result::Result<T, DummyDbError>;

pub fn open_db_connection(project_root: &Path) -> RusqliteResult<Connection> {
    eprintln!("[DB] WARNING: Database is disabled. Opening a dummy connection.");
    let db_path = project_root.join(".brain_db.sqlite3");
    eprintln!("[DB] Would have connected to: {:?}", db_path);
    Ok(Connection)
}

pub fn initialize_database(_conn: &Connection) -> RusqliteResult<()> {
    eprintln!("[DB] WARNING: Database is disabled. Skipping schema initialization.");
    Ok(())
}