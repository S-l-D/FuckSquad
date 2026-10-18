use tetrislib::dto::model::{Blockage, Cell, Design, Layout, Net, PadLocation, Row, TetrisParams};
use std::fs::OpenOptions;
use std::io::Read;
use crate::infra::status_message::StatusMessage;

pub struct LayoutLogic {}

impl LayoutLogic {
    pub fn read_file(
        path: String,
    ) -> Result<Layout, StatusMessage> {
        // let mut file = OpenOptions::new().read(true).open(&path).map_err(|e| {
        //     println!("{e}");
        //     StatusMessage::make_internal_server_error_message("")
        // })?;
        // let mut string = String::new();
        //
        // file.read_to_string(&mut string).map_err(|e| {
        //     println!("{e}");
        //     StatusMessage::make_internal_server_error_message("")
        // });
        //
        // let layout: Layout = serde_json::from_str(&string).map_err(|e| {
        //     println!("{e}");
        //     StatusMessage::make_internal_server_error_message("")
        // })?;

        let layout = Layout {
            design: Design {
                chip_width: 2000.0,
                chip_height: 1000.0,
                row_height: 40.0,
                row_spacing: 0.0,
                grid_size: 5.0,
                left_boundary: 0.0,
                right_boundary: 2000.0,
                top_boundary: 1000.0,
                bottom_boundary: 0.0,
            },

            rows: vec![
                Row {
                    id: "R1".to_string(),
                    y: 0.0,
                    height: 40.0,
                    width: 2000.0,
                    available: true,
                },
                Row {
                    id: "R2".to_string(),
                    y: 40.0,
                    height: 40.0,
                    width: 2000.0,
                    available: true,
                },
            ],

            blockages: vec![
                Blockage {
                    id: "BLK1".to_string(),
                    x: 800.0,
                    y: 0.0,
                    width: 200.0,
                    height: 120.0,
                },
                Blockage {
                    id: "BLK2".to_string(),
                    x: 1500.0,
                    y: 200.0,
                    width: 150.0,
                    height: 80.0,
                },
            ],

            cells: vec![
                Cell {
                    id: "C1".to_string(),
                    x: Some(100.0),
                    y: Some(100.0),
                    width: 30.0,
                    height: 40.0,
                    cell_type: "NAND2".to_string(),
                    priority: 3,
                },
                Cell {
                    id: "C2".to_string(),
                    x: Some(300.0),
                    y: Some(400.0),
                    width: 20.0,
                    height: 40.0,
                    cell_type: "INV".to_string(),
                    priority: 1,
                },
            ],

            nets: vec![
                Net {
                    net_id: "N1".to_string(),
                    cells: vec![
                        "C1".to_string(),
                        "C2".to_string()
                    ],
                    weight: 1.0,
                },
            ],

            pad_locations: vec![
                PadLocation {
                    id: "PAD_LEFT".to_string(),
                    x: 0.0,
                    y: 500.0,
                },
                PadLocation {
                    id: "PAD_RIGHT".to_string(),
                    x: 2000.0,
                    y: 500.0,
                },
            ],

            tetris_params: Some(TetrisParams {
                ordering_strategy: "connectivity".to_string(),
                cost_function: "skyline_height".to_string(),
                site_width: 5.0,
                rotation_allowed: false,
                allow_multistory: false,
                gap_penalty: 0.1,
                boundary_penalty: 0.05,
                seed: 42,
            }),
        };

        Ok(layout)
    }
}