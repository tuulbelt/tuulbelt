#!/bin/bash
# Initialize or resume web session
# - Detects environment (Web vs CLI)
# - Sets up credentials for Web
# - Initializes tracking file
# - Creates session for current meta branch

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/tracking-lib.sh"

# Auto-detect if we should disable colors
# In Web environment, default to no colors (terminal doesn't interpret ANSI codes)
NO_COLOR=false
if [ "${CLAUDE_CODE_REMOTE}" = "true" ]; then
  NO_COLOR=true
fi

# Colors (disabled in non-interactive terminals)
if [ "$NO_COLOR" = true ]; then
  GREEN=''
  YELLOW=''
  BLUE=''
  NC=''
else
  GREEN='\033[0;32m'
  YELLOW='\033[1;33m'
  BLUE='\033[0;34m'
  NC='\033[0m'
fi

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
# Note: In Claude Code Web, git submodule update --init may fail because
# it can't clone from external GitHub URLs through the proxy.
# Fallback: Use direct git clone for each submodule.
echo ""
echo "Ensuring all submodules are initialized..."

# Try standard submodule init first
if git submodule update --init --recursive >/dev/null 2>&1; then
  echo -e "${GREEN}✓${NC} Submodules initialized (standard method)"
else
  echo "  Standard submodule init failed, using direct clone fallback..."

  # Parse .gitmodules and clone each submodule directly
  while IFS= read -r line; do
    if [[ "$line" =~ path\ =\ (.+) ]]; then
      SUBMODULE_PATH="${BASH_REMATCH[1]}"
    elif [[ "$line" =~ url\ =\ (.+) ]]; then
      SUBMODULE_URL="${BASH_REMATCH[1]}"

      # Check if submodule is actually a valid git repository
      # A proper submodule should be able to run git rev-parse successfully
      NEEDS_CLONE=false
      if [ -d "$SUBMODULE_PATH" ]; then
        if ! git -C "$SUBMODULE_PATH" rev-parse --git-dir >/dev/null 2>&1; then
          # Directory exists but git commands fail - broken state, needs re-clone
          NEEDS_CLONE=true
          echo "  ⚠ $SUBMODULE_PATH has broken .git - re-cloning..."
          rm -rf "$SUBMODULE_PATH"
        else
          echo "  ✓ $SUBMODULE_PATH (already initialized)"
        fi
      else
        NEEDS_CLONE=true
      fi

      if [ "$NEEDS_CLONE" = true ]; then
        echo "  → Cloning $SUBMODULE_PATH..."
        if git clone "$SUBMODULE_URL" "$SUBMODULE_PATH" >/dev/null 2>&1; then
          echo "  ✓ $SUBMODULE_PATH"
        else
          echo "  ⚠ Failed to clone $SUBMODULE_PATH"
        fi
      fi
    fi
  done < .gitmodules

  echo -e "${GREEN}✓${NC} Submodules initialized (direct clone method)"
fi

echo ""
echo -e "${GREEN}🎉 Session ready!${NC}"
echo ""
echo "Next steps:"
echo "  - Make changes to submodules"
echo "  - Use: ./scripts/web/checkout-submodule-branch.sh <submodule>"
echo "  - Use: ./scripts/web/show-status.sh to see current state"
