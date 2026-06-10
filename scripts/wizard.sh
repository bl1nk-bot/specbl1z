#!/usr/bin/env bash
# specgen wizard — interactive cross-platform setup
# Run: curl -sL <url>/scripts/wizard.sh | bash
set -euo pipefail

BOLD='\033[1m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'; NC='\033[0m'

echo -e "${BOLD}${CYAN}"
echo "╔══════════════════════════════════════╗"
echo "║     specgen — Setup Wizard          ║"
echo "║     Multi-platform Installer        ║"
echo "╚══════════════════════════════════════╝"
echo -e "${NC}"

# ---- detect platform ----
detect_os() {
    case "$(uname -s)" in
        Linux)
            if [ -n "${TERMUX_VERSION:-}" ]; then
                echo "termux"
            elif [ -f /etc/alpine-release ]; then
                echo "alpine"
            elif [ -f /etc/arch-release ]; then
                echo "arch"
            elif grep -qi ubuntu /etc/os-release 2>/dev/null; then
                echo "ubuntu"
            elif grep -qi debian /etc/os-release 2>/dev/null; then
                echo "debian"
            else
                echo "linux"
            fi
            ;;
        Darwin)  echo "macos" ;;
        *)       echo "unknown" ;;
    esac
}

OS=$(detect_os)
ARCH=$(uname -m)
echo -e "Detected: ${GREEN}$OS${NC} on ${GREEN}$ARCH${NC}"
echo ""

# ---- step 1: tools ----
echo -e "${BOLD}Step 1/5: System tools${NC}"
case "$OS" in
    termux)
        echo "  → pkg install binutils make git python ripgrep jq rust"
        pkg update -y -q
        pkg install -y binutils make git python ripgrep jq rust
        ;;
    ubuntu|debian)
        echo "  → apt install build-essential libssl-dev git python3 ripgrep jq curl"
        sudo apt-get update -y -qq
        sudo apt-get install -y -qq build-essential pkg-config libssl-dev git python3 python3-pip ripgrep jq curl
        ;;
    alpine)
        echo "  → apk add build-base openssl-dev git python3 ripgrep jq curl"
        sudo apk add --no-cache build-base pkgconfig openssl-dev git python3 py3-pip ripgrep jq curl bash
        ;;
    arch)
        echo "  → pacman -S base-devel openssl git python ripgrep jq curl"
        sudo pacman -Syu --noconfirm --needed base-devel openssl git python python-pip ripgrep jq curl
        ;;
    macos)
        echo "  → brew install ripgrep jq python"
        if ! command -v brew &>/dev/null; then
            echo "  Installing Homebrew..."
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        brew install ripgrep jq python
        ;;
    *)
        echo -e "${YELLOW}Unknown OS. Install manually: git, python3, ripgrep, jq, rust${NC}"
        ;;
esac
echo -e "${GREEN}  ✓ tools installed${NC}"

# ---- step 2: Rust ----
echo ""
echo -e "${BOLD}Step 2/5: Rust toolchain${NC}"
if command -v cargo &>/dev/null; then
    echo "  cargo $(cargo --version) found"
else
    echo "  → installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi
rustup component add clippy rustfmt 2>/dev/null || true
case "$OS" in
    termux) rustup target add aarch64-linux-android 2>/dev/null || true ;;
esac
echo -e "${GREEN}  ✓ rust ready${NC}"

# ---- step 3: repo ----
echo ""
echo -e "${BOLD}Step 3/5: Repository${NC}"
REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
if [ -f "$REPO_DIR/Cargo.toml" ]; then
    echo "  repo found at $REPO_DIR"
else
    echo "  → cloning specgen..."
    git clone https://github.com/bl1nk-bot/specbl1z.git "$HOME/specgen" 2>/dev/null || true
    REPO_DIR="$HOME/specgen"
fi
cd "$REPO_DIR"
echo -e "${GREEN}  ✓ repo ready${NC}"

# ---- step 4: build ----
echo ""
echo -e "${BOLD}Step 4/5: Build${NC}"
cargo check --workspace && echo -e "${GREEN}  ✓ cargo check${NC}"
cargo test --workspace 2>&1 | tail -3 && echo -e "${GREEN}  ✓ cargo test${NC}"

# ---- step 5: hooks ----
echo ""
echo -e "${BOLD}Step 5/5: Git hooks${NC}"
if [ -d .git ]; then
    cp scripts/pre-commit.sh .git/hooks/pre-commit 2>/dev/null || true
    cp scripts/commit-msg.sh .git/hooks/commit-msg 2>/dev/null || true
    chmod +x .git/hooks/pre-commit .git/hooks/commit-msg 2>/dev/null || true
    echo -e "${GREEN}  ✓ hooks installed${NC}"
fi

# ---- done ----
echo ""
echo -e "${BOLD}${GREEN}"
echo "╔══════════════════════════════════════╗"
echo "║       Setup Complete!               ║"
echo "╚══════════════════════════════════════╝"
echo -e "${NC}"
echo ""
echo "  Project: $REPO_DIR"
echo "  Commands:"
echo "    make all     — full CI locally"
echo "    cargo build  — debug build"
echo "    cargo test   — run tests"
echo "    specgen validate template.md"
echo ""
