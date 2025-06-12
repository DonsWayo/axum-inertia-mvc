use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub monitor_type: String,
    pub check_interval: i32,
    pub timeout: i32,
    pub is_active: bool,
    pub metadata: Option<JsonValue>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMonitor {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub monitor_type: String,
    pub check_interval: i32,
    pub timeout: i32,
    pub is_active: bool,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMonitor {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub monitor_type: Option<String>,
    pub check_interval: Option<i32>,
    pub timeout: Option<i32>,
    pub is_active: Option<bool>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MonitorType {
    Http,
    Tcp,
    Ping,
    Dns,
    Custom,
}

impl From<String> for MonitorType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "http" => MonitorType::Http,
            "tcp" => MonitorType::Tcp,
            "ping" => MonitorType::Ping,
            "dns" => MonitorType::Dns,
            _ => MonitorType::Custom,
        }
    }
}

impl From<MonitorType> for String {
    fn from(mt: MonitorType) -> Self {
        match mt {
            MonitorType::Http => "http".to_string(),
            MonitorType::Tcp => "tcp".to_string(),
            MonitorType::Ping => "ping".to_string(),
            MonitorType::Dns => "dns".to_string(),
            MonitorType::Custom => "custom".to_string(),
        }
    }
}