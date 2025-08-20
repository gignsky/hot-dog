use dioxus::prelude::*;
mod guide_component;
// use guide_component::App;

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
