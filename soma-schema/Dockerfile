# soma-schema CLI image.
#
# Build (the context is this directory):
#   docker build -t soma-schema .
# Run: the CLI's default migrations root is ./migrations, and the working directory is /,
# so mount yours at /migrations:
#   docker run --rm -e DATABASE_URL=postgres://... -v "$PWD/migrations:/migrations:ro" soma-schema up
#   docker run --rm -e DATABASE_URL=postgres://... -v "$PWD/migrations:/migrations:ro" soma-schema status --check

# The locked dependencies need rustc >= 1.88; keep in step with the other soma images.
FROM rust:1.96-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release --locked --features cli --bin soma-schema

FROM debian:bookworm-slim
# ca-certificates: system trust store for any TLS the CLI makes.
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 soma \
    && useradd --uid 10001 --gid soma --no-create-home --shell /sbin/nologin soma
COPY --from=builder /build/target/release/soma-schema /usr/local/bin/soma-schema
USER 10001
ENTRYPOINT ["soma-schema"]
