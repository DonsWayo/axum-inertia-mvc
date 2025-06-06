use crate::error::DbError;
use crate::models::monitor::{CreateMonitor, Monitor, UpdateMonitor};
use sqlx::PgPool;

pub struct MonitorRepository;

impl MonitorRepository {
    pub async fn create(pool: &PgPool, monitor: CreateMonitor) -> Result<Monitor, DbError> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            INSERT INTO monitors (name, display_name, description, url, monitor_type, check_interval, timeout, is_active, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            monitor.name,
            monitor.display_name,
            monitor.description,
            monitor.url,
            monitor.monitor_type,
            monitor.check_interval,
            monitor.timeout,
            monitor.is_active,
            monitor.metadata
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Monitor>, DbError> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            SELECT * FROM monitors WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Option<Monitor>, DbError> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            SELECT * FROM monitors WHERE name = $1
            "#,
            name
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn list_active(pool: &PgPool) -> Result<Vec<Monitor>, DbError> {
        let results = sqlx::query_as!(
            Monitor,
            r#"
            SELECT * FROM monitors WHERE is_active = true ORDER BY display_name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<Monitor>, DbError> {
        let results = sqlx::query_as!(
            Monitor,
            r#"
            SELECT * FROM monitors ORDER BY display_name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn update(pool: &PgPool, id: i32, monitor: UpdateMonitor) -> Result<Monitor, DbError> {
        let result = sqlx::query_as!(
            Monitor,
            r#"
            UPDATE monitors
            SET 
                name = COALESCE($2, name),
                display_name = COALESCE($3, display_name),
                description = COALESCE($4, description),
                url = COALESCE($5, url),
                monitor_type = COALESCE($6, monitor_type),
                check_interval = COALESCE($7, check_interval),
                timeout = COALESCE($8, timeout),
                is_active = COALESCE($9, is_active),
                metadata = COALESCE($10, metadata)
            WHERE id = $1
            RETURNING *
            "#,
            id,
            monitor.name,
            monitor.display_name,
            monitor.description,
            monitor.url,
            monitor.monitor_type,
            monitor.check_interval,
            monitor.timeout,
            monitor.is_active,
            monitor.metadata
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            DELETE FROM monitors WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}