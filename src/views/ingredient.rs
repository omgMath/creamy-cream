use crate::components::Ingredients as IngredientsComponent;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/ingredients.css");

#[component]
pub fn Ingredients() -> Element {
    rsx! {
        link { rel: "stylesheet", href: CSS }
        h1 { "Ingredients" }
        div {
            a { href: "/ingredients/create", "Add Ingredient" }
        }
        IngredientsComponent {}
        div {
            a { href: "/ingredients/create", "Add Ingredient" }
        }
    }
}
