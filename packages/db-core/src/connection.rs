use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use crate::error::DbError;

/// Database connection pool for PostgreSQL
pub type DbPool = Arc<PgPool>;

/// Initialize the database connection pool
pub async fn init_db_pool() -> Result<DbPool, DbError> {
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        DbError::ConnectionError("DATABASE_URL environment variable not set".to_string())
    })?;
    
    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|e| DbError::ConnectionError(e.to_string()))?;
    
    Ok(Arc::new(pool))
}

