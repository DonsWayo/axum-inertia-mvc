pub mod send_email;

use graphile_worker::WorkerOptions;
use send_email::SendEmail;

pub fn register_tasks(options: WorkerOptions) -> WorkerOptions {
    options.define_job::<SendEmail>()
}
