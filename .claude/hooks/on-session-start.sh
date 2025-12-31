#!/bin/bash
# Session start hook - runs automatically when Claude Code session starts
# Integrates Web workflow for submodule management

set -e

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Check and install gh CLI if not available (both Web and CLI)
if ! command -v gh &> /dev/null; then
  echo "📦 GitHub CLI (gh) not found - installing..."

  # Download and install gh binary
  GH_VERSION="2.40.1"
  GH_TARBALL="/tmp/gh_${GH_VERSION}_linux_amd64.tar.gz"

  if curl -sL "https://github.com/cli/cli/releases/download/v${GH_VERSION}/gh_${GH_VERSION}_linux_amd64.tar.gz" -o "$GH_TARBALL" 2>/dev/null; then
    tar -xzf "$GH_TARBALL" -C /tmp 2>/dev/null
    if sudo mv "/tmp/gh_${GH_VERSION}_linux_amd64/bin/gh" /usr/local/bin/ 2>/dev/null; then
      rm -rf "$GH_TARBALL" "/tmp/gh_${GH_VERSION}_linux_amd64" 2>/dev/null
      echo "✓ GitHub CLI installed successfully (version $(gh --version | head -1))"
    else
      echo "⚠️  Failed to install gh CLI (sudo required)"
    fi
  else
    echo "⚠️  Failed to download gh CLI"
  fi
  echo ""
fi

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
