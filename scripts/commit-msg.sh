#!/bin/bash
# commit-msg hook — enforce [MARKER] format + Linear issue reference
# Installed via: make hooks

COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")
FIRST_LINE=$(head -n 1 "$COMMIT_MSG_FILE")

# Allowed markers — must match GitHub issue labels + Linear conventions
# [FEAT] [FIX] [DOC] [CHORE] [SANDBOX] [CI] [SEC] [PERF] [TEST] [REFACTOR]
VALID_MARKERS="FEAT|FIX|DOC|CHORE|SANDBOX|CI|SEC|PERF|TEST|REFACTOR|DESIGN|PLAN|SPEC|CORE|CLI|SERVER"

# Regex: [MARKER] Description (BNK-123)  OR  MARKER: Description (BNK-123)
# Linear issue reference (BNK-NNN) is encouraged in body or after description
PATTERN="^(\[($VALID_MARKERS)\]|($VALID_MARKERS):) .+"

if [[ ! $FIRST_LINE =~ $PATTERN ]]; then
    echo "ERROR: Invalid commit message format."
    echo "Must start with: [MARKER] Description"
    echo ""
    echo "Valid markers: $VALID_MARKERS"
    echo ""
    echo "Examples:"
    echo "  [FEAT] add database abstraction layer (BNK-21)"
    echo "  [FIX] resolve clippy warnings (BNK-22)"
    echo "  [SANDBOX] test Daytona connection end-to-end"
    echo "  [CI] add cross-platform build matrix"
    echo ""
    exit 1
fi

# Encourage Linear reference but don't block
if ! echo "$COMMIT_MSG" | grep -qE 'BNK-[0-9]+'; then
    echo "Note: No Linear issue reference (BNK-NNN) found. Consider linking an issue."
fi

echo "Commit marker: $(echo $FIRST_LINE | grep -oE "($VALID_MARKERS)")"
