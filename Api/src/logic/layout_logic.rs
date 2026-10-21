use tetrislib::dto::model::{Blockage, Cell, Design, Layout, Net, PadLocation, Row, TetrisParams};
use std::fs::OpenOptions;
use std::io::Read;
use crate::infra::status_message::StatusMessage;

pub struct LayoutLogic {}

impl LayoutLogic {
    pub fn read_file(
        path: String,
    ) -> Result<Layout, StatusMessage> {
        let path = format!("/data/{}", path);
        println!("Reading path: {}", path);
        let mut file = OpenOptions::new().read(true).open(&path).map_err(|e| {
            println!("{e}");
            StatusMessage::make_internal_server_error_message("")
        })?;
        let mut string = String::new();

        file.read_to_string(&mut string).map_err(|e| {
            println!("{e}");
            StatusMessage::make_internal_server_error_message("")
        })?;

        let layout: Layout = serde_json::from_str(&string).map_err(|e| {
            println!("{e}");
            StatusMessage::make_internal_server_error_message("")
        })?;

        Ok(layout)
    }
}