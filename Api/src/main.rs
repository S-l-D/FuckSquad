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

    let server_clone = server.clone();
    HttpServer::new(move || {
        // let auth = common::security::GrantsMiddleware::with_extractor(common::auth::extract);
        // let cors = Cors::permissive();

        println!("Starting server on {:#?}", server_clone);

        App::new()
            // .app_data(Data::new(AppData::new(
            //     Option::from(pool.clone()),
            //     config.clone(),
            //     env!("CARGO_PKG_NAME").to_string(),
            //     env!("CARGO_PKG_VERSION").to_string(),
            // )))
            // .app_data(Data::new(actix_web::web::PayloadConfig::new(1 << 30)))
            // .app_data(common::config::json_config())
            // .wrap(common::log::log())
            // .wrap(auth)
            // .wrap(cors)
            .configure(app_config)
    })
    .bind(format!("{}:{}", server.host, server.port))?
    .run()
    .await?;

    Ok(())
}
