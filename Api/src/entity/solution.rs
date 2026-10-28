use chrono::NaiveDateTime;
use common::entity::{EntityTrait, FromRow, History};
use common::enums::r_status::RStatus;
use serde::{Deserialize, Serialize};
use tetrislib::dto::solution_request::SolutionRequest;
use tetrislib::dto::solution_response::SolutionResponse;
use uuid::Uuid;

#[derive(
    Clone, Serialize, Deserialize, Debug, PartialEq, Default, FromRow, EntityTrait, History,
)]
pub struct Solution {
    pub uuid: Uuid,
    pub updu: Uuid,
    pub updt: NaiveDateTime,
    pub ver: i32,
    pub rstatus: RStatus,
    pub project: Uuid,
    pub input_path: String,          // pointer to file with unplaced layout
    pub output_path: Option<String>, // pointer to file with layout
    pub name: String,
    pub version: String,
}

impl Solution {
    pub fn from_request(request: &SolutionRequest) -> Solution {
        Solution {
            uuid: request.id.unwrap_or(Uuid::new_v4()),
            updu: Default::default(),
            updt: Default::default(),
            ver: 0,
            rstatus: Default::default(),
            project: request.project.clone(),
            input_path: request.input_path.clone(),
            output_path: request.output_path.clone(),
            name: request.name.clone(),
            version: request.version.clone(),
        }
    }

    pub fn to_response(&self) -> SolutionResponse {
        let files = crate::logic::solution_files::SolutionFiles::default();
        let source = files.source(self.uuid);
        let result = files.result(self.uuid);
        SolutionResponse {
            id: self.uuid.clone(),
            project: self.project.clone(),
            input_path: if source.is_file() {
                source.to_string_lossy().into_owned()
            } else {
                String::new()
            },
            output_path: result
                .is_file()
                .then(|| result.to_string_lossy().into_owned()),
            name: self.name.clone(),
            version: self.version.clone(),
        }
    }

    pub fn to_response_list(values: Vec<Solution>) -> Vec<SolutionResponse> {
        values
            .into_iter()
            .map(|ent| Solution::to_response(&ent))
            .collect()
    }
}
