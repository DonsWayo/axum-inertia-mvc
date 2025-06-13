FROM rust:1.87-slim as builder

WORKDIR /app

# Install Node.js for frontend assets
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl ca-certificates && \
    curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y --no-install-recommends nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./
COPY app/Cargo.toml ./app/

# Build dependencies
RUN cargo build --release --package rustgenie

# Copy the frontend files
COPY app/package.json ./app/
COPY app/vite.config.ts ./app/
COPY app/tsconfig.json ./app/
COPY app/tsconfig.node.json ./app/
COPY app/components.json ./app/

# Install npm dependencies
WORKDIR /app/app
RUN npm install

# Copy the source code
WORKDIR /app
COPY app/src ./app/src

# Build frontend assets
WORKDIR /app/app
RUN npm run build

# Build the application
WORKDIR /app
RUN cargo build --release --package rustgenie

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary and assets from the builder stage
COPY --from=builder /app/target/release/rustgenie /app/rustgenie
COPY --from=builder /app/app/dist /app/dist

# Set the entrypoint
ENTRYPOINT ["/app/rustgenie"]
