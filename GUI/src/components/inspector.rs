use crate::model::Layout;
use dioxus::prelude::*;

#[component]
pub fn Inspector(layout: Layout, selected_id: Option<String>) -> Element {
    let selected = selected_id
        .as_deref()
        .and_then(|id| layout.cells.iter().find(|c| c.id == id));
    let placed = layout.cells.iter().filter(|c| c.is_placed()).count();
    let unplaced = layout.cells.len() - placed;

    rsx! {
        aside { class: "inspector",
            h2 { "Design properties" }
            div { class: "properties",
                div { span { "Width" } strong { "{layout.design.chip_width}" } }
                div { span { "Height" } strong { "{layout.design.chip_height}" } }
                div { span { "Grid size" } strong { "{layout.design.grid_size}" } }
                div { span { "Total cells" } strong { "{layout.cells.len()}" } }
                div { span { "Unplaced cells" } strong { "{unplaced}" } }
            }
            div { class: "panel-divider" }
            h2 { "Selected cell" }
            if let Some(cell) = selected {
                div { class: "selection-name", "{cell.id}" }
                div { class: "properties",
                    div { span { "Type" } strong { "{cell.cell_type}" } }
                    div { span { "Priority" } strong { "{cell.priority}" } }
                    div { span { "X (from left)" } strong { "{cell.x.unwrap_or_default()}" } }
                    div { span { "Y (from bottom)" } strong { "{cell.y.unwrap_or_default()}" } }
                    div { span { "Width" } strong { "{cell.width}" } }
                    div { span { "Height" } strong { "{cell.height}" } }
                }
            } else {
                p { class: "muted", "Click a cell in the layout to inspect its properties." }
            }
            div { class: "panel-divider" }
            p { class: "muted", "Rows, blockages, nets, and pad locations are loaded from JSON but are not displayed yet." }
        }
    }
}
