FROM rust:1.84-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.21
WORKDIR /app
COPY --from=builder /app/target/release/poly-logger-rust /app/logger

CMD ["./logger"]

