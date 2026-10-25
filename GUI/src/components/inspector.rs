use dioxus::prelude::*;
use crate::model::Layout;

#[component]
pub fn Inspector(layout: Layout, selected_id: Option<String>) -> Element {
    let selected = selected_id
        .as_deref()
        .and_then(|id| layout.cells.iter().find(|c| c.id == id));
    let placed = layout.cells.iter().filter(|c| c.is_placed()).count();
    let unplaced = layout.cells.len() - placed;

    rsx! {
        aside { class: "inspector",
            h2 { "Параметры design" }
            div { class: "properties",
                div { span { "Ширина" } strong { "{layout.design.chip_width}" } }
                div { span { "Высота" } strong { "{layout.design.chip_height}" } }
                div { span { "Шаг сетки" } strong { "{layout.design.grid_size}" } }
                div { span { "Всего cells" } strong { "{layout.cells.len()}" } }
                div { span { "Без координат" } strong { "{unplaced}" } }
            }
            div { class: "panel-divider" }
            h2 { "Выбранная ячейка" }
            if let Some(cell) = selected {
                div { class: "selection-name", "{cell.id}" }
                div { class: "properties",
                    div { span { "Тип" } strong { "{cell.cell_type}" } }
                    div { span { "Приоритет" } strong { "{cell.priority}" } }
                    div { span { "X (слева)" } strong { "{cell.x.unwrap_or_default()}" } }
                    div { span { "Y (снизу)" } strong { "{cell.y.unwrap_or_default()}" } }
                    div { span { "Ширина" } strong { "{cell.width}" } }
                    div { span { "Высота" } strong { "{cell.height}" } }
                }
            } else {
                p { class: "muted", "Нажми на прямоугольник на поле, чтобы увидеть его параметры." }
            }
            div { class: "panel-divider" }
            p { class: "muted", "rows, blockages, nets и pad_locations загружаются вместе с JSON, но пока не рисуются." }
        }
    }
}
