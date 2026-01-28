use crate::api::models::IngredientForm;
use crate::entity::property::PropertyType;
use crate::entity::{db, ingredient, property};
use dioxus::prelude::*;
use sea_orm::EntityTrait;
use sea_orm::QueryOrder;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

#[get("/api/ingredients")]
pub async fn get_ingredients() -> Result<Vec<ingredient::Model>> {
    let connection = db::create_connection().await?;
    let ingredients = ingredient::Entity::find()
        .order_by_asc(ingredient::Column::Name)
        .all(connection)
        .await?;
    Ok(ingredients)
}

#[post("/api/ingredients")]
pub async fn create_ingredient(form: IngredientForm) -> Result<String> {
    info!("Creating ingredient: {:?}", form);
    let connection = db::create_connection().await?;
    let i = ingredient::ActiveModel {
        name: Set(form.name),
        ..Default::default()
    };
    let new_ingredient = i.insert(connection).await?;
    let ingredient_id = new_ingredient.id.clone();
    _link_property(
        connection.clone(),
        ingredient_id.clone(),
        PropertyType::Fat,
        form.fat,
    )
    .await;
    _link_property(
        connection.clone(),
        ingredient_id.clone(),
        PropertyType::Sugar,
        form.sugar,
    )
    .await;
    _link_property(
        connection.clone(),
        ingredient_id.clone(),
        PropertyType::Water,
        form.water,
    )
    .await;
    _link_property(
        connection.clone(),
        ingredient_id.clone(),
        PropertyType::Density,
        form.density,
    )
    .await;
    Ok("Ingredient created".to_string())
}

async fn _link_property(
    connection: DatabaseConnection,
    ingredient_id: i32,
    property: PropertyType,
    value: f64,
) {
    let nutrient: property::ActiveModel = property::ActiveModel {
        ingredient_id: Set(ingredient_id),
        property: Set(property),
        value: Set(value),
        ..Default::default()
    };
    let _ = nutrient.insert(&connection).await;
}
