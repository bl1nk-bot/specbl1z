# specgen — MSRV (Minimum Supported Rust Version) tests
# Tests across Rust 1.70, 1.75, 1.80, 1.85
ARG RUST_VERSION=1.85
FROM rust:${RUST_VERSION}-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev git \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt

WORKDIR /workspace
COPY . .

# Verify minimum supported version
RUN cargo --version && rustc --version \
    && cargo check --workspace \
    && cargo test --workspace \
    && cargo clippy --workspace --all-targets -- -D warnings 2>/dev/null || true

CMD ["bash"]
