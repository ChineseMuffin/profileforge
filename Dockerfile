FROM mcr.microsoft.com/devcontainers/rust:1-bookworm AS rust-base

WORKDIR /workspace/profileforge

FROM rust-base AS dev

RUN apt-get update \
    && apt-get install -y --no-install-recommends make chromium chromium-sandbox fonts-ipafont-gothic \
    && rm -rf /var/lib/apt/lists/*

FROM rust-base AS builder

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY src ./src
COPY templates ./templates

RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends chromium chromium-sandbox fonts-ipafont-gothic passwd \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 profileforge \
    && mkdir -p /data \
    && chown profileforge:profileforge /data

ENV CHROMIUM_PATH=/usr/bin/chromium

COPY --from=builder /workspace/profileforge/target/release/profileforge /usr/local/bin/profileforge

WORKDIR /data
USER profileforge

ENTRYPOINT ["profileforge"]
