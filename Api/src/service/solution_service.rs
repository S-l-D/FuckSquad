use crate::entity::solution::Solution;
use crate::logic::solution_logic::SolutionLogic;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use common::config::app_data::AppData;
use common::model::status_message::StatusMessage;
use tetrislib::dto::solution_request::SolutionRequest;
use uuid::Uuid;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/solution")
            .app_data(web::PayloadConfig::new(16 * 1024 * 1024))
            .service(create)
            .service(update)
            .service(upload_source)
            .service(source)
            .service(result)
            .service(delete)
            .service(get),
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

#[delete("/{id}")]
pub async fn delete(
    app_data: web::Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let id: Uuid = req.match_info().get("id").unwrap().parse().unwrap();
    let res = web::block(move || SolutionLogic::delete(app_data, id))
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
    let res = web::block(move || SolutionLogic::find(app_data, id))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            StatusMessage::make_internal_server_error_message("")
        })??;
    Ok(HttpResponse::Ok().json(res.to_response()))
}

#[put("/{id}/source")]
pub async fn upload_source(
    app_data: web::Data<AppData>,
    id: web::Path<Uuid>,
    body: web::Bytes,
) -> Result<HttpResponse, StatusMessage> {
    let layout = web::block(move || SolutionLogic::upload_source(app_data, id.into_inner(), &body))
        .await
        .map_err(block_error)??;
    Ok(HttpResponse::Ok().json(layout))
}

#[get("/{id}/source")]
pub async fn source(
    app_data: web::Data<AppData>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, StatusMessage> {
    let layout = web::block(move || SolutionLogic::read_layout(app_data, id.into_inner(), false))
        .await
        .map_err(block_error)??;
    Ok(HttpResponse::Ok().json(layout))
}

#[get("/{id}/result")]
pub async fn result(
    app_data: web::Data<AppData>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, StatusMessage> {
    let layout = web::block(move || SolutionLogic::read_layout(app_data, id.into_inner(), true))
        .await
        .map_err(block_error)??;
    Ok(HttpResponse::Ok().json(layout))
}

fn block_error(error: actix_web::error::BlockingError) -> StatusMessage {
    eprintln!("{error}");
    StatusMessage::make_internal_server_error_message("Could not access solution layout")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, http::StatusCode, test};
    use common::config::{Config, DatabaseConfig, ServerConfig, ServicesConfig};

    #[actix_web::test]
    async fn invalid_octet_stream_is_http_400_before_database_access() {
        let config = Config {
            server: ServerConfig {
                host: "127.0.0.1".into(),
                port: 0,
                root: None,
            },
            database: DatabaseConfig { url: String::new() },
            services: ServicesConfig {
                schema: String::new(),
                socket: None,
                env: None,
                suffix: None,
            },
        };
        let app_data =
            AppData::new_with_public_bytes(None, config, "test".into(), "test".into(), vec![0]);
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_data))
                .configure(init_routes),
        )
        .await;
        let url = format!("/solution/{}/source", Uuid::new_v4());
        for body in ["not json", "{}", r#"{"design": {}}"#] {
            let request = test::TestRequest::put()
                .uri(&url)
                .insert_header(("Content-Type", "application/octet-stream"))
                .set_payload(body)
                .to_request();
            let response = test::call_service(&app, request).await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            let error: StatusMessage = test::read_body_json(response).await;
            assert!(error.message.contains("not a Layout"));
        }
        let request = test::TestRequest::put()
            .uri(&url)
            .insert_header(("Content-Type", "application/octet-stream"))
            .set_payload(vec![b' '; 16 * 1024 * 1024 + 1])
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::PAYLOAD_TOO_LARGE
        );
    }
}
