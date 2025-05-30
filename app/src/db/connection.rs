use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use crate::db::error::DatabaseError;

/// Database connection pool for PostgreSQL
pub type DbPool = Arc<PgPool>;

/// Initialize the database connection pool
pub async fn init_db_pool() -> Result<DbPool, DatabaseError> {
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        DatabaseError::ConnectionError("DATABASE_URL environment variable not set".to_string())
    })?;
    
    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
    
    // Run migrations
    sqlx::migrate!("./src/db/migrations")
        .run(&pool)
        .await
        .map_err(|e| DatabaseError::MigrationError(e.to_string()))?;
    
    Ok(Arc::new(pool))
}
