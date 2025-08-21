use crate::*;

// #[derive(Clone)]
// struct TitleState(String);

#[component]
pub fn NavBar() -> Element {
    // let title = use_context::<TitleState>();
    rsx! {
        // use_context_provider(|| TitleState("HotDawg! 🌭".to_string()));
        div {
            id: "title",
            Link {
                to: Route::DogView,
                // h1 { "{title.0}"}
                h1 { "HotDawg! 🌭" }
            }
            Link {
                to: Route::Favorites,
                id: "heart",
                "❤️"
            }
        }
        Outlet::<Route> {}
    }
}
