FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app
## Planner stage: Cache dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

## Builder stage: Build binary
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Install system dependencies
RUN apt-get update && apt-get -y upgrade && apt-get install -y libclang-dev pkg-config
# Build dependencies - this is the caching Docker layer
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

## Runtime stage: Copy binary to new image and run 
FROM ubuntu:20.04 AS runtime
WORKDIR app
COPY --from=builder /app/target/release/artemis /usr/local/bin
# Install openssl and ca-certificates
RUN apt-get update && apt install -y openssl && apt install -y ca-certificates
ENTRYPOINT /usr/local/bin/artemis
