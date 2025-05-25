use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct EmailRequest {
    to: String,
    subject: String,
    body: String,
}

#[derive(Serialize)]
pub struct JobResponse {
    job_id: String,
    status: String,
}

async fn queue_email(
    State(state): State<AppState>,
    Json(request): Json<EmailRequest>,
) -> Result<Json<JobResponse>, StatusCode> {
    let worker_service = &state.worker_service;
    
    let job_id = worker_service
        .queue_email(&request.to, &request.subject, &request.body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(JobResponse {
        job_id,
        status: "queued".to_string(),
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/jobs/email", post(queue_email))
}
