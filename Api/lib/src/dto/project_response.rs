use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub solution: Vec<SolutionResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolutionResponse {
    pub id: Uuid,
    pub input_path: String, // pointer to file with unplaced layout
    pub output_path: Option<String>, // pointer to file with layout
    pub name: String,
    pub version: String,
}