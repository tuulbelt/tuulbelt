#!/bin/bash
# Create worktree for feature development (CLI only)

set -e

FEATURE_NAME="$1"

# Get main repo root (works from worktrees too)
COMMON_GIT_DIR="$(git rev-parse --git-common-dir)"
# Convert to absolute path if relative
if [[ "$COMMON_GIT_DIR" != /* ]]; then
  COMMON_GIT_DIR="$(pwd)/$COMMON_GIT_DIR"
fi
REPO_ROOT="$(dirname "$COMMON_GIT_DIR")"
SANITIZED_NAME=$(echo "$FEATURE_NAME" | tr '/' '-')
WORKTREE_DIR="$REPO_ROOT/.claude/worktrees/$SANITIZED_NAME"

# Check if worktree already exists
if [ -d "$WORKTREE_DIR" ]; then
  echo "❌ Worktree already exists: $WORKTREE_DIR"
  echo "Use /work-switch to resume or /work-cleanup to remove"
  exit 1
fi

# Create worktree directory
mkdir -p "$REPO_ROOT/.claude/worktrees"

# Check if branch already exists (local or remote)
echo "Checking branch status..."
if git show-ref --verify --quiet "refs/heads/$FEATURE_NAME"; then
  # Branch exists locally - use it
  echo "Branch exists locally, checking out in worktree..."
  git worktree add "$WORKTREE_DIR" "$FEATURE_NAME"
elif git show-ref --verify --quiet "refs/remotes/origin/$FEATURE_NAME"; then
  # Branch exists remotely - fetch and checkout
  echo "Branch exists remotely, fetching and checking out..."
  git fetch origin "$FEATURE_NAME:$FEATURE_NAME"
  git worktree add "$WORKTREE_DIR" "$FEATURE_NAME"
else
  # New branch - create it
  echo "Creating new branch in worktree..."
  git worktree add -b "$FEATURE_NAME" "$WORKTREE_DIR"
fi

# Initialize submodules in worktree
echo "Initializing submodules..."
cd "$WORKTREE_DIR"
git submodule update --init --recursive

# Install hooks (meta + submodules)
echo "Installing hooks..."
"$REPO_ROOT/scripts/workflow/install-hooks.sh" > /dev/null 2>&1

# Update tracking file
echo "Updating tracking file..."
"$REPO_ROOT/scripts/cli/update-cli-tracking.sh" add "$WORKTREE_DIR" "$FEATURE_NAME"

echo ""
echo "✓ Worktree created successfully!"
echo ""
echo "📁 Worktree path: $WORKTREE_DIR"
echo "🌿 Branch: $FEATURE_NAME"
echo ""
echo "Next steps:"
echo "  cd $WORKTREE_DIR"
echo "  # Make your changes"
echo "  git add ."
echo "  git commit -m 'feat: implement feature'"
echo "  /work-pr  # Create PRs"
echo ""
