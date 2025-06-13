use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StatusEvent {
    #[serde(with = "crate::time_serde")]
    pub time: OffsetDateTime,
    pub monitor_id: i32,
    pub status: String,
    pub response_time: Option<i32>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
    pub metadata: Option<JsonValue>,
    #[serde(with = "crate::time_serde")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStatusEvent {
    pub monitor_id: i32,
    pub status: String,
    pub response_time: Option<i32>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatusType {
    Operational,
    Degraded,
    PartialOutage,
    MajorOutage,
    Maintenance,
    Unknown,
}

impl From<String> for StatusType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "operational" => StatusType::Operational,
            "degraded" => StatusType::Degraded,
            "partial_outage" => StatusType::PartialOutage,
            "major_outage" => StatusType::MajorOutage,
            "maintenance" => StatusType::Maintenance,
            _ => StatusType::Unknown,
        }
    }
}

impl From<StatusType> for String {
    fn from(st: StatusType) -> Self {
        match st {
            StatusType::Operational => "operational".to_string(),
            StatusType::Degraded => "degraded".to_string(),
            StatusType::PartialOutage => "partial_outage".to_string(),
            StatusType::MajorOutage => "major_outage".to_string(),
            StatusType::Maintenance => "maintenance".to_string(),
            StatusType::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StatusHourlyStat {
    #[serde(with = "crate::time_serde::option")]
    pub bucket: Option<OffsetDateTime>,
    pub monitor_id: Option<i32>,
    pub check_count: Option<i64>,
    pub operational_count: Option<i64>,
    pub incident_count: Option<i64>,
    pub avg_response_time: Option<i32>,
    pub min_response_time: Option<i32>,
    pub max_response_time: Option<i32>,
    pub p95_response_time: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StatusDailyStat {
    #[serde(with = "crate::time_serde::option")]
    pub bucket: Option<OffsetDateTime>,
    pub monitor_id: Option<i32>,
    pub check_count: Option<i64>,
    pub operational_count: Option<i64>,
    pub incident_count: Option<i64>,
    pub uptime_percentage: Option<f64>,
    pub avg_response_time: Option<i32>,
    pub p95_response_time: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorStatusSummary {
    pub monitor_id: i32,
    pub current_status: String,
    #[serde(with = "crate::time_serde")]
    pub last_check_time: OffsetDateTime,
    pub uptime_24h: f64,
    pub uptime_7d: f64,
    pub uptime_30d: f64,
    pub uptime_90d: f64,
    pub avg_response_time_24h: Option<i32>,
    pub incident_count_24h: i64,
}