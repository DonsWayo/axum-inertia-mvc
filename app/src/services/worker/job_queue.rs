use graphile_worker::{WorkerOptions, WorkerUtils, Job};
use serde_json::Value;
use db_core::DbPool;
use std::sync::Arc;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum WorkerError {
    #[error("Failed to create job: {0}")]
    JobCreationError(String),
    
    #[error("Failed to initialize worker: {0}")]
    InitializationError(String),
}

pub struct WorkerService {
    utils: WorkerUtils,
}

impl WorkerService {
    pub async fn new(pool: DbPool) -> Result<Self, WorkerError> {
        // Initialize a worker with minimal configuration just to get the utils
        let worker = WorkerOptions::default()
            .schema("graphile_worker")
            .pg_pool(pool.as_ref().clone())
            .init()
            .await
            .map_err(|e| WorkerError::InitializationError(e.to_string()))?;

        // Create the utils helper
        let utils = worker.create_utils();

        Ok(Self { utils })
    }

    /// Queue a job to be processed by the worker
    pub async fn queue_job(
        &self,
        task_name: &str,
        payload: Value,
    ) -> Result<String, WorkerError> {
        // Add the job to the queue
        let job: Job = self.utils
            .add_raw_job(
                task_name,
                payload,
                Default::default(),
            )
            .await
            .map_err(|e| WorkerError::JobCreationError(e.to_string()))?;

        info!("Queued job {:?} with task {}", job, task_name);

        // Convert job ID to string
        Ok(job.id().to_string())
    }

    /// Queue an email to be sent
    pub async fn queue_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<String, WorkerError> {
        let payload = serde_json::json!({
            "to": to,
            "subject": subject,
            "body": body
        });

        self.queue_job("send_email", payload).await
    }
}
