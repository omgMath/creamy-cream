use crate::api::models::{
    CalculationResult, IngredientConstraint, IngredientWeight, Nutrients, DEFAULT_TARGET_RATIO,
};
use crate::entity::{db, ingredient, property};
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Serialize;

#[cfg(not(target_arch = "wasm32"))]
use nlopt::{Algorithm, Nlopt, Target};

#[derive(Serialize, Debug, Clone)]
struct IngredientWithProperties {
    pub id: i32,
    pub name: String,
    pub fat: f64,
    pub sugar: f64,
    pub water: f64,
    pub density: f64,
}

impl Nutrients {
    /// Weighted sum of nutrients given weights and ingredients
    fn weighted_sum(weights: &[f64], ingredients: &[IngredientWithProperties]) -> Nutrients {
        let mut mix = Nutrients {
            fat: 0.0,
            sugar: 0.0,
            water: 0.0,
        };
        for (w, ing) in weights.iter().zip(ingredients) {
            mix.fat += w * ing.fat;
            mix.sugar += w * ing.sugar;
            mix.water += w * ing.water;
        }
        mix
    }
}

fn normalize_weights(weights: &Vec<f64>) -> Vec<f64> {
    let sum: f64 = weights.iter().sum();
    if sum > 0.0 {
        return weights.iter().map(|w| w / sum).collect();
    }
    weights.to_vec()
}

#[post("/api/calculate")]
pub async fn calculate(
    ingredient_constraints: Vec<IngredientConstraint>,
    target_amount_g: Option<f64>,
) -> Result<CalculationResult> {
    info!(
        "Calculating with ingredient constraints: {:?}",
        ingredient_constraints
    );
    let connection = db::create_connection().await?;
    let rows = ingredient::Entity::find()
        .filter(
            ingredient::Column::Id.is_in(
                ingredient_constraints
                    .iter()
                    .map(|c| c.id)
                    .collect::<Vec<_>>(),
            ),
        )
        .find_with_related(property::Entity)
        .all(connection)
        .await?;
    let mut ingredients_by_id = std::collections::HashMap::new();
    for (ingredient, properties) in rows.into_iter() {
        let mut fat = 0.0;
        let mut sugar = 0.0;
        let mut water = 0.0;
        let mut density = 0.0;

        for prop in properties {
            match prop.property {
                property::PropertyType::Fat => fat = prop.value / 100.0,
                property::PropertyType::Sugar => sugar = prop.value / 100.0,
                property::PropertyType::Water => water = prop.value / 100.0,
                property::PropertyType::Density => density = prop.value,
            }
        }

        ingredients_by_id.insert(
            ingredient.id,
            IngredientWithProperties {
                id: ingredient.id,
                name: ingredient.name,
                fat,
                sugar,
                water,
                density,
            },
        );
    }

    let ingredients = ingredient_constraints
        .iter()
        .map(|c| ingredients_by_id.get(&c.id).unwrap().clone())
        .collect::<Vec<_>>();
    let mins = ingredient_constraints
        .iter()
        .map(|c| c.min.unwrap_or(0.0))
        .collect::<Vec<_>>();
    let maxs = ingredient_constraints
        .iter()
        .map(|c| c.max.unwrap_or(1.0))
        .collect::<Vec<_>>();

    let weights = optimize_nlopt(&ingredients, DEFAULT_TARGET_RATIO, &mins, &maxs);
    info!("Optimized weights: {:?}", weights);

    let mix = Nutrients::weighted_sum(&weights, &ingredients);
    info!("Mix: {:?}", mix);

    Ok(CalculationResult {
        weights: weights
            .iter()
            .enumerate()
            .map(|(i, w)| IngredientWeight {
                ingredient_id: ingredients[i].id,
                weight: w * target_amount_g.unwrap_or(1.0),
            })
            .collect(),
        nutrients: mix.clone(),
        expected_volume_ml: weights
            .iter()
            .zip(ingredients.iter())
            .map(|(w, ing)| w * target_amount_g.unwrap_or(1.0) / ing.density * 1000.0)
            .sum(),
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn optimize_nlopt(
    ingredients: &[IngredientWithProperties],
    target: Nutrients,
    mins: &[f64],
    maxs: &[f64],
) -> Vec<f64> {
    let n: usize = ingredients.len();

    // Define the objective: squared error vs target
    let mut opt = Nlopt::new(
        Algorithm::Cobyla,
        n,
        move |x: &[f64], _grad: Option<&mut [f64]>, _user_data: &mut ()| {
            let mix = Nutrients::weighted_sum(x, ingredients);
            let df = mix.fat - target.fat;
            let ds = mix.sugar - target.sugar;
            let dw = mix.water - target.water;
            df * df + ds * ds + dw * dw
        },
        Target::Minimize,
        (),
    );

    // bounds
    opt.set_lower_bounds(mins).unwrap();
    opt.set_upper_bounds(maxs).unwrap();
    opt.add_equality_constraint(
        |x: &[f64], _grad: Option<&mut [f64]>, _data: &mut ()| x.iter().sum::<f64>() - 1.0,
        (),
        1e-8, // tolerance
    )
    .unwrap();

    let mut x0 = vec![0.0; n];
    for i in 0..n {
        x0[i] = (mins[i] + maxs[i]) / 2.0;
    }
    x0 = normalize_weights(&x0);

    match opt.optimize(&mut x0) {
        Ok(_value) => info!("Found optimal value"),
        Err((state, _value)) => {
            if matches!(state, nlopt::FailState::RoundoffLimited) {
                info!(
                    "Optimization limited by roundoff, best solution found: {:?}",
                    x0
                );
            } else {
                let message = format!("Error optimizing with NLopt: {:?}", state);
                error!(message);
                panic!("{}", message);
            }
        }
    }
    x0
}
