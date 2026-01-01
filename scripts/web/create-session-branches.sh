#!/bin/bash
# Create feature branch and initialize web session
# Usage: ./create-session-branches.sh <feature-name>

set -e

FEATURE_NAME="$1"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel)"

source "$SCRIPT_DIR/tracking-lib.sh"

if [ -z "$FEATURE_NAME" ]; then
  echo "Usage: ./create-session-branches.sh <feature-name>"
  exit 1
fi

echo "🌐 Creating web session for: $FEATURE_NAME"
echo ""

# Check if already on a feature branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
  echo "⚠️  Warning: Already on feature branch: $CURRENT_BRANCH"
  echo ""
  echo "Options:"
  echo "  1. Finish current work: /work-pr, merge, /work-cleanup"
  echo "  2. Switch to main first: git checkout main"
  echo ""
  exit 1
fi

# Check if branch already exists
if git show-ref --verify --quiet "refs/heads/$FEATURE_NAME"; then
  echo "Branch $FEATURE_NAME already exists locally"
  echo "Checking out existing branch..."
  git checkout "$FEATURE_NAME"
elif git show-ref --verify --quiet "refs/remotes/origin/$FEATURE_NAME"; then
  echo "Branch $FEATURE_NAME exists remotely"
  echo "Fetching and checking out..."
  git fetch origin "$FEATURE_NAME:$FEATURE_NAME"
  git checkout "$FEATURE_NAME"
else
  echo "Creating new branch..."
  git checkout -b "$FEATURE_NAME"
fi

echo "✓ On feature branch: $FEATURE_NAME"
echo ""

# Initialize tracking file if needed
if ! tracking_file_exists; then
  echo "Initializing tracking file..."
  init_tracking_file
fi

# Create session entry
if ! has_session "$FEATURE_NAME"; then
  echo "Creating session entry..."
  create_session "$FEATURE_NAME"
else
  echo "✓ Session already exists for this branch"
fi

# Initialize submodules
echo ""
echo "Initializing submodules..."
git submodule update --init --recursive >/dev/null 2>&1 || true
echo "✓ Submodules initialized"

# Install hooks
echo ""
echo "Installing hooks..."
"$REPO_ROOT/scripts/workflow/install-hooks.sh" > /dev/null 2>&1
echo "✓ Hooks installed"

echo ""
echo "✓ Web session created successfully!"
echo ""
echo "📁 Branch: $FEATURE_NAME"
echo "🌿 Session: active"
echo ""
echo "Next steps:"
echo "  1. Make your changes"
echo "  2. git add . && git commit -m 'feat: implement feature'"
echo "  3. /work-status  # Check status"
echo "  4. /work-pr      # Create PRs"
echo ""
