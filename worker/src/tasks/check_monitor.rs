use db_core::models::monitor::Monitor;
use db_core::models::status_event::CreateStatusEvent;
use db_core::repositories::{MonitorRepository, StatusEventRepository};
use db_core::DbPool;
use chrono::Utc;
use serde_json::json;
use graphile_worker::{IntoTaskHandlerResult, WorkerContext, TaskHandler};
use reqwest::Client;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckMonitor {
    pub monitor_id: i32,
}

impl TaskHandler for CheckMonitor {
    const IDENTIFIER: &'static str = "check_monitor";

    async fn run(self, ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        // Get database pool from context
        let pool = ctx.get_ext::<DbPool>()
            .ok_or_else(|| "Database pool not found in context".to_string())?;
        
        info!("Checking monitor {}", self.monitor_id);
        
        // Get monitor details
        debug!("Fetching monitor {} from database", self.monitor_id);
        let monitor = match MonitorRepository::find_by_id(&pool, self.monitor_id).await {
            Ok(Some(m)) => m,
            Ok(None) => {
                error!("Monitor {} not found", self.monitor_id);
                return Ok::<(), String>(());
            }
            Err(e) => {
                error!("Failed to fetch monitor {}: {}", self.monitor_id, e);
                return Err(e.to_string());
            }
        };
        
        if !monitor.is_active {
            info!("Monitor {} is not active, skipping check", self.monitor_id);
            return Ok::<(), String>(());
        }
        
        // Perform the check based on monitor type
        let check_result = match monitor.monitor_type.as_str() {
            "http" => check_http_monitor(&monitor).await,
            "tcp" => check_tcp_monitor(&monitor).await,
            _ => {
                error!("Unknown monitor type: {}", monitor.monitor_type);
                return Ok::<(), String>(());
            }
        };
        
        // Record the status event
        let (status, response_time) = match &check_result {
            Ok((status, response_time, _status_code)) => {
                info!("Monitor {} check succeeded: status={}, response_time={}ms", 
                    self.monitor_id, status, response_time);
                (status.clone(), Some(*response_time as i32))
            },
            Err(error_msg) => {
                error!("Monitor {} check failed: {}", self.monitor_id, error_msg);
                ("major_outage".to_string(), None)
            },
        };
        
        let event = match check_result {
            Ok((status, response_time, status_code)) => {
                CreateStatusEvent {
                    monitor_id: monitor.id,
                    status,
                    response_time: Some(response_time as i32),
                    status_code,
                    error_message: None,
                    metadata: Some(json!({
                        "checked_at": Utc::now().to_rfc3339(),
                        "monitor_type": monitor.monitor_type,
                    })),
                }
            },
            Err(error_msg) => {
                CreateStatusEvent {
                    monitor_id: monitor.id,
                    status: "major_outage".to_string(),
                    response_time: None,
                    status_code: None,
                    error_message: Some(error_msg),
                    metadata: Some(json!({
                        "checked_at": Utc::now().to_rfc3339(),
                        "monitor_type": monitor.monitor_type,
                    })),
                }
            },
        };
        
        if let Err(e) = StatusEventRepository::create(&pool, event).await {
            error!("Failed to record status event: {}", e);
            return Err(e.to_string());
        }
        
        debug!("Status event created for monitor {}: status={}, response_time={:?}ms", 
            self.monitor_id, 
            status,
            response_time
        );
        
        info!("Monitor {} check completed", self.monitor_id);
        
        // Schedule the next check for this monitor
        // We use a simple approach: always ensure there's a next check scheduled
        if let Some(pool) = ctx.get_ext::<DbPool>() {
            let utils = graphile_worker::WorkerUtils::new(pool.as_ref().clone(), "graphile_worker".to_string());
            let next_run = chrono::Utc::now() + chrono::Duration::seconds(monitor.check_interval as i64);
            let job_spec = graphile_worker::JobSpec {
                run_at: Some(next_run),
                job_key: Some(format!("monitor_{}_next", monitor.id)),
                job_key_mode: Some(graphile_worker::JobKeyMode::Replace),
                ..Default::default()
            };
            
            match utils.add_job(CheckMonitor { monitor_id: self.monitor_id }, job_spec).await {
                Ok(_) => debug!("Scheduled next check for monitor {} at {}", self.monitor_id, next_run),
                Err(e) => error!("Failed to schedule next check for monitor {}: {}", self.monitor_id, e),
            }
        }
        
        Ok::<(), String>(())
    }
}

async fn check_http_monitor(monitor: &Monitor) -> Result<(String, u64, Option<i32>), String> {
    let url = monitor.url.as_ref()
        .ok_or("No URL configured for HTTP monitor")?;
    
    debug!("Performing HTTP check for URL: {}", url);
    
    let client = Client::builder()
        .timeout(Duration::from_secs(monitor.timeout as u64))
        .user_agent("StatusMonitor/1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let start_time = std::time::Instant::now();
    
    let response = timeout(
        Duration::from_secs(monitor.timeout as u64),
        client.get(url).send()
    )
    .await
    .map_err(|_| format!("Request timed out after {} seconds", monitor.timeout))?
    .map_err(|e| format!("HTTP request failed: {}", e))?;
    
    let response_time = start_time.elapsed().as_millis() as u64;
    let status_code = response.status().as_u16() as i32;
    
    debug!("HTTP response: status_code={}, response_time={}ms", status_code, response_time);
    
    // Determine status based on response
    let status = if response.status().is_success() {
        if response_time > 3000 {
            "degraded".to_string()
        } else {
            "operational".to_string()
        }
    } else if response.status().is_server_error() {
        "major_outage".to_string()
    } else if response.status().is_client_error() {
        "partial_outage".to_string()
    } else {
        "degraded".to_string()
    };
    
    Ok((status, response_time, Some(status_code)))
}

async fn check_tcp_monitor(monitor: &Monitor) -> Result<(String, u64, Option<i32>), String> {
    use tokio::net::TcpStream;
    
    let url = monitor.url.as_ref()
        .ok_or("No URL configured for TCP monitor")?;
    
    // Parse host and port from URL
    let (host, port) = if url.starts_with("tcp://") {
        let without_prefix = &url[6..];
        parse_host_port(without_prefix)?
    } else {
        parse_host_port(url)?
    };
    
    let start_time = std::time::Instant::now();
    
    let result = timeout(
        Duration::from_secs(monitor.timeout as u64),
        TcpStream::connect(format!("{}:{}", host, port))
    )
    .await;
    
    let response_time = start_time.elapsed().as_millis() as u64;
    
    match result {
        Ok(Ok(_)) => {
            let status = if response_time > 1000 {
                "degraded".to_string()
            } else {
                "operational".to_string()
            };
            Ok((status, response_time, None))
        },
        Ok(Err(e)) => Err(format!("TCP connection failed: {}", e)),
        Err(_) => Err(format!("Connection timed out after {} seconds", monitor.timeout)),
    }
}

fn parse_host_port(url: &str) -> Result<(String, u16), String> {
    if let Some(colon_pos) = url.rfind(':') {
        let host = url[..colon_pos].to_string();
        let port_str = &url[colon_pos + 1..];
        let port = port_str.parse::<u16>()
            .map_err(|_| format!("Invalid port: {}", port_str))?;
        Ok((host, port))
    } else {
        Err("No port specified in URL".to_string())
    }
}