#!/bin/bash
# Display status of current web session
# Shows tracked submodules, branches, commits, and PR status
# Usage: ./show-status.sh [--no-color|--color]
# Note: Auto-detects Web environment and defaults to plain text output

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || dirname "$(dirname "$SCRIPT_DIR")")"

# Load GitHub credentials
source "$REPO_ROOT/scripts/lib/load-credentials.sh"

source "$SCRIPT_DIR/tracking-lib.sh"

# Auto-detect if we should disable colors
# In Web environment, default to no colors (terminal doesn't interpret ANSI codes)
NO_COLOR=false
if [ "${CLAUDE_CODE_REMOTE}" = "true" ]; then
  NO_COLOR=true
fi

# Allow explicit override with flags
if [ "$1" = "--color" ]; then
  NO_COLOR=false
elif [ "$1" = "--no-color" ] || [ "$1" = "--plain" ]; then
  NO_COLOR=true
fi

# Colors (disabled if --no-color)
if [ "$NO_COLOR" = true ]; then
  GREEN=''
  YELLOW=''
  BLUE=''
  CYAN=''
  NC=''
else
  GREEN='\033[0;32m'
  YELLOW='\033[1;33m'
  BLUE='\033[0;34m'
  CYAN='\033[0;36m'
  NC='\033[0m'
fi

# Check if in Web environment
if [ "${CLAUDE_CODE_REMOTE}" != "true" ]; then
  echo "Not in Claude Code Web environment"
  exit 0
fi

# Get current meta branch
CURRENT_META_BRANCH=$(get_current_meta_branch)
if [ -z "$CURRENT_META_BRANCH" ]; then
  echo "ERROR: Not on any branch"
  exit 1
fi

# Check if tracking file exists
if ! tracking_file_exists; then
  echo "No tracking file found"
  echo "Run: ./scripts/web/init-session.sh"
  exit 1
fi

# Check if session exists
if ! has_session "$CURRENT_META_BRANCH"; then
  echo "No session found for branch: $CURRENT_META_BRANCH"
  echo "Run: ./scripts/web/init-session.sh"
  exit 1
fi

# Get session info
SESSION_INFO=$(get_session_info "$CURRENT_META_BRANCH")
SESSION_ID=$(echo "$SESSION_INFO" | jq -r '.session_id')
CREATED_AT=$(echo "$SESSION_INFO" | jq -r '.created_at')
STATUS=$(echo "$SESSION_INFO" | jq -r '.status')

# Display session header
echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC}  ${BLUE}Web Session Status${NC}                                              ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Meta Branch:${NC} $CURRENT_META_BRANCH"
echo -e "${BLUE}Session ID:${NC} $SESSION_ID"
echo -e "${BLUE}Created:${NC} $CREATED_AT"
echo -e "${BLUE}Status:${NC} $STATUS"
echo ""

# Get submodules
SUBMODULES=$(get_session_submodules "$CURRENT_META_BRANCH")

if [ -z "$SUBMODULES" ]; then
  echo "No submodules tracked yet"
  echo ""
  echo "To add a submodule:"
  echo "  ./scripts/web/manage-submodule-branch.sh <submodule-path>"
  exit 0
fi

# Display table header
echo -e "${CYAN}┌────────────────────────────────────┬─────────┬─────────┬──────────┬───────────┐${NC}"
printf "${CYAN}│${NC} %-34s ${CYAN}│${NC} %-7s ${CYAN}│${NC} %-7s ${CYAN}│${NC} %-8s ${CYAN}│${NC} %-9s ${CYAN}│${NC}\n" \
  "Submodule" "Changes" "Commits" "PR" "Merged"
echo -e "${CYAN}├────────────────────────────────────┼─────────┼─────────┼──────────┼───────────┤${NC}"

