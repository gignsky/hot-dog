use crate::backend::save_dog;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());

#[derive(Clone)]
struct TitleState(String);

#[allow(dead_code)]
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
pub fn userInterface() -> Element {
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
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div {
            id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }

        div {
            id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" },
            button { onclick: move |_| async move {
                let current = img_src.cloned().unwrap();
                img_src.restart();
                _ = save_dog(current).await;
            }, id: "save", "save!" }
        }
    }
}
