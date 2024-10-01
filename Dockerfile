# Stage 1: Build the Rust application
FROM rust:1.72 as builder

# Create a new empty shell project
WORKDIR /usr/src/app
RUN cargo new --bin rust_shorten
WORKDIR /usr/src/app/rust_shorten

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY src ./src

# Build for release
RUN rm ./target/release/deps/rust_shorten*
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

# Install OpenSSL and ca-certificates
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -ms /bin/bash myuser

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/rust_shorten/target/release/rust_shorten /usr/local/bin/

# Set the ownership of the binary to the non-root user
RUN chown myuser:myuser /usr/local/bin/rust_shorten

# Switch to the non-root user
USER myuser

# Set the startup command to run your binary
CMD ["rust_shorten"]