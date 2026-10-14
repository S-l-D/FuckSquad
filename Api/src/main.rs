use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::service::app_config;

#[derive(Debug, Clone)]
struct ServerConfig {
    pub host: String,
    pub port: u32,
}

pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_method()
            .allow_any_header();


        App::new()
            // .app_data(Data::new(AppData::new(
            //     Option::from(pool.clone()),
            //     config.clone(),
            //     env!("CARGO_PKG_NAME").to_string(),
            //     env!("CARGO_PKG_VERSION").to_string(),
            // )))
            // .wrap(common::log::log())
            // .wrap(auth)
            .wrap(cors)
            .configure(app_config)
    })
    .bind(format!("{}:{}", server.host, server.port))?
    .run()
    .await?;

    Ok(())
}
