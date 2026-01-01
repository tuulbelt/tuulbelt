#!/bin/bash
# Update CLI workspace tracking file

set -e

ACTION="$1"  # add, update, remove, update-submodule
WORKTREE_DIR="$2"
FEATURE_NAME="$3"

# Validate required parameters for each action
case "$ACTION" in
  add)
    if [ -z "$WORKTREE_DIR" ] || [ -z "$FEATURE_NAME" ]; then
      echo "Error: 'add' requires <worktree-dir> <feature-name>"
      echo "Usage: $0 add <worktree-dir> <feature-name>"
      exit 1
    fi
    ;;
  update)
    if [ -z "$WORKTREE_DIR" ]; then
      echo "Error: 'update' requires <worktree-dir>"
      echo "Usage: $0 update <worktree-dir>"
      exit 1
    fi
    ;;
  remove)
    if [ -z "$WORKTREE_DIR" ]; then
      echo "Error: 'remove' requires <worktree-dir>"
      echo "Usage: $0 remove <worktree-dir>"
      exit 1
    fi
    ;;
  update-submodule)
    if [ -z "$WORKTREE_DIR" ] || [ -z "$4" ] || [ -z "$5" ]; then
      echo "Error: 'update-submodule' requires <worktree-dir> <feature-name> <submodule-path> <submodule-branch>"
      echo "Usage: $0 update-submodule <worktree-dir> <feature-name> <submodule-path> <submodule-branch> [has-changes] [commits-count]"
      exit 1
    fi
    ;;
  *)
    echo "Error: Unknown action '$ACTION'"
    echo "Usage: $0 <add|update|remove|update-submodule> <worktree-dir> [args...]"
    exit 1
    ;;
esac

# Get main repo root (works from worktrees too)
COMMON_GIT_DIR="$(git rev-parse --git-common-dir)"
# Convert to absolute path if relative
if [[ "$COMMON_GIT_DIR" != /* ]]; then
  COMMON_GIT_DIR="$(pwd)/$COMMON_GIT_DIR"
fi
REPO_ROOT="$(dirname "$COMMON_GIT_DIR")"
TRACKING_FILE="$REPO_ROOT/.claude/cli-workspace-tracking.json"

# Initialize tracking file if it doesn't exist
if [ ! -f "$TRACKING_FILE" ]; then
  cat > "$TRACKING_FILE" <<EOF
{
  "version": "1.0",
  "environment": "cli",
  "worktrees": {}
}
EOF
fi

# Helper: Get current timestamp in ISO 8601 format
get_timestamp() {
  date -u +"%Y-%m-%dT%H:%M:%SZ"
}

# Action: Add new worktree entry
if [ "$ACTION" = "add" ]; then
  TIMESTAMP=$(get_timestamp)

  # Create entry using jq
  jq --arg path "$WORKTREE_DIR" \
     --arg branch "$FEATURE_NAME" \
     --arg ts "$TIMESTAMP" \
     '.worktrees[$path] = {
       "meta_branch": $branch,
       "created_at": $ts,
       "updated_at": $ts,
       "status": "active",
       "submodules": {}
     }' "$TRACKING_FILE" > "$TRACKING_FILE.tmp"

  mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"

elif [ "$ACTION" = "update" ]; then
  # Update existing worktree entry (e.g., after PR creation)
  TIMESTAMP=$(get_timestamp)

  jq --arg path "$WORKTREE_DIR" \
     --arg ts "$TIMESTAMP" \
     '.worktrees[$path].updated_at = $ts' "$TRACKING_FILE" > "$TRACKING_FILE.tmp"

  mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"

elif [ "$ACTION" = "remove" ]; then
  # Remove worktree entry
  jq --arg path "$WORKTREE_DIR" \
     'del(.worktrees[$path])' "$TRACKING_FILE" > "$TRACKING_FILE.tmp"

  mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"

elif [ "$ACTION" = "update-submodule" ]; then
  # Update submodule info within a worktree
  SUBMODULE_PATH="$4"
  SUBMODULE_BRANCH="$5"
  HAS_CHANGES="${6:-false}"
  COMMITS_COUNT="${7:-0}"

  TIMESTAMP=$(get_timestamp)

  jq --arg worktree "$WORKTREE_DIR" \
     --arg submodule "$SUBMODULE_PATH" \
     --arg branch "$SUBMODULE_BRANCH" \
     --arg ts "$TIMESTAMP" \
     --argjson changes "$HAS_CHANGES" \
     --argjson commits "$COMMITS_COUNT" \
     '.worktrees[$worktree].submodules[$submodule] = {
       "branch": $branch,
       "created_at": $ts,
       "has_changes": $changes,
       "commits_count": $commits,
       "last_commit_sha": null,
       "pr_url": null,
       "pr_number": null,
       "pr_state": null,
       "pr_merged": false
     }' "$TRACKING_FILE" > "$TRACKING_FILE.tmp"

  mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"

fi
