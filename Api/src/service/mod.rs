use actix_web::web;

mod info_service;
mod project_service;
mod solution_service;

pub fn app_config(config: &mut web::ServiceConfig) {
    info_service::init_routes(config);
    project_service::init_routes(config);
    solution_service::init_routes(config);
}
