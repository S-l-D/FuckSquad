use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRequest {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
}