#!/bin/bash
# Git commit and push using credentials from .env
# Usage: ./scripts/git-commit-with-auth.sh [repo-path] "commit message"

set -e

REPO_PATH="${1:-.}"
COMMIT_MSG="$2"

if [ -z "$COMMIT_MSG" ]; then
  echo "Usage: $0 [repo-path] \"commit message\""
  echo "Example: $0 /tmp/my-tool \"feat: add new feature\""
  exit 1
fi

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
if [ -z "$GITHUB_TOKEN" ] || [ -z "$GITHUB_USERNAME" ] || [ -z "$GITHUB_EMAIL" ]; then
  echo "ERROR: GITHUB_TOKEN, GITHUB_USERNAME, or GITHUB_EMAIL not set in .env"
  exit 1
fi

# Export for gh CLI
export GITHUB_TOKEN

# Navigate to repo
cd "$REPO_PATH"

# Set local git config (only affects this repo)
git config user.name "$GITHUB_USERNAME"
git config user.email "$GITHUB_EMAIL"

echo "✓ Git config set:"
echo "  Name: $(git config user.name)"
echo "  Email: $(git config user.email)"
echo "  Repo: $(pwd)"
echo ""

# Stage all changes
git add -A

# Check if there are changes to commit
if git diff --cached --quiet; then
  echo "No changes to commit"
  exit 0
fi

# Commit (no Claude Code attribution)
git commit -m "$COMMIT_MSG"

echo ""
echo "✓ Committed with author: $(git log -1 --format='%an <%ae>')"
echo "  Commit: $(git log -1 --format='%h - %s')"
