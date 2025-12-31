#!/bin/bash
# Show CLI workspace status

set -e

REPO_ROOT="$(git rev-parse --show-toplevel)"
TRACKING_FILE="$REPO_ROOT/.claude/cli-workspace-tracking.json"

# Print header
echo "┌──────────────────────────────────────────────────────────────┐"
echo "│ CLI Workspace Status                                         │"
echo "└──────────────────────────────────────────────────────────────┘"
echo ""

# Check if tracking file exists
if [ ! -f "$TRACKING_FILE" ]; then
  echo "No active worktrees found."
  echo ""
  echo "Use /work-init to create a new workspace."
  exit 0
fi

# Check if there are any worktrees
WORKTREE_COUNT=$(jq '.worktrees | length' "$TRACKING_FILE")

if [ "$WORKTREE_COUNT" -eq 0 ]; then
  echo "No active worktrees found."
  echo ""
  echo "Use /work-init to create a new workspace."
  exit 0
fi

# Display each worktree
jq -r '.worktrees | to_entries[] | @json' "$TRACKING_FILE" | while IFS= read -r entry; do
  WORKTREE_PATH=$(echo "$entry" | jq -r '.key')
  META_BRANCH=$(echo "$entry" | jq -r '.value.meta_branch')
  STATUS=$(echo "$entry" | jq -r '.value.status')
  CREATED_AT=$(echo "$entry" | jq -r '.value.created_at')
  UPDATED_AT=$(echo "$entry" | jq -r '.value.updated_at')

  # Get commits count in meta repo
  if [ -d "$WORKTREE_PATH" ]; then
    cd "$WORKTREE_PATH"
    META_COMMITS=$(git rev-list --count HEAD ^origin/main 2>/dev/null || echo "0")
  else
    META_COMMITS="0 (worktree not found)"
  fi

  echo "Worktree: $WORKTREE_PATH"
  echo "Meta Branch: $META_BRANCH ($META_COMMITS commits)"
  echo "Status: $STATUS"
  echo "Created: $CREATED_AT"
  echo "Updated: $UPDATED_AT"
  echo ""

  # Display submodules table
  SUBMODULE_COUNT=$(echo "$entry" | jq '.value.submodules | length')

  if [ "$SUBMODULE_COUNT" -gt 0 ]; then
    echo "┌────────────────────────────────┬─────────┬─────────┬──────┐"
    echo "│ Submodule                      │ Changes │ Commits │ PR   │"
    echo "├────────────────────────────────┼─────────┼─────────┼──────┤"

    echo "$entry" | jq -r '.value.submodules | to_entries[] | "\(.key)|\(.value.has_changes)|\(.value.commits_count)|\(.value.pr_number // "-")"' | while IFS='|' read -r submodule has_changes commits pr; do
      # Truncate long submodule names
      SHORT_NAME=$(basename "$submodule")
      if [ ${#SHORT_NAME} -gt 30 ]; then
        SHORT_NAME="${SHORT_NAME:0:27}..."
      fi

      # Format PR
      if [ "$pr" = "-" ] || [ "$pr" = "null" ]; then
        PR_DISPLAY="-"
      else
        PR_DISPLAY="#$pr"
      fi

      # Format changes
      if [ "$has_changes" = "true" ]; then
        CHANGES_DISPLAY="Yes"
      else
        CHANGES_DISPLAY="No"
      fi

      printf "│ %-30s │ %-7s │ %-7s │ %-4s │\n" "$SHORT_NAME" "$CHANGES_DISPLAY" "$commits" "$PR_DISPLAY"
    done

    echo "└────────────────────────────────┴─────────┴─────────┴──────┘"
  else
    echo "No submodule changes tracked."
  fi

  echo ""
done
