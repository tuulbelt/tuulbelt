#!/bin/bash
#
# Pre-Tool Hook: Ensure submodule is on correct feature branch before edit
#
# This hook runs before Write/Edit operations on submodule files and:
# 1. Detects if the target file is in a submodule (tools/*)
# 2. In Web environment, ensures submodule is on the correct feature branch
# 3. Allows the operation to proceed
#
# This is Web-only - CLI uses worktrees which handle branching differently.

# Read tool input from stdin (JSON format)
TOOL_INPUT=$(cat)

# Extract file path from the tool input JSON
# Handle both Write (file_path) and Edit (file_path) tools
FILE_PATH=$(echo "$TOOL_INPUT" | jq -r '.toolUse.input.file_path // empty' 2>/dev/null)

# If no file path found, allow the operation
if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Check if file is in a submodule (tools/*)
if [[ ! "$FILE_PATH" =~ ^tools/ ]]; then
  # Not a submodule file, allow operation
  exit 0
fi

# Extract submodule path (e.g., "tools/test-flakiness-detector")
SUBMODULE=$(echo "$FILE_PATH" | grep -o '^tools/[^/]*')

if [ -z "$SUBMODULE" ]; then
  # Couldn't extract submodule, allow operation
  exit 0
fi

# Only run in Web environment
# CLI uses worktrees which handle branching at workspace level
if [ "${CLAUDE_CODE_REMOTE}" != "true" ]; then
  exit 0
fi

# Get repository root
REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)

if [ -z "$REPO_ROOT" ]; then
  # Not in a git repo, allow operation
  exit 0
fi

# Check if web scripts exist
if [ ! -f "$REPO_ROOT/scripts/web/manage-submodule-branch.sh" ]; then
  # Script not available, allow operation
  exit 0
fi

# Ensure submodule is on correct feature branch
# Run silently - don't clutter output for every file edit
"$REPO_ROOT/scripts/web/manage-submodule-branch.sh" "$SUBMODULE" >/dev/null 2>&1

# Always allow the operation - branch management is best-effort
# If it fails, the user can manually run the script
exit 0
