use dioxus::prelude::*;
mod guide_component;
mod hello_world_desktop;
// use guide_component::App;
use hello_world_desktop::App;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

// #[component]
// fn App() -> Element {
//     rsx! {
//         "HotDogg!!"
//         guide_component::App {}
//     }
// }
