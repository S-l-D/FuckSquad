use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::service::app_config;
use actix_web::web::Data;
use common::config::app_data::AppData;
use dotenv::dotenv;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

mod dao;
mod entity;
mod logic;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = common::config::Config::from_env().unwrap();
    let main_config = common::config::Config::from_env().unwrap();

    let project_name = env!("CARGO_PKG_NAME").to_string();
    common::log::initialization_logging(project_name.clone());
    common::batch::log_scheduler(project_name.clone()).await;

    let manager =
        PostgresConnectionManager::new(format!("{}", config.database.url).parse().unwrap(), NoTls);
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();


    let validate_app_data = AppData::new(
        Option::from(pool.clone()),
        config.clone(),
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
    );

    let _ = actix_web::web::block(move || {
        let mut conn = validate_app_data.conn();
        dao::validate_all(&mut conn);
    })
        .await;

    unsafe { std::env::set_var("RUST_LOG", "actix_web=info") };

    HttpServer::new(move || {
        let auth = common::security::GrantsMiddleware::with_extractor(common::auth::extract);
        let cors = Cors::permissive();

        App::new()
            .app_data(Data::new(AppData::new(
                Option::from(pool.clone()),
                config.clone(),
                env!("CARGO_PKG_NAME").to_string(),
                env!("CARGO_PKG_VERSION").to_string(),
            )))
            .wrap(common::log::log())
            .wrap(auth)
            .wrap(cors)
            .app_data(Data::new(actix_web::web::PayloadConfig::new(1 << 30)))
            .app_data(common::config::json_config())
            .configure(app_config)
    })
        .bind(format!(
            "{}:{}",
            main_config.server.host, main_config.server.port
        ))?
        .run()
        .await?;

    Ok(())
}
