use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct IngredientForm {
    pub name: String,
    pub fat: f64,
    pub sugar: f64,
    pub water: f64,
    pub density: f64,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct IngredientConstraint {
    pub id: i32,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Nutrients {
    pub fat: f64,
    pub sugar: f64,
    pub water: f64,
}

pub const DEFAULT_TARGET_RATIO: Nutrients = Nutrients {
    fat: 0.2,
    sugar: 0.15,
    water: 0.6,
};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct IngredientWeight {
    pub ingredient_id: i32,
    pub weight: f64,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CalculationResult {
    pub weights: Vec<IngredientWeight>,
    pub nutrients: Nutrients,
    pub expected_volume_ml: f64,
}
