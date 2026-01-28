use super::super::components::{Footer, Header};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col w-full app-bg text-primary",
            Header {}
            main { class: "w-full max-w-6xl grow mx-auto px-6 py-12",
                div { class: "surface-elevated border border-token rounded-lg p-6 shadow-sm space-y-4",
                    Outlet::<Route> {}
                }
            }
            Footer {}
        }
    }
}
