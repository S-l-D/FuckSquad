use actix_web::{HttpResponse, Responder, get, web};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("info").route("", web::get().to(uptime)));
}

// #[get("uptime")]
pub async fn uptime() -> impl Responder {
    HttpResponse::Ok().json("Uptime")
}
