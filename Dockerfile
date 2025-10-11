FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*


# Create app directory
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*



# Create app directory
WORKDIR /app
# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/axum_backend /usr/local/bin/axum_backend


# Expose the port your app runs on
EXPOSE 3000

# Run the binary
CMD ["axum_backend"]
