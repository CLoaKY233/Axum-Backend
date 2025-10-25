FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
RUN cargo init --bin
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm -f src/*.rs
COPY src ./src
RUN cargo build --release


FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/axum_backend /usr/local/bin/axum_backend
EXPOSE 3000
USER nonroot:nonroot
CMD ["axum_backend"]
