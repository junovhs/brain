// BRAIN Protocol v2 Data Models

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskGraph {
    pub version: u32,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub id: String,
    pub label: String,
    pub goal: Option<String>,
    pub status: String,
    pub needs: Vec<String>,
    #[serde(rename = "contextFiles")]
    pub context_files: Option<Vec<String>>,
    #[serde(rename = "acceptanceCriteria")]
    pub acceptance_criteria: Option<Vec<AcceptanceCriterion>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AcceptanceCriterion {
    pub description: String,
    #[serde(rename = "type")]
    pub check_type: String,
    pub file: String,
    pub assertion: Option<String>,
    pub value: Option<String>,
}
