use crate::api::models::{CalculationResult, IngredientConstraint};
use dioxus::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use {
    crate::api::models::{IngredientWeight, Nutrients, DEFAULT_TARGET_RATIO},
    crate::entity::{db, ingredient, property},
    dioxus_logger::tracing::{error, info},
    sea_orm::ColumnTrait,
    sea_orm::EntityTrait,
    sea_orm::QueryFilter,
    serde::Serialize,
};

#[cfg(not(target_arch = "wasm32"))]
use nlopt::{Algorithm, Nlopt, Target};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Debug, Clone)]
struct IngredientWithProperties {
    pub id: i32,
    pub name: String,
    pub fat: f64,
    pub sugar: f64,
    pub water: f64,
    pub density: f64,
}

#[cfg(not(target_arch = "wasm32"))]
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

/// Compute a feasible initial point that respects all bounds and sums to 1.0.
///
/// Strategy: start each weight at its lower bound, then distribute the
/// remaining weight (1.0 − Σmins) proportionally across the available
/// slack (max − min) of each ingredient.
///
/// Returns `Err` if the constraints are infeasible (sum of mins > 1.0
/// or sum of maxs < 1.0).
#[cfg(not(target_arch = "wasm32"))]
fn feasible_initial_point(
    mins: &[f64],
    maxs: &[f64],
) -> std::result::Result<Vec<f64>, ServerFnError> {
    let n = mins.len();
    let sum_mins: f64 = mins.iter().sum();
    let sum_maxs: f64 = maxs.iter().sum();

    if sum_mins > 1.0 + 1e-9 {
        return Err(ServerFnError::new(format!(
            "Infeasible constraints: minimum weights sum to {sum_mins:.4}, which exceeds 1.0"
        )));
    }
    if sum_maxs < 1.0 - 1e-9 {
        return Err(ServerFnError::new(format!(
            "Infeasible constraints: maximum weights sum to {sum_maxs:.4}, which is less than 1.0"
        )));
    }

    // Start at lower bounds
    let mut x0: Vec<f64> = mins.to_vec();
    let remaining = 1.0 - sum_mins;

    if remaining > 1e-12 {
        // Distribute remaining weight proportionally to available slack
        let slacks: Vec<f64> = (0..n).map(|i| maxs[i] - mins[i]).collect();
        let total_slack: f64 = slacks.iter().sum();

        if total_slack > 1e-12 {
            for i in 0..n {
                let add = remaining * slacks[i] / total_slack;
                x0[i] += add;
            }
        }
    }

    // Clamp to bounds (handles floating-point drift)
    for i in 0..n {
        x0[i] = x0[i].clamp(mins[i], maxs[i]);
    }

    // Re-normalize to exactly 1.0 if there's tiny drift
    let sum: f64 = x0.iter().sum();
    if (sum - 1.0).abs() > 1e-12 {
        for v in &mut x0 {
            *v /= sum;
        }
    }

    Ok(x0)
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
        .filter_map(|c| ingredients_by_id.get(&c.id).cloned())
        .collect::<Vec<_>>();
    if ingredients.len() != ingredient_constraints.len() {
        return Err(ServerFnError::new("Some ingredient IDs were not found").into());
    }
    let mins = ingredient_constraints
        .iter()
        .map(|c| c.min.unwrap_or(0.0))
        .collect::<Vec<_>>();
    let maxs = ingredient_constraints
        .iter()
        .map(|c| c.max.unwrap_or(1.0))
        .collect::<Vec<_>>();

    let weights = optimize_nlopt(&ingredients, DEFAULT_TARGET_RATIO, &mins, &maxs)?;
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
) -> std::result::Result<Vec<f64>, ServerFnError> {
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

    // Validate that lower bounds don't exceed upper bounds
    for i in 0..n {
        if mins[i] > maxs[i] + 1e-9 {
            return Err(ServerFnError::new(format!(
                "Lower bound ({}) exceeds upper bound ({}) for ingredient {}",
                mins[i], maxs[i], i
            )));
        }
    }

    // bounds
    opt.set_lower_bounds(mins).unwrap();
    opt.set_upper_bounds(maxs).unwrap();
    opt.add_equality_constraint(
        |x: &[f64], _grad: Option<&mut [f64]>, _data: &mut ()| x.iter().sum::<f64>() - 1.0,
        (),
        1e-8, // tolerance
    )
    .unwrap();

    let mut x0 = feasible_initial_point(mins, maxs)?;

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
                error!("{}", message);
                return Err(ServerFnError::new(message));
            }
        }
    }
    Ok(x0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: assert all values are within [min, max] and sum ≈ 1.0
    fn assert_feasible(x0: &[f64], mins: &[f64], maxs: &[f64]) {
        let sum: f64 = x0.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6, "Sum should be ~1.0, got {sum}");
        for i in 0..x0.len() {
            assert!(
                x0[i] >= mins[i] - 1e-9,
                "x0[{i}]={} below min {}",
                x0[i],
                mins[i]
            );
            assert!(
                x0[i] <= maxs[i] + 1e-9,
                "x0[{i}]={} above max {}",
                x0[i],
                maxs[i]
            );
        }
    }

    #[test]
    fn feasible_point_equal_bounds() {
        let mins = vec![0.0, 0.0, 0.0];
        let maxs = vec![1.0, 1.0, 1.0];
        let x0 = feasible_initial_point(&mins, &maxs).unwrap();
        assert_feasible(&x0, &mins, &maxs);
    }

    #[test]
    fn feasible_point_high_lower_bounds() {
        // This is the exact failing scenario from the bug report:
        // mins sum to 0.85, so remaining 0.15 must be distributed
        let mins = vec![0.25, 0.1, 0.5];
        let maxs = vec![1.0, 1.0, 1.0];
        let x0 = feasible_initial_point(&mins, &maxs).unwrap();
        assert_feasible(&x0, &mins, &maxs);
    }

    #[test]
    fn feasible_point_tight_bounds() {
        let mins = vec![0.3, 0.3, 0.3];
        let maxs = vec![0.4, 0.4, 0.4];
        let x0 = feasible_initial_point(&mins, &maxs).unwrap();
        assert_feasible(&x0, &mins, &maxs);
    }

    #[test]
    fn feasible_point_exact_bounds() {
        let mins = vec![0.5, 0.3, 0.2];
        let maxs = vec![0.5, 0.3, 0.2];
        let x0 = feasible_initial_point(&mins, &maxs).unwrap();
        assert_feasible(&x0, &mins, &maxs);
        assert!((x0[0] - 0.5).abs() < 1e-9);
        assert!((x0[1] - 0.3).abs() < 1e-9);
        assert!((x0[2] - 0.2).abs() < 1e-9);
    }

    #[test]
    fn feasible_point_two_ingredients() {
        let mins = vec![0.0, 0.0];
        let maxs = vec![1.0, 1.0];
        let x0 = feasible_initial_point(&mins, &maxs).unwrap();
        assert_feasible(&x0, &mins, &maxs);
    }

    #[test]
    fn infeasible_mins_too_large() {
        let mins = vec![0.5, 0.5, 0.5];
        let maxs = vec![1.0, 1.0, 1.0];
        let result = feasible_initial_point(&mins, &maxs);
        assert!(result.is_err());
    }

    #[test]
    fn infeasible_maxs_too_small() {
        let mins = vec![0.0, 0.0, 0.0];
        let maxs = vec![0.2, 0.2, 0.2];
        let result = feasible_initial_point(&mins, &maxs);
        assert!(result.is_err());
    }

    #[test]
    fn optimize_nlopt_with_high_lower_bounds() {
        // End-to-end: the scenario that caused InvalidArgs
        let ingredients = vec![
            IngredientWithProperties {
                id: 1,
                name: "Cream".into(),
                fat: 0.36,
                sugar: 0.0,
                water: 0.58,
                density: 0.994,
            },
            IngredientWithProperties {
                id: 4,
                name: "Sugar".into(),
                fat: 0.0,
                sugar: 1.0,
                water: 0.0,
                density: 1.55,
            },
            IngredientWithProperties {
                id: 3,
                name: "Milk".into(),
                fat: 0.036,
                sugar: 0.05,
                water: 0.87,
                density: 1.035,
            },
        ];
        let target = Nutrients {
            fat: 0.2,
            sugar: 0.15,
            water: 0.6,
        };
        let mins = vec![0.25, 0.1, 0.5];
        let maxs = vec![1.0, 1.0, 1.0];

        let result = optimize_nlopt(&ingredients, target, &mins, &maxs);
        assert!(
            result.is_ok(),
            "optimize_nlopt should succeed, got: {:?}",
            result
        );
        let weights = result.unwrap();
        let sum: f64 = weights.iter().sum();
        assert!(
            (sum - 1.0).abs() < 1e-4,
            "Weights should sum to ~1.0, got {sum}"
        );
    }
}
