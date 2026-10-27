use crate::entity::solution::Solution;
use crate::logic::solution_logic::SolutionLogic;
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use common::config::app_data::AppData;
use common::model::status_message::StatusMessage;
use tetrislib::dto::solution_request::SolutionRequest;
use uuid::Uuid;


pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("solution")
            .service(create)
            .service(update)
            .service(delete)
            .service(get)
    );
}


#[post("")]
pub async fn create(
    app_data: web::Data<AppData>,
    entity_req: Json<SolutionRequest>,
) -> Result<HttpResponse, StatusMessage> {
    let res =
        web::block(move || SolutionLogic::create(app_data, Solution::from_request(&entity_req)))
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
    entity_req: Json<SolutionRequest>,
) -> Result<HttpResponse, StatusMessage> {
    let res =
        web::block(move || SolutionLogic::update(app_data, Solution::from_request(&entity_req)))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}

#[delete("{id}")]
pub async fn delete(
    app_data: web::Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let id: Uuid = req.match_info().get("id").unwrap().parse().unwrap();
    let res =
        web::block(move || SolutionLogic::delete(app_data, id))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}

#[get("{id}")]
pub async fn get(
    app_data: web::Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let id: Uuid = req.match_info().get("id").unwrap().parse().unwrap();
    let res =
        web::block(move || SolutionLogic::find(app_data, id))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}
