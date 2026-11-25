use super::canvas::ViewLayout;
use super::SelectedObject;
use dioxus::prelude::*;

#[component]
pub fn Inspector(layout: ViewLayout, selected_object: Option<SelectedObject>) -> Element {
    let selected = match &selected_object {
        Some(SelectedObject::Cell(id)) => layout.cell(id),
        _ => None,
    };
    let placed = layout.placed_count();
    let layout = layout.raw();
    let blockage = match &selected_object {
        Some(SelectedObject::Blockage(id)) => layout.blockages.iter().find(|b| &b.id == id),
        _ => None,
    };
    let pad = match &selected_object {
        Some(SelectedObject::Pad(id)) => layout.pad_locations.iter().find(|p| &p.id == id),
        _ => None,
    };
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
            h2 { if blockage.is_some() { "Selected blockage" } else if pad.is_some() { "Selected pad" } else { "Selected cell" } }
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
            } else if let Some(blockage) = blockage {
                div { class: "selection-name", "{blockage.id}" }
                div { class: "properties",
                    div { span { "Type" } strong { "Blockage" } }
                    div { span { "X (from left)" } strong { "{blockage.x}" } }
                    div { span { "Y (from bottom)" } strong { "{blockage.y}" } }
                    div { span { "Width" } strong { "{blockage.width}" } }
                    div { span { "Height" } strong { "{blockage.height}" } }
                }
            } else if let Some(pad) = pad {
                div { class: "selection-name", "{pad.id}" }
                div { class: "properties",
                    div { span { "Type" } strong { "Pad" } }
                    div { span { "X (from left)" } strong { "{pad.x}" } }
                    div { span { "Y (from bottom)" } strong { "{pad.y}" } }
                }
            } else {
                p { class: "muted", "Click a cell, pad, or blockage in the layout to inspect its properties." }
            }
            div { class: "panel-divider" }
            p { class: "muted", "Pads are orange; blockages have diagonal grey hatching. Rows and nets are loaded from JSON but are not displayed yet." }
        }
    }
}
