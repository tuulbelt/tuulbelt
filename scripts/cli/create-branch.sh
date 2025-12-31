#!/bin/bash
# Create feature branch without worktree (CLI only)

set -e

FEATURE_NAME="$1"
REPO_ROOT="$(git rev-parse --show-toplevel)"

# Check if branch already exists (local or remote)
echo "Checking branch status..."
if git show-ref --verify --quiet "refs/heads/$FEATURE_NAME"; then
  # Branch exists locally - checkout
  echo "Branch exists locally, checking out..."
  git checkout "$FEATURE_NAME"
elif git show-ref --verify --quiet "refs/remotes/origin/$FEATURE_NAME"; then
  # Branch exists remotely - fetch and checkout
  echo "Branch exists remotely, fetching and checking out..."
  git fetch origin "$FEATURE_NAME:$FEATURE_NAME"
  git checkout "$FEATURE_NAME"
else
  # New branch - create it
  echo "Creating new branch..."
  git checkout -b "$FEATURE_NAME"
fi

# Initialize submodules if needed
echo "Updating submodules..."
git submodule update --init --recursive

# Install hooks (meta + submodules)
echo "Installing hooks..."
"$REPO_ROOT/scripts/workflow/install-hooks.sh" > /dev/null 2>&1

# Update tracking file (use main repo path for branch-based workflow)
echo "Updating tracking file..."
"$REPO_ROOT/scripts/cli/update-cli-tracking.sh" add "$REPO_ROOT" "$FEATURE_NAME"

echo ""
echo "✓ Feature branch created successfully!"
echo ""
echo "🌿 Branch: $FEATURE_NAME"
echo "📁 Working directory: $REPO_ROOT"
echo ""
echo "Next steps:"
echo "  # Make your changes"
echo "  git add ."
echo "  git commit -m 'feat: implement feature'"
echo "  /work-pr  # Create PRs"
echo ""
