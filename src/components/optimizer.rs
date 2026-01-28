use super::{BarChart, BarData};
use crate::api::calculate::calculate;
use crate::api::ingredient::get_ingredients;
use crate::api::models::{IngredientConstraint, Nutrients, DEFAULT_TARGET_RATIO};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use std::collections::HashSet;

const NUMBER_OF_INGREDIENTS_DISPLAYED: usize = 5;

fn prepare_data_for_barchart(nutrients: &Nutrients) -> Vec<BarData> {
    vec![
        BarData {
            label: "Water".into(),
            value: nutrients.water * 100.0,
        },
        BarData {
            label: "Fat".into(),
            value: nutrients.fat * 100.0,
        },
        BarData {
            label: "Sugar".into(),
            value: nutrients.sugar * 100.0,
        },
    ]
}

#[component]
pub fn Optimizer() -> Element {
    let mut search_term: Signal<String> = use_signal(|| String::new());
    let mut target_amount_g: Signal<f64> = use_signal(|| 1.0);
    let ingredients_response = use_resource(|| get_ingredients());
    let ingredient_id_to_name = use_memo(move || {
        let mut map = std::collections::HashMap::new();
        if let Some(Ok(ingredients)) = &*ingredients_response.read() {
            for ingredient in ingredients.iter() {
                map.insert(ingredient.id, ingredient.name.clone());
            }
        }
        map
    });
    let mut ingredient_constraints = use_signal(|| Vec::<IngredientConstraint>::new());
    let ingredient_ids_for_optimization = use_memo(move || {
        let set: HashSet<i32> = ingredient_constraints
            .read()
            .iter()
            .map(|constraint| constraint.id)
            .collect();
        set
    });
    let filtered_ingredients = use_memo(move || {
        if let Some(Ok(ingredients)) = &*ingredients_response.read() {
            ingredients
                .iter()
                .filter(|ingredient| {
                    (search_term().is_empty()
                        || ingredient
                            .name
                            .to_lowercase()
                            .contains(&search_term().to_lowercase()))
                        && !ingredient_ids_for_optimization()
                            .iter()
                            .any(|id| id == &ingredient.id)
                })
                .cloned()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    });
    let calculation_result = use_resource(move || async move {
        if ingredient_constraints.len() > 0 {
            calculate(
                ingredient_constraints()
                    .iter()
                    .map(|c| IngredientConstraint {
                        id: c.id,
                        min: c.min.is_some().then(|| c.min.unwrap() / target_amount_g()),
                        max: c.max.is_some().then(|| c.max.unwrap() / target_amount_g()),
                    })
                    .collect(),
                if target_amount_g() != 1.0 {
                    Some(target_amount_g())
                } else {
                    None
                },
            )
            .await
            .ok()
        } else {
            None
        }
    });

    info!(
        "Ingredient IDs for optimization: {:?}",
        ingredient_ids_for_optimization()
    );
    let ingredients_class = "flex flex-wrap space-x-2 space-y-2";
    let buttons_class = "button-primary rounded-full inline h-9 min-w-max";
    rsx! {
        h1 { "Optimizer" }
        div { class: "grid sm:grid-cols-2 sm:gap-8 gap-4",
            div { class: "space-y-4",
                h2 { "Available Ingredients" }
                input {
                    id: "search_ingredients",
                    placeholder: "Type to search ingredients...",
                    oninput: move |event| async move {
                        search_term.set(event.value());
                    },
                }
                if filtered_ingredients().len() > 0 {
                    div { class: ingredients_class,
                        for ingredient in filtered_ingredients().into_iter().take(NUMBER_OF_INGREDIENTS_DISPLAYED) {
                            button {
                                class: buttons_class,
                                onclick: move |_| async move {
                                    if !ingredient_ids_for_optimization.read().contains(&ingredient.id) {
                                        ingredient_constraints
                                            .write()
                                            .push(IngredientConstraint {
                                                id: ingredient.id,
                                                min: None,
                                                max: None,
                                            });
                                    }
                                },
                                "{ingredient.name} +"
                            }
                        }
                    }
                } else {
                    "No ingredients found, do you want to "
                    {
                        rsx! {
                            a { href: "/ingredients/create", "create it" }
                        }
                    }
                    "?"
                }
            }
            div { class: "space-y-4",
                h2 { "Selected Ingredients" }
                input {
                    id: "target_amount_g",
                    placeholder: "Target amount (in g, optional)",
                    oninput: move |event| async move {
                        target_amount_g.set(event.value().parse().unwrap_or(1.0));
                    },
                }
                if ingredient_constraints.len() > 0 {
                    div { class: ingredients_class,
                        for (index , constraint) in ingredient_constraints.iter().enumerate() {
                            div { class: "flex space-x-2 items-center",
                                button {
                                    class: "button-danger rounded-full min-w-9",
                                    onclick: move |_| async move {
                                        ingredient_constraints.write().remove(index);
                                    },
                                    "x"
                                }
                                div { class: "min-w-max",
                                    "{ingredient_id_to_name.get(&constraint.id).unwrap()}"
                                }
                                input {
                                    r#type: "number",
                                    id: "constraint_min_{constraint.id}",
                                    name: "Min. (in g, optional)",
                                    placeholder: "Min. (in g, optional)",
                                    min: "0",
                                    max: "{target_amount_g()}",
                                    step: "0.01",
                                    value: ingredient_constraints()[index].min,
                                    oninput: move |event| async move {
                                        let parsed_value = event.value().parse().unwrap_or(0.0);
                                        let candidates = [target_amount_g(), parsed_value];
                                        let min = candidates
                                            .iter()
                                            .filter(|n| **n >= 0.0)
                                            .min_by(|a, b| a.total_cmp(b))
                                            .unwrap();
                                        ingredient_constraints.write()[index].min = Some(*min);
                                    },
                                }
                                input {
                                    r#type: "number",
                                    id: "constraint_max_{constraint.id}",
                                    name: "Max. (in g, optional)",
                                    placeholder: "Max. (in g, optional)",
                                    min: "0",
                                    max: "{target_amount_g()}",
                                    step: "0.01",
                                    value: ingredient_constraints()[index].max,
                                    oninput: move |event| async move {
                                        let parsed_value = event.value().parse().unwrap_or(target_amount_g());
                                        let candidates = [target_amount_g(), parsed_value];
                                        let min = candidates.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
                                        ingredient_constraints.write()[index].max = Some(*min);
                                    },
                                }
                            
                            }
                        }
                    }
                } else {
                    "No ingredients selected"
                }
            }
        }
        if let Some(Some(result)) = &*calculation_result.read() {
            div { class: "space-y-4",
                h2 { "Optimization Result" }
                ul {
                    for weight in result.weights.iter() {
                        li {
                            "{ingredient_id_to_name.get(&weight.ingredient_id).unwrap()}: {weight.weight:.2}"
                        }
                    }
                }
                BarChart {
                    data: prepare_data_for_barchart(&result.nutrients),
                    label: "Your Ratios".to_string(),
                    background_data: prepare_data_for_barchart(&DEFAULT_TARGET_RATIO),
                    background_label: "Optimal Ratios".to_string(),
                }
                if target_amount_g() != 1.0 {
                    div {
                        "Those amounts will correspond to approximately "
                        b { "{result.expected_volume_ml:.2}ml" }
                    }
                } else {
                    div {
                        b { "1g" }
                        " of ingredients will correspond to approximately "
                        b { "{result.expected_volume_ml:.2}ml" }
                    }
                }
            }
        }
    }
}
