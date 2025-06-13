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

    #[test]
    fn test_deserialize_camelcase_fields_correctly() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop(); 
        path.pop(); 
        let project_root = path;

        // 1. ACTION: Load the task graph using the production function.
        let task_graph = load_task_graph(&project_root).expect("Failed to load task graph for test");

        // 2. SETUP: Find a task that is known to have the optional camelCase fields.
        let task_f1_1 = task_graph.tasks.iter()
            .find(|t| t.id == "F1.1-implement-repl-loop")
            .expect("Task F1.1-implement-repl-loop not found in test data");

        // 3. ASSERTION: Verify that the optional fields are now correctly parsed as `Some`.
        assert!(task_f1_1.goal.is_some(), "The 'goal' field should be Some.");
        assert!(task_f1_1.context_files.is_some(), "The 'contextFiles' field should be Some.");
        assert!(task_f1_1.acceptance_criteria.is_some(), "The 'acceptanceCriteria' field should be Some.");

        // 4. ASSERTION: Spot-check the content to ensure data integrity.
        let ac = task_f1_1.acceptance_criteria.as_ref().unwrap();
        assert_eq!(ac.len(), 2, "Should have parsed 2 acceptance criteria.");
        assert_eq!(ac[0].description, "main.rs uses a loop to repeatedly read user input from stdin.");
        assert_eq!(ac[0].check_type, "text_check");
    }
}