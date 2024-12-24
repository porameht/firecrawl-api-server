# Build stage
FROM rust:1.76-slim-bullseye as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build dependencies - this is the caching Docker layer!
RUN cargo build --release

# Production stage
FROM debian:bullseye-slim

# Create a non-root user
RUN useradd -ms /bin/bash appuser

# Install OpenSSL - required for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/firecrawl-api-server /app/firecrawl-api-server
COPY .env /app/.env

# Use the non-root user
RUN chown -R appuser:appuser /app
USER appuser

# Expose port 80
EXPOSE 80

# Expose port 80
EXPOSE 80

# Set environment variable for binding to port 80
ENV RUST_LOG=info
ENV PORT=80

# Run the binary
CMD ["./firecrawl-api-server"] 