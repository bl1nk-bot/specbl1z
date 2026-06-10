#!/bin/bash
# Pre-commit hook — fast checks for Termux
# Full CI (check+test+clippy) runs on GitHub Actions, not here.
set -euo pipefail

echo "Pre-commit checks..."

# 1. Cargo fmt (fast, no compilation)
if ! cargo fmt --all -- --check 2>/dev/null; then
    echo "FAIL: cargo fmt — run 'cargo fmt --all' to fix"
    exit 1
fi
echo "  fmt: OK"

# 2. Basic syntax check on staged .rs files only (fast)
STAGED_RS=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)
if [ -n "$STAGED_RS" ]; then
    echo "$STAGED_RS" | while read f; do
        if ! rustfmt --check "$f" 2>/dev/null; then
            echo "  $f: needs format"
        fi
    done
fi

echo "Pre-commit: PASS"
