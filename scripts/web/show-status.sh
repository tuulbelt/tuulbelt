#!/bin/bash
# Display status of current web session
# Shows tracked submodules, branches, commits, and PR status

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/tracking-lib.sh"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

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
  # Get submodule info from tracking
  SUBMOD_INFO=$(jq ".sessions[\"$CURRENT_META_BRANCH\"].submodules[\"$submodule\"]" \
    "$REPO_ROOT/$TRACKING_FILE")

  HAS_CHANGES=$(echo "$SUBMOD_INFO" | jq -r '.has_changes')
  COMMITS_COUNT=$(echo "$SUBMOD_INFO" | jq -r '.commits_count')
  PR_NUMBER=$(echo "$SUBMOD_INFO" | jq -r '.pr_number')
  PR_MERGED=$(echo "$SUBMOD_INFO" | jq -r '.pr_merged')

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
