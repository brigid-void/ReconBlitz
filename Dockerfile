FROM rust:1.70-slim-bullseye as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

# Create non-root user for security
RUN addgroup --system reconblitz \
    && adduser --system --ingroup reconblitz reconblitz

# Set working directory and transfer ownership
WORKDIR /app
RUN chown -R reconblitz:reconblitz /app

# Copy binary with ownership
COPY --from=builder --chown=reconblitz:reconblitz /app/target/release/reconblitz /usr/local/bin

# Install security tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    cargo-audit \
    && rm -rf /var/lib/apt/lists/*

# Scan for vulnerabilities
RUN cargo audit --deny warnings

# Switch to non-root user
USER reconblitz

ENTRYPOINT ["reconblitz"]
