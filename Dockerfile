FROM rust:1.92.0-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libpq-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release -p api

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libpq5 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN useradd --uid 10001 --create-home --home-dir /app --shell /usr/sbin/nologin appuser
COPY --from=builder /app/target/release/api /app/api
RUN chown appuser:appuser /app/api

USER appuser
EXPOSE 3000
CMD ["/app/api"]
