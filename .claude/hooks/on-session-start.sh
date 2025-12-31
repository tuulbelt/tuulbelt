#!/bin/bash
# Session start hook - runs automatically when Claude Code session starts
# Integrates Web workflow for submodule management

set -e

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Only run in Web environment
if [ "${CLAUDE_CODE_REMOTE}" = "true" ]; then
  echo "🌐 Claude Code Web detected - initializing Web workflow..."
  echo ""

  # Run web-setup (sets up credentials, tracking, etc.)
  if [ -f "scripts/web/init-session.sh" ]; then
    ./scripts/web/init-session.sh
    echo ""

    # Show current status
    if [ -f "scripts/web/show-status.sh" ]; then
      echo "Current session status:"
      echo ""
      ./scripts/web/show-status.sh
      echo ""
      echo "Tip: Run /web-status anytime to see current state"
    fi
  else
    echo "⚠️  Web scripts not found - workflow not initialized"
    echo "This might be an older checkout. Pull latest changes."
  fi
else
  # CLI environment - no special setup needed
  echo "💻 Claude Code CLI detected - using standard workflow"
fi

echo ""
echo "Session ready! 🚀"
