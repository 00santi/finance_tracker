FROM rust:1.95-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/finance_tracker /app/app

EXPOSE 7878
CMD ["./app"]
