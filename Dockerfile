# specgen sandbox — reproducible dev environment
# Build: docker build -t specgen .
# Run:   docker run -it --rm -v "$PWD:/workspace" -w /workspace specgen bash

FROM rust:1.85-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev \
    git python3 python3-pip ripgrep jq \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt \
    && rustup target add aarch64-linux-android

WORKDIR /workspace
COPY scripts/setup.sh /usr/local/bin/specgen-setup
RUN chmod +x /usr/local/bin/specgen-setup

CMD ["bash"]
