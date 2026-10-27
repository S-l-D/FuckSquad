use crate::entity::project::Project;
use crate::logic::project_logic::ProjectLogic;
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use common::config::app_data::AppData;
use common::logic::LogicTrait;
use common::model::jwt_token::JwtToken;
use common::model::status_message::StatusMessage;
use tetrislib::dto::project_request::ProjectRequest;
use uuid::Uuid;


pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("project")
            .service(create)
            .service(update)
            .service(delete)
            .service(get)
    );
}


#[post("")]
pub async fn create(
    app_data: web::Data<AppData>,
    entity_req: Json<ProjectRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let jwt_token = JwtToken::from_request(req, app_data.get_public_bytes());
    let res =
        web::block(move || ProjectLogic::create(app_data, Project::from_request(&entity_req), jwt_token))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}

#[put("")]
pub async fn update(
    app_data: web::Data<AppData>,
    entity_req: Json<ProjectRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let jwt_token = JwtToken::from_request(req, app_data.get_public_bytes());
    let res =
        web::block(move || ProjectLogic::update(app_data, Project::from_request(&entity_req), jwt_token))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}

#[delete("/{id}")]
pub async fn delete(
    app_data: web::Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let id: Uuid = req.match_info().get("id").unwrap().parse().unwrap();
    let jwt_token = JwtToken::from_request(req, app_data.get_public_bytes());
    let res =
        web::block(move || ProjectLogic::delete(app_data, id, jwt_token))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}

#[get("/{id}")]
pub async fn get(
    app_data: web::Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let id: Uuid = req.match_info().get("id").unwrap().parse().unwrap();
    let res =
        web::block(move || ProjectLogic::find(app_data, id))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}
