#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::OnceCell;
#[cfg(not(target_arch = "wasm32"))]
use sea_orm::{Database, DatabaseConnection, DbErr};

#[cfg(not(target_arch = "wasm32"))]
static DB: OnceCell<DatabaseConnection> = OnceCell::new();

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_connection() -> Result<&'static DatabaseConnection, DbErr> {
    if let Some(conn) = DB.get() {
        return Ok(conn);
    }

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
