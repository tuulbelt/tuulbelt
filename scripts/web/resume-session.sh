#!/bin/bash
# Resume web session after restart
# - Restores session from tracking file
# - Checks out submodule branches
# - Shows current session status

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/tracking-lib.sh"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "🔄 Resuming Web Session..."
echo ""

# Step 1: Detect environment
if [ "${CLAUDE_CODE_REMOTE}" != "true" ]; then
  echo -e "${YELLOW}ℹ️  Not in Claude Code Web environment${NC}"
  echo "Web workflow not needed - using default CLI workflow"
  exit 0
fi

echo -e "${GREEN}✓${NC} Detected Claude Code Web environment"

# Step 2: Setup credentials
echo ""
echo "Setting up git credentials..."
"$SCRIPT_DIR/setup-credentials.sh"

# Step 3: Get current meta branch
CURRENT_META_BRANCH=$(get_current_meta_branch)
if [ -z "$CURRENT_META_BRANCH" ]; then
  echo "ERROR: Not on any branch"
  exit 1
fi

echo ""
echo -e "${BLUE}📍 Current meta branch:${NC} $CURRENT_META_BRANCH"

# Step 4: Initialize tracking file if needed, or resume session
if ! tracking_file_exists; then
  echo ""
  echo "No tracking file found - initializing..."
  init_tracking_file
  create_session "$CURRENT_META_BRANCH"
  echo ""
  echo -e "${YELLOW}🆕${NC} Created new session for: $CURRENT_META_BRANCH"
elif ! has_session "$CURRENT_META_BRANCH"; then
  echo ""
  echo "No existing session for this branch - creating..."
  create_session "$CURRENT_META_BRANCH"
  echo ""
  echo -e "${YELLOW}🆕${NC} Created new session for: $CURRENT_META_BRANCH"
else
  echo ""
  echo -e "${GREEN}✓${NC} Resuming existing session for: $CURRENT_META_BRANCH"

  # Step 5: Checkout submodule branches if they exist in tracking
  SUBMODULES=$(get_session_submodules "$CURRENT_META_BRANCH")
  if [ -n "$SUBMODULES" ]; then
    echo ""
    echo "Restoring submodule branches..."

    REPO_ROOT=$(get_repo_root)
    cd "$REPO_ROOT"

    while IFS= read -r submodule; do
      # Get branch name from tracking file
      BRANCH=$(jq -r ".sessions[\"$CURRENT_META_BRANCH\"].submodules[\"$submodule\"].branch" "$REPO_ROOT/$TRACKING_FILE")

      if [ -n "$BRANCH" ] && [ "$BRANCH" != "null" ]; then
        echo "  📦 $submodule → $BRANCH"

        # Change to submodule directory
        cd "$REPO_ROOT/$submodule"

        # Check if branch exists (local or remote)
        if git show-ref --verify --quiet "refs/heads/$BRANCH" 2>/dev/null; then
          # Branch exists locally - checkout
          git checkout "$BRANCH" >/dev/null 2>&1
        elif git show-ref --verify --quiet "refs/remotes/origin/$BRANCH" 2>/dev/null; then
          # Branch exists remotely - fetch and checkout
          git fetch origin "$BRANCH:$BRANCH" >/dev/null 2>&1
          git checkout "$BRANCH" >/dev/null 2>&1
        else
          echo "    ⚠️  Branch $BRANCH not found (local or remote) - creating new"
          git checkout -b "$BRANCH" >/dev/null 2>&1
        fi

        # Return to repo root for next iteration
        cd "$REPO_ROOT"
      fi
    done <<< "$SUBMODULES"

    echo ""
    echo -e "${GREEN}✓${NC} Submodule branches restored"
  else
    echo "  (no submodules tracked yet)"
  fi
fi

# Step 6: Initialize all git submodules
echo ""
echo "Ensuring all submodules are initialized..."
git submodule update --init --recursive >/dev/null 2>&1 || true
echo -e "${GREEN}✓${NC} Submodules initialized"

echo ""
echo -e "${GREEN}🎉 Session resumed successfully!${NC}"
