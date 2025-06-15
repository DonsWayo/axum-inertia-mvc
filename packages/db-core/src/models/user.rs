//! User model for OIDC authentication

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User entity representing authenticated users
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub oidc_subject: String,
    pub email: Option<String>,
    pub name: Option<String>,
    #[serde(with = "crate::time_serde")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::time_serde")]
    pub updated_at: time::OffsetDateTime,
    #[serde(with = "crate::time_serde::option")]
    pub last_login_at: Option<time::OffsetDateTime>,
}

/// Data for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub oidc_subject: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

/// Data for updating an existing user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub name: Option<String>,
} 