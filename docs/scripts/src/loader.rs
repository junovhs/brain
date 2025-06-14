// FILE: docs/scripts/src/loader.rs

use crate::model::TaskGraph;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// A shared utility function to load and parse the tasks.yaml file.
pub fn load_task_graph(project_root: &Path) -> Result<TaskGraph> {
    let tasks_path = project_root.join("docs/state/tasks.yaml");
    let content = fs::read_to_string(&tasks_path)
        .with_context(|| format!("Failed to read task graph at {:?}", tasks_path))?;
    let graph: TaskGraph = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML from {:?}", tasks_path))?;
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // THE FIX: The test now checks for a task that exists in the current tasks.yaml
    // and verifies the 'contextQuery' field instead of the old fields.
    #[tokio::test]
    async fn test_deserialize_camelcase_fields_correctly() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // Navigate from docs/scripts directory to the project root
        path.pop(); 
        path.pop(); 
        let project_root = path;

        // 1. ACTION: Load the task graph using the production function.
        let task_graph = load_task_graph(&project_root).expect("Failed to load task graph for test");

        // 2. SETUP: Find a task that is known to have the optional camelCase contextQuery field.
        let task_final_1 = task_graph.tasks.iter()
            .find(|t| t.id == "FINAL-1-add-deps")
            .expect("Task FINAL-1-add-deps not found in test data");

        // 3. ASSERTION: Verify that the optional `contextQuery` field is now correctly parsed as `Some`.
        assert!(task_final_1.context_query.is_some(), "The 'contextQuery' field should be Some.");
        
        // 4. ASSERTION: Spot-check the content to ensure data integrity.
        let cq = task_final_1.context_query.as_ref().unwrap();
        assert_eq!(cq.token_budget, 2000);
        assert!(cq.prompt.contains("Add 'reqwest'"));
    }
}