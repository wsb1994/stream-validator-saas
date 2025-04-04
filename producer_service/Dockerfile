FROM rust:1.85-slim as builder
WORKDIR /app
# Install necessary build tools
RUN apt-get update && apt-get install -y --no-install-recommends \
 llvm-dev \
 clang \
 cmake \
 pkg-config \
 libssl-dev \
 && rm -rf /var/lib/apt/lists/*
# Copy only Cargo files first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

COPY src ./src
RUN cargo build --release
# Strip symbols from the binary to reduce size
RUN strip /app/target/release/producer_service
# Final stage: minimal runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
 ca-certificates \
 ffmpeg \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# Copy the stripped binary from the builder stage
COPY --from=builder /app/target/release/producer_service ./producer_service
# Copy the shared .env file from the current directory.

CMD ["./producer_service"]