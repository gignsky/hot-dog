use crate::*;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_resource(super::super::backend::list_dogs).suspend()?;
    rsx! {
        div {
            id: "favorites",
            div {
                id: "favorites-container",
                for (ident, url) in favorites().unwrap() {
                    div {
                        key: "{ident}",
                        class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
