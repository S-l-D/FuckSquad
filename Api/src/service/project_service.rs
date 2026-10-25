use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::web::Json;
use tetrislib::dto::project_request::ProjectRequest;
use crate::infra::status_message::StatusMessage;
use crate::logic::layout_logic::LayoutLogic;
use crate::service::layout_service::read_file;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("project")
            .service(read_file)
    );
}


#[post("")]
pub async fn create(
    req: HttpRequest,
    body: Json<ProjectRequest>,
) -> Result<HttpResponse, StatusMessage> {
    let path: String = req.match_info().get("path").unwrap().parse().unwrap();

    let res =
        web::block(move || LayoutLogic::read_file(path))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res))
}