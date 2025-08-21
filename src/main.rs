use dioxus::prelude::*;

mod backend;
mod components;
mod frontend;

use crate::components::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        frontend::userInterface {}
    }
}
