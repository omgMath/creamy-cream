use crate::api::ingredient::get_ingredients;
use dioxus::prelude::*;

#[component]
pub fn Ingredients() -> Element {
    let results = use_resource(|| get_ingredients());

    rsx! {
        table {
            thead {
                tr {
                    th { "Ingredient Name" }
                }
            }
            tbody {
                if let Some(response) = &*results.read() {
                    match response {
                        Ok(ingredients) => rsx! {
                            for ingredient in ingredients.iter() {
                                tr {
                                    td { "{ingredient.name}" }
                                }
                            }
                        },
                        Err(err) => rsx! { "Failed to fetch response: {err}" },
                    }
                } else {
                    "Loading..."
                }
            }
        }
    }
}
