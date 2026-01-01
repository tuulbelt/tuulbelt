#!/bin/bash
# Initialize workspace (environment-aware)

set -e

# Show help
if [ "$1" = "--help" ] || [ "$1" = "-h" ] || [ -z "$1" ]; then
  cat <<EOF
/work-init - Create a new workspace for feature development

Usage:
  /work-init <feature-name> [--no-worktree]
  /work-init --help

Parameters:
  feature-name       Branch name (e.g., feature/my-feature)
  --no-worktree      (Optional, CLI only) Use branch instead of worktree
  --help, -h         Show this help message

Examples:
  /work-init feature/component-prop-validator
  /work-init fix/docs-typo --no-worktree

Branch Naming:
  Format: <type>/<description>
  Types: feature, fix, chore, refactor

Environment Behavior:
  CLI (default):         Creates worktree at .claude/worktrees/
  CLI (--no-worktree):   Creates branch in main working directory
  Web:                   Creates feature branch (no worktrees)

EOF
  exit 0
fi

FEATURE_NAME="$1"
NO_WORKTREE="${2:-}"

# Validate branch name format
if ! echo "$FEATURE_NAME" | grep -qE '^(feature|fix|chore|refactor)/[a-z0-9-]+$'; then
  echo "❌ Invalid branch name format"
  echo "Expected: <type>/<description>"
  echo "Types: feature, fix, chore, refactor"
  echo "Example: feature/component-prop-validator"
  exit 1
fi

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Detect environment
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment: Always use branch-based
  echo "🌐 Web environment detected - creating feature branch..."
  "$SCRIPT_DIR/../web/create-session-branches.sh" "$FEATURE_NAME"
else
  # CLI environment: Worktree or branch
  if [ "$NO_WORKTREE" = "--no-worktree" ]; then
    echo "💻 CLI environment - creating feature branch (no worktree)..."
    "$SCRIPT_DIR/../cli/create-branch.sh" "$FEATURE_NAME"
  else
    echo "💻 CLI environment - creating worktree..."
    "$SCRIPT_DIR/../cli/create-worktree.sh" "$FEATURE_NAME"
  fi
fi
