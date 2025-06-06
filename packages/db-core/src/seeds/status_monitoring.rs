use crate::error::DbError;
use crate::DbPool;
use sqlx::PgPool;
use serde_json::json;

pub async fn seed_monitors(pool: &PgPool) -> Result<(), DbError> {
    // Clear existing data
    sqlx::query!("DELETE FROM status_events").execute(pool).await?;
    sqlx::query!("DELETE FROM monitors").execute(pool).await?;
    sqlx::query!("DELETE FROM incidents").execute(pool).await?;
    
    // Seed CRM Application monitors
    let crm_monitors = vec![
        // CRM Frontend
        (
            "crm-web",
            "CRM Web Application",
            "https://crm.conek.cloud",
            "http",
            60, // check every minute
            30, // 30 second timeout
            json!({
                "service_group": "CRM Application",
                "service_category": "frontend",
                "priority": 1,
                "expected_status_code": 200,
                "check_ssl": true
            })
        ),
        // CRM Backend API
        (
            "crm-backend",
            "CRM Backend API",
            "https://crm-backend.conek.cloud",
            "http",
            30, // check every 30 seconds
            10, // 10 second timeout
            json!({
                "service_group": "CRM Application",
                "service_category": "backend",
                "priority": 1,
                "expected_status_code": 200,
                "check_json": true,
                "expected_response": {"status": "healthy"}
            })
        ),
        // Random test monitor
        (
            "test-api",
            "Test API Service",
            "https://jsonplaceholder.typicode.com/posts/1",
            "http",
            120, // check every 2 minutes
            10, // 10 second timeout
            json!({
                "service_group": "Test Services",
                "service_category": "api",
                "priority": 3,
                "expected_status_code": 200,
                "description": "Test monitor for a public API"
            })
        )
    ];
    
    
    
    // Insert all monitors
    for (name, display_name, url, monitor_type, interval, timeout, metadata) in 
        crm_monitors.into_iter() {
        
        sqlx::query!(
            r#"
            INSERT INTO monitors (name, display_name, url, monitor_type, check_interval, timeout, is_active, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, true, $7)
            "#,
            name,
            display_name,
            url,
            monitor_type,
            interval,
            timeout,
            metadata
        )
        .execute(pool)
        .await?;
    }
    
    // Seed some sample incidents
    let incidents = vec![
        (
            "Scheduled Database Maintenance",
            "We will be performing database maintenance on CRM PostgreSQL. The service may experience brief interruptions.",
            "warning",
            vec![4], // CRM Database monitor ID (adjust based on actual IDs)
            false,
            json!({
                "scheduled": true,
                "estimated_duration": "2 hours"
            })
        ),
        (
            "Redis Performance Degradation",
            "We are investigating slow response times from the Redis cache. This may cause slower page loads.",
            "warning",
            vec![5], // CRM Redis monitor ID
            false,
            json!({
                "impact": "minor",
                "team": "infrastructure"
            })
        ),
    ];
    
    for (title, message, severity, affected_monitors, is_resolved, metadata) in incidents {
        sqlx::query!(
            r#"
            INSERT INTO incidents (title, message, severity, affected_monitors, is_resolved, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            title,
            message,
            severity,
            &affected_monitors[..],
            is_resolved,
            metadata
        )
        .execute(pool)
        .await?;
    }
    
    println!("✅ Seeded monitors and incidents successfully!");
    
    Ok(())
}

pub async fn run_seeds(pool: DbPool) -> Result<(), DbError> {
    seed_monitors(&pool).await?;
    Ok(())
}