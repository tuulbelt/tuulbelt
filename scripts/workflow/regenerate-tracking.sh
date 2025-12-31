#!/bin/bash
# Regenerate tracking file from current git state

set -e

REPO_ROOT="$(git rev-parse --show-toplevel)"

# Detect environment
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  TRACKING_FILE="$REPO_ROOT/.claude/web-session-tracking.json"
  ENVIRONMENT="web"
else
  TRACKING_FILE="$REPO_ROOT/.claude/cli-workspace-tracking.json"
  ENVIRONMENT="cli"
fi

echo "Regenerating $ENVIRONMENT tracking file..."
echo ""

# Create backup
if [ -f "$TRACKING_FILE" ]; then
  BACKUP="$TRACKING_FILE.backup.$(date +%Y%m%d-%H%M%S)"
  cp "$TRACKING_FILE" "$BACKUP"
  echo "✓ Backed up existing file to: $BACKUP"
  echo ""
fi

if [ "$ENVIRONMENT" = "cli" ]; then
  # CLI: Scan for worktrees
  WORKTREES_DIR="$REPO_ROOT/.claude/worktrees"

  # Initialize tracking file
  cat > "$TRACKING_FILE" <<EOF
{
  "version": "1.0",
  "environment": "cli",
  "worktrees": {}
}
EOF

  if [ -d "$WORKTREES_DIR" ]; then
    for worktree_path in "$WORKTREES_DIR"/*; do
      if [ -d "$worktree_path" ]; then
        cd "$worktree_path"
        BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
        TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

        echo "Found worktree: $worktree_path (branch: $BRANCH)"

        # Add to tracking file
        jq --arg path "$worktree_path" \
           --arg branch "$BRANCH" \
           --arg ts "$TIMESTAMP" \
           '.worktrees[$path] = {
             "meta_branch": $branch,
             "created_at": $ts,
             "updated_at": $ts,
             "status": "active",
             "submodules": {}
           }' "$TRACKING_FILE" > "$TRACKING_FILE.tmp"

        mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"
      fi
    done
  fi

  cd "$REPO_ROOT"

else
  # Web: Scan for current session
  CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")
  TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

  # Initialize tracking file
  cat > "$TRACKING_FILE" <<EOF
{
  "version": "1.0",
  "environment": "web",
  "sessions": {
    "current": {
      "feature_branch": "$CURRENT_BRANCH",
      "created_at": "$TIMESTAMP",
      "updated_at": "$TIMESTAMP",
      "status": "active",
      "meta_repo": {
        "branch": "$CURRENT_BRANCH",
        "has_changes": false,
        "commits_count": 0,
        "pr_url": null,
        "pr_number": null,
        "pr_state": null,
        "pr_merged": false
      },
      "submodules": {}
    }
  }
}
EOF

fi

echo ""
echo "✓ Tracking file regenerated: $TRACKING_FILE"
echo ""
echo "Review the file and adjust as needed."
