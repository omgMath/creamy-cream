use crate::api::models::IngredientForm;
use crate::entity::ingredient;
use dioxus::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use {
    crate::entity::property::PropertyType,
    crate::entity::{db, property},
    dioxus_logger::tracing::info,
    sea_orm::EntityTrait,
    sea_orm::QueryOrder,
    sea_orm::{ActiveModelTrait, DatabaseConnection, Set},
};

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
    let ingredient_id = new_ingredient.id;
    link_property(connection, ingredient_id, PropertyType::Fat, form.fat).await?;
    link_property(connection, ingredient_id, PropertyType::Sugar, form.sugar).await?;
    link_property(connection, ingredient_id, PropertyType::Water, form.water).await?;
    link_property(
        connection,
        ingredient_id,
        PropertyType::Density,
        form.density,
    )
    .await?;
    Ok("Ingredient created".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn link_property(
    connection: &DatabaseConnection,
    ingredient_id: i32,
    property: PropertyType,
    value: f64,
) -> std::result::Result<(), sea_orm::DbErr> {
    let nutrient = property::ActiveModel {
        ingredient_id: Set(ingredient_id),
        property: Set(property),
        value: Set(value),
        ..Default::default()
    };
    nutrient.insert(connection).await?;
    Ok(())
}
