#!/bin/bash
# scripts/bump-version.sh
# Usage: ./scripts/bump-version.sh <new_version>

NEW_VERSION=$1

if [ -z "$NEW_VERSION" ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi

echo "Bumping version to $NEW_VERSION across the workspace..."

# Update root Cargo.toml if it has a version (workspace usually doesn't, but check)
# In this project, root Cargo.toml is a workspace only.

# Update VERSION file
if [ -f "VERSION" ]; then
    echo "Updating VERSION file"
    echo "$NEW_VERSION" > VERSION
fi

# Update crates
crates=("core" "cli" "api" "sandbox")

for crate in "${crates[@]}"; do
    if [ -f "$crate/Cargo.toml" ]; then
        echo "Updating $crate/Cargo.toml"
        # Use sed to update the version line specifically in the [package] section
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$crate/Cargo.toml"
    fi
done

# Update gh-extension
if [ -f "tools/triage/gh-bl1nk-triage" ]; then
    echo "Updating tools/triage/gh-bl1nk-triage"
    sed -i "s/^# Version: .*/# Version: $NEW_VERSION/" tools/triage/gh-bl1nk-triage
fi

# Update templates
if [ -f "templates/spec-workflow.json" ]; then
    echo "Updating templates/spec-workflow.json"
    sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" templates/spec-workflow.json
fi
if [ -f "templates/spec-workflow.toml" ]; then
    echo "Updating templates/spec-workflow.toml"
    sed -i "s/version = \".*\"/version = \"$NEW_VERSION\"/" templates/spec-workflow.toml
fi

# Update Cargo.lock
cargo check

echo "Done! Version bumped to $NEW_VERSION"
