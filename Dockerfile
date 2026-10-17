FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends make chromium fonts-ipafont-gothic \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace/profileforge
