use crate::error::DbError;
use crate::models::incident::{Incident, CreateIncident, UpdateIncident};
use sqlx::PgPool;
use chrono::Utc;

pub struct IncidentRepository;

impl IncidentRepository {
    pub async fn create(pool: &PgPool, incident: CreateIncident) -> Result<Incident, DbError> {
        let incident = sqlx::query_as!(
            Incident,
            r#"
            INSERT INTO incidents (
                title, message, severity, affected_monitors, 
                started_at, metadata
            )
            VALUES ($1, $2, $3, $4::INTEGER[], $5, $6)
            RETURNING id, title, message, severity, 
                affected_monitors as "affected_monitors!: Vec<i32>",
                created_at, updated_at, started_at, resolved_at, 
                is_resolved, metadata
            "#,
            incident.title,
            incident.message,
            incident.severity,
            &incident.affected_monitors[..],
            incident.started_at.unwrap_or_else(Utc::now),
            incident.metadata
        )
        .fetch_one(pool)
        .await?;

        Ok(incident)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        update: UpdateIncident,
    ) -> Result<Incident, DbError> {
        let incident = sqlx::query_as!(
            Incident,
            r#"
            UPDATE incidents
            SET
                title = COALESCE($2, title),
                message = COALESCE($3, message),
                severity = COALESCE($4, severity),
                affected_monitors = COALESCE($5::INTEGER[], affected_monitors),
                resolved_at = COALESCE($6, resolved_at),
                is_resolved = COALESCE($7, is_resolved),
                metadata = COALESCE($8, metadata),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, title, message, severity, 
                affected_monitors as "affected_monitors!: Vec<i32>",
                created_at, updated_at, started_at, resolved_at, 
                is_resolved, metadata
            "#,
            id,
            update.title,
            update.message,
            update.severity,
            update.affected_monitors.as_ref().map(|v| &v[..]),
            update.resolved_at,
            update.is_resolved,
            update.metadata
        )
        .fetch_one(pool)
        .await?;

        Ok(incident)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), DbError> {
        sqlx::query!(
            "DELETE FROM incidents WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Incident>, DbError> {
        let incident = sqlx::query_as!(
            Incident,
            r#"
            SELECT id, title, message, severity, 
                affected_monitors as "affected_monitors!: Vec<i32>",
                created_at, updated_at, started_at, resolved_at, 
                is_resolved, metadata
            FROM incidents
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(incident)
    }

    pub async fn list_active(pool: &PgPool) -> Result<Vec<Incident>, DbError> {
        let incidents = sqlx::query_as!(
            Incident,
            r#"
            SELECT id, title, message, severity, 
                affected_monitors as "affected_monitors!: Vec<i32>",
                created_at, updated_at, started_at, resolved_at, 
                is_resolved, metadata
            FROM incidents
            WHERE is_resolved = false
            ORDER BY severity DESC, created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(incidents)
    }

    pub async fn list_all(pool: &PgPool, limit: i64) -> Result<Vec<Incident>, DbError> {
        let incidents = sqlx::query_as!(
            Incident,
            r#"
            SELECT id, title, message, severity, 
                affected_monitors as "affected_monitors!: Vec<i32>",
                created_at, updated_at, started_at, resolved_at, 
                is_resolved, metadata
            FROM incidents
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(pool)
        .await?;

        Ok(incidents)
    }
}