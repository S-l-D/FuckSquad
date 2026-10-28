use crate::model::{Cell, Layout};
use dioxus::prelude::*;

fn zoom_at_center(
    mut zoom: Signal<f64>,
    mut pan_x: Signal<f64>,
    mut pan_y: Signal<f64>,
    factor: f64,
    width: f64,
    height: f64,
) {
    let previous = zoom();
    let next = (previous * factor).clamp(1.0, 64.0);
    let center_x = pan_x() + width / (2.0 * previous);
    let center_y = pan_y() + height / (2.0 * previous);
    zoom.set(next);
    pan_x.set((center_x - width / (2.0 * next)).clamp(0.0, width - width / next));
    pan_y.set((center_y - height / (2.0 * next)).clamp(0.0, height - height / next));
}

fn move_view(
    mut pan_x: Signal<f64>,
    mut pan_y: Signal<f64>,
    zoom: f64,
    width: f64,
    height: f64,
    horizontal: f64,
    vertical: f64,
) {
    let visible_width = width / zoom;
    let visible_height = height / zoom;
    pan_x.set((pan_x() + visible_width * horizontal).clamp(0.0, width - visible_width));
    pan_y.set((pan_y() + visible_height * vertical).clamp(0.0, height - visible_height));
}

fn fit_placed_cells(layout: &Layout) -> (f64, f64, f64) {
    let design_w = layout.design.chip_width;
    let design_h = layout.design.chip_height;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for cell in layout.cells.iter().filter(|c| c.is_placed() && c.width > 0.0 && c.height > 0.0) {
        let x = cell.x.unwrap_or_default();
        let y = cell.y.unwrap_or_default();
        min_x = min_x.min(x);
        max_x = max_x.max(x + cell.width);
        // Translate from bottom-left coordinates to SVG top-left coordinates.
        min_y = min_y.min(design_h - y - cell.height);
        max_y = max_y.max(design_h - y);
    }
    if !min_x.is_finite() { return (1.0, 0.0, 0.0); }
    let object_w = (max_x - min_x).max(1.0) * 1.5;
    let object_h = (max_y - min_y).max(1.0) * 1.5;
    let zoom = (design_w / object_w).min(design_h / object_h).clamp(1.0, 64.0);
    let visible_w = design_w / zoom;
    let visible_h = design_h / zoom;
    let pan_x = ((min_x + max_x) / 2.0 - visible_w / 2.0).clamp(0.0, design_w - visible_w);
    let pan_y = ((min_y + max_y) / 2.0 - visible_h / 2.0).clamp(0.0, design_h - visible_h);
    (zoom, pan_x, pan_y)
}

#[component]
pub fn ChipCanvas(layout: Layout, selected_id: Option<String>, on_select: EventHandler<String>) -> Element {
    let mut zoom = use_signal(|| 1.0_f64);
    let mut pan_x = use_signal(|| 0.0_f64);
    let mut pan_y = use_signal(|| 0.0_f64);
    let mut show_grid = use_signal(|| true);

    let width = layout.design.chip_width;
    let height = layout.design.chip_height;
    if !(width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0) {
        return rsx! { div { class: "notice error", "design.chip_width and chip_height must be positive finite values." } };
    }

    let level = zoom();
    let visible_w = width / level;
    let visible_h = height / level;
    let x = pan_x().clamp(0.0, width - visible_w);
    let y = pan_y().clamp(0.0, height - visible_h);
    let view_box = format!("{x} {y} {visible_w} {visible_h}");

    // Skip fine grid levels when zoomed out: keep roughly <= 40 grid lines per axis.
    let grid_base = layout.design.grid_size.max(1.0);
    let grid_step = grid_base * (visible_w.max(visible_h) / (40.0 * grid_base)).ceil().max(1.0);
    let grid_path = format!("M {grid_step} 0 L 0 0 0 {grid_step}");
    let (fit_zoom, fit_pan_x, fit_pan_y) = fit_placed_cells(&layout);
    let placed = layout.cells.iter().filter(|cell| cell.is_placed()).count();
    let total = layout.cells.len();

    rsx! {
        section { class: "canvas-column",
            div { class: "canvas-toolbar",
                div { class: "toolbar-group",
                    button { class: "tool-button", title: "Zoom in", onclick: move |_| zoom_at_center(zoom, pan_x, pan_y, 1.5, width, height), "+" }
                    button { class: "tool-button", title: "Zoom out", onclick: move |_| zoom_at_center(zoom, pan_x, pan_y, 1.0 / 1.5, width, height), "−" }
                    button { class: "tool-button", title: "Show the entire layout", onclick: move |_| { zoom.set(1.0); pan_x.set(0.0); pan_y.set(0.0); }, "Fit layout" }
                    button { class: "tool-button", title: "Zoom to placed cells", onclick: move |_| { zoom.set(fit_zoom); pan_x.set(fit_pan_x); pan_y.set(fit_pan_y); }, "Fit cells" }
                }
                div { class: "toolbar-group",
                    button { class: "tool-button", title: "Pan left", onclick: move |_| move_view(pan_x, pan_y, zoom(), width, height, -0.25, 0.0), "←" }
                    button { class: "tool-button", title: "Pan up", onclick: move |_| move_view(pan_x, pan_y, zoom(), width, height, 0.0, -0.25), "↑" }
                    button { class: "tool-button", title: "Pan down", onclick: move |_| move_view(pan_x, pan_y, zoom(), width, height, 0.0, 0.25), "↓" }
                    button { class: "tool-button", title: "Pan right", onclick: move |_| move_view(pan_x, pan_y, zoom(), width, height, 0.25, 0.0), "→" }
                }
                label { class: "grid-control",
                    input { r#type: "checkbox", checked: show_grid(), onchange: move |_| show_grid.set(!show_grid()) }
                    "Grid"
                }
                span { class: "zoom-display", "{(level * 100.0).round()}%" }
            }
            div { class: "canvas-frame",
                svg {
                    class: "chip-svg",
                    view_box: "{view_box}",
                    preserve_aspect_ratio: "xMidYMid meet",
                    role: "img",
                    "aria-label": "Cell placement on the chip",
                    defs {
                        pattern {
                            id: "chip-grid",
                            width: "{grid_step}",
                            height: "{grid_step}",
                            pattern_units: "userSpaceOnUse",
                            path { d: "{grid_path}", fill: "none", stroke: "#35465e", stroke_width: "1" }
                        }
                    }
                    rect { x: "0", y: "0", width: "{width}", height: "{height}", fill: "#121c2d" }
                    if show_grid() {
                        rect { x: "0", y: "0", width: "{width}", height: "{height}", fill: "url(#chip-grid)" }
                    }
                    rect { x: "0", y: "0", width: "{width}", height: "{height}", fill: "none", stroke: "#8391a5", stroke_width: "2" }
                    for cell in layout.cells.iter().filter(|c| c.is_placed() && c.width > 0.0 && c.height > 0.0) {
                        CellRect {
                            key: "{cell.id}",
                            cell: cell.clone(),
                            chip_height: height,
                            selected: selected_id.as_deref() == Some(cell.id.as_str()),
                            on_select,
                        }
                    }
                }
                div { class: "origin-indicator", "Origin: bottom left (0, 0)" }
            }
            div { class: "canvas-footer",
                span { "Layout: {width} × {height}" }
                span { "Placed: {placed} / {total}" }
                span { "Grid: {layout.design.grid_size}" }
            }
        }
    }
}

#[component]
fn CellRect(cell: Cell, chip_height: f64, selected: bool, on_select: EventHandler<String>) -> Element {
    let x = cell.x.unwrap_or_default();
    // SVG has top-left origin; our layout uses bottom-left origin.
    let y_svg = chip_height - cell.y.unwrap_or_default() - cell.height;
    let center_x = x + cell.width / 2.0;
    let center_y = y_svg + cell.height / 2.0;
    let fill = match cell.cell_type.as_str() {
        "NAND2" => "#38bdf8",
        "INV" => "#818cf8",
        "NOR2" => "#34d399",
        _ => "#60a5fa",
    };
    let outline = if selected { "#fbbf24" } else { "#091827" };
    let outline_width = if selected { "3" } else { "1" };
    let cell_id = cell.id.clone();
    let label = cell.id.clone();

    rsx! {
        g { class: "cell",
            rect {
                x: "{x}", y: "{y_svg}",
                width: "{cell.width}", height: "{cell.height}",
                rx: "2",
                fill: "{fill}", stroke: "{outline}", stroke_width: "{outline_width}",
                onclick: move |_| on_select.call(cell_id.clone()),
            }
            if cell.width >= 20.0 && cell.height >= 15.0 {
                text {
                    x: "{center_x}", y: "{center_y}",
                    text_anchor: "middle", dominant_baseline: "middle",
                    font_size: "12", font_weight: "600", fill: "#071425",
                    pointer_events: "none",
                    "{label}"
                }
            }
        }
    }
}
