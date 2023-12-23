FROM rust:latest as builder

# Install libclang
RUN apt-get update && \
    apt-get install -y libclang-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/nostr-preview ./nostr-preview

# Runtime image
FROM debian:bullseye-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/nostr-preview /app/nostr-preview

# Run the app
CMD ./nostr-preview
