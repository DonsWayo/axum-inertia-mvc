//! User repository for database operations

use sqlx::PgPool;
use crate::error::DbError;
use crate::models::user::{User, CreateUser, UpdateUser};

/// Repository for user database operations
pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    /// Create a new UserRepository instance
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(user)
    }

    /// Find user by OIDC subject
    pub async fn find_by_oidc_subject(&self, oidc_subject: &str) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE oidc_subject = $1"
        )
        .bind(oidc_subject)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create(&self, create_user: CreateUser) -> Result<User, DbError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (oidc_subject, email, name, last_login_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING *
            "#
        )
        .bind(&create_user.oidc_subject)
        .bind(&create_user.email)
        .bind(&create_user.name)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(user)
    }

    /// Update user's last login time
    pub async fn update_last_login(&self, id: i32) -> Result<(), DbError> {
        sqlx::query(
            "UPDATE users SET last_login_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Update user information
    pub async fn update(&self, id: i32, update_user: UpdateUser) -> Result<User, DbError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users 
            SET email = $2, name = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&update_user.email)
        .bind(&update_user.name)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(user)
    }

    /// Find or create user by OIDC subject
    pub async fn find_or_create_by_oidc_subject(
        &self,
        oidc_subject: &str,
        email: Option<&str>,
        name: Option<&str>,
    ) -> Result<User, DbError> {
        // Try to find existing user
        if let Some(user) = self.find_by_oidc_subject(oidc_subject).await? {
            // Update last login time
            self.update_last_login(user.id).await?;
            return Ok(user);
        }

        // Create new user
        let create_user = CreateUser {
            oidc_subject: oidc_subject.to_string(),
            email: email.map(|s| s.to_string()),
            name: name.map(|s| s.to_string()),
        };

        self.create(create_user).await
    }

    /// List all users (for admin purposes)
    pub async fn list(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, DbError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(users)
    }

    /// Count total users
    pub async fn count(&self) -> Result<i64, DbError> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users"
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(count.0)
    }

    /// Delete user by ID
    pub async fn delete(&self, id: i32) -> Result<(), DbError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }
} 