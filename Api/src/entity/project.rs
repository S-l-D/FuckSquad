use chrono::NaiveDateTime;
use common::entity::{EntityTrait, FromRow, History};
use common::enums::r_status::RStatus;
use serde::{Deserialize, Serialize};
use tetrislib::dto::project_request::ProjectRequest;
use tetrislib::dto::project_response::ProjectResponse;
use uuid::Uuid;


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default, FromRow, EntityTrait, History)]
pub struct Project {
    pub uuid: Uuid,
    pub updu: Uuid,
    pub updt: NaiveDateTime,
    pub ver: i32,
    pub rstatus: RStatus,
    pub name: String,
    pub description: String,
}



impl Project {
    pub fn from_request(req: &ProjectRequest) -> Project {
        Project {
            uuid: req.id.unwrap_or(Uuid::new_v4()).clone(),
            updu: Default::default(),
            updt: Default::default(),
            ver: 0,
            rstatus: Default::default(),
            name: req.name.clone(),
            description: req.description.clone().clone(),
        }
    }

    pub fn to_response(&self) -> ProjectResponse {
        ProjectResponse {
            id: self.uuid.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            solutions: Vec::new(),
        }
    }

    pub fn to_response_list(values: Vec<Project>) -> Vec<ProjectResponse> {
        values.into_iter().map(|ent| Project::to_response(&ent)).collect()
    }
}
