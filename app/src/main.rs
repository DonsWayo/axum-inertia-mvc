use axum::{Router, extract::FromRef};
use axum_inertia::{vite, InertiaConfig};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use std::sync::Arc;

mod routes;
mod services;

// Define a combined application state
#[derive(Clone)]
struct AppState {
    db_pool: db_core::DbPool,
    inertia: InertiaConfig,
    worker_service: Arc<services::worker::WorkerService>,
}

// Implement FromRef for DbPool
impl FromRef<AppState> for db_core::DbPool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

// Implement FromRef for InertiaConfig
impl FromRef<AppState> for InertiaConfig {
    fn from_ref(state: &AppState) -> Self {
        state.inertia.clone()
    }
}

#[tokio::main]
async fn main() {
    // Initialize database connection pool
    let db_pool = db_core::init_pool().await.expect("Failed to initialize database pool");
    
    // Initialize worker service
    let worker_service = services::worker::WorkerService::new(db_pool.clone())
        .await
        .expect("Failed to initialize worker service");
    
    // Configure Inertia for development
    let inertia = vite::Development::default()
        .port(5173)
        .main("src/main.tsx")
        .lang("en")
        .title("RustGenie")
        .react()
        .into_config();

    // Create combined app state
    let app_state = AppState {
        db_pool,
        inertia,
        worker_service: Arc::new(worker_service),
    };
    
    // Create router with combined state
    let app = Router::new()
        .nest_service(
            "/assets",
            ServeDir::new("dist/assets"),
        )
        .merge(routes::home::router())
        .merge(routes::jobs::router())
        .merge(routes::status::router())
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started at http://{}", addr);
    
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
