#!/bin/bash
# Pre-commit hook to run devops checks

echo "Running pre-commit quality checks..."
./scripts/devops.sh

if [ $? -ne 0 ]; then
    echo "Checks failed. Commit aborted."
    exit 1
fi

# Note: To enable commit-msg validation, run:
# cp scripts/commit-msg.sh .git/hooks/commit-msg
