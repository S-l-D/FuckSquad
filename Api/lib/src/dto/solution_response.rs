use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SolutionResponse {
    pub id: Uuid,
    pub project: Uuid,
    pub input_path: String, // empty until source.json has been uploaded
    pub output_path: Option<String>, // present only when result.json exists
    pub name: String,
    pub version: String,
}
