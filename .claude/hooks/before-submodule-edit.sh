#!/bin/bash
# Hook to run before editing any submodule file
# Ensures we're on the correct feature branch in Web environment
# Usage: source this before working on submodules

ensure_submodule_branch() {
  local file_path="$1"

  # Check if file is in a submodule
  if [[ ! "$file_path" =~ ^tools/ ]]; then
    # Not a submodule file, nothing to do
    return 0
  fi

  # Extract submodule path (e.g., "tools/file-based-semaphore")
  local submodule=$(echo "$file_path" | grep -o '^tools/[^/]*')

  if [ -z "$submodule" ]; then
    return 0
  fi

  # Only in Web environment
  if [ "${CLAUDE_CODE_REMOTE}" != "true" ]; then
    return 0
  fi

  local repo_root=$(git rev-parse --show-toplevel 2>/dev/null)

  # Check if web scripts exist
  if [ ! -f "$repo_root/scripts/web/manage-submodule-branch.sh" ]; then
    return 0
  fi

  echo "📝 Ensuring $submodule is on correct feature branch..."

  # Run submodule branch management
  "$repo_root/scripts/web/manage-submodule-branch.sh" "$submodule" >/dev/null 2>&1

  if [ $? -eq 0 ]; then
    echo "✓ $submodule ready for editing"
  else
    echo "⚠️  Could not setup feature branch for $submodule"
    echo "   Run manually: ./scripts/web/manage-submodule-branch.sh $submodule"
  fi
}

# Export function so it can be called
export -f ensure_submodule_branch
