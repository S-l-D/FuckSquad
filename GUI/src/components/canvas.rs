use super::SelectedObject;
use dioxus::html::{
    geometry::{PixelsRect, WheelDelta},
    input_data::MouseButton,
    InteractionLocation, MountedData, PointerInteraction,
};
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::rc::Rc;
use tetrislib::dto::model::{Blockage, Cell, Layout, PadLocation};

struct PreparedLayout {
    layout: Layout,
    cell_path: String,
    placed: usize,
    fit: (f64, f64, f64),
    min_label_scale: f64,
    member_nets: HashMap<String, Vec<usize>>,
    cell_indices: HashMap<String, usize>,
}

/// Cloning a layout for component props must not copy all of its cells and nets.
#[derive(Clone)]
pub struct ViewLayout(Rc<PreparedLayout>);

impl PartialEq for ViewLayout {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl ViewLayout {
    pub fn new(layout: Layout) -> Self {
        let mut cell_path = String::new();
        let mut placed = 0;
        let mut min_label_scale = f64::INFINITY;
        let mut cell_indices = HashMap::with_capacity(layout.cells.len());
        for (index, cell) in layout.cells.iter().enumerate() {
            cell_indices.insert(cell.id.clone(), index);
            if cell.is_placed() {
                placed += 1;
            }
            if valid_cell(cell) {
                append_cell_path(&mut cell_path, cell, layout.design.chip_height);
                let required = (16.0 / cell.height)
                    .max((cell.id.chars().count() as f64 * 7.0 + 6.0) / cell.width);
                min_label_scale = min_label_scale.min(required);
            }
        }
        let mut member_nets: HashMap<String, Vec<usize>> = HashMap::new();
        for (net_index, net) in layout.nets.iter().enumerate() {
            for member in net.cells.iter().chain(net.pads.iter()) {
                member_nets
                    .entry(member.clone())
                    .or_default()
                    .push(net_index);
            }
        }
        let fit = fit_placed_cells(&layout);
        Self(Rc::new(PreparedLayout {
            layout,
            cell_path,
            placed,
            fit,
            min_label_scale,
            member_nets,
            cell_indices,
        }))
    }

    pub fn raw(&self) -> &Layout {
        &self.0.layout
    }

    pub fn placed_count(&self) -> usize {
        self.0.placed
    }

    fn related_ids(&self, selection: Option<&SelectedObject>) -> HashSet<&str> {
        let id = match selection {
            Some(SelectedObject::Cell(id) | SelectedObject::Pad(id)) => id.as_str(),
            _ => return HashSet::new(),
        };
        self.0
            .member_nets
            .get(id)
            .into_iter()
            .flatten()
            .flat_map(|&index| {
                let net = &self.0.layout.nets[index];
                net.cells.iter().chain(net.pads.iter()).map(String::as_str)
            })
            .filter(|member| *member != id)
            .collect()
    }

    pub fn cell(&self, id: &str) -> Option<&Cell> {
        self.0
            .cell_indices
            .get(id)
            .map(|&index| &self.0.layout.cells[index])
    }
}

fn valid_cell(cell: &Cell) -> bool {
    cell.is_placed()
        && cell.width.is_finite()
        && cell.height.is_finite()
        && cell.width > 0.0
        && cell.height > 0.0
}

fn append_cell_path(path: &mut String, cell: &Cell, chip_height: f64) {
    let x = cell.x.unwrap_or_default();
    let y = chip_height - cell.y.unwrap_or_default() - cell.height;
    let _ = write!(
        path,
        "M{x} {y}h{}v{}h-{}Z",
        cell.width, cell.height, cell.width
    );
}

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
    for cell in layout.cells.iter().filter(|cell| valid_cell(cell)) {
        let x = cell.x.unwrap_or_default();
        let y = cell.y.unwrap_or_default();
        min_x = min_x.min(x);
        max_x = max_x.max(x + cell.width);
        // Translate from bottom-left coordinates to SVG top-left coordinates.
        min_y = min_y.min(design_h - y - cell.height);
        max_y = max_y.max(design_h - y);
    }
    if !min_x.is_finite() {
        return (1.0, 0.0, 0.0);
    }
    let object_w = (max_x - min_x).max(1.0) * 1.5;
    let object_h = (max_y - min_y).max(1.0) * 1.5;
    let zoom = (design_w / object_w)
        .min(design_h / object_h)
        .clamp(1.0, 64.0);
    let visible_w = design_w / zoom;
    let visible_h = design_h / zoom;
    let pan_x = ((min_x + max_x) / 2.0 - visible_w / 2.0).clamp(0.0, design_w - visible_w);
    let pan_y = ((min_y + max_y) / 2.0 - visible_h / 2.0).clamp(0.0, design_h - visible_h);
    (zoom, pan_x, pan_y)
}

#[component]
pub fn ChipCanvas(
    layout: ViewLayout,
    selected_object: Option<SelectedObject>,
    on_select: EventHandler<SelectedObject>,
) -> Element {
    let mut zoom = use_signal(|| 1.0_f64);
    let mut pan_x = use_signal(|| 0.0_f64);
    let mut pan_y = use_signal(|| 0.0_f64);
    let mut show_grid = use_signal(|| true);
    let mut drag_start = use_signal(|| None::<(f64, f64, f64, f64)>);
    let mut dragged = use_signal(|| false);
    let mut svg_size = use_signal(|| (1.0_f64, 1.0_f64));
    let mut svg_element = use_signal(|| None::<Event<MountedData>>);

    let width = layout.raw().design.chip_width;
    let height = layout.raw().design.chip_height;
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
    let grid_base = layout.raw().design.grid_size.max(1.0);
    let grid_step = grid_base
        * (visible_w.max(visible_h) / (40.0 * grid_base))
            .ceil()
            .max(1.0);
    let grid_path = format!("M {grid_step} 0 L 0 0 0 {grid_step}");
    let (fit_zoom, fit_pan_x, fit_pan_y) = layout.0.fit;
    let placed = layout.0.placed;
    let total = layout.raw().cells.len();
    let (pixel_w, pixel_h) = svg_size();
    let pixel_scale = (pixel_w / visible_w).min(pixel_h / visible_h);

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
                    tabindex: "0",
                    onmounted: move |event| {
                        svg_element.set(Some(event.clone()));
                        spawn(async move {
                            if let Ok(rect) = event.get_client_rect().await {
                                svg_size.set((rect.size.width.max(1.0), rect.size.height.max(1.0)));
                            }
                        });
                    },
                    onmousedown: move |event| {
                        if event.trigger_button() == Some(MouseButton::Primary) {
                            let point = event.client_coordinates();
                            drag_start.set(Some((point.x, point.y, point.x, point.y)));
                            dragged.set(false);
                        }
                    },
                    onmousemove: move |event| {
                        if let Some((start_x, start_y, last_x, last_y)) = drag_start() {
                            let point = event.client_coordinates();
                            let dx = point.x - last_x;
                            let dy = point.y - last_y;
                            if dx.abs() + dy.abs() > 0.0 {
                                let (pixel_w, pixel_h) = svg_size();
                                let scale = (pixel_w / (width / zoom())).min(pixel_h / (height / zoom()));
                                pan_x.set((pan_x() - dx / scale).clamp(0.0, width - width / zoom()));
                                pan_y.set((pan_y() - dy / scale).clamp(0.0, height - height / zoom()));
                                drag_start.set(Some((start_x, start_y, point.x, point.y)));
                                if (point.x - start_x).abs() + (point.y - start_y).abs() > 3.0 { dragged.set(true); }
                            }
                        }
                    },
                    onmouseup: move |_| drag_start.set(None),
                    onmouseleave: move |_| drag_start.set(None),
                    onwheel: move |event| {
                        event.prevent_default();
                        let direction = match event.delta() {
                            WheelDelta::Pixels(delta) => delta.y,
                            WheelDelta::Lines(delta) => delta.y,
                            WheelDelta::Pages(delta) => delta.y,
                        };
                        if direction != 0.0 {
                            zoom_at_center(zoom, pan_x, pan_y, if direction < 0.0 { 1.15 } else { 1.0 / 1.15 }, width, height);
                        }
                    },
                    onkeydown: move |event| {
                        let key = event.key().to_string();
                        match key.as_str() {
                            "ArrowLeft" => move_view(pan_x, pan_y, zoom(), width, height, -0.1, 0.0),
                            "ArrowRight" => move_view(pan_x, pan_y, zoom(), width, height, 0.1, 0.0),
                            "ArrowUp" => move_view(pan_x, pan_y, zoom(), width, height, 0.0, -0.1),
                            "ArrowDown" => move_view(pan_x, pan_y, zoom(), width, height, 0.0, 0.1),
                            "+" | "=" => zoom_at_center(zoom, pan_x, pan_y, 1.5, width, height),
                            "-" | "_" => zoom_at_center(zoom, pan_x, pan_y, 1.0 / 1.5, width, height),
                            _ => return,
                        }
                        event.prevent_default();
                    },
                    view_box: "{view_box}",
                    preserve_aspect_ratio: "xMidYMid meet",
                    role: "img",
                    "aria-label": "Cell placement, pads, and blockages on the chip",
                    defs {
                        pattern {
                            id: "blockage-hatch",
                            width: "12", height: "12",
                            pattern_units: "userSpaceOnUse",
                            path {
                                d: "M -3 3 L 3 -3 M 0 12 L 12 0 M 9 15 L 15 9",
                                fill: "none", stroke: "#a3a3a3", stroke_width: "1.5",
                                stroke_opacity: "0.65",
                            }
                        }
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
                    BaseCells { layout: layout.clone() }
                    rect {
                        x: "0", y: "0", width: "{width}", height: "{height}",
                        fill: "transparent", pointer_events: "all",
                        onclick: {
                            let layout = layout.clone();
                            move |event: MouseEvent| {
                                if dragged() { return; }
                                let point = event.client_coordinates();
                                let mounted = svg_element();
                                let layout = layout.clone();
                                spawn(async move {
                                    if let Some(mounted) = mounted {
                                        if let Ok(rect) = mounted.get_client_rect().await {
                                            if let Some((world_x, world_y)) = client_to_world(rect, point.x, point.y, x, y, visible_w, visible_h) {
                                                if let Some(cell) = layout.raw().cells.iter().rev().find(|cell| {
                                                    valid_cell(cell) && {
                                                        let cx = cell.x.unwrap_or_default();
                                                        let cy = height - cell.y.unwrap_or_default() - cell.height;
                                                        world_x >= cx && world_x <= cx + cell.width
                                                            && world_y >= cy && world_y <= cy + cell.height
                                                    }
                                                }) {
                                                    on_select.call(SelectedObject::Cell(cell.id.clone()));
                                                }
                                            }
                                        }
                                    }
                                });
                            }
                        },
                    }
                    SelectedOverlays {
                        layout: layout.clone(),
                        selected_object: selected_object.clone(),
                        on_select,
                        dragged,
                    }
                    CellLabels { layout: layout.clone(), x, y, visible_w, visible_h, pixel_scale }
                }
                div { class: "origin-indicator", "Origin: bottom left (0, 0)" }
            }
            div { class: "canvas-footer",
                span { "Layout: {width} × {height}" }
                span { "Placed: {placed} / {total}" }
                span { "Grid: {layout.raw().design.grid_size}" }
                span { "Drag to pan · Scroll to zoom · Arrow keys to pan · +/− to zoom" }
                if matches!(selected_object, Some(SelectedObject::Cell(_) | SelectedObject::Pad(_))) {
                    span { "Bright red: selected item · Muted red: items sharing a net" }
                }
            }
        }
    }
}

fn client_to_world(
    rect: PixelsRect,
    client_x: f64,
    client_y: f64,
    view_x: f64,
    view_y: f64,
    view_w: f64,
    view_h: f64,
) -> Option<(f64, f64)> {
    // preserveAspectRatio="meet" adds margins when the SVG and viewBox differ in aspect ratio.
    let scale = (rect.size.width / view_w).min(rect.size.height / view_h);
    if !scale.is_finite() || scale <= 0.0 {
        return None;
    }
    let left = rect.origin.x + (rect.size.width - view_w * scale) / 2.0;
    let top = rect.origin.y + (rect.size.height - view_h * scale) / 2.0;
    let x = (client_x - left) / scale;
    let y = (client_y - top) / scale;
    if x < 0.0 || y < 0.0 || x > view_w || y > view_h {
        return None;
    }
    Some((view_x + x, view_y + y))
}

#[component]
fn BaseCells(layout: ViewLayout) -> Element {
    // One path replaces tens of thousands of SVG nodes and event handlers.
    rsx! {
        path {
            d: "{layout.0.cell_path}",
            fill: "#7475d8", stroke: "#091827", stroke_width: "1",
            pointer_events: "none",
        }
    }
}

#[component]
fn SelectedOverlays(
    layout: ViewLayout,
    selected_object: Option<SelectedObject>,
    on_select: EventHandler<SelectedObject>,
    dragged: Signal<bool>,
) -> Element {
    let height = layout.raw().design.chip_height;
    let related = layout.related_ids(selected_object.as_ref());
    let mut related_path = String::new();
    for id in &related {
        if let Some(cell) = layout.cell(id) {
            if valid_cell(cell) {
                append_cell_path(&mut related_path, cell, height);
            }
        }
    }
    let selected_cell = match selected_object.as_ref() {
        Some(SelectedObject::Cell(id)) => layout.cell(id).filter(|cell| valid_cell(cell)),
        _ => None,
    };
    rsx! {
        path { d: "{related_path}", fill: "#b85868", stroke: "#091827", stroke_width: "1", pointer_events: "none" }
        if let Some(cell) = selected_cell {
            rect {
                x: "{cell.x.unwrap_or_default()}",
                y: "{height - cell.y.unwrap_or_default() - cell.height}",
                width: "{cell.width}", height: "{cell.height}", rx: "2",
                fill: "#ff404d", stroke: "#091827", stroke_width: "1",
                pointer_events: "none",
            }
        }
        g { class: "blockages",
            for blockage in layout.raw().blockages.iter().filter(|b| {
                b.x.is_finite() && b.y.is_finite()
                    && b.width.is_finite() && b.height.is_finite()
                    && b.width > 0.0 && b.height > 0.0
            }) {
                BlockageRect {
                    key: "{blockage.id}",
                    blockage: blockage.clone(),
                    chip_height: height,
                    selected: matches!(selected_object.as_ref(), Some(SelectedObject::Blockage(id)) if id == &blockage.id),
                    on_select: move |object| { if !dragged() { on_select.call(object); } },
                }
            }
        }
        for pad in layout.raw().pad_locations.iter().filter(|pad| pad.x.is_finite() && pad.y.is_finite()) {
            PadRect {
                key: "{pad.id}",
                pad: pad.clone(),
                chip_height: height,
                selected: matches!(selected_object.as_ref(), Some(SelectedObject::Pad(id)) if id == &pad.id),
                related: related.contains(pad.id.as_str()),
                on_select: move |object| { if !dragged() { on_select.call(object); } },
            }
        }
    }
}

#[component]
fn CellLabels(
    layout: ViewLayout,
    x: f64,
    y: f64,
    visible_w: f64,
    visible_h: f64,
    pixel_scale: f64,
) -> Element {
    if !pixel_scale.is_finite() || pixel_scale < layout.0.min_label_scale {
        return rsx! {};
    }
    let height = layout.raw().design.chip_height;
    let font_size = 12.0 / pixel_scale;
    rsx! {
        g { class: "cell-labels", pointer_events: "none",
            for cell in layout.raw().cells.iter().filter(|cell| {
                if !valid_cell(cell) { return false; }
                let cx = cell.x.unwrap_or_default();
                let cy = height - cell.y.unwrap_or_default() - cell.height;
                cx < x + visible_w && cx + cell.width > x
                    && cy < y + visible_h && cy + cell.height > y
                    && cell.height * pixel_scale >= 16.0
                    && cell.width * pixel_scale >= cell.id.chars().count() as f64 * 7.0 + 6.0
            }) {
                text {
                    key: "{cell.id}",
                    x: "{cell.x.unwrap_or_default() + cell.width / 2.0}",
                    y: "{height - cell.y.unwrap_or_default() - cell.height / 2.0}",
                    text_anchor: "middle", dominant_baseline: "middle",
                    font_size: "{font_size}", font_weight: "600", fill: "#071425",
                    "{cell.id}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_and_pad_connections_work_in_both_directions() {
        let mut layout: Layout =
            serde_json::from_str(include_str!("../../examples/layout.json")).unwrap();
        layout.nets[0].pads.push("PAD_LEFT".into());
        let view = ViewLayout::new(layout.clone());
        assert!(view
            .related_ids(Some(&SelectedObject::Pad("PAD_LEFT".into())))
            .contains("C1"));
        assert!(view
            .related_ids(Some(&SelectedObject::Cell("C1".into())))
            .contains("PAD_LEFT"));
        assert!(!view
            .related_ids(Some(&SelectedObject::Pad("PAD_RIGHT".into())))
            .contains("C1"));

        layout.nets[0].pads.clear();
        layout.nets[0].cells.push("PAD_LEFT".into());
        assert!(ViewLayout::new(layout)
            .related_ids(Some(&SelectedObject::Pad("PAD_LEFT".into())))
            .contains("C1"));
    }
}

#[component]
fn PadRect(
    pad: PadLocation,
    chip_height: f64,
    selected: bool,
    related: bool,
    on_select: EventHandler<SelectedObject>,
) -> Element {
    // Pad locations are points; center a 10×10 square on each point.
    let x = pad.x - 5.0;
    let y = chip_height - pad.y - 5.0;
    let fill = if selected {
        "#ff404d"
    } else if related {
        "#b85868"
    } else {
        "#f59e0b"
    };
    let id = pad.id.clone();
    rsx! {
        g { class: "pad",
            title { "Pad: {pad.id}" }
            rect {
                x: "{x}", y: "{y}", width: "10", height: "10",
                fill: "{fill}", stroke: "#091827", stroke_width: "1",
                onclick: move |_| on_select.call(SelectedObject::Pad(id.clone())),
            }
        }
    }
}

#[component]
fn BlockageRect(
    blockage: Blockage,
    chip_height: f64,
    selected: bool,
    on_select: EventHandler<SelectedObject>,
) -> Element {
    let y = chip_height - blockage.y - blockage.height;
    let center_x = blockage.x + blockage.width / 2.0;
    let center_y = y + blockage.height / 2.0;
    let font_size = 12.0_f64
        .min(blockage.height * 0.6)
        .min(blockage.width * 0.8 / (blockage.id.chars().count().max(1) as f64 * 0.7));
    let outline = if selected { "#ff404d" } else { "#a3a3a3" };
    let outline_width = if selected { "3" } else { "1" };
    let id = blockage.id.clone();
    rsx! {
        g { class: "blockage",
            title { "Blockage: {blockage.id}" }
            rect {
                x: "{blockage.x}", y: "{y}",
                width: "{blockage.width}", height: "{blockage.height}", rx: "2",
                fill: "url(#blockage-hatch)", stroke: "{outline}", stroke_width: "{outline_width}",
                pointer_events: "all",
                onclick: move |_| on_select.call(SelectedObject::Blockage(id.clone())),
            }
            text {
                x: "{center_x}", y: "{center_y}",
                text_anchor: "middle", dominant_baseline: "middle",
                font_size: "{font_size}", font_weight: "600", fill: "#e2edf9",
                stroke: "#121c2d", stroke_width: "2", style: "paint-order: stroke;",
                pointer_events: "none",
                "{blockage.id}"
            }
        }
    }
}
