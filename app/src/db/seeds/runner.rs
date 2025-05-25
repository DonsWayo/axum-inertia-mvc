use crate::db::DbPool;
use std::error::Error;
use tracing::info;
use super::documents;

/// Run all seed operations
pub async fn run_all_seeds(pool: DbPool) -> Result<(), Box<dyn Error>> {
    info!("Running all database seeds...");
    
    // Run document seeds
    documents::seed(pool.clone()).await?;
    
    info!("All seeds completed successfully");
    Ok(())
}

/// Run a specific seed by name
pub async fn run_seed(pool: DbPool, seed_name: &str) -> Result<(), Box<dyn Error>> {
    match seed_name {
        "documents" => {
            info!("Running documents seed...");
            documents::seed(pool).await?;
            info!("Documents seed completed successfully");
        },
        _ => return Err(format!("Unknown seed: {}", seed_name).into()),
    }
    
    Ok(())
}
