use actix_web::web;

mod info_service;
mod layout_service;
mod project_service;

pub fn app_config(config: &mut web::ServiceConfig) {
    info_service::init_routes(config);
    layout_service::init_routes(config);
}
