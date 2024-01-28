# Stage 1: Build Stage
FROM rust:1.75-alpine as builder

# Install musl-dev to provide C runtime files
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy only the necessary files for building
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Runtime Stage
FROM alpine:latest as runtime

WORKDIR /app

# Copy only the built artifacts from the builder stage
COPY --from=builder /app/target/release/rust_shorten /app/rust_shorten

ENV PORT=3000

EXPOSE 3000

# Run the application
CMD ["./rust_shorten"]
