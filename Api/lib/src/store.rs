

// pub async fn create(
//     payload: crate::dto::record_request::RecordRequest,
// ) -> Result<crate::dto::record_response::RecordResponse, String> {
//     common::store::post(
//         common::enums::module::Module::Ns,
//         "/record".to_string().as_str(),
//         payload,
//         None,
//     )
//         .await
// }

use crate::dto::model::Layout;
use crate::infra::store::{get, post, put};
use crate::dto::project_request::ProjectRequest;
use crate::dto::project_response::ProjectResponse;
use crate::dto::solution_request::SolutionRequest;
use crate::dto::solution_response::SolutionResponse;

pub async fn all_projects() -> Result<Vec<ProjectResponse>, String> {
     get("project/all").await
}

pub async fn create_project(request: ProjectRequest) -> Result<ProjectResponse, String> {
     post("project", request).await
}

pub async fn update_project(request: ProjectRequest) -> Result<ProjectResponse, String> {
     put("project", request).await
}

pub async fn create_solution(request: SolutionRequest) -> Result<SolutionResponse, String> {
     post("solution", request).await
}

pub async fn read_file(
     path: String,
) -> Result<Layout, String> {
     println!("Reading file: {}", path);
     get(
          format!("layout/file/{}", path).as_str(),
     ).await
}
