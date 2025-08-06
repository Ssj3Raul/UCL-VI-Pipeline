# 1. Build stage
FROM rust:1.88 as builder
WORKDIR /app

# Install build dependencies for building native crates
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake pkg-config libssl-dev clang make build-essential

COPY . .
RUN rustc --version && cargo --version
RUN ls -la /app
RUN cargo build --release

# 2. Runtime stage (small image)
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/collector /usr/local/bin/collector

CMD ["/usr/local/bin/collector"]