use crate::dto::solution_response::SolutionResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub solutions: Vec<SolutionResponse>,
}
