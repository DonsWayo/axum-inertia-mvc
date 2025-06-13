use crate::error::DbError;
use crate::DbPool;
use sqlx::PgPool;
use serde_json::json;

pub async fn seed_monitors(pool: &PgPool) -> Result<(), DbError> {
    // Clear existing data
    sqlx::query!("DELETE FROM status_events").execute(pool).await?;
    sqlx::query!("DELETE FROM monitors").execute(pool).await?;
    sqlx::query!("DELETE FROM incidents").execute(pool).await?;
    
    // Seed CRM Application monitors with a mix of working and failing services
    let crm_monitors = vec![
        // CRM Frontend - Working
        (
            "crm-web",
            "CRM Web Application",
            "https://httpbin.org/status/200",
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
        // CRM Backend API - Will fail (404)
        (
            "crm-backend",
            "CRM Backend API",
            "https://httpbin.org/status/404",
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
        // Database Monitor - Will fail intermittently (50% chance)
        (
            "crm-database",
            "CRM Database",
            "https://httpbin.org/status/200,500",
            "http",
            45, // check every 45 seconds
            10, // 10 second timeout
            json!({
                "service_group": "CRM Application",
                "service_category": "database",
                "priority": 1,
                "expected_status_code": 200,
                "description": "PostgreSQL database (simulated with 50% failure rate)"
            })
        ),
        // Redis Cache - Working but slow (simulated degraded performance)
        (
            "crm-redis",
            "CRM Redis Cache",
            "https://httpbin.org/delay/4",
            "http",
            60, // check every minute
            10, // 10 second timeout
            json!({
                "service_group": "CRM Application",
                "service_category": "cache",
                "priority": 2,
                "expected_status_code": 200,
                "description": "Redis cache (simulated slow response)"
            })
        ),
        // Random test monitor - Working
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
        ),
        // External Service - Will timeout
        (
            "external-api",
            "External Payment API",
            "https://httpbin.org/delay/15",
            "http",
            90, // check every 90 seconds
            5, // 5 second timeout (will timeout since delay is 15s)
            json!({
                "service_group": "External Services",
                "service_category": "payment",
                "priority": 1,
                "expected_status_code": 200,
                "description": "External payment provider (simulated timeout)"
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
    
    // Note: We'll fetch the actual monitor IDs after creation
    let db_monitor_id = sqlx::query!("SELECT id FROM monitors WHERE name = 'crm-database'")
        .fetch_optional(pool)
        .await?
        .map(|r| r.id);
        
    let redis_monitor_id = sqlx::query!("SELECT id FROM monitors WHERE name = 'crm-redis'")
        .fetch_optional(pool)
        .await?
        .map(|r| r.id);
    
    // Seed some sample incidents
    let mut incidents = vec![];
    
    if let Some(db_id) = db_monitor_id {
        incidents.push((
            "Scheduled Database Maintenance",
            "We will be performing database maintenance on CRM PostgreSQL. The service may experience brief interruptions.",
            "warning",
            vec![db_id],
            false,
            json!({
                "scheduled": true,
                "estimated_duration": "2 hours"
            })
        ));
    }
    
    if let Some(redis_id) = redis_monitor_id {
        incidents.push((
            "Redis Performance Degradation",
            "We are investigating slow response times from the Redis cache. This may cause slower page loads.",
            "warning",
            vec![redis_id],
            false,
            json!({
                "impact": "minor",
                "team": "infrastructure"
            })
        ));
    }
    
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