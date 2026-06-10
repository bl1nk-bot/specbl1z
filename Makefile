.PHONY: all check test lint fmt clean build release hooks install setup wizard test-integration docker-build docker-all bench

# Default: run all checks
all: fmt lint test check

# Check compilation
check:
	cargo check --workspace --all-targets

# Run tests
test:
	cargo test --workspace

# Lint with clippy
lint:
	cargo clippy --workspace --all-targets -- -D warnings

# Format check
fmt:
	cargo fmt --all -- --check

# Format apply
fmt-fix:
	cargo fmt --all

# Clean build artifacts
clean:
	cargo clean

# Debug build
build:
	cargo build --workspace

# Release build
release:
	cargo build --workspace --release

# Full bootstrap from zero
setup:
	bash scripts/setup.sh

# Interactive wizard
wizard:
	bash scripts/wizard.sh

# Real-world integration test
test-integration:
	bash scripts/integration-test.sh

# Docker builds (single platform)
docker-build:
	docker build -f docker/platforms/debian.Dockerfile -t specgen:debian .

# Docker all platforms
docker-all:
	docker compose -f docker/docker-compose.yml build

# Benchmarks
bench:
	cargo bench -p specgen-sandbox

# Install git hooks
hooks:
	cp scripts/pre-commit.sh .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	cp scripts/commit-msg.sh .git/hooks/commit-msg
	chmod +x .git/hooks/commit-msg
	@echo "Hooks installed: pre-commit, commit-msg"
