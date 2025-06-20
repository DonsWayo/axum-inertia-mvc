use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_inertia::Inertia;
use serde_json::json;
use db_core::DbPool;
use crate::services::document_service::DocumentService;

// We're using a generic parameter to allow the router to be merged with any state
// that can provide both DbPool and InertiaConfig via FromRef
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    DbPool: axum::extract::FromRef<S>,
    axum_inertia::InertiaConfig: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", get(index))
}

async fn index(
    State(pool): State<DbPool>,
    inertia: Inertia,
) -> impl IntoResponse {
    // Use the document service instead of repository directly
    let service = DocumentService::new(pool.clone());
    
    // Get all documents from the database
    let documents = match service.get_all().await {
        Ok(docs) => docs,
        Err(_) => vec![] // Return empty array if there's an error
    };

    // Get monitor status data for the dashboard
    let status_data = match crate::services::monitor_service::MonitorService::get_status_page_data(&pool).await {
        Ok(data) => Some(data),
        Err(_) => None
    };
    
    // Render the dashboard view with document and monitor data
    inertia.render("Dashboard", json!({
        "message": "Welcome to RustGenie - Your Full-Stack Rust Application",
        "documents": documents,
        "statusData": status_data
    }))
}
