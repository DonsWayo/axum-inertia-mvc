use sqlx::PgPool;
use sqlx::migrate::Migrator;
use std::env;
use std::sync::Arc;
use crate::error::DbError;

/// Database connection pool for PostgreSQL
pub type DbPool = Arc<PgPool>;

/// Path to migrations directory
static MIGRATOR: Migrator = sqlx::migrate!("src/migrations");

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

/// Initialize the database pool and run migrations
pub async fn init_pool() -> Result<DbPool, DbError> {
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        DbError::ConnectionError("DATABASE_URL environment variable not set".to_string())
    })?;
    
    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|e| DbError::ConnectionError(e.to_string()))?;
    
    // Run migrations before wrapping in Arc
    MIGRATOR.run(&pool)
        .await
        .map_err(|e| DbError::MigrationError(e.to_string()))?;
    
    Ok(Arc::new(pool))
}

