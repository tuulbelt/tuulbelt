#!/bin/bash
# Create pull requests for changed repositories

set -e

# Show help
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
  cat <<EOF
/work-pr - Create pull requests for changed repositories

Usage:
  /work-pr [--meta] [--submodules]
  /work-pr --help

Flags:
  --meta         Create PR for meta repo only
  --submodules   Create PRs for submodules only
  (no flags)     Create PRs for both meta and submodules

Behavior:
  1. Detects current workspace (worktree or branch)
  2. Finds repos with uncommitted/unpushed changes
  3. Pushes branches to remote
  4. Creates PRs via gh CLI
  5. Updates tracking file with PR URLs

Examples:
  /work-pr                  # Create PRs for all changed repos
  /work-pr --meta           # Meta repo only
  /work-pr --submodules     # Submodules only

Requirements:
  - gh CLI installed and authenticated
  - Changes committed to feature branch
  - Remote repository exists

EOF
  exit 0
fi

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check for gh CLI
if ! command -v gh &> /dev/null; then
  echo "❌ ERROR: gh CLI not found"
  echo "Install: brew install gh (macOS) or see https://cli.github.com"
  echo "Then authenticate: gh auth login"
  exit 1
fi

# Detect environment and delegate
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment
  "$SCRIPT_DIR/../web/create-web-prs.sh" "$@"
else
  # CLI environment
  "$SCRIPT_DIR/../cli/create-cli-prs.sh" "$@"
fi
