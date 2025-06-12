use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Incident {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub severity: String,
    pub affected_monitors: Vec<i32>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub started_at: OffsetDateTime,
    pub resolved_at: Option<OffsetDateTime>,
    pub is_resolved: bool,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIncident {
    pub title: String,
    pub message: String,
    pub severity: String,
    pub affected_monitors: Vec<i32>,
    pub started_at: Option<OffsetDateTime>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIncident {
    pub title: Option<String>,
    pub message: Option<String>,
    pub severity: Option<String>,
    pub affected_monitors: Option<Vec<i32>>,
    pub resolved_at: Option<OffsetDateTime>,
    pub is_resolved: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}