use sqlx::PgPool;
use crate::error::DbError;

pub async fn reset_database() -> Result<(), DbError> {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| DbError::ConnectionError("DATABASE_URL not set".to_string()))?;
    
    // Parse the URL to get the database name
    let url = url::Url::parse(&database_url)
        .map_err(|e| DbError::ConnectionError(format!("Invalid DATABASE_URL: {}", e)))?;
    
    let db_name = url.path().trim_start_matches('/');
    
    // Connect to postgres database to drop and recreate
    let postgres_url = database_url.replace(&format!("/{}", db_name), "/postgres");
    let postgres_pool = PgPool::connect(&postgres_url).await
        .map_err(|e| DbError::ConnectionError(e.to_string()))?;
    
    // Drop existing connections
    sqlx::query(&format!(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid()",
        db_name
    ))
    .execute(&postgres_pool)
    .await
    .map_err(|e| DbError::QueryError(e.to_string()))?;
    
    // Drop the database
    sqlx::query(&format!("DROP DATABASE IF EXISTS {}", db_name))
        .execute(&postgres_pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
    
    // Create the database
    sqlx::query(&format!("CREATE DATABASE {}", db_name))
        .execute(&postgres_pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
    
    println!("Database {} dropped and recreated", db_name);
    
    // Now connect to the new database and run migrations
    let pool = PgPool::connect(&database_url).await
        .map_err(|e| DbError::ConnectionError(e.to_string()))?;
    
    // Run migrations
    sqlx::migrate!("src/migrations")
        .run(&pool)
        .await
        .map_err(|e| DbError::MigrationError(e.to_string()))?;
    
    println!("Migrations completed successfully");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Only run manually
    async fn test_reset_database() {
        reset_database().await.unwrap();
    }
}