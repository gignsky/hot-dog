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

static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());

fn main() {
    dioxus::launch(App);
}

#[derive(Clone)]
struct TitleState(String);

#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>,
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

fn use_music_player_provider() {
    let song = use_signal(|| "Drift Away".to_string());
    use_context_provider(|| MusicPlayer { song });
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDawg! 🌭".to_string()));
    use_music_player_provider();
    rsx! {
        document::Stylesheet{href: MAIN_CSS}
        Title {}
        Player {}
        DogView {}
    }
}
#[component]
fn Player() -> Element {
    rsx! {
        h3 {"Now Playing {SONG}"}
        button {
            onclick: move |_| *SONG.write() = "Vienna".to_string(),
            "Shuffle"
        }
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div {
            id: "title",
            h1 { "{title.0}"}
        }
    }
}

#[component]
pub fn DogView() -> Element {
    let skip = move |evt| {};
    // let save = move |evt| {};
    // let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let mut img_src = use_signal(|| "".to_string());
    // let save = move |_| {
    //     img_src.set("https://hips.hearstapps.com/hmg-prod/images/dog-puppy-on-garden-royalty-free-image-1586966191.jpg?crop=0.752xw:1.00xh;0.175xw,0&resize=1200:*")
    // };
    let fetch_new = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message);
    };

    rsx! {
        div {
            id: "dogview",
            img { src: "{img_src}"}
        }

        div {
            id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: fetch_new, id: "save", "save!" }
        }
    }
}
