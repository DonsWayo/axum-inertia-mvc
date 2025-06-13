use db_core::error::DbError;
use db_core::models::monitor::{CreateMonitor, Monitor, UpdateMonitor};
use db_core::models::status_event::{
    CreateStatusEvent, MonitorStatusSummary, StatusEvent, StatusDailyStat,
};
use db_core::models::incident::Incident;
use db_core::repositories::{MonitorRepository, StatusEventRepository, IncidentRepository};
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use db_core::DbPool;
use tracing::debug;

fn month_name(month: time::Month) -> &'static str {
    match month {
        time::Month::January => "Jan",
        time::Month::February => "Feb", 
        time::Month::March => "Mar",
        time::Month::April => "Apr",
        time::Month::May => "May",
        time::Month::June => "Jun",
        time::Month::July => "Jul",
        time::Month::August => "Aug",
        time::Month::September => "Sep",
        time::Month::October => "Oct",
        time::Month::November => "Nov",
        time::Month::December => "Dec",
    }
}

pub struct MonitorService;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorWithStatus {
    pub monitor: Monitor,
    pub current_status: String,
    #[serde(with = "db_core::time_serde::option")]
    pub last_check_time: Option<OffsetDateTime>,
    pub uptime_percentage: f64,
    pub daily_stats: Vec<StatusDailyStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceGroup {
    pub name: String,
    pub description: Option<String>,
    pub monitors: Vec<MonitorWithStatus>,
    pub overall_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusPageData {
    pub all_operational: bool,
    #[serde(with = "db_core::time_serde")]
    pub last_updated: OffsetDateTime,
    pub monitors: Vec<MonitorWithStatus>,
    pub incidents: Vec<Incident>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorDetailData {
    pub monitor: Monitor,
    pub summary: MonitorStatusSummary,
    pub tracker_data: Vec<TrackerDataPoint>,
    pub recent_events: Vec<StatusEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerDataPoint {
    pub date: String,
    pub tooltip: String,
    pub status: String,
}

impl MonitorService {
    pub async fn create_monitor(
        pool: &DbPool,
        monitor: CreateMonitor,
    ) -> Result<Monitor, DbError> {
        MonitorRepository::create(pool, monitor).await
    }

    pub async fn get_monitor(pool: &DbPool, id: i32) -> Result<Option<Monitor>, DbError> {
        MonitorRepository::find_by_id(pool, id).await
    }

    pub async fn update_monitor(
        pool: &DbPool,
        id: i32,
        monitor: UpdateMonitor,
    ) -> Result<Monitor, DbError> {
        MonitorRepository::update(pool, id, monitor).await
    }

    pub async fn delete_monitor(pool: &DbPool, id: i32) -> Result<(), DbError> {
        MonitorRepository::delete(pool, id).await
    }

    pub async fn record_status_event(
        pool: &DbPool,
        event: CreateStatusEvent,
    ) -> Result<StatusEvent, DbError> {
        StatusEventRepository::create(pool, event).await
    }

    pub async fn get_status_page_data(pool: &DbPool) -> Result<StatusPageData, DbError> {
        let monitors = MonitorRepository::list_active(pool).await?;
        let incidents = IncidentRepository::list_active(pool).await?;
        let mut monitors_with_status = Vec::new();
        let mut all_operational = true;

        for monitor in monitors {
            let summary = StatusEventRepository::get_monitor_summary(pool, monitor.id).await?;
            let daily_stats = StatusEventRepository::get_daily_stats(pool, monitor.id, 90).await?;
            
            debug!("Monitor {} daily_stats count: {}", monitor.id, daily_stats.len());
            if !daily_stats.is_empty() {
                debug!("First stat: {:?}", daily_stats.first());
                debug!("Last stat: {:?}", daily_stats.last());
            }
            
            if summary.current_status != "operational" {
                all_operational = false;
            }

            monitors_with_status.push(MonitorWithStatus {
                monitor,
                current_status: summary.current_status,
                last_check_time: Some(summary.last_check_time),
                uptime_percentage: summary.uptime_90d,
                daily_stats,
            });
        }
        
        // If there are active incidents, we're not fully operational
        if !incidents.is_empty() {
            all_operational = false;
        }

        Ok(StatusPageData {
            all_operational,
            last_updated: OffsetDateTime::now_utc(),
            monitors: monitors_with_status,
            incidents,
        })
    }

    pub async fn get_monitor_detail(
        pool: &DbPool,
        monitor_id: i32,
    ) -> Result<Option<MonitorDetailData>, DbError> {
        let monitor = match MonitorRepository::find_by_id(pool, monitor_id).await? {
            Some(m) => m,
            None => return Ok(None),
        };

        let summary = StatusEventRepository::get_monitor_summary(pool, monitor_id).await?;
        let tracker_raw = StatusEventRepository::get_status_tracker_data(pool, monitor_id, 90).await?;
        let recent_events = StatusEventRepository::get_recent_events(pool, monitor_id, 20).await?;

        let tracker_data: Vec<TrackerDataPoint> = tracker_raw
            .into_iter()
            .map(|(date, status)| {
                let tooltip = match status.as_str() {
                    "operational" => "Operational",
                    "degraded" => "Degraded Performance",
                    "partial_outage" => "Partial Outage",
                    "major_outage" => "Major Outage",
                    "maintenance" => "Maintenance",
                    _ => "Unknown",
                };

                TrackerDataPoint {
                    date: format!("{:02} {}, {}", date.day(), month_name(date.month()), date.year()),
                    tooltip: tooltip.to_string(),
                    status,
                }
            })
            .collect();

        Ok(Some(MonitorDetailData {
            monitor,
            summary,
            tracker_data,
            recent_events,
        }))
    }

    pub async fn get_all_monitors(pool: &DbPool) -> Result<Vec<Monitor>, DbError> {
        MonitorRepository::list_all(pool).await
    }
}