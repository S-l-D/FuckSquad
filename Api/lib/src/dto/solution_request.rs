use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolutionRequest {
    pub id: Option<Uuid>,
    pub project: Uuid,
    #[serde(default)]
    pub input_path: String, // legacy field; storage paths are managed by the server
    pub output_path: Option<String>, // pointer to file with layout
    pub name: String,
    pub version: String,
}
