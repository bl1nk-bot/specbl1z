# specgen — Android ARM64 cross-compile
# Builds specgen for Termux/Android target
FROM rust:1.85-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev \
    git python3 ripgrep jq curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt \
    && rustup target add aarch64-linux-android

# Android NDK for linking
ENV ANDROID_NDK_HOME=/android-ndk
RUN curl -sL https://dl.google.com/android/repository/android-ndk-r27c-linux.zip -o /tmp/ndk.zip \
    && unzip -q /tmp/ndk.zip -d /tmp \
    && mv /tmp/android-ndk-r27c /android-ndk \
    && rm /tmp/ndk.zip

ENV CC_aarch64_linux_android=/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang
ENV AR_aarch64_linux_android=/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar
ENV CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang

WORKDIR /workspace
COPY . .
RUN cargo build --workspace --release --target aarch64-linux-android \
    && echo "Binary ready: target/aarch64-linux-android/release/specgen"

CMD ["bash"]
