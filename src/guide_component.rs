use dioxus::{logger::tracing, prelude::*};

// #[derive(Props, PartialEq, Clone)]
// struct DogAppProps {
//     breed: String,
// }

#[component]
fn DogApp(breed: String) -> Element {
    tracing::info!("Rendered with breed: {breed}");
    rsx! {
        "Breed: {breed}"
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        // Header {}
        DogApp { breed: "kangel" }
        // Footer {}
    }
}
