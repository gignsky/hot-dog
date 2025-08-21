use crate::backend::{del_all, del_dog, list_dogs};
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
                div {
                    id: "delete-all",
                    button {
                        onclick: move |_| async move {
                            _ = del_all().await;
                            favorites_resource.restart();
                            // Link {
                            //     id: "back-to-home",
                            //     to: Route::DogView,
                            // };
                        },
                        "Remove ALL Favorites!!!"
                    },
                }
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
