# Axum Inertia MVC

A monolithic application built with Axum, Inertia.js, React, TailwindCSS, and PostgreSQL.

## Project Structure

```
axum-inertia-mvc/
├── src/
│   ├── db/             # Database layer
│   │   ├── migrations/ # Database migrations
│   │   ├── models/     # Data models
│   │   ├── seeds/      # Seed data
│   │   └── repositories/ # Data access layer
│   ├── routes/         # Route definitions
│   ├── views/          # Frontend components
│   │   ├── pages/      # React page components
│   │   ├── components/ # Reusable React components
│   │   └── layouts/    # Layout components
│   └── main.tsx        # React entry point
├── Cargo.toml          # Rust dependencies
├── package.json        # Node.js dependencies
└── vite.config.ts      # Vite configuration
```

## Prerequisites

- Rust (latest stable)
- Node.js (v22+)
- PostgreSQL (v17+)

## Development Setup

### 1. Set up PostgreSQL database

You can either use a local PostgreSQL installation or the provided Docker Compose setup:

#### Option A: Using Docker Compose (recommended)

```bash
# Start PostgreSQL with TimescaleDB and pgAI
docker-compose up -d

# Copy the example environment file
cp .env.example .env
```

#### Option B: Using local PostgreSQL

```bash
# Create a PostgreSQL database
psql -U postgres -c "CREATE DATABASE axum_inertia_mvc;"

# Copy the example environment file and update with your database credentials
cp .env.example .env
```

Edit the `.env` file to match your PostgreSQL configuration.

### 2. Install cargo-watch for Rust live reloading

```bash
cargo install cargo-watch
```

### 3. Install Node.js dependencies

```bash
npm install
```

### 4. Start the Vite development server

```bash
npm run dev
```

### 5. Start the Rust server with live reloading

In a separate terminal:

```bash
cargo watch -x run -w src -i src/views
```

This will watch for changes in the `src` directory, but ignore the `src/views` directory (which contains the frontend code). This ensures that the server only restarts when you change Rust code, not when you modify React components.

On first run, the application will automatically:
1. Run database migrations to create necessary tables
2. Seed the database with initial document data

## Building for Production

### 1. Build the frontend assets

```bash
npm run build
```

### 2. Build the Rust server

```bash
cargo build --release
```

### 3. Run the production server

```bash
./target/release/axum-inertia-mvc
```

## Features

- **Axum 0.8.4**: Modern Rust web framework
- **Inertia.js**: Server-driven SPA without building an API
- **React**: UI library with TypeScript
- **TailwindCSS**: Utility-first CSS framework
- **Vite**: Fast frontend build tool
- **PostgreSQL**: Robust relational database
- **SQLx**: Type-safe SQL toolkit for Rust
- **Database Migrations**: Automatic schema management
- **Seeding**: Automatic data seeding for development

## Development Notes

- The Vite configuration is set to only watch the `src/views/pages` directory to improve performance
- Use `cargo watch` for automatic server reloading during development
- The Inertia.js integration allows for a monolithic application structure while maintaining a modern frontend experience
- The database layer is structured with a clear separation of concerns:
  - **Models**: Define the structure of database entities
  - **Repositories**: Handle database operations for specific models
  - **Migrations**: Manage database schema changes
  - **Seeds**: Populate the database with initial data
- PostgreSQL is used as the primary database with SQLx for type-safe queries
