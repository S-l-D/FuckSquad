mod components;
// mod model;

use components::canvas::ChipCanvas;
use components::SelectedObject;
use components::inspector::Inspector;
use components::projects::{LayoutSelection, ProjectsPanel};
use dioxus::prelude::*;
use tetrislib::dto::model;
use tetrislib::dto::model::Layout;

fn main() {
    dioxus::launch(App);
}

#[component]
#[allow(non_snake_case)]
fn App() -> Element {
    let mut selection = use_signal(|| None::<LayoutSelection>);
    let mut layout_resource = use_resource(move || async move {
        match selection() {
            Some(selected) => {
                tetrislib::store::solution_layout(&selected.solution, selected.output)
                    .await
                    .map(Some)
            }
            None => Ok::<Option<Layout>, String>(None),
        }
    });
    let mut selected_id = use_signal(|| None::<SelectedObject>);

    let state = layout_resource.cloned();
    let active = selection();
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        div { class: "application",
            header { class: "app-header",
                div {
                    h1 { "Chip Layout Viewer" }
                    p { "Cell placement visualization" }
                }
                button {
                    class: "reload-button",
                    disabled: active.is_none(),
                    onclick: move |_| { selected_id.set(None); layout_resource.clear(); layout_resource.restart(); },
                    "Refresh layout"
                }
            }
            div { class: "app-body",
                ProjectsPanel {
                    selected: active.clone(),
                    onclear: move |_| { selection.set(None); selected_id.set(None); layout_resource.clear(); },
                    onselect: move |selected| {
                        selected_id.set(None);
                        layout_resource.clear();
                        selection.set(Some(selected));
                    },
                }
                div { class: "layout-content",
                    if let Some(selected) = active.as_ref() {
                        div { class: "layout-heading",
                            strong { "{selected.name}" }
                            span { if selected.output { "Output layout" } else { "Input layout" } }
                        }
                    }
                    if active.is_none() {
                        div { class: "notice empty-viewer",
                            h2 { "Choose a layout" }
                            p { "Select Input or Output under a solution to view it here." }
                            p { class: "muted", "Each solution contains one input layout and, after processing, one output layout." }
                        }
                    } else {
                    match state {
                        None => rsx! {
                            div { class: "notice", "Loading layout…" }
                        },
                        Some(Err(message)) => rsx! {
                            div { class: "notice error",
                                h2 { "Could not load layout" }
                                p { "{message}" }
                                button { class: "tool-button", onclick: move |_| layout_resource.restart(), "Retry" }
                            }
                        },
                        Some(Ok(Some(layout))) => rsx! {
                            main { class: "workspace",
                                ChipCanvas {
                                    layout: layout.clone(),
                                    selected_object: selected_id(),
                                    on_select: move |id| selected_id.set(Some(id)),
                                }
                                Inspector { layout, selected_object: selected_id() }
                            }
                        },
                        Some(Ok(None)) => rsx! {},
                    }
                    }
                }
            }
        }
    }
}
