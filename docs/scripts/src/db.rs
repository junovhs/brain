// FILE: docs/scripts/src/db.rs
// Adapted from diranalyze/backend/src/db_manage.rs
// Manages the SQLite database connection and schema for the brain-cli.

use rusqlite::{Connection, Result as RusqliteResult};
use std::path::Path;

/// Opens a connection to the project's a SQLite database.
pub fn open_db_connection(project_root: &Path) -> RusqliteResult<Connection> {
    let db_path = project_root.join(".brain_db.sqlite3");
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

/// Initializes the database schema.
pub fn initialize_database(conn: &Connection) -> RusqliteResult<()> {
    println!("[DB] Initializing BRAIN database schema...");
    conn.execute_batch(
        "BEGIN;
        -- Stores snapshots of the project state, linked to tasks.
        CREATE TABLE IF NOT EXISTS ProjectVersions (
            version_id INTEGER PRIMARY KEY AUTOINCREMENT,
            parent_version_id INTEGER,
            task_id_completed TEXT, -- Which task completion triggered this version
            timestamp TEXT NOT NULL,
            description TEXT,
            CONSTRAINT fk_parent_version
                FOREIGN KEY (parent_version_id)
                REFERENCES ProjectVersions (version_id)
                ON DELETE CASCADE
        );
        -- Stores the state of every file in a given version.
        CREATE TABLE IF NOT EXISTS VersionFiles (
            version_file_id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_version_id INTEGER NOT NULL,
            file_path TEXT NOT NULL,
            content_hash TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            CONSTRAINT fk_project_version
                FOREIGN KEY (project_version_id)
                REFERENCES ProjectVersions (version_id)
                ON DELETE CASCADE,
            UNIQUE (project_version_id, file_path)
        );
        COMMIT;"
    )?;
    println!("[DB] Schema initialization complete.");
    Ok(())
}