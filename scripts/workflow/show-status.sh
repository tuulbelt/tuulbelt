#!/bin/bash
# Show workspace status (environment-aware)

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
  cat <<EOF
/work-status - Display current workspace state

Usage:
  /work-status
  /work-status --help

Shows:
  - Current environment (CLI or Web)
  - Active workspaces/sessions
  - Branch names
  - Changed files count
  - Commits count
  - PR status

EOF
  exit 0
fi

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment
  "$SCRIPT_DIR/../web/show-web-status.sh"
else
  # CLI environment
  "$SCRIPT_DIR/../cli/show-cli-status.sh"
fi
