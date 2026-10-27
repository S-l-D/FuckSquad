use actix_web::{get, web, HttpResponse};
use common::model::status_message::StatusMessage;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("info")
            .service(uptime)
    );
}

#[get("uptime")]
pub async fn uptime() -> Result<HttpResponse, StatusMessage> {
    Ok(HttpResponse::Ok().json("Uptime"))
}
