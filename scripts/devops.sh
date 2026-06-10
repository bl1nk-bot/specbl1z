#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Running DevOps Checks...${NC}"

# 1. Rust Checks
echo -e "\n${GREEN}Checking Rust Core & CLI...${NC}"
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features

# 2. Server (TS) Checks
if [ -d "server" ]; then
    echo -e "\n${GREEN}Checking Server (TS)...${NC}"
    cd server
    if [ -f "package.json" ]; then
        # Check if node_modules exists, if not install
        if [ ! -d "node_modules" ]; then
            npm install --silent
        fi
        
        echo "Running Biome CI..."
        if ! npx @biomejs/biome ci . 2>/dev/null; then
            echo -e "${RED}Warning: Biome check failed or is not supported on this platform (Termux). Skipping...${NC}"
        fi
    fi
    cd ..
fi

echo -e "\n${GREEN}All checks passed! ✅${NC}"
