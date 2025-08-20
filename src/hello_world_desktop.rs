use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        "Hello, World!"
        div {
            class: "bg-blue-100",
            button {
                onclick: move |_| tracing::info!("Clicked"),
                "Click me!"
            }
        }
    }
}
