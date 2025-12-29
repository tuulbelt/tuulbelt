#!/bin/bash
# Wrapper for gh CLI that ensures project-specific credentials are used
# Usage: source scripts/gh-wrapper.sh
#        gh-tuulbelt <command>
# Or: alias gh='gh-tuulbelt' in your shell

gh-tuulbelt() {
  # Load credentials from .env if not already set
  if [ -z "$GH_TOKEN" ]; then
    if [ -f "$(git rev-parse --show-toplevel 2>/dev/null)/.env" ]; then
      source "$(git rev-parse --show-toplevel)/.env"
      export GH_TOKEN="$GITHUB_TOKEN"
    else
      echo "ERROR: .env file not found in repository root"
      return 1
    fi
  fi

  # Run gh with GH_TOKEN explicitly set
  GH_TOKEN="$GH_TOKEN" command gh "$@"
}

# Export the function so it's available in subshells
export -f gh-tuulbelt

echo "✓ gh-tuulbelt wrapper loaded"
echo "Usage: gh-tuulbelt <command>"
echo "Or add to ~/.zshrc: alias gh='gh-tuulbelt'"
