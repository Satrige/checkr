FROM rust:1.91-slim-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() { println!(\"build cache warmup\"); }" > src/main.rs
RUN cargo build --release && rm -rf target/release/deps/* src

COPY . .
RUN cargo build --release && strip target/release/checkr

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN useradd -m -u 10001 appuser \
    && apt-get update && apt-get install -y --no-install-recommends ca-certificates tzdata iproute2 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/checkr /usr/local/bin/checkr

ENV CONFIG_PATH=/app/config.json
EXPOSE 3000

USER appuser

CMD ["sh", "-c", "exec checkr --config ${CONFIG_PATH}"]
