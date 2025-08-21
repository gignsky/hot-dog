use crate::backend::{del_dog, list_dogs};
use crate::*;

#[component]
pub fn Favorites() -> Element {
    let mut favorites_resource = use_resource(list_dogs);
    let favorites = favorites_resource.suspend()?;
    rsx! {
        div {
            id: "favorites",
            div {
                id: "favorites-container",
                for (ident, url) in favorites().unwrap() {
                    div {
                        key: "{ident}",
                        class: "favorite-dog",
                        img { src: "{url}" },
                        button {
                            onclick: move |_| async move {
                                _ = del_dog(ident).await;
                                favorites_resource.restart();
                            },
                            "❌"
                        }
                    }
                }
            }
        }
    }
}
