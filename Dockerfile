# Build stage
FROM rust:1.75-slim-bullseye as builder

WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/firecrawl-api-server .

# Create a non-root user
RUN useradd -m -u 1001 appuser && \
    chown appuser:appuser /usr/local/bin/firecrawl-api-server

# Switch to non-root user
USER appuser

# Expose port 80
EXPOSE 80

# Set environment variable for binding to port 80
ENV RUST_LOG=info
ENV PORT=80

# Run the binary
CMD ["./firecrawl-api-server"] 