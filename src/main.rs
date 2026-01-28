use dioxus::prelude::*;
use tracing::Level;
mod views;
use views::{Home, IngredientCreate, Ingredients, Layout};
pub mod api;
mod components;
pub mod entity;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        
        #[route("/ingredients")]
        Ingredients { },

        #[route("/ingredients/create")]
        IngredientCreate { },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link {
            rel: "icon",
            sizes: "32x32",
            href: asset!("/assets/favicon-32x32.png"),
        }
        document::Link {
            rel: "icon",
            sizes: "48x48",
            href: asset!("/assets/favicon-48x48.png"),
        }
        document::Link {
            rel: "icon",
            sizes: "192x192",
            href: asset!("/assets/favicon-192x192.png"),
        }
        document::Link {
            rel: "icon",
            sizes: "512x512",
            href: asset!("/assets/favicon-512x512.png"),
        }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "152x152",
            href: asset!("/assets/favicon-152x152.png"),
        }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "180x180",
            href: asset!("/assets/favicon-180x180.png"),
        }

        document::Link { rel: "stylesheet", href: asset!("/assets/styling/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Meta {
            name: "description",
            content: "Optimize your ice cream creaminess with science!",
        }
        document::Title { "Creamy Cream " }

        Router::<Route> {}
    }
}
