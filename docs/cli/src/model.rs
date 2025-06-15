// ===== FILE: brain/docs/cli/src/model.rs  ===== //
use serde::{Deserialize, Serialize};

// THE FIX: Added Manifest struct for STOR-02
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub tasks_yaml_sha256: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskGraph {
    pub version: u32,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub goal: Option<String>,
    pub status: String,
    pub needs: Vec<String>,
    #[serde(default)]
    pub context_files: Option<Vec<String>>,
    #[serde(default)]
    pub context_query: Option<ContextQuery>,
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
// ===== END brain/docs/cli/src/model.rs ===== //