pub mod status_monitoring;

use crate::DbPool;
use crate::error::DbError;

pub async fn run_all_seeds(pool: DbPool) -> Result<(), DbError> {
    // Run seeds for status monitoring data (monitors, incidents, etc.)
    // If additional seed modules are added, invoke them here.

    // Status monitoring seed data
    status_monitoring::run_seeds(pool.clone()).await?;

    Ok(())
}