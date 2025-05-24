# Axum Inertia MVC

A basic MVC application built with Axum, Inertia.js, React, and TailwindCSS.

## Project Structure

```
axum-inertia-mvc/
├── src/
│   ├── routes/         # Route definitions
│   ├── models/         # Data models
│   ├── pages/          # React page components
│   └── main.tsx        # React entry point
├── Cargo.toml          # Rust dependencies
├── package.json        # Node.js dependencies
└── vite.config.ts      # Vite configuration
```

## Prerequisites

- Rust (latest stable)
- Node.js (v22+)
- npm or yarn

## Development Setup

### 1. Install cargo-watch for Rust live reloading

```bash
cargo install cargo-watch
```

### 2. Install Node.js dependencies

```bash
npm install
```

### 3. Start the Vite development server

```bash
npm run dev
```

### 4. Start the Rust server with live reloading

In a separate terminal:

```bash
cargo watch -x run -w src -i src/views
```

This will watch for changes in the `src` directory, but ignore the `src/views` directory (which contains the frontend code). This ensures that the server only restarts when you change Rust code, not when you modify React components.

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

## Development Notes

- The Vite configuration is set to only watch the `src/pages` directory to improve performance
- Use `cargo watch` for automatic server reloading during development
- The Inertia.js integration allows for a monolithic application structure while maintaining a modern frontend experience
