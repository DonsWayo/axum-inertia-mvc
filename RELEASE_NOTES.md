# Release Notes - v1.0.0

## Initial Release

### Features
- **Status Monitoring**: Complete monitoring system with HTTP and TCP checks
- **Real-time Updates**: TimescaleDB continuous aggregates with 1-minute refresh
- **Incident Management**: Create and manage incidents with severity levels
- **Service Grouping**: Organize monitors by service groups
- **Enhanced UI**: Beautiful status page with status tracker visualizations
- **Worker System**: Self-scheduling background jobs for continuous monitoring
- **Terraform Provider**: Infrastructure as Code support for monitor management

### Technical Highlights
- Rust backend with Axum web framework
- React + TypeScript frontend with Inertia.js
- TimescaleDB for time-series data
- GraphileWorker for background jobs
- Terraform provider for IaC

### Components
- Web application (port 8000)
- Worker service for background monitoring
- PostgreSQL + TimescaleDB database
- Terraform provider for monitor management