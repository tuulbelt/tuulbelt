#!/bin/bash
# Migrate commits from main to feature branch
# Cherry-picks all commits on main that aren't on the feature branch

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/tracking-lib.sh"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

if [ $# -lt 1 ]; then
  echo "Usage: $0 <submodule-path>"
  exit 1
fi

SUBMODULE="$1"
REPO_ROOT=$(get_repo_root)
SUBMODULE_PATH="$REPO_ROOT/$SUBMODULE"

if [ ! -d "$SUBMODULE_PATH" ]; then
  echo "ERROR: Submodule not found: $SUBMODULE"
  exit 1
fi

CURRENT_META_BRANCH=$(get_current_meta_branch)

echo "🔀 Migrating commits to feature branch..."
echo ""
echo -e "${BLUE}Submodule:${NC} $SUBMODULE"
echo -e "${BLUE}Feature branch:${NC} $CURRENT_META_BRANCH"
echo ""

cd "$SUBMODULE_PATH"

# Ensure we're on the feature branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "$CURRENT_META_BRANCH" ]; then
  echo "Checking out feature branch..."
  git checkout "$CURRENT_META_BRANCH"
fi

# Fetch latest from origin
echo "Fetching latest from origin..."
git fetch origin main >/dev/null 2>&1

# Find commits on main that aren't on feature branch
COMMITS_TO_CHERRY_PICK=$(git log --oneline "$CURRENT_META_BRANCH..origin/main" --reverse | awk '{print $1}')

if [ -z "$COMMITS_TO_CHERRY_PICK" ]; then
  echo -e "${GREEN}✓${NC} No commits to migrate - feature branch is up to date"
  cd "$REPO_ROOT"
  exit 0
fi

COMMIT_COUNT=$(echo "$COMMITS_TO_CHERRY_PICK" | wc -l)
echo -e "${YELLOW}Found $COMMIT_COUNT commit(s) to migrate:${NC}"
git log --oneline "$CURRENT_META_BRANCH..origin/main" --reverse
echo ""

# Cherry-pick each commit
echo "Cherry-picking commits..."
for commit in $COMMITS_TO_CHERRY_PICK; do
  echo "  Cherry-picking $commit..."
  if git cherry-pick "$commit"; then
    echo -e "  ${GREEN}✓${NC} Success"
  else
    echo -e "  ${YELLOW}⚠️  Conflict detected${NC}"
    echo "  Please resolve conflicts and run: git cherry-pick --continue"
    exit 1
  fi
done

echo ""
echo -e "${GREEN}✓${NC} All commits migrated successfully"

# Push to origin
echo "Pushing to origin..."
git push origin "$CURRENT_META_BRANCH"

echo -e "${GREEN}✓${NC} Pushed to remote"

# Update tracking
LAST_COMMIT=$(git rev-parse HEAD)
cd "$REPO_ROOT"

source "$SCRIPT_DIR/tracking-lib.sh"
update_submodule_commits "$CURRENT_META_BRANCH" "$SUBMODULE" "$LAST_COMMIT" "$COMMIT_COUNT"

echo ""
echo -e "${GREEN}🎉 Migration complete!${NC}"
echo "Migrated $COMMIT_COUNT commit(s) to $CURRENT_META_BRANCH"
