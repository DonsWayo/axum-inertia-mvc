use db_core::reset::reset_database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Resetting database...");
    reset_database().await?;
    println!("Database reset complete!");
    Ok(())
}