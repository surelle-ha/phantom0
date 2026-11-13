
# ==========================================
# BUILD STAGE
# ==========================================
FROM rust:1-bookworm AS builder

WORKDIR /app

# Copy entire Cargo workspace
COPY . .

# Build the selected service
RUN cargo build --release -p notification-service

# ==========================================
# RUNTIME STAGE
# ==========================================
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/notification-service /app/notification-service

EXPOSE 8334

CMD ["/app/notification-service"]
