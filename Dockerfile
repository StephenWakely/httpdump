# Build
FROM rust:1.66.0 AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN cargo build --release

# Run
FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/httpdump httpdump
ENTRYPOINT ["./httpdump"]
