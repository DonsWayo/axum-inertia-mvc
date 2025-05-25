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

## Docker Deployment

### Production

To run the application in production mode:

```bash
docker-compose --profile prod up -d
```

This will build and run the app and worker services using the production Dockerfiles.

### Development with Live Reload

For development with live reload (changes to your code will automatically rebuild and restart the services):

```bash
# Start the development container
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

This setup provides:
- Live reload for the Rust backend (using cargo-watch)
- Hot module replacement for the React frontend (using Vite)
- Shared volume mounts so you can edit code on your host machine
- Automatic database migrations and seeding
- A unified development environment for both app and worker

## Component Documentation

For more detailed information about each component:

- [App Documentation](./app/README.md) - Web application details
- [Worker Documentation](./worker/README.md) - Background job processing details

## License

MIT
