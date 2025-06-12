pub mod status_monitoring;

use crate::DbPool;
use crate::error::DbError;

pub async fn run_all_seeds(_pool: DbPool) -> Result<(), DbError> {
    // For now, we don't have any seeds to run
    // Add seed functions here as needed
    Ok(())
}