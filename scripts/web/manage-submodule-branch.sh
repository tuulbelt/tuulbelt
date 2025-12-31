#!/bin/bash
# Manage submodule feature branches
# Usage: ./manage-submodule-branch.sh <submodule-path>
# - Checks if branch exists for current meta branch
# - Creates branch if needed (named after meta branch)
# - Checks out the branch
# - Updates tracking file

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/tracking-lib.sh"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Check arguments
if [ $# -lt 1 ]; then
  echo "Usage: $0 <submodule-path>"
  echo "Example: $0 tools/file-based-semaphore"
  exit 1
fi

SUBMODULE="$1"
REPO_ROOT=$(get_repo_root)
SUBMODULE_PATH="$REPO_ROOT/$SUBMODULE"

# Validate submodule exists
if [ ! -d "$SUBMODULE_PATH" ]; then
  echo -e "${RED}ERROR:${NC} Submodule not found: $SUBMODULE"
  exit 1
fi

# Get current meta branch (this will be the submodule branch name too)
CURRENT_META_BRANCH=$(get_current_meta_branch)
if [ -z "$CURRENT_META_BRANCH" ]; then
  echo -e "${RED}ERROR:${NC} Not on any branch"
  exit 1
fi

echo "🔀 Managing submodule branch..."
echo ""
echo -e "${BLUE}Meta branch:${NC} $CURRENT_META_BRANCH"
echo -e "${BLUE}Submodule:${NC} $SUBMODULE"
echo ""

# Ensure session exists
if ! has_session "$CURRENT_META_BRANCH"; then
  echo -e "${YELLOW}⚠️  Session not initialized. Run: ./scripts/web/init-session.sh${NC}"
  exit 1
fi

# Navigate to submodule
cd "$SUBMODULE_PATH"

# Check if branch exists locally
if git show-ref --verify --quiet "refs/heads/$CURRENT_META_BRANCH"; then
  # Branch exists locally
  echo -e "${GREEN}✓${NC} Branch exists locally: $CURRENT_META_BRANCH"
  git checkout "$CURRENT_META_BRANCH"
  echo -e "${GREEN}✓${NC} Checked out branch: $CURRENT_META_BRANCH"

elif git show-ref --verify --quiet "refs/remotes/origin/$CURRENT_META_BRANCH"; then
  # Branch exists on remote
  echo -e "${YELLOW}ℹ️${NC} Branch exists on remote: $CURRENT_META_BRANCH"
  git checkout -b "$CURRENT_META_BRANCH" "origin/$CURRENT_META_BRANCH"
  echo -e "${GREEN}✓${NC} Checked out remote branch: $CURRENT_META_BRANCH"

else
  # Branch doesn't exist - create it
  echo -e "${YELLOW}🆕${NC} Creating new branch: $CURRENT_META_BRANCH"

  # Get current branch to create from (prefer main, fallback to current)
  if git show-ref --verify --quiet "refs/heads/main"; then
    SOURCE_BRANCH="main"
  elif git show-ref --verify --quiet "refs/remotes/origin/main"; then
    SOURCE_BRANCH="origin/main"
  else
    SOURCE_BRANCH=$(git branch --show-current)
  fi

  echo "  Creating from: $SOURCE_BRANCH"
  git checkout -b "$CURRENT_META_BRANCH" "$SOURCE_BRANCH"
  echo -e "${GREEN}✓${NC} Created branch: $CURRENT_META_BRANCH"

  # Push to remote with upstream tracking
  echo "  Pushing to remote..."
  git push -u origin "$CURRENT_META_BRANCH"
  echo -e "${GREEN}✓${NC} Pushed to remote: origin/$CURRENT_META_BRANCH"
fi

# Go back to repo root
cd "$REPO_ROOT"

# Add submodule to tracking if not already there
if ! submodule_in_session "$CURRENT_META_BRANCH" "$SUBMODULE"; then
  add_submodule_to_session "$CURRENT_META_BRANCH" "$SUBMODULE"
fi

echo ""
echo -e "${GREEN}🎉 Submodule branch ready!${NC}"
echo ""
echo "Submodule: $SUBMODULE"
echo "Branch: $CURRENT_META_BRANCH"
echo "Path: $SUBMODULE_PATH"
