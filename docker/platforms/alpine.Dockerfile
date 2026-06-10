# specgen — Alpine (musl, minimal)
# Smallest image, static binary target
FROM rust:1.85-alpine

RUN apk add --no-cache \
    build-base pkgconfig openssl-dev openssl-libs-static \
    git python3 py3-pip ripgrep jq curl bash

RUN rustup component add clippy rustfmt \
    && rustup target add x86_64-unknown-linux-musl

WORKDIR /workspace
COPY . .
RUN cargo build --workspace --release --target x86_64-unknown-linux-musl \
    && cargo test --workspace

CMD ["bash"]
