#!/bin/bash
#
# Pre-Write Hook: Prevent modification of protected files
#
# This hook runs before Claude Code writes to any file and blocks
# writes to files that should only be modified by package managers
# or contain sensitive information.

# Read tool input from stdin (JSON format)
TOOL_INPUT=$(cat)

# Extract file path from the tool input JSON
FILE_PATH=$(echo "$TOOL_INPUT" | jq -r '.toolUse.input.file_path // .toolUse.input.path // empty' 2>/dev/null)

# If no file path found, allow the operation
if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Protected file patterns
PROTECTED_PATTERNS=(
  "package-lock.json"
  "yarn.lock"
  "pnpm-lock.yaml"
  "Cargo.lock"
  "*.env"
  "*.env.*"
  ".env"
  ".env.local"
  ".env.production"
  "*.secret.*"
  "*.private.*"
  "*.key"
  "*.pem"
  "credentials.json"
  "secrets.json"
)

# Get just the filename from the path
FILENAME=$(basename "$FILE_PATH")

# Check if file matches any protected pattern
for pattern in "${PROTECTED_PATTERNS[@]}"; do
  # Convert glob pattern to regex for matching
  if [[ "$FILENAME" == $pattern ]]; then
    echo "🛑 BLOCKED: Cannot modify protected file: $FILE_PATH"
    echo ""
    echo "This file is protected because:"
    case "$FILENAME" in
      package-lock.json|yarn.lock|pnpm-lock.yaml)
        echo "  - Lock files should only be modified by package managers"
        echo "  - Use: npm install, yarn install, or pnpm install"
        ;;
      Cargo.lock)
        echo "  - Lock files should only be modified by cargo"
        echo "  - Use: cargo update or cargo build"
        ;;
      *.env*|.env*)
        echo "  - Environment files may contain secrets"
        echo "  - Modify manually or use environment management tools"
        ;;
      *.secret.*|*.private.*|*.key|*.pem|credentials.json|secrets.json)
        echo "  - This file may contain sensitive information"
        echo "  - Modify manually with appropriate security measures"
        ;;
    esac
    echo ""
    exit 2  # Exit code 2 = blocking error
  fi
done

# Check if file path contains sensitive directories
if echo "$FILE_PATH" | grep -qE '(/\.git/|/\.ssh/|/\.aws/|/\.config/gcloud/)'; then
  echo "🛑 BLOCKED: Cannot modify files in sensitive directory: $FILE_PATH"
  echo ""
  echo "This directory contains system or credential files."
  echo "Modify these files manually with appropriate tools."
  echo ""
  exit 2
fi

# All checks passed - allow the write
exit 0
