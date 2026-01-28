use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing;

#[cfg(not(target_arch = "wasm32"))]
static DB: OnceCell<DatabaseConnection> = OnceCell::new();

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_connection() -> Result<&'static DatabaseConnection, DbErr> {
    // only executed the first time

    use crate::entity::{ingredient, property};
    let conn = Database::connect("sqlite://db-dev.sqlite?mode=rwc").await?;

    // schema sync — async ok here
    conn.get_schema_builder()
        .register(ingredient::Entity)
        .register(property::Entity)
        .sync(&conn)
        .await?;
    tracing::info!("Database schema synced");
    let _ = DB.set(conn);

    Ok(DB.get().unwrap())
}
