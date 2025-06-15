// ===== FILE: brain/docs/cli/src/db.rs  ===== //
// !!! WARNING: DATABASE FUNCTIONALITY IS TEMPORARILY DISABLED !!!
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

// THE FIX: Define an error type that is Send + Sync, compatible with anyhow.
// Using `thiserror` would be ideal, but for now a simple string is fine.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct DummyDbError(String);

// A dummy Connection struct that contains a Mutex, to satisfy the `.lock()` call.
pub struct Connection(Arc<Mutex<()>>);

impl Connection {
    // Provide a lock method that returns a dummy guard.
    pub fn lock(&self) -> Result<MutexGuard<'_, ()>, DummyDbError> {
        Ok(self.0.lock().unwrap())
    }
}

// THE FIX: The Result type now uses our anyhow-compatible dummy error.
pub type RusqliteResult<T> = std::result::Result<T, DummyDbError>;

pub fn open_db_connection(project_root: &Path) -> RusqliteResult<Connection> {
    eprintln!("[DB] WARNING: Database is disabled. Opening a dummy connection.");
    let db_path = project_root.join(".brain_db.sqlite3");
    eprintln!("[DB] Would have connected to: {:?}", db_path);
    Ok(Connection(Arc::new(Mutex::new(()))))
}

pub fn initialize_database(_conn: &Connection) -> RusqliteResult<()> {
    eprintln!("[DB] WARNING: Database is disabled. Skipping schema initialization.");
    Ok(())
}
// ===== END brain/docs/cli/src/db.rs ===== //