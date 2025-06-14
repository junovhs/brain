// FILE: docs/scripts/src/reflect.rs

use crate::loader::load_task_graph;
use crate::AppState;
use anyhow::{anyhow, Result, Context};
use serde_json;
use std::fs;

/// This module will be responsible for generating historical records (ADRs)
/// for completed tasks, fulfilling the 'reflector' agent's role.

// THE FIX: The function is now declared as async.
pub async fn run(state: &AppState, task_id: &str) -> Result<()> {
    // 1. Load the task graph from `tasks.yaml`.
    let task_graph = load_task_graph(&state.project_root)?;

    // 2. Find the specified task by its ID.
    let task = task_graph
        .tasks
        .iter()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task ID '{}' not found in tasks.yaml", task_id))?;

    println!("// Found task to reflect on: [{}] {}", task.id, task.label);
    
    // 3. Serialize the task details into a JSON payload.
    let payload = serde_json::to_string_pretty(&task)
        .with_context(|| "Failed to serialize task to JSON for reflector payload")?;
    
    println!("\n// --- Reflector AI Payload (Source Material) ---");
    println!("{}", payload);
    println!("// --- End of Payload ---");

    // 4. (Future) Call LLM to get ADR content. For now, use a placeholder.
    let adr_content = format!(
        "# ADR-{} {}\n\nThis is a placeholder for the generated ADR content.",
        task.id, task.label
    );

    // 5. Sanitize the task label to create a valid filename.
    let safe_label = task.label.to_lowercase()
        .replace(" ", "_")
        .replace("/", "")
        .replace(":", "");
    
    // 6. Construct the output path.
    let history_dir = state.project_root.join("docs/history/");
    if !history_dir.exists() {
        fs::create_dir_all(&history_dir)
            .with_context(|| format!("Failed to create history directory at {:?}", history_dir))?;
    }
    let file_name = format!("ADR-{}_{}.md", task.id, safe_label);
    let output_path = history_dir.join(file_name);

    // 7. Save the generated ADR to the new file.
    fs::write(&output_path, adr_content)
        .with_context(|| format!("Failed to write ADR to file at {:?}", output_path))?;
    
    println!("\n// Successfully generated placeholder ADR.");
    println!("// Saved to: {:?}", output_path);

    Ok(())
}