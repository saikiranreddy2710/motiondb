# ============================================================
# MotionDB — Multi-stage Docker Build
# ============================================================
# Build:  docker build -t motiondb .
# Run:    docker run -d -p 5431:5431 -p 5432:5432 \
#           -v motiondb-data:/var/lib/motiondb/data motiondb
# ============================================================

# Stage 1: Build
FROM rust:1.82-bookworm AS builder

WORKDIR /build

# Cache dependency build: copy manifests first, build deps, then copy source
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY proto/ proto/

RUN cargo build --release --bin motiond \
    && strip target/release/motiond

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r motiondb \
    && useradd -r -g motiondb -d /var/lib/motiondb -s /sbin/nologin motiondb \
    && mkdir -p /var/lib/motiondb/data /docker-entrypoint-initdb.d \
    && chown -R motiondb:motiondb /var/lib/motiondb

COPY --from=builder /build/target/release/motiond /usr/local/bin/motiond
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

VOLUME ["/var/lib/motiondb/data"]

# PG wire protocol + gRPC
EXPOSE 5431 5432

USER motiondb

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD ["motiond", "--version"] || exit 1

ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["motiond", "--data-dir", "/var/lib/motiondb/data", "-H", "0.0.0.0"]
