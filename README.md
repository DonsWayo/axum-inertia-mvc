# Axum Inertia MVC with Background Worker

A modern Rust monorepo that combines Axum for API development with a background job processing system using graphile_worker.

![Dashboard Screenshot](docs/images/dashboard.png)

## Project Structure

This project is organized as a Rust workspace with multiple crates:

- `app`: The main web application using Axum and Inertia.js
- `worker`: A background job processing service using graphile_worker

## Key Features

- **Modern Web Framework**: Built with Axum, a high-performance web framework for Rust
- **Background Job Processing**: Utilizes graphile_worker for reliable PostgreSQL-based job queue
- **MVC Architecture**: Follows the Model-View-Controller pattern for clean code organization
- **Database Integration**: Uses SQLx for type-safe PostgreSQL interactions
- **API Endpoints**: RESTful API design with JSON responses
- **Docker Development**: Fully containerized development environment with live reload

## Prerequisites

- **Rust**: 1.87.0 or newer
- **Node.js**: 22.0.0 or newer (for frontend assets)
- **PostgreSQL**: 17 or newer
- **Docker**: Latest version (optional, for containerized deployment)

## Environment Setup

Create a `.env` file in the project root with the following variables:

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/axum_inertia_mvc
```

## Quick Start

1. **Setup the database**:

```bash
psql -c "CREATE DATABASE axum_inertia_mvc"
```

2. **Run the worker**:

```bash
cargo run --bin worker
```

3. **Run the web application**:

```bash
cargo run --bin axum-inertia-mvc
```

## Testing the Job Queue

Test the job queue by sending a POST request to the email endpoint:

```bash
curl -X POST http://localhost:8000/api/jobs/email \
  -H "Content-Type: application/json" \
  -d '{"to":"user@example.com","subject":"Test Email","body":"This is a test email."}'
```

## Docker Development & Deployment

This project is fully containerized for both development and production environments.

### Development with Live Reload

The recommended way to develop this application is using Docker with live reload:

```bash
# Start the development container and database
docker-compose --profile dev up -d

# Access the development container shell
docker-compose exec dev bash
```

Once inside the container, you can run the app and worker with live reload:

```bash
# In one terminal (inside the container)
cd /app && cargo watch -x 'run --bin axum-inertia-mvc' -w app/src -i app/src/views

# In another terminal (inside the container)
cd /app && cargo watch -x 'run --bin worker' -w worker/src

# For frontend development (in a third terminal inside the container)
cd /app/app && npm run dev
```

#### Development Environment Features

- **Live Reload**: Changes to Rust code automatically trigger recompilation and restart
- **Hot Module Replacement**: Frontend changes update in real-time without full page reloads
- **Shared Volume**: Edit code on your host machine, changes reflect immediately in the container
- **Unified Environment**: Consistent development environment for all team members
- **Database Integration**: PostgreSQL with TimescaleDB and vectorizer worker included
- **Automatic Migrations**: Database schema is automatically created and seeded

### Production Deployment

To run the application in production mode:

```bash
docker-compose --profile prod up -d
```

This builds and runs the optimized production containers for both the app and worker services.

## Component Documentation

For more detailed information about each component:

- [App Documentation](./app/README.md) - Web application details
- [Worker Documentation](./worker/README.md) - Background job processing details

## License

MIT
