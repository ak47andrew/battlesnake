FROM rust:latest AS builder

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/battlesnake .

# Expose the port your Rust server uses
EXPOSE 9100

# Run the binary
CMD ["./battlesnake"]