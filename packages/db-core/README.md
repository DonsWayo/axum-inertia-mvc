# db-core

This crate provides the database layer for the RustGenie monorepo. It defines all models, migrations, repositories, and seed logic for the main application and worker services.

## Features
- **PostgreSQL/TimescaleDB** integration via [SQLx](https://github.com/launchbadge/sqlx)
- **Migrations**: All schema migrations are in `src/migrations/`
- **Models**: Rust structs for all database tables in `src/models/`
- **Repositories**: Query logic for each entity in `src/repositories/`
- **Seeds**: Populate the database with initial/sample data in `src/seeds/`
- **Database Reset**: Utility to drop, recreate, and migrate the database for local development

## Directory Structure

```
packages/db-core/
├── src/
│   ├── bin/                # CLI binaries (reset_db, seed_db)
│   ├── connection.rs       # Pool and migration logic
│   ├── error.rs            # Custom error types
│   ├── lib.rs              # Crate root
│   ├── migrations/         # SQL migration files
│   ├── models/             # Rust models for each table
│   ├── repositories/       # Query logic for each entity
│   ├── seeds/              # Seed logic for initial/sample data
│   └── reset.rs            # Database reset logic
├── Cargo.toml
└── README.md               # (this file)
```

## Usage

### 1. Migrations
Run all migrations (ensure your `DATABASE_URL` is set):

```bash
sqlx migrate run --source packages/db-core/src/migrations
```

### 2. Seeding
Seed the database with sample data:

```bash
cargo run --bin seed_db -p db-core
```

### 3. Resetting the Database
Drop, recreate, and migrate the database (DANGER: destroys all data!):

```bash
cargo run --bin reset_db -p db-core
```

## Development Workflow
- Add new tables/columns as `.sql` files in `src/migrations/`
- Add/update models in `src/models/`
- Add/update query logic in `src/repositories/`
- Add/update seed logic in `src/seeds/`
- Run migrations and seeds as above

## Environment
- Requires a running PostgreSQL/TimescaleDB instance (see project root `docker-compose.yml`)
- Set `DATABASE_URL` to point to your database

## Binaries
- `reset_db`: Drops, recreates, and migrates the database
- `seed_db`: Runs all seed logic to populate tables with sample data

## License
MIT 