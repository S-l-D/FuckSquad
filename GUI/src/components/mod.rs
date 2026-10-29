pub mod canvas;
pub mod inspector;
pub mod projects;

#[derive(Clone, PartialEq)]
pub enum SelectedObject {
    Cell(String),
    Blockage(String),
}
