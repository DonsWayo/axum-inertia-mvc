use dotenv::dotenv;
use graphile_worker::WorkerOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

mod tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    info!("Starting worker...");

    // Get database connection string from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Initialize the worker with options
    let mut options = WorkerOptions::default()
        .concurrency(5)
        .schema("graphile_worker")
        .pg_pool(pool);
        
    // Register all tasks
    options = tasks::register_tasks(options);
    
    // Initialize the worker
    let worker = options.init().await?;

    // Create a utils helper to add jobs
    let utils = worker.create_utils();

    // Add a test job for demonstration
    utils.add_job(
        tasks::send_email::SendEmail {
            to: "test@example.com".to_string(),
            subject: "Test Email".to_string(),
            body: "This is a test email from the worker".to_string(),
        },
        Default::default(),
    )
    .await?;

    info!("Added test email job");

    // Run the worker
    worker.run().await?;

    Ok(())
}
