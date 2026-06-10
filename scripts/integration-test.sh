#!/usr/bin/env bash
# Real-world integration test — full pipeline
# Tests: build, migrate, serve, health check, memory CRUD
set -euo pipefail

GREEN='\033[0;32m'; RED='\033[0;31m'; NC='\033[0m'
pass() { echo -e "${GREEN}[PASS]${NC} $*"; }
fail() { echo -e "${RED}[FAIL]${NC} $*"; exit 1; }

BIN="${SPECGEN_BIN:-cargo run --}"
DB="data/test_integration.db"
PORT="${TEST_PORT:-13999}"

cd "$(dirname "$0")/.."

# ---- 1. Build ----
echo "=== 1. Build ==="
cargo build --workspace --release || fail "build failed"
pass "cargo build --release"

# ---- 2. Clean DB ----
echo "=== 2. Migrate ==="
rm -f "$DB"
cargo run -- db init 2>&1 || fail "db init failed"
[ -f "$DB" ] || fail "database file not created"
pass "db init + schema migration"

# ---- 3. Validate templates ----
echo "=== 3. Validate ==="
for tmpl in templates/*.toml templates/*.md templates/*.json 2>/dev/null; do
    [ -f "$tmpl" ] || continue
    echo -n "  $tmpl ... "
    if cargo run -- validate "$tmpl" 2>&1; then
        pass "validate $(basename $tmpl)"
    else
        fail "validate $(basename $tmpl)"
    fi
done

# ---- 4. API server ----
echo "=== 4. API Server ==="
cargo run -- serve --port "$PORT" &
SERVER_PID=$!
sleep 2

# Health check
HEALTH=$(curl -sf http://localhost:$PORT/health 2>/dev/null) || fail "health check failed"
[ "$HEALTH" = "OK" ] || fail "health != OK: $HEALTH"
pass "health check: $HEALTH"

# Memory insert
INSERT=$(curl -sf -X POST http://localhost:$PORT/api/memory \
    -H "Content-Type: application/json" \
    -d '{"scope":1,"category":1,"topic":1,"key":"test_key","value":"integration test","confidence":1.0,"status":"active","created_at":0,"updated_at":0,"version":1,"tags":[],"access_level":"private"}' 2>/dev/null)
echo "  insert: $INSERT"
echo "$INSERT" | grep -q '"success"' || fail "memory insert failed"
pass "memory insert"

# Memory query
QUERY=$(curl -sf http://localhost:$PORT/api/memory 2>/dev/null)
echo "  query:  $QUERY"
echo "$QUERY" | grep -q 'test_key' || fail "memory query: test_key not found"
pass "memory query"

# Cleanup
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true

# ---- 5. DB Status ----
echo "=== 5. DB Status ==="
cargo run -- db status 2>&1 || fail "db status failed"
pass "db status"

# ---- 6. Cleanup ----
rm -f "$DB"
echo ""
echo -e "${GREEN}======================"
echo "ALL INTEGRATION TESTS PASSED"
echo -e "======================${NC}"
