use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::service::app_config;
use crate::infra::server_config::ServerConfig;

mod dao;
mod entity;
mod logic;
mod service;
mod infra;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let server = ServerConfig::load_or_default();

    println!("Starting server with config: {:#?}", server);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_method()
            .allow_any_header();


        App::new()
            .wrap(cors)
            .configure(app_config)
    })
        .workers(server.workers as usize)
    .bind(format!("{}:{}", server.host, server.port))?
    .run()
    .await?;

    Ok(())
}
