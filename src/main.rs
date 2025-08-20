use dioxus::prelude::*;
// mod guide_component;
// mod hello_world_desktop;
// use guide_component::App;
// use hello_world_desktop::App;
// mod guide_rsx;
// use guide_rsx::App;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{href: MAIN_CSS}
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDawg! 🌭"}
        }
    }
}

#[component]
pub fn DogView() -> Element {
    let skip = move |evt| {};
    // let save = move |evt| {};
    // let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let save = move |_| {
        img_src.set("https://hips.hearstapps.com/hmg-prod/images/dog-puppy-on-garden-royalty-free-image-1586966191.jpg?crop=0.752xw:1.00xh;0.175xw,0&resize=1200:*")
    };

    rsx! {
        div {
            id: "dogview",
            img { src: "{img_src}"}
        }

        div {
            id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}
