use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Layout {
    pub design: Design,
    #[serde(default)]
    pub rows: Vec<Row>,
    #[serde(default)]
    pub blockages: Vec<Blockage>,
    #[serde(default)]
    pub cells: Vec<Cell>,
    #[serde(default)]
    pub nets: Vec<Net>,
    #[serde(default)]
    pub pad_locations: Vec<PadLocation>,
    #[serde(default)]
    pub tetris_params: Option<TetrisParams>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Design {
    pub chip_width: f64,
    pub chip_height: f64,
    pub row_height: f64,
    pub row_spacing: f64,
    pub grid_size: f64,
    pub left_boundary: f64,
    pub right_boundary: f64,
    pub top_boundary: f64,
    pub bottom_boundary: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Cell {
    pub id: String,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    pub width: f64,
    pub height: f64,
    #[serde(rename = "type")]
    pub cell_type: String,
    pub priority: i32,
}

impl Cell {
    pub fn is_placed(&self) -> bool {
        matches!((self.x, self.y), (Some(x), Some(y)) if x.is_finite() && y.is_finite())
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Row {
    pub id: String,
    pub y: f64,
    pub height: f64,
    pub width: f64,
    pub available: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Blockage {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Net {
    pub net_id: String,
    pub cells: Vec<String>,
    pub weight: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PadLocation {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TetrisParams {
    pub ordering_strategy: String,
    pub cost_function: String,
    pub site_width: f64,
    pub rotation_allowed: bool,
    pub allow_multistory: bool,
    pub gap_penalty: f64,
    pub boundary_penalty: f64,
    pub seed: u64,
}
