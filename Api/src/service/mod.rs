use actix_web::web;

mod info_service;

pub fn app_config(config: &mut web::ServiceConfig) {
    info_service::init_routes(config);
}
