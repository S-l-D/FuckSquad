

// pub async fn create(
//     payload: crate::dto::record_request::RecordRequest,
// ) -> Result<crate::dto::record_response::RecordResponse, String> {
//     common::store::post(
//         common::enums::module::Module::Ns,
//         "/record".to_string().as_str(),
//         payload,
//         None,
//     )
//         .await
// }

use crate::dto::model::Layout;
use crate::infra::store::{get};

pub async fn read_file(
     path: String,
) -> Result<Layout, String> {
     println!("Reading file: {}", path);
     get(
          format!("data/{}", path).as_str(),
     ).await
}