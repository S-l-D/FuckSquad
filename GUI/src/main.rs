mod components;
// mod model;

use components::canvas::ChipCanvas;
use components::inspector::Inspector;
use dioxus::prelude::*;
use tetrislib::dto::model::{Blockage, Cell, Design, Layout, Net, PadLocation, Row, TetrisParams};
use tetrislib::dto::model;

fn main() {
    dioxus::launch(App);
}

async fn fetch_layout() -> Result<Layout, String> {
    tetrislib::store::read_file("any".to_string()).await
}

#[component]
#[allow(non_snake_case)]
fn App() -> Element {
    let mut layout_resource = use_resource(fetch_layout);
    let mut selected_id = use_signal(|| None::<String>);

    let state = layout_resource.cloned();
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        div { class: "application",
            header { class: "app-header",
                div {
                    h1 { "Chip Layout Viewer" }
                    p { "Визуализация размещения ячеек" }
                }
                button {
                    class: "reload-button",
                    onclick: move |_| { selected_id.set(None); layout_resource.restart(); },
                    "Обновить данные"
                }
            }
            match state {
                None => rsx! {
                    div { class: "notice", "Загрузка схемы..." }
                },
                Some(Err(message)) => rsx! {
                    div { class: "notice error",
                        h2 { "Ошибка загрузки" }
                        p { "{message}" }
                        p { "Убедись, что запущен backend на порту 3001 и настроен proxy в Dioxus.toml." }
                    }
                },
                Some(Ok(layout)) => rsx! {
                    main { class: "workspace",
                        ChipCanvas {
                            layout: layout.clone(),
                            selected_id: selected_id(),
                            on_select: move |id| selected_id.set(Some(id)),
                        }
                        Inspector { layout, selected_id: selected_id() }
                    }
                },
            }
        }
    }
}
