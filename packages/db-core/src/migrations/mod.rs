use sqlx::PgPool;
use crate::error::DbError;

/// Run all database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), DbError> {
    sqlx::migrate!("./src/migrations")
        .run(pool)
        .await
        .map_err(|e| DbError::MigrationError(e.to_string()))?;
    
    Ok(())
}