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
    // Load environment variables
    dotenv::dotenv().ok();
    
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
        db_pool: db_pool.clone(),
        inertia,
        worker_service: Arc::new(worker_service),
    };

    // Create auth router with all middleware built-in
    let auth_middleware_router = routes::auth::create_auth_router::<AppState>()
        .await
        .expect("Failed to create auth router");
    
    // Create main router with combined state
    let app = Router::new()
        .nest_service(
            "/assets",
            ServeDir::new("dist/assets"),
        )
        .merge(routes::home::router())
        .merge(routes::jobs::router())
        .merge(routes::monitors::router())
        .merge(routes::status::router())
        .merge(auth_middleware_router)
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started at http://{}", addr);
    println!("Auth routes available:");
    println!("  - /auth/login (login page)");
    println!("  - /auth/signin (start OIDC flow - redirects to provider)");
    println!("  - /auth/protected (requires login - will redirect to OIDC provider)");
    println!("  - /auth/maybe-protected (optional login)");
    println!("  - /auth/callback (OIDC callback endpoint)");
    println!("  - /auth/logout");
    println!();
    println!("Make sure to set these environment variables:");
    println!("  - OIDC_ISSUER_URL");
    println!("  - OIDC_CLIENT_ID");
    println!("  - OIDC_CLIENT_SECRET");
    println!("  - OIDC_REDIRECT_URL (should be: http://localhost:8000/auth/callback)");
    
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
