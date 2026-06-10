# specgen — Debian/Ubuntu (x86_64)
# Default production image
FROM rust:1.85-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev \
    git python3 python3-pip ripgrep jq curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt \
    && rustup target add aarch64-linux-android x86_64-unknown-linux-gnu

WORKDIR /workspace
COPY . .
RUN cargo build --workspace --release \
    && cargo test --workspace \
    && cargo clippy --workspace --all-targets -- -D warnings

CMD ["bash"]
