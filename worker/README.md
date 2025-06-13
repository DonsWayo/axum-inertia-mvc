# RustGenie - Background Worker

A robust background job processing service built with Rust and graphile_worker, designed to work with the RustGenie web application.

## Overview

This worker service processes background jobs queued by the main web application. It uses graphile_worker, a high-performance job queue system that leverages PostgreSQL for reliable job storage and processing.

## Features

- **Task-based Architecture**: Modular task definitions with proper error handling
- **PostgreSQL-based Queue**: Jobs are stored in PostgreSQL for durability and reliability
- **Scalable**: Can be run as multiple instances for increased throughput
- **Fault-tolerant**: Jobs are retried automatically on failure
- **Email Sending**: Example task for sending emails

## Prerequisites

- **Rust**: 1.87.0 or newer
- **PostgreSQL**: 17 or newer
- **Docker**: Latest version (optional, for containerized deployment)

## Environment Setup

Create a `.env` file in the project root with the following variables:

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/rustgenie
```

## Running the Worker

```bash
cargo run --bin worker
```

## Task Implementation

The worker is designed to be easily extensible with new task types. Each task is implemented as a struct that implements the `TaskHandler` trait:

```rust
use graphile_worker::{IntoTaskHandlerResult, WorkerContext, TaskHandler};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SendEmail {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl TaskHandler for SendEmail {
    const IDENTIFIER: &'static str = "send_email";

    async fn run(self, _ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        // Email sending logic here
        Ok::<(), String>(())
    }
}
```

## Adding New Tasks

To add a new task type:

1. Create a new file in `src/tasks/` for your task
2. Implement the `TaskHandler` trait for your task struct
3. Register the task in `src/tasks/mod.rs`
4. Add an API endpoint in the app to queue the new task

## Docker Deployment

The worker includes a Dockerfile for containerized deployment:

```bash
docker build -f worker/Dockerfile -t axum-inertia-worker .
```

You can also use docker-compose to run the entire stack:

```bash
docker-compose up -d
```

## Architecture

The worker uses a modular architecture:

- **main.rs**: Entry point that initializes the worker and database connection
- **tasks/**: Contains task implementations
  - **mod.rs**: Registers all available tasks
  - **send_email.rs**: Example email sending task

## Integration with the Web Application

The worker integrates with the main web application through a shared PostgreSQL database. The app queues jobs by inserting records into the database, and the worker processes these jobs asynchronously.

See the [App Documentation](../app/README.md) for details on how the web application queues jobs.
