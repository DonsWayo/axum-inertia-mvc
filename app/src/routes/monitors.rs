use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_inertia::Inertia;
use serde_json::json;
use db_core::DbPool;
use crate::services::monitor_service::MonitorService;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    DbPool: axum::extract::FromRef<S>,
    axum_inertia::InertiaConfig: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/monitors", get(monitors_list))
        .route("/monitors/new", get(monitor_new))
        .route("/monitors/{id}", get(monitor_detail))
        .route("/monitors/{id}/edit", get(monitor_edit))
}

async fn monitors_list(
    State(pool): State<DbPool>,
    inertia: Inertia,
) -> impl IntoResponse {
    match MonitorService::get_status_page_data(&pool).await {
        Ok(data) => {
            inertia.render("Monitors", json!({
                "monitors": data.monitors,
            }))
        }
        Err(_) => inertia.render("Monitors", json!({
            "monitors": [],
        }))
    }
}

async fn monitor_new(inertia: Inertia) -> impl IntoResponse {
    inertia.render("MonitorEdit", json!({
        "isNew": true,
    }))
}

async fn monitor_detail(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    inertia: Inertia,
) -> impl IntoResponse {
    match MonitorService::get_monitor_detail(&pool, id).await {
        Ok(Some(data)) => {
            inertia.render("MonitorDetail", json!({
                "monitor": data.monitor,
                "summary": data.summary,
                "tracker_data": data.tracker_data,
                "recent_events": data.recent_events,
            }))
        }
        Ok(None) => {
            // Monitor not found - redirect to monitors list
            inertia.render("Monitors", json!({
                "monitors": [],
                "error": "Monitor not found",
            }))
        }
        Err(_) => {
            inertia.render("Monitors", json!({
                "monitors": [],
                "error": "Failed to load monitor details",
            }))
        }
    }
}

async fn monitor_edit(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    inertia: Inertia,
) -> impl IntoResponse {
    match MonitorService::get_monitor(&pool, id).await {
        Ok(Some(monitor)) => {
            inertia.render("MonitorEdit", json!({
                "monitor": monitor,
                "isNew": false,
            }))
        }
        Ok(None) => {
            // Monitor not found - redirect to monitors list
            inertia.render("Monitors", json!({
                "monitors": [],
                "error": "Monitor not found",
            }))
        }
        Err(_) => {
            inertia.render("Monitors", json!({
                "monitors": [],
                "error": "Failed to load monitor",
            }))
        }
    }
} 