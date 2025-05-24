use axum::{
    Router,
    routing::get,
    response::IntoResponse,
};
use axum_inertia::{Inertia, InertiaConfig};
use serde_json::json;

pub fn router() -> Router<InertiaConfig> {
    Router::new()
        .route("/", get(index))
}

async fn index(i: Inertia) -> impl IntoResponse {
    i.render(
        "Dashboard",
        json!({
            "message": "Welcome to Axum Inertia MVC with Views Structure",
        }),
    )
}
