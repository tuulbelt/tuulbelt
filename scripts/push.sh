#!/bin/bash
# Git push using credentials from .env
# Usage: ./scripts/push.sh [repo-path] [remote] [branch]

set -e

REPO_PATH="${1:-.}"
REMOTE="${2:-origin}"
# Detect current branch instead of defaulting to main
BRANCH="${3:-$(git -C "${REPO_PATH}" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")}"

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

# Pre-push validation
echo "Running pre-push validations..."

# 1. Check README for broken links (only if README.md exists)
if [ -f "README.md" ]; then
  echo "  → Checking README links..."

  # Extract markdown links: [text](path) format
  # Using sed for cross-platform compatibility (macOS doesn't have grep -P)
  broken_links=()
  while IFS= read -r link; do
    # Skip empty lines
    [ -z "$link" ] && continue

    # Skip external links (http/https)
    if [[ "$link" == http* ]]; then
      continue
    fi
    # Skip anchor links (#section)
    if [[ "$link" == \#* ]]; then
      continue
    fi

    # Check if file or directory exists
    if [ ! -f "$link" ] && [ ! -d "$link" ]; then
      broken_links+=("$link")
    fi
  done < <(sed -n 's/.*](\([^)]*\)).*/\1/p' README.md 2>/dev/null || true)

  if [ ${#broken_links[@]} -gt 0 ]; then
    echo ""
    echo "  ✗ Broken links found in README.md:"
    for link in "${broken_links[@]}"; do
      echo "    - $link"
    done
    echo ""
    echo "  Fix these links before pushing."
    echo "  (Tip: Links to submodules require submodules to be initialized)"
    exit 1
  fi

  echo "    ✓ README links valid"
fi

# 2. Verify git submodules are initialized (only if .gitmodules exists)
if [ -f ".gitmodules" ]; then
  echo "  → Verifying submodules..."

  uninitialized=$(git submodule status | grep -c '^-' || true)
  if [ "$uninitialized" -gt 0 ]; then
    echo ""
    echo "  ✗ Found $uninitialized uninitialized submodule(s)"
    echo "  Run: git submodule update --init --recursive"
    exit 1
  fi

  echo "    ✓ Submodules initialized"
fi

echo "✓ Pre-push validations passed"
echo ""

# Pull latest changes to avoid conflicts (GitHub Actions may have committed)
echo "Pulling latest changes from $REMOTE/$BRANCH..."
git pull --rebase "$REMOTE" "$BRANCH" || {
  echo "ERROR: Failed to pull and rebase. Please resolve conflicts manually."
  exit 1
}
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
