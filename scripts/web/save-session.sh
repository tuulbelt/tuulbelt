#!/bin/bash
# Save web session state to git
# - Commits tracking file changes
# - Ensures session state persists across filesystem recreation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/tracking-lib.sh"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Detect environment
if [ "${CLAUDE_CODE_REMOTE}" != "true" ]; then
  # Not in Web environment - nothing to save
  exit 0
fi

REPO_ROOT=$(get_repo_root)
cd "$REPO_ROOT"

# Check if tracking file exists and has changes
if [ ! -f "$TRACKING_FILE" ]; then
  # No tracking file - nothing to save
  exit 0
fi

# Check if tracking file has uncommitted changes
if git diff --quiet HEAD -- "$TRACKING_FILE" 2>/dev/null && \
   ! git ls-files --others --exclude-standard | grep -q "^${TRACKING_FILE}$" 2>/dev/null; then
  # No changes to save
  exit 0
fi

echo "💾 Saving session state..."

# Add tracking file to git
git add "$TRACKING_FILE"

# Commit if there are staged changes
if ! git diff --cached --quiet 2>/dev/null; then
  CURRENT_BRANCH=$(get_current_meta_branch)
  git commit -m "chore(web): update session tracking for $CURRENT_BRANCH" >/dev/null 2>&1 || true
  echo -e "${GREEN}✓${NC} Session state saved to git"
else
  echo -e "${YELLOW}ℹ️${NC} No changes to save"
fi
