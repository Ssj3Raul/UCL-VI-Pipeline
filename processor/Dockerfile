# 1. Build stage
FROM rust:1.88 as builder
WORKDIR /app

RUN echo "ROOT CONTEXT:" && ls -la / && echo "APP CONTEXT:" && ls -la /app && echo "DONE"

# Install build dependencies for building native crates
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake pkg-config libssl-dev clang make build-essential

COPY . .
RUN rustc --version && cargo --version
RUN ls -la /app
RUN ls -la /app/processor    

# Set environment variable to disable SQLx compile-time checking
ENV SQLX_OFFLINE=false
RUN cargo build --release

# 2. Runtime stage (small image)
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/processor /usr/local/bin/processor

CMD ["/usr/local/bin/processor"]
