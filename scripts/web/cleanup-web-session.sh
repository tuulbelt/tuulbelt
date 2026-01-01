#!/bin/bash
# Cleanup web session (branch-based workflow)

set -e

FEATURE_NAME="$1"
FORCE_FLAG="$2"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel)"

source "$SCRIPT_DIR/tracking-lib.sh"

if [ -z "$FEATURE_NAME" ]; then
  echo "Usage: ./cleanup-web-session.sh <feature-name> [--force]"
  exit 1
fi

echo "Cleaning up web session: $FEATURE_NAME"
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

# Switch to main if currently on feature branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$CURRENT_BRANCH" = "$FEATURE_NAME" ]; then
  # Check for uncommitted changes before switching
  if [ -n "$(git status --porcelain)" ]; then
    echo "❌ ERROR: Cannot switch branches - uncommitted changes"
    echo ""
    echo "Please commit or stash your changes first:"
    echo "  git add . && git commit -m 'message'"
    echo "  OR"
    echo "  git stash"
    echo ""
    exit 1
  fi

  echo "Switching to main branch..."
  git checkout main
  git pull origin main
  echo "  ✓ On main branch"
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

# Delete remote branch (ask in interactive mode)
if git show-ref --verify --quiet "refs/remotes/origin/$FEATURE_NAME"; then
  echo ""
  if [ -t 0 ]; then
    # Interactive mode - ask user
    read -p "Delete remote branch? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      git push origin --delete "$FEATURE_NAME"
      echo "  ✓ Deleted remote branch: $FEATURE_NAME"
    fi
  else
    # Non-interactive mode - skip remote deletion
    echo "⚠ Non-interactive mode: Skipping remote branch deletion"
    echo "  Run manually if needed: git push origin --delete $FEATURE_NAME"
  fi
fi

echo ""

# Delete submodule branches
echo "Cleaning up submodule branches..."

while read -r submodule; do
  cd "$REPO_ROOT/$submodule"

  SUBMODULE_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")

  if [ "$SUBMODULE_BRANCH" = "$FEATURE_NAME" ]; then
    # Switch to main if on feature branch
    git checkout main 2>/dev/null || true
    git pull origin main 2>/dev/null || true
  fi

  # Delete local branch if exists
  if git show-ref --verify --quiet "refs/heads/$FEATURE_NAME"; then
    git branch -D "$FEATURE_NAME" 2>/dev/null || true
    echo "  ✓ Deleted $submodule local branch"
  fi

  cd "$REPO_ROOT"
done < <(git submodule foreach --quiet 'echo $path')

echo ""

# Remove session from tracking file
if has_session "$FEATURE_NAME"; then
  echo "Removing session from tracking file..."

  TRACKING_PATH="$REPO_ROOT/$TRACKING_FILE"
  jq "del(.sessions[\"$FEATURE_NAME\"])" "$TRACKING_PATH" > "$TRACKING_PATH.tmp"
  mv "$TRACKING_PATH.tmp" "$TRACKING_PATH"

  echo "  ✓ Session removed from tracking"
  echo ""
fi

echo "✓ Web session cleaned up successfully!"
