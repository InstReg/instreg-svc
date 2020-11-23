FROM rust:1.38.0 as builder
WORKDIR     /rust

# Download the cargo target
RUN         rustup target add x86_64-unknown-linux-musl

# create dummy application, s.t. cargo can download all dependencies
RUN         mkdir -p /rust/app/src && echo 'fn main(){}' > app/src/main.rs
WORKDIR     /rust/app

# Build & cache dependencies
# Copy all fiels including .cargo/config to use faster local mirror
COPY        . .
RUN         cargo build --release --target x86_64-unknown-linux-musl

# Build production binary
RUN         touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

# Production container
FROM        scratch
COPY        --from=builder /rust/app/target/x86_64-unknown-linux-musl/release/microservice_app /app
ENTRYPOINT  ["/app"]
