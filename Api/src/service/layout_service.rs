use actix_web::{get, web, HttpRequest, HttpResponse};
use crate::infra::status_message::StatusMessage;
use crate::logic::layout_logic::LayoutLogic;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("layout")
            .service(read_file)
    );
}


#[get("file/{path:.*}")]
pub async fn read_file(
    req: HttpRequest,
) -> Result<HttpResponse, StatusMessage> {
    let path: String = req.match_info().get("path").unwrap().parse().unwrap();

    let res =
        web::block(move || LayoutLogic::read_file(path))
            .await
            .map_err(|e| {
                eprintln!("{:?}", e);
                StatusMessage::make_internal_server_error_message("")
            })??;
    Ok(HttpResponse::Ok().json(res))
}