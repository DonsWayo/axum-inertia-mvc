use axum::Router;
use axum_inertia::vite;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() {
    // Configure Inertia for development
    let inertia = vite::Development::default()
        .port(5173)
        .main("src/main.tsx")
        .lang("en")
        .title("Axum Inertia MVC")
        .react()
        .into_config();

    // Create router
    let app = Router::new()
        .merge(routes::home::router())
        .nest_service(
            "/assets",
            ServeDir::new("dist/assets"),
        )
        .with_state(inertia);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started at http://{}", addr);
    
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
