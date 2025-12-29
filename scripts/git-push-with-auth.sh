#!/bin/bash
# Git push using credentials from .env
# Usage: ./scripts/git-push-with-auth.sh [repo-path] [remote] [branch]

set -e

REPO_PATH="${1:-.}"
REMOTE="${2:-origin}"
BRANCH="${3:-main}"

# Load .env from tuulbelt meta repo
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
META_REPO="$(dirname "$SCRIPT_DIR")"

if [ ! -f "$META_REPO/.env" ]; then
  echo "ERROR: .env file not found at $META_REPO/.env"
  exit 1
fi

# Source .env
set -a
source "$META_REPO/.env"
set +a

# Validate required variables
if [ -z "$GITHUB_TOKEN" ]; then
  echo "ERROR: GITHUB_TOKEN not set in .env"
  exit 1
fi

# Export for gh CLI and git
export GITHUB_TOKEN

# Navigate to repo
cd "$REPO_PATH"

echo "Pushing to $REMOTE/$BRANCH from $(pwd)..."
echo ""

# Push
git push "$REMOTE" "$BRANCH"

echo ""
echo "✓ Pushed successfully"

# Push tags if any
if git tag -l | grep -q .; then
  echo ""
  read -p "Push tags too? (y/n) " -n 1 -r
  echo
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push "$REMOTE" --tags
    echo "✓ Tags pushed"
  fi
fi
