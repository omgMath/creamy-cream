use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "camelCase"
)]
pub enum PropertyType {
    Fat,
    Sugar,
    Water,
    Density,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "property")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ingredient_id: i32,
    #[sea_orm(belongs_to, from = "ingredient_id", to = "id")]
    pub ingredient: HasOne<super::ingredient::Entity>,
    #[sea_orm(check = r#"property IN ('fat', 'sugar', 'water', 'density')"#)]
    pub property: PropertyType,
    pub value: f64,
}

impl ActiveModelBehavior for ActiveModel {}
