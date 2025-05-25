use axum::{Router, extract::{Extension, FromRef}};
use axum_inertia::{vite, InertiaConfig};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod routes;
mod db;
mod services;

// Define a combined application state
#[derive(Clone)]
struct AppState {
    db_pool: db::DbPool,
    inertia: InertiaConfig,
}

// Implement FromRef for DbPool
impl FromRef<AppState> for db::DbPool {
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
    let db_pool = db::init_db_pool().await.expect("Failed to initialize database pool");
    
    // Run seeds before starting the server
    db::seeds::runner::run_all_seeds(db_pool.clone()).await.expect("Failed to run seeds");
    
    // Configure Inertia for development
    let inertia = vite::Development::default()
        .port(5173)
        .main("src/main.tsx")
        .lang("en")
        .title("Axum Inertia MVC")
        .react()
        .into_config();

    // Create combined app state
    let app_state = AppState {
        db_pool,
        inertia,
    };
    
    // Create router with combined state
    let app = Router::new()
        .nest_service(
            "/assets",
            ServeDir::new("dist/assets"),
        )
        .merge(routes::home::router())
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started at http://{}", addr);
    
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
