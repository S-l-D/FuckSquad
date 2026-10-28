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
use crate::dto::project_request::ProjectRequest;
use crate::dto::project_response::ProjectResponse;
use crate::dto::solution_request::SolutionRequest;
use crate::dto::solution_response::SolutionResponse;
use crate::infra::store::{delete, get, post, put, put_bytes};

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

pub async fn update_solution(request: SolutionRequest) -> Result<SolutionResponse, String> {
    put("solution", request).await
}

pub async fn delete_project(id: &str) -> Result<ProjectResponse, String> {
    delete(&format!("project/{id}")).await
}

pub async fn delete_solution(id: &str) -> Result<SolutionResponse, String> {
    delete(&format!("solution/{id}")).await
}

pub async fn upload_solution_source(id: &str, bytes: Vec<u8>) -> Result<Layout, String> {
    put_bytes(&format!("solution/{id}/source"), bytes).await
}

pub async fn solution_layout(id: &str, output: bool) -> Result<Layout, String> {
    let file = if output { "result" } else { "source" };
    get(&format!("solution/{id}/{file}")).await
}

pub async fn read_file(path: String) -> Result<Layout, String> {
    println!("Reading file: {}", path);
    get(format!("layout/file/{}", path).as_str()).await
}
