#!/bin/bash
#
# Post-Write Hook: Auto-format code after writes
#
# This hook runs after Claude Code writes to a file and automatically
# formats the code according to language-specific standards.

# Read tool input from stdin (JSON format)
TOOL_INPUT=$(cat)

# Extract file path from the tool input JSON
FILE_PATH=$(echo "$TOOL_INPUT" | jq -r '.toolUse.input.file_path // .toolUse.input.path // empty' 2>/dev/null)

# If no file path found, exit silently
if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Check if file exists (might have been deleted)
if [ ! -f "$FILE_PATH" ]; then
  exit 0
fi

# Determine file type and format accordingly
case "$FILE_PATH" in
  *.ts|*.tsx|*.js|*.jsx)
    # TypeScript/JavaScript formatting
    if command -v npx &> /dev/null; then
      # Try prettier if available in project
      if [ -f "package.json" ]; then
        npx prettier --write "$FILE_PATH" 2>/dev/null || true
      fi
    fi
    ;;

  *.rs)
    # Rust formatting
    if command -v rustfmt &> /dev/null; then
      rustfmt "$FILE_PATH" 2>/dev/null || true
    fi
    ;;

  *.json)
    # JSON formatting
    if command -v jq &> /dev/null; then
      # Format JSON with jq (4-space indent)
      temp_file=$(mktemp)
      if jq --indent 2 '.' "$FILE_PATH" > "$temp_file" 2>/dev/null; then
        mv "$temp_file" "$FILE_PATH"
      else
        rm -f "$temp_file"
      fi
    elif command -v npx &> /dev/null && [ -f "package.json" ]; then
      npx prettier --write "$FILE_PATH" 2>/dev/null || true
    fi
    ;;

  *.md|*.markdown)
    # Markdown formatting
    if command -v npx &> /dev/null && [ -f "package.json" ]; then
      npx prettier --write "$FILE_PATH" 2>/dev/null || true
    fi
    ;;

  *.toml)
    # TOML formatting (if taplo is installed)
    if command -v taplo &> /dev/null; then
      taplo fmt "$FILE_PATH" 2>/dev/null || true
    fi
    ;;

  *.yaml|*.yml)
    # YAML formatting
    if command -v npx &> /dev/null && [ -f "package.json" ]; then
      npx prettier --write "$FILE_PATH" 2>/dev/null || true
    fi
    ;;
esac

# Exit successfully (don't block on formatting failures)
exit 0
