# --- Stage 1: Build ---
FROM rust:1.75-slim AS builder

# Create a dummy project to cache dependencies
RUN cargo new --bin rusty_exchange
WORKDIR /rusty_exchange

# Copy only the dependency list
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build dependencies only (this layer is cached)
RUN cargo build --release
RUN rm src/*.rs

# Now copy the actual source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/rusty_exchange*
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

# Install OpenSSL (required for HTTPS requests) and CA certificates
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /rusty_exchange/target/release/rusty_exchange /usr/local/bin/rusty_exchange

# Set the entrypoint
ENTRYPOINT ["rusty_exchange"]