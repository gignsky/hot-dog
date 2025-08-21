use crate::backend::save_dog;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>,
}

fn use_music_player_provider() {
    let song = use_signal(|| "Drift Away".to_string());
    use_context_provider(|| MusicPlayer { song });
}

#[component]
pub fn userInterface() -> Element {
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
