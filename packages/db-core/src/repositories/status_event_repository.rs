use crate::error::DbError;
use crate::models::status_event::{
    CreateStatusEvent, MonitorStatusSummary, StatusDailyStat, StatusEvent, StatusHourlyStat,
};
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;

pub struct StatusEventRepository;

impl StatusEventRepository {
    pub async fn create(pool: &PgPool, event: CreateStatusEvent) -> Result<StatusEvent, DbError> {
        let result = sqlx::query_as!(
            StatusEvent,
            r#"
            INSERT INTO status_events (time, monitor_id, status, response_time, status_code, error_message, metadata)
            VALUES (NOW(), $1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            event.monitor_id,
            event.status,
            event.response_time,
            event.status_code,
            event.error_message,
            event.metadata
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_recent_events(
        pool: &PgPool,
        monitor_id: i32,
        limit: i64,
    ) -> Result<Vec<StatusEvent>, DbError> {
        let results = sqlx::query_as!(
            StatusEvent,
            r#"
            SELECT * FROM status_events 
            WHERE monitor_id = $1 
            ORDER BY time DESC 
            LIMIT $2
            "#,
            monitor_id,
            limit
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn get_events_in_range(
        pool: &PgPool,
        monitor_id: i32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<StatusEvent>, DbError> {
        let results = sqlx::query_as!(
            StatusEvent,
            r#"
            SELECT * FROM status_events 
            WHERE monitor_id = $1 AND time >= $2 AND time <= $3
            ORDER BY time DESC
            "#,
            monitor_id,
            start_time,
            end_time
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn get_latest_status(
        pool: &PgPool,
        monitor_id: i32,
    ) -> Result<Option<StatusEvent>, DbError> {
        let result = sqlx::query_as!(
            StatusEvent,
            r#"
            SELECT * FROM status_events 
            WHERE monitor_id = $1 
            ORDER BY time DESC 
            LIMIT 1
            "#,
            monitor_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_hourly_stats(
        pool: &PgPool,
        monitor_id: i32,
        hours: i32,
    ) -> Result<Vec<StatusHourlyStat>, DbError> {
        let start_time = Utc::now() - Duration::hours(hours as i64);
        
        let results = sqlx::query_as!(
            StatusHourlyStat,
            r#"
            SELECT 
                bucket as "bucket?",
                monitor_id as "monitor_id?",
                check_count as "check_count?",
                operational_count as "operational_count?",
                incident_count as "incident_count?",
                avg_response_time,
                min_response_time,
                max_response_time,
                p95_response_time
            FROM status_hourly_stats
            WHERE monitor_id = $1 AND bucket >= $2
            ORDER BY bucket DESC
            "#,
            monitor_id,
            start_time
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn get_daily_stats(
        pool: &PgPool,
        monitor_id: i32,
        days: i32,
    ) -> Result<Vec<StatusDailyStat>, DbError> {
        let start_time = Utc::now() - Duration::days(days as i64);
        
        let results = sqlx::query_as!(
            StatusDailyStat,
            r#"
            SELECT 
                bucket as "bucket?",
                monitor_id as "monitor_id?",
                check_count as "check_count?",
                operational_count as "operational_count?",
                incident_count as "incident_count?",
                uptime_percentage::FLOAT8 as "uptime_percentage?",
                avg_response_time,
                p95_response_time
            FROM status_daily_stats
            WHERE monitor_id = $1 AND bucket >= $2
            ORDER BY bucket DESC
            "#,
            monitor_id,
            start_time
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn get_monitor_summary(
        pool: &PgPool,
        monitor_id: i32,
    ) -> Result<MonitorStatusSummary, DbError> {
        // Get latest status
        let latest_status = sqlx::query!(
            r#"
            SELECT status, time as last_check_time
            FROM status_events
            WHERE monitor_id = $1
            ORDER BY time DESC
            LIMIT 1
            "#,
            monitor_id
        )
        .fetch_optional(pool)
        .await?;

        let (current_status, last_check_time) = match latest_status {
            Some(row) => (row.status, row.last_check_time),
            None => ("unknown".to_string(), Utc::now()),
        };

        // Calculate uptime percentages
        let uptime_24h = sqlx::query!(
            r#"
            SELECT COALESCE(
                COUNT(CASE WHEN status = 'operational' THEN 1 END)::FLOAT / 
                NULLIF(COUNT(*), 0)::FLOAT * 100, 100
            ) as uptime
            FROM status_events 
            WHERE monitor_id = $1 AND time >= NOW() - INTERVAL '24 hours'
            "#,
            monitor_id
        )
        .fetch_one(pool)
        .await?
        .uptime.unwrap_or(100.0);

        let uptime_7d = sqlx::query!(
            r#"
            SELECT COALESCE(
                COUNT(CASE WHEN status = 'operational' THEN 1 END)::FLOAT / 
                NULLIF(COUNT(*), 0)::FLOAT * 100, 100
            ) as uptime
            FROM status_events 
            WHERE monitor_id = $1 AND time >= NOW() - INTERVAL '7 days'
            "#,
            monitor_id
        )
        .fetch_one(pool)
        .await?
        .uptime.unwrap_or(100.0);

        let uptime_30d = sqlx::query!(
            r#"
            SELECT COALESCE(
                COUNT(CASE WHEN status = 'operational' THEN 1 END)::FLOAT / 
                NULLIF(COUNT(*), 0)::FLOAT * 100, 100
            ) as uptime
            FROM status_events 
            WHERE monitor_id = $1 AND time >= NOW() - INTERVAL '30 days'
            "#,
            monitor_id
        )
        .fetch_one(pool)
        .await?
        .uptime.unwrap_or(100.0);

        let uptime_90d = sqlx::query!(
            r#"
            SELECT COALESCE(
                COUNT(CASE WHEN status = 'operational' THEN 1 END)::FLOAT / 
                NULLIF(COUNT(*), 0)::FLOAT * 100, 100
            ) as uptime
            FROM status_events 
            WHERE monitor_id = $1 AND time >= NOW() - INTERVAL '90 days'
            "#,
            monitor_id
        )
        .fetch_one(pool)
        .await?
        .uptime.unwrap_or(100.0);

        // Get average response time
        let avg_response_time_24h = sqlx::query!(
            r#"
            SELECT AVG(response_time)::INTEGER as avg_time
            FROM status_events 
            WHERE monitor_id = $1 AND time >= NOW() - INTERVAL '24 hours'
            "#,
            monitor_id
        )
        .fetch_one(pool)
        .await?
        .avg_time;

        // Get incident count
        let incident_count_24h = sqlx::query!(
            r#"
            SELECT COUNT(*)::BIGINT as count
            FROM status_events 
            WHERE monitor_id = $1 
            AND time >= NOW() - INTERVAL '24 hours'
            AND status != 'operational'
            "#,
            monitor_id
        )
        .fetch_one(pool)
        .await?
        .count.unwrap_or(0);

        Ok(MonitorStatusSummary {
            monitor_id,
            current_status,
            last_check_time,
            uptime_24h,
            uptime_7d,
            uptime_30d,
            uptime_90d,
            avg_response_time_24h,
            incident_count_24h,
        })
    }

    pub async fn get_status_tracker_data(
        pool: &PgPool,
        monitor_id: i32,
        days: i32,
    ) -> Result<Vec<(DateTime<Utc>, String)>, DbError> {
        let start_time = Utc::now() - Duration::days(days as i64);
        
        let results: Vec<(DateTime<Utc>, String)> = sqlx::query!(
            r#"
            WITH daily_status AS (
                SELECT 
                    DATE_TRUNC('day', time) as day,
                    CASE 
                        WHEN COUNT(CASE WHEN status != 'operational' THEN 1 END) = 0 THEN 'operational'
                        WHEN COUNT(CASE WHEN status = 'major_outage' THEN 1 END) > 0 THEN 'major_outage'
                        WHEN COUNT(CASE WHEN status = 'partial_outage' THEN 1 END) > 0 THEN 'partial_outage'
                        WHEN COUNT(CASE WHEN status = 'degraded' THEN 1 END) > 0 THEN 'degraded'
                        WHEN COUNT(CASE WHEN status = 'maintenance' THEN 1 END) > 0 THEN 'maintenance'
                        ELSE 'unknown'
                    END as daily_status
                FROM status_events
                WHERE monitor_id = $1 AND time >= $2
                GROUP BY DATE_TRUNC('day', time)
            )
            SELECT day as "day!", daily_status as "status!"
            FROM daily_status
            ORDER BY day
            "#,
            monitor_id,
            start_time
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| (r.day, r.status))
        .collect();

        Ok(results)
    }
}