#!/bin/bash
# Cleanup CLI workspace (worktree or branch)

set -e

FEATURE_NAME="$1"
FORCE_FLAG="$2"

# Get main repo root (works from worktrees too)
COMMON_GIT_DIR="$(git rev-parse --git-common-dir)"
# Convert to absolute path if relative
if [[ "$COMMON_GIT_DIR" != /* ]]; then
  COMMON_GIT_DIR="$(pwd)/$COMMON_GIT_DIR"
fi
REPO_ROOT="$(dirname "$COMMON_GIT_DIR")"
SANITIZED_NAME=$(echo "$FEATURE_NAME" | tr '/' '-')
WORKTREE_DIR="$REPO_ROOT/.claude/worktrees/$SANITIZED_NAME"

echo "Cleaning up workspace: $FEATURE_NAME"
echo ""

# Check if --force flag is provided
if [ "$FORCE_FLAG" != "--force" ]; then
  echo "Checking PR status..."

  # Check if gh CLI is available
  if command -v gh &> /dev/null; then
    # Check meta repo PR status
    cd "$REPO_ROOT"
    if gh pr view "$FEATURE_NAME" &>/dev/null; then
      PR_STATE=$(gh pr view "$FEATURE_NAME" --json state -q .state)
      if [ "$PR_STATE" != "MERGED" ]; then
        echo "  ❌ Meta repo PR not merged (state: $PR_STATE)"
        echo ""
        echo "Wait for PR to be merged, or use --force to cleanup anyway:"
        echo "  /work-cleanup $FEATURE_NAME --force"
        exit 1
      fi
      echo "  ✓ Meta repo PR: merged"
    fi
  else
    echo "  ⚠ gh CLI not available, skipping PR verification"
    echo "  Install: brew install gh"
  fi
  echo ""
fi

# Remove worktree if it exists
if [ -d "$WORKTREE_DIR" ]; then
  echo "Removing worktree..."
  cd "$REPO_ROOT"
  git worktree remove "$WORKTREE_DIR" --force
  echo "  ✓ Removed: $WORKTREE_DIR"
  echo ""
fi

# Delete branches
echo "Deleting branches..."
cd "$REPO_ROOT"

# Delete local branch
if git show-ref --verify --quiet "refs/heads/$FEATURE_NAME"; then
  git branch -D "$FEATURE_NAME"
  echo "  ✓ Deleted local branch: $FEATURE_NAME"
fi

# Ask about remote deletion
if git show-ref --verify --quiet "refs/remotes/origin/$FEATURE_NAME"; then
  echo ""
  read -p "Delete remote branch? (y/n) " -n 1 -r
  echo
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin --delete "$FEATURE_NAME"
    echo "  ✓ Deleted remote branch: $FEATURE_NAME"
  fi
fi

echo ""

# Update tracking file
if [ -f "$REPO_ROOT/.claude/cli-workspace-tracking.json" ]; then
  echo "Updating tracking file..."
  "$REPO_ROOT/scripts/cli/update-cli-tracking.sh" remove "$WORKTREE_DIR" "$FEATURE_NAME"
  echo "  ✓ Tracking file updated"
  echo ""
fi

# Return to main branch
git checkout main
git pull origin main

echo "✓ Workspace cleaned up successfully!"
