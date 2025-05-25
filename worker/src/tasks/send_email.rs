use graphile_worker::{IntoTaskHandlerResult, WorkerContext, TaskHandler};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use tracing::info;

#[derive(Deserialize, Serialize)]
pub struct SendEmail {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl TaskHandler for SendEmail {
    const IDENTIFIER: &'static str = "send_email";

    async fn run(self, _ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        // In a real implementation, you would connect to an email service here
        // For now, we'll just log the email details
        info!(
            "Would send email to: {}, subject: {}, body: {}",
            self.to, self.subject, self.body
        );
        
        // Simulate some processing time
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        info!("Email task completed successfully");
        
        Ok::<(), String>(())
    }
}
