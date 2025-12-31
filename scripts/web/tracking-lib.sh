#!/bin/bash
# Library functions for managing web-session-tracking.json
# Provides CRUD operations for tracking file

# Tracking file location (relative to repo root)
TRACKING_FILE=".claude/web-session-tracking.json"

# Get repo root
get_repo_root() {
  git rev-parse --show-toplevel 2>/dev/null
}

# Get current meta branch
get_current_meta_branch() {
  git branch --show-current
}

# Check if tracking file exists
tracking_file_exists() {
  local repo_root=$(get_repo_root)
  [ -f "$repo_root/$TRACKING_FILE" ]
}

# Initialize empty tracking file
init_tracking_file() {
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  # Create .claude directory if it doesn't exist
  mkdir -p "$(dirname "$tracking_path")"

  # Create empty tracking structure
  cat > "$tracking_path" <<'EOF'
{
  "version": "1.0",
  "sessions": {}
}
EOF

  echo "✓ Initialized tracking file: $TRACKING_FILE"
}

# Check if session exists for a meta branch
has_session() {
  local meta_branch="$1"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! tracking_file_exists; then
    return 1
  fi

  jq -e ".sessions[\"$meta_branch\"] != null" "$tracking_path" >/dev/null 2>&1
}

# Create new session entry for meta branch
create_session() {
  local meta_branch="$1"
  local session_id="${2:-${CLAUDE_CODE_SESSION_ID:-unknown}}"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! tracking_file_exists; then
    init_tracking_file
  fi

  local timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

  # Create session entry
  jq --arg branch "$meta_branch" \
     --arg created "$timestamp" \
     --arg updated "$timestamp" \
     --arg sid "$session_id" \
     '.sessions[$branch] = {
       "created_at": $created,
       "updated_at": $updated,
       "session_id": $sid,
       "environment": "web",
       "status": "active",
       "submodules": {}
     }' "$tracking_path" > "$tracking_path.tmp"

  mv "$tracking_path.tmp" "$tracking_path"
  echo "✓ Created session for branch: $meta_branch"
}

# Get all submodules for a session
get_session_submodules() {
  local meta_branch="$1"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! has_session "$meta_branch"; then
    return 1
  fi

  jq -r ".sessions[\"$meta_branch\"].submodules | keys[]" "$tracking_path" 2>/dev/null
}

# Check if submodule is tracked in session
submodule_in_session() {
  local meta_branch="$1"
  local submodule="$2"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! has_session "$meta_branch"; then
    return 1
  fi

  jq -e ".sessions[\"$meta_branch\"].submodules[\"$submodule\"] != null" "$tracking_path" >/dev/null 2>&1
}

# Add submodule to session
add_submodule_to_session() {
  local meta_branch="$1"
  local submodule="$2"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! has_session "$meta_branch"; then
    echo "ERROR: Session does not exist for branch: $meta_branch"
    return 1
  fi

  local timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

  # Add submodule entry
  jq --arg branch "$meta_branch" \
     --arg submod "$submodule" \
     --arg created "$timestamp" \
     --arg updated "$timestamp" \
     '.sessions[$branch].submodules[$submod] = {
       "branch": $branch,
       "created_at": $created,
       "has_changes": false,
       "commits_count": 0,
       "last_commit_sha": null,
       "pr_url": null,
       "pr_number": null,
       "pr_state": null,
       "pr_merged": false
     } | .sessions[$branch].updated_at = $updated' "$tracking_path" > "$tracking_path.tmp"

  mv "$tracking_path.tmp" "$tracking_path"
  echo "✓ Added $submodule to session: $meta_branch"
}

# Update submodule commit info
update_submodule_commits() {
  local meta_branch="$1"
  local submodule="$2"
  local commit_sha="$3"
  local commits_count="$4"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! submodule_in_session "$meta_branch" "$submodule"; then
    echo "ERROR: Submodule $submodule not in session $meta_branch"
    return 1
  fi

  local timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

  jq --arg branch "$meta_branch" \
     --arg submod "$submodule" \
     --arg sha "$commit_sha" \
     --arg count "$commits_count" \
     --arg updated "$timestamp" \
     '.sessions[$branch].submodules[$submod].has_changes = true |
      .sessions[$branch].submodules[$submod].commits_count = ($count | tonumber) |
      .sessions[$branch].submodules[$submod].last_commit_sha = $sha |
      .sessions[$branch].updated_at = $updated' "$tracking_path" > "$tracking_path.tmp"

  mv "$tracking_path.tmp" "$tracking_path"
}

# Update submodule PR info
update_submodule_pr() {
  local meta_branch="$1"
  local submodule="$2"
  local pr_url="$3"
  local pr_number="$4"
  local pr_state="${5:-open}"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! submodule_in_session "$meta_branch" "$submodule"; then
    echo "ERROR: Submodule $submodule not in session $meta_branch"
    return 1
  fi

  local timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

  jq --arg branch "$meta_branch" \
     --arg submod "$submodule" \
     --arg url "$pr_url" \
     --arg num "$pr_number" \
     --arg state "$pr_state" \
     --arg updated "$timestamp" \
     '.sessions[$branch].submodules[$submod].pr_url = $url |
      .sessions[$branch].submodules[$submod].pr_number = ($num | tonumber) |
      .sessions[$branch].submodules[$submod].pr_state = $state |
      .sessions[$branch].submodules[$submod].pr_merged = ($state == "merged") |
      .sessions[$branch].updated_at = $updated' "$tracking_path" > "$tracking_path.tmp"

  mv "$tracking_path.tmp" "$tracking_path"
}

# Get session info as JSON
get_session_info() {
  local meta_branch="$1"
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! has_session "$meta_branch"; then
    echo "{}"
    return 1
  fi

  jq ".sessions[\"$meta_branch\"]" "$tracking_path"
}

# List all sessions
list_all_sessions() {
  local repo_root=$(get_repo_root)
  local tracking_path="$repo_root/$TRACKING_FILE"

  if ! tracking_file_exists; then
    echo "[]"
    return
  fi

  jq -r '.sessions | keys[]' "$tracking_path"
}
