#!/usr/bin/env bash
# specgen setup/bootstrap — makes project reproducible from zero
# Run: curl -sL <raw-url>/scripts/setup.sh | bash
#   or: ./scripts/setup.sh
set -euo pipefail

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
log()  { echo -e "${GREEN}[SETUP]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
err()  { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# ---- detect platform ----
detect_platform() {
    if [ -n "${TERMUX_VERSION:-}" ]; then
        echo "termux"
    elif [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            ubuntu|debian) echo "debian" ;;
            alpine)        echo "alpine" ;;
            arch)          echo "arch" ;;
            *)             echo "linux-unknown" ;;
        esac
    elif [ "$(uname)" = "Darwin" ]; then
        echo "macos"
    elif [ "$(uname -o 2>/dev/null)" = "Msys" ] || [ "$(uname -o 2>/dev/null)" = "Cygwin" ]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

PLATFORM=$(detect_platform)
log "platform: $PLATFORM"

# ---- 1. system packages ----
install_system_deps() {
    case "$PLATFORM" in
        termux)
            log "installing Termux packages..."
            pkg update -y
            pkg install -y binutils make git python ripgrep jq rust 2>/dev/null || true
            ;;
        debian)
            log "installing Debian/Ubuntu packages..."
            sudo apt-get update -y
            sudo apt-get install -y build-essential pkg-config libssl-dev \
                curl git python3 python3-pip ripgrep jq 2>/dev/null || true
            ;;
        alpine)
            log "installing Alpine packages..."
            apk add --no-cache build-base pkgconfig openssl-dev \
                curl git python3 py3-pip ripgrep jq bash 2>/dev/null || true
            ;;
        arch)
            log "installing Arch packages..."
            sudo pacman -Syu --noconfirm base-devel openssl \
                curl git python python-pip ripgrep jq 2>/dev/null || true
            ;;
        macos)
            log "installing macOS packages (homebrew)..."
            brew install ripgrep jq python 2>/dev/null || warn "some brew packages may have failed"
            ;;
        windows)
            warn "Windows detected. Install manually: git, python3, ripgrep, jq, rust via https://rustup.rs"
            ;; 
        *)
            warn "unknown platform: install git, python3, ripgrep, jq manually"
            ;;
    esac
}

install_system_deps

# ---- 2. Rust toolchain ----
install_rust() {
    if command -v cargo &>/dev/null; then
        log "cargo $(cargo --version) already installed"
        # ensure components
        rustup component add clippy rustfmt 2>/dev/null || true
        rustup target add aarch64-linux-android 2>/dev/null || true
    else
        log "installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        # shellcheck disable=SC1090
        source "$HOME/.cargo/env"
        rustup component add clippy rustfmt
        rustup target add aarch64-linux-android 2>/dev/null || true
    fi
}

install_rust

# ---- 3. verify all tools ----
log "verifying tools..."
check_tool() {
    if command -v "$1" &>/dev/null; then
        log "  $1: $(command -v "$1")"
    else
        err "  $1: NOT FOUND — install manually"
    fi
}

check_tool cargo
check_tool rustc
check_tool rustfmt
check_tool git
check_tool python3
check_tool rg
check_tool jq

# ---- 4. clone repo if missing ----
REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
if [ -f "$REPO_DIR/Cargo.toml" ]; then
    log "project found at $REPO_DIR"
else
    REPO_URL="${SPECGEN_REPO_URL:-https://github.com/bl1nk-bot/specbl1z.git}"
    log "cloning $REPO_URL..."
    git clone "$REPO_URL" "$HOME/specgen"
    REPO_DIR="$HOME/specgen"
fi

# ---- 5. build & verify ----
cd "$REPO_DIR"
log "cargo check..."
cargo check --workspace || warn "cargo check failed (may need network for deps)"
log "cargo test..."
cargo test --workspace 2>&1 | tail -5 || warn "tests have issues"

log "===== SETUP COMPLETE ====="
log "project: $REPO_DIR"
log "commands:"
log "  make all    — run full CI locally"
log "  cargo build — debug build"
log "  cargo test  — run tests"
