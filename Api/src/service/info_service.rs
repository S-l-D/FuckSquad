use crate::infra::status_message::StatusMessage;
use actix_web::{HttpResponse, get, web};

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