# Display each submodule
REPO_ROOT=$(get_repo_root)
while IFS= read -r submodule; do
  # Get REAL-TIME status for each submodule
  SUBMOD_PATH="$REPO_ROOT/$submodule"

  # Get current branch and count commits vs main (real-time)
  COMMITS_COUNT="0"
  CURRENT_BRANCH=""
  if [ -d "$SUBMOD_PATH" ]; then
    cd "$SUBMOD_PATH"
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
    if [ -n "$CURRENT_BRANCH" ] && [ "$CURRENT_BRANCH" != "main" ]; then
      # Count commits on current branch that aren't on main
      COMMITS_COUNT=$(git rev-list --count main.."$CURRENT_BRANCH" 2>/dev/null || echo "0")
    fi
    cd "$REPO_ROOT"
  fi

  # Check for ANY changes (real-time)
  # "Changes" = Yes if branch has commits vs main OR uncommitted changes
  HAS_CHANGES="false"
  if [ -d "$SUBMOD_PATH" ]; then
    cd "$SUBMOD_PATH"
    # Check for uncommitted changes in working tree
    if ! git diff --quiet 2>/dev/null || ! git diff --cached --quiet 2>/dev/null; then
      HAS_CHANGES="true"
    # Or check if branch has commits compared to main
    elif [ "$COMMITS_COUNT" != "0" ] && [ "$COMMITS_COUNT" != "null" ]; then
      HAS_CHANGES="true"
    fi
    cd "$REPO_ROOT"
  fi

  # Check for PR using gh CLI (real-time)
  PR_NUMBER="null"
  PR_MERGED="false"
  if [ -d "$SUBMOD_PATH" ] && [ -n "$CURRENT_BRANCH" ] && [ "$CURRENT_BRANCH" != "main" ]; then
    cd "$SUBMOD_PATH"
    # Get PR number for current branch (including merged PRs)
    PR_INFO=$(gh pr list --state all --head "$CURRENT_BRANCH" --json number,state --jq '.[0]' 2>/dev/null || echo "null")
    if [ "$PR_INFO" != "null" ] && [ -n "$PR_INFO" ]; then
      PR_NUMBER=$(echo "$PR_INFO" | jq -r '.number' 2>/dev/null || echo "null")
      PR_STATE=$(echo "$PR_INFO" | jq -r '.state' 2>/dev/null || echo "OPEN")
      if [ "$PR_STATE" = "MERGED" ]; then
        PR_MERGED="true"
      fi
    fi
    cd "$REPO_ROOT"
  fi

  # Format output
  CHANGES_STR=$([ "$HAS_CHANGES" = "true" ] && echo -e "${YELLOW}Yes${NC}" || echo "No")
  COMMITS_STR=$([ "$COMMITS_COUNT" != "0" ] && [ "$COMMITS_COUNT" != "null" ] && echo "$COMMITS_COUNT" || echo "-")
  PR_STR=$([ "$PR_NUMBER" != "null" ] && echo "#$PR_NUMBER" || echo "-")
  MERGED_STR=$([ "$PR_MERGED" = "true" ] && echo -e "${GREEN}Yes${NC}" || echo "No")

  # Truncate submodule name if too long
  SUBMOD_SHORT=$(echo "$submodule" | sed 's/^tools\///')
  if [ ${#SUBMOD_SHORT} -gt 34 ]; then
    SUBMOD_SHORT="${SUBMOD_SHORT:0:31}..."
  fi

  printf "${CYAN}│${NC} %-34s ${CYAN}│${NC} %-7s ${CYAN}│${NC} %-7s ${CYAN}│${NC} %-8s ${CYAN}│${NC} %-9s ${CYAN}│${NC}\n" \
    "$SUBMOD_SHORT" "$CHANGES_STR" "$COMMITS_STR" "$PR_STR" "$MERGED_STR"

done <<< "$SUBMODULES"

echo -e "${CYAN}└────────────────────────────────────┴─────────┴─────────┴──────────┴───────────┘${NC}"
echo ""
