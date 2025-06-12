use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use axum_inertia::Inertia;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::OffsetDateTime;
use db_core::DbPool;
use db_core::models::monitor::{CreateMonitor, UpdateMonitor};
use db_core::models::status_event::{CreateStatusEvent, StatusType};
use db_core::models::incident::{CreateIncident, UpdateIncident};
use db_core::repositories::IncidentRepository;
use crate::services::monitor_service::MonitorService;

#[derive(Debug, Deserialize)]
struct HeartbeatRequest {
    timestamp: String,
    metadata: Option<serde_json::Value>,
    stats: Option<HeartbeatStats>,
}

#[derive(Debug, Deserialize)]
struct HeartbeatStats {
    sent: u64,
    failed: u64,
    uptime: u64,
}

#[derive(Debug, Serialize)]
struct HeartbeatResponse {
    success: bool,
    timestamp: String,
    message: Option<String>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    DbPool: axum::extract::FromRef<S>,
    axum_inertia::InertiaConfig: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/status", get(status_page))
        .route("/api/monitors", get(list_monitors).post(create_monitor))
        .route("/api/monitors/{id}", get(get_monitor).put(update_monitor).delete(delete_monitor))
        .route("/api/monitors/{id}/events", post(record_event))
        .route("/api/heartbeat/{monitor_id}", post(receive_heartbeat))
        .route("/api/incidents", get(list_incidents).post(create_incident))
        .route("/api/incidents/{id}", put(update_incident).delete(delete_incident))
}

async fn status_page(
    State(pool): State<DbPool>,
    inertia: Inertia,
) -> impl IntoResponse {
    match MonitorService::get_status_page_data(&pool).await {
        Ok(data) => {
            inertia.render("EnhancedStatusPage", json!({
                "statusData": data,
            }))
        }
        Err(_) => inertia.render("EnhancedStatusPage", json!({
            "statusData": {
                "all_operational": true,
                "last_updated": OffsetDateTime::now_utc(),
                "monitors": [],
                "incidents": []
            }
        }))
    }
}

async fn list_monitors(State(pool): State<DbPool>) -> impl IntoResponse {
    match MonitorService::get_all_monitors(&pool).await {
        Ok(monitors) => Json(monitors).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn create_monitor(
    State(pool): State<DbPool>,
    Json(monitor): Json<CreateMonitor>,
) -> impl IntoResponse {
    match MonitorService::create_monitor(&pool, monitor).await {
        Ok(monitor) => (StatusCode::CREATED, Json(monitor)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn get_monitor(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match MonitorService::get_monitor(&pool, id).await {
        Ok(Some(monitor)) => Json(monitor).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn update_monitor(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(monitor): Json<UpdateMonitor>,
) -> impl IntoResponse {
    match MonitorService::update_monitor(&pool, id, monitor).await {
        Ok(monitor) => Json(monitor).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn delete_monitor(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match MonitorService::delete_monitor(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn record_event(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(mut event): Json<CreateStatusEvent>,
) -> impl IntoResponse {
    event.monitor_id = id;
    match MonitorService::record_status_event(&pool, event).await {
        Ok(event) => (StatusCode::CREATED, Json(event)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn list_incidents(State(pool): State<DbPool>) -> impl IntoResponse {
    match IncidentRepository::list_active(&pool).await {
        Ok(incidents) => Json(incidents).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn create_incident(
    State(pool): State<DbPool>,
    Json(incident): Json<CreateIncident>,
) -> impl IntoResponse {
    match IncidentRepository::create(&pool, incident).await {
        Ok(incident) => (StatusCode::CREATED, Json(incident)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn update_incident(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateIncident>,
) -> impl IntoResponse {
    match IncidentRepository::update(&pool, id, update).await {
        Ok(incident) => Json(incident).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn delete_incident(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match IncidentRepository::delete(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn receive_heartbeat(
    State(pool): State<DbPool>,
    Path(monitor_id): Path<String>,
    Json(heartbeat): Json<HeartbeatRequest>,
) -> impl IntoResponse {
    // Parse monitor ID
    let monitor_id = match monitor_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return Json(HeartbeatResponse {
                success: false,
                timestamp: OffsetDateTime::now_utc().to_string(),
                message: Some("Invalid monitor ID".to_string()),
            }).into_response();
        }
    };

    // Record the heartbeat as a status event
    let event = CreateStatusEvent {
        monitor_id,
        status: StatusType::Operational.into(),
        response_time: Some(1), // Heartbeats are always "1ms" since they're internal
        status_code: Some(200), // Heartbeats are successful
        error_message: Some("Heartbeat received".to_string()),
        metadata: heartbeat.metadata,
    };

    match MonitorService::record_status_event(&pool, event).await {
        Ok(_) => {
            Json(HeartbeatResponse {
                success: true,
                timestamp: OffsetDateTime::now_utc().to_string(),
                message: Some("Heartbeat recorded successfully".to_string()),
            }).into_response()
        }
        Err(e) => {
            Json(HeartbeatResponse {
                success: false,
                timestamp: OffsetDateTime::now_utc().to_string(),
                message: Some(format!("Failed to record heartbeat: {}", e)),
            }).into_response()
        }
    }
}