#!/bin/bash
# commit-msg hook to enforce markers for labeling

COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")
FIRST_LINE=$(head -n 1 "$COMMIT_MSG_FILE")

# Allowed markers based on project standards and TODO.md
# Format: [MARKER] Description or MARKER: Description
VALID_MARKERS="FEAT|FIX|DOC|DESIGN|PLAN|SPEC|CORE|CLI|SERVER|SEC|REVIEW|LEARN|LOOP|CHORE"

# Regex pattern
# Matches: [FEAT] title or FEAT: title
PATTERN="^(\[($VALID_MARKERS)\]|($VALID_MARKERS):) .+"

if [[ ! $FIRST_LINE =~ $PATTERN ]]; then
    echo "❌ Error: Invalid commit message format."
    echo "Your commit message must start with a marker for labeling."
    echo ""
    echo "Allowed Markers: $VALID_MARKERS"
    echo "Examples:"
    echo "  [FEAT] add new memory engine"
    echo "  DESIGN: update storage architecture"
    echo "  [FIX] resolve clippy warnings"
    echo ""
    exit 1
fi

echo "✅ Commit message marker detected: $(echo $FIRST_LINE | grep -oE "($VALID_MARKERS)")"
