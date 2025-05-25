FROM rust:1.87-slim as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./
COPY worker/Cargo.toml ./worker/
COPY app/Cargo.toml ./app/

# Create dummy source files for dependency caching
RUN mkdir -p app/src worker/src && \
    echo 'fn main() { println!("Placeholder") }' > app/src/main.rs && \
    echo 'fn main() { println!("Placeholder") }' > worker/src/main.rs

# Build dependencies
RUN cargo build --release --package worker

# Copy the actual source code
COPY worker/src ./worker/src

# Build the application
RUN cargo build --release --package worker

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/worker /app/worker

# Set the entrypoint
ENTRYPOINT ["/app/worker"]
