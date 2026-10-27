use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolutionRequest {
    pub id: Option<Uuid>,
    pub project: Uuid,
    pub input_path: String, // pointer to file with unplaced layout
    pub output_path: Option<String>, // pointer to file with layout
    pub name: String,
    pub version: String,
}