// FILE: docs/scripts/src/model.rs

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskGraph {
    pub version: u32,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // This correctly handles all camelCase fields.
pub struct Task {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub goal: Option<String>,
    pub status: String,
    pub needs: Vec<String>,
    #[serde(default)]
    pub context_files: Option<Vec<String>>, // Deprecated, but kept for backward compatibility.
    #[serde(default)]
    pub context_query: Option<ContextQuery>, // The new way to specify context.
    #[serde(default)]
    pub acceptance_criteria: Option<Vec<AcceptanceCriterion>>,
    #[serde(default)]
    pub test_file: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextQuery {
    pub prompt: String,
    pub token_budget: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AcceptanceCriterion {
    pub description: String,
    #[serde(rename = "type")]
    pub check_type: String,
    pub file: String,
    #[serde(default)]
    pub assertion: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}