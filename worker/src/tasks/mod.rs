pub mod send_email;
pub mod check_monitor;
pub mod schedule_monitors;

use graphile_worker::WorkerOptions;
use send_email::SendEmail;
use check_monitor::CheckMonitor;

pub fn register_tasks(options: WorkerOptions) -> WorkerOptions {
    options
        .define_job::<SendEmail>()
        .define_job::<CheckMonitor>()
}
