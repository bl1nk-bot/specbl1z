#!/usr/bin/env bash
# Real-world integration test — full pipeline
# Tests: build, migrate, serve, health check, memory CRUD
set -euo pipefail

# Standardized Colors
INFO='\033[0;36m'; SUCCESS='\033[0;32m'; ERROR='\033[0;31m'; DEBUG='\033[0;33m'; NC='\033[0m'

msg() { echo -e "${INFO}info:${NC} $*"; }
pass() { echo -e "${SUCCESS}success:${NC} ✅ $*"; }
fail() { echo -e "${ERROR}error:${NC} ❌ $*"; exit 1; }
debug() { if [ "${INTEGRATION_DEBUG:-0}" = "1" ]; then echo -e "${DEBUG}debug:${NC} $*"; fi; }

BIN="${SPECGEN_BIN:-./target/release/specgen}"
DB="data/test_integration.db"
PORT="${TEST_PORT:-13999}"

cd "$(dirname "$0")/.."

# ---- 1. Build ----
msg "Building workspace..."
cargo build --workspace --release || fail "cargo build failed"
test -f "$BIN" || fail "binary not found at $BIN"
pass "workspace built"

# ---- 2. Clean DB ----
msg "Initializing database..."
rm -f "$DB"
"$BIN" db init --database "$DB" --force 2>&1 || fail "db init failed"
[ -f "$DB" ] || fail "database file not created"
pass "database initialized"

# ---- 3. Validate templates ----
msg "Validating templates..."
for tmpl in templates/*.toml templates/*.md templates/*.json; do
    [ -f "$tmpl" ] || continue
    debug "checking $tmpl"
    if "$BIN" validate "$tmpl" 2>&1; then
        pass "validated $(basename $tmpl)"
    else
        fail "validation failed for $tmpl"
    fi
done

# ---- 4. API server ----
msg "Starting API server on port $PORT..."
"$BIN" serve --port "$PORT" &
SERVER_PID=$!
sleep 2

# Health check
HEALTH=$(curl -sf http://localhost:$PORT/health) || fail "health check failed"
[ "$HEALTH" = "OK" ] || fail "health check returned: $HEALTH"
pass "health check OK"

# Memory insert
msg "Testing memory CRUD..."
INSERT=$(curl -sf -X POST http://localhost:$PORT/api/memory \
    -H "Content-Type: application/json" \
    -d '{"scope":1,"category":1,"topic":1,"key":"test_key","value":"integration test","confidence":1.0,"status":"active","created_at":0,"updated_at":0,"version":1,"tags":[],"access_level":"private"}')
debug "insert response: $INSERT"
echo "$INSERT" | grep -q '"status":"success"' || fail "memory insert failed"
pass "memory entry created"

# Memory query
QUERY=$(curl -sf http://localhost:$PORT/api/memory)
debug "query response: $QUERY"
echo "$QUERY" | grep -q 'test_key' || fail "memory query failed (key not found)"
pass "memory entry retrieved"

# Cleanup
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true

# ---- 5. DB Status ----
msg "Final status check..."
"$BIN" db status --database "$DB" 2>&1 || fail "db status failed"
pass "status check complete"

# ---- 6. Cleanup ----
rm -f "$DB"
echo ""
echo -e "${SUCCESS}======================================${NC}"
echo -e "${SUCCESS}  ALL INTEGRATION TESTS PASSED ✨${NC}"
echo -e "${SUCCESS}======================================${NC}"
