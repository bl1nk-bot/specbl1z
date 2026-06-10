# specgen — Fresh sandbox (bare)
# Simulates empty environment to test setup.sh bootstrap
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl git ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# No Rust, no cargo, no build tools — setup.sh must install everything
WORKDIR /workspace
COPY scripts/setup.sh /setup.sh
RUN chmod +x /setup.sh

# Test that setup.sh bootstraps from zero
RUN /setup.sh

CMD ["bash"]
