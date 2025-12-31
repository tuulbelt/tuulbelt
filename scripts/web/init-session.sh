#!/bin/bash
# Initialize or resume web session
# - Detects environment (Web vs CLI)
# - Sets up credentials for Web
# - Initializes tracking file
# - Creates session for current meta branch

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/tracking-lib.sh"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "🌐 Initializing Web Session..."
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

# Step 4: Initialize tracking file if needed
if ! tracking_file_exists; then
  echo ""
  echo "Initializing tracking file..."
  init_tracking_file
fi

# Step 5: Check if session exists for current meta branch
echo ""
if has_session "$CURRENT_META_BRANCH"; then
  echo -e "${GREEN}✓${NC} Resuming existing session for: $CURRENT_META_BRANCH"

  # List submodules in session
  SUBMODULES=$(get_session_submodules "$CURRENT_META_BRANCH")
  if [ -n "$SUBMODULES" ]; then
    echo ""
    echo "Tracked submodules:"
    while IFS= read -r submodule; do
      echo "  - $submodule"
    done <<< "$SUBMODULES"
  else
    echo "  (no submodules tracked yet)"
  fi
else
  echo -e "${YELLOW}🆕${NC} Creating new session for: $CURRENT_META_BRANCH"
  create_session "$CURRENT_META_BRANCH"
fi

# Step 6: Initialize all git submodules
echo ""
echo "Ensuring all submodules are initialized..."
git submodule update --init --recursive >/dev/null 2>&1 || true
echo -e "${GREEN}✓${NC} Submodules initialized"

echo ""
echo -e "${GREEN}🎉 Session ready!${NC}"
echo ""
echo "Next steps:"
echo "  - Make changes to submodules"
echo "  - Use: ./scripts/web/checkout-submodule-branch.sh <submodule>"
echo "  - Use: ./scripts/web/show-status.sh to see current state"
