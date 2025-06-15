// File: docs/cli/src/utils.rs
use anyhow::{Context, Result};
use sha2::{Digest, Sha256}; 
use std::fs;
use std::path::Path;

/// Calculates the SHA-256 hash of a file's content.
///
/// Reads the file at the given path and returns its SHA-256 hash
/// as a lowercase hexadecimal string.
#[allow(dead_code)]
pub fn calculate_file_hash(path: &Path) -> Result<String> {
    let file_bytes = fs::read(path)
        .with_context(|| format!("Failed to read file for hashing at {:?}", path))?;
    
    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let hash_result = hasher.finalize();
    
    Ok(format!("{:x}", hash_result))
}