use db_core::{repositories::MonitorRepository, DbPool};
use crate::tasks::check_monitor::CheckMonitor;
use graphile_worker::JobSpec;
use tracing::info;

/// Schedule recurring checks for all active monitors
pub async fn schedule_monitors_periodically(
    pool: DbPool,
    utils: &graphile_worker::WorkerUtils,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    info!("Scheduling initial monitor checks");
    
    let monitors = MonitorRepository::list_active(&pool).await?;
    let monitor_count = monitors.len();
    
    for monitor in monitors {
        info!("Scheduling initial check for monitor {}: {} (every {} seconds)", 
            monitor.id, monitor.display_name, monitor.check_interval);
        
        // Schedule immediate check with a unique key
        let job_spec = JobSpec {
            job_key: Some(format!("monitor_{}_initial", monitor.id)),
            job_key_mode: Some(graphile_worker::JobKeyMode::Replace),
            ..Default::default()
        };
        
        utils.add_job(
            CheckMonitor {
                monitor_id: monitor.id,
            },
            job_spec,
        ).await?;
    }
    
    info!("Scheduled initial checks for {} monitors", monitor_count);
    
    Ok(())
}

