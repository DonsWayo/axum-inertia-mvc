use db_core::seeds::run_all_seeds;
use db_core::init_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running database seeds...");
    let pool = init_pool().await?;
    run_all_seeds(pool).await?;
    println!("Database seeding complete!");
    Ok(())
} 