# =============================================================================
# ACC LMS - Backend Dockerfile (Rust Microservices)
# =============================================================================
# Multi-stage build for optimized production images
# Stages: chef → planner → builder → development → production
#
# Rust Version: 1.85 (see ADR-001 in _docs/development/development-standards.md)
#
# Build args:
#   SERVICE_NAME - Name of the service to build (auth, users, courses, etc.)
#
# Usage:
#   docker build --build-arg SERVICE_NAME=auth -t acc-lms-auth .
#   docker build --build-arg SERVICE_NAME=auth --target development -t acc-lms-auth-dev .
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1: Chef - Prepare cargo-chef for dependency caching
# -----------------------------------------------------------------------------
FROM rust:1.85-slim-bookworm AS chef

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked

WORKDIR /app

# -----------------------------------------------------------------------------
# Stage 2: Planner - Generate recipe.json for dependency caching
# -----------------------------------------------------------------------------
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# -----------------------------------------------------------------------------
# Stage 3: Builder - Build dependencies (cached) and application
# -----------------------------------------------------------------------------
FROM chef AS builder

ARG SERVICE_NAME=auth

# Copy recipe and build dependencies (this layer is cached)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build the application
COPY . .
RUN cargo build --release --bin ${SERVICE_NAME}

# -----------------------------------------------------------------------------
# Stage 4: Development - Full Rust environment with hot reload
# -----------------------------------------------------------------------------
FROM rust:1-slim-bookworm AS development

ARG SERVICE_NAME=auth

# Install development dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for hot reload
RUN cargo install cargo-watch --locked

WORKDIR /app

# Copy source code
COPY . .

# Expose service port
EXPOSE 8080

# Environment variables
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug
ENV SERVICE_NAME=${SERVICE_NAME}

# Development command with hot reload
CMD ["sh", "-c", "cargo watch -x 'run --bin ${SERVICE_NAME}'"]

# -----------------------------------------------------------------------------
# Stage 5: Production - Minimal runtime image
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim AS production

ARG SERVICE_NAME=auth

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false appuser

WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/${SERVICE_NAME} /app/service

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose service port
EXPOSE 8080

# Environment variables
ENV RUST_LOG=info
ENV SERVICE_PORT=8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the service
CMD ["/app/service"]
