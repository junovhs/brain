// ===== FILE: brain/docs/cli/src/manifest.rs  ===== //
use crate::model::Manifest;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

fn get_manifest_path(project_root: &Path) -> PathBuf {
    let brain_dir = project_root.join(".brain");
    if !brain_dir.exists() {
        // Create the .brain directory if it doesn't exist.
        if let Err(e) = fs::create_dir_all(&brain_dir) {
            eprintln!("Warning: Could not create .brain directory: {}. Manifest operations will fail.", e);
        }
    }
    brain_dir.join("manifest.json")
}

pub fn read_manifest(project_root: &Path) -> Result<Manifest> {
    let manifest_path = get_manifest_path(project_root);
    if !manifest_path.exists() {
        // Return a default/empty manifest if the file doesn't exist
        return Ok(Manifest {
            tasks_yaml_sha256: "".to_string(),
        });
    }

    let content = fs::read_to_string(&manifest_path)
        .with_context(|| format!("Failed to read manifest at {:?}", manifest_path))?;
    let manifest: Manifest = serde_json::from_str(&content)
        .with_context(|| "Failed to parse manifest JSON")?;
    Ok(manifest)
}

pub fn write_manifest(project_root: &Path, manifest: &Manifest) -> Result<()> {
    let manifest_path = get_manifest_path(project_root);
    let updated_content = serde_json::to_string_pretty(manifest)
        .with_context(|| "Failed to serialize manifest to JSON")?;
    
    // Use a temporary file for atomic write, similar to how tasks.yaml will be handled.
    let temp_path = manifest_path.with_extension("json.tmp");
    fs::write(&temp_path, updated_content)
        .with_context(|| format!("Failed to write temporary manifest to {:?}", temp_path))?;
    fs::rename(&temp_path, &manifest_path)
        .with_context(|| format!("Failed to rename temporary manifest to {:?}", manifest_path))?;
    
    Ok(())
}
// ===== END brain/docs/cli/src/manifest.rs ===== //