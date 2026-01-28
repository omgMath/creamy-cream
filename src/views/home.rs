use crate::components::Optimizer;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Optimizer {}
    }
}
