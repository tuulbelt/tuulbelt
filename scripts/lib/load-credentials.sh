#!/bin/bash
# Unified credential loading for all GitHub operations
# Source this at the top of any script that needs GitHub access
#
# Usage:
#   source "$(dirname "$0")/../lib/load-credentials.sh"
#   # or
#   source "$REPO_ROOT/scripts/lib/load-credentials.sh"

# Detect repository root
if [ -z "$REPO_ROOT" ]; then
  REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$HOME/tuulbelt")"
fi

# Load .env file
if [ ! -f "$REPO_ROOT/.env" ]; then
  echo "❌ ERROR: .env file not found at $REPO_ROOT/.env"
  echo "Create one from .env.example with your GitHub credentials"
  exit 1
fi

# Source .env
set -a
source "$REPO_ROOT/.env"
set +a

# Validate required variables
if [ -z "$GITHUB_TOKEN" ]; then
  echo "❌ ERROR: GITHUB_TOKEN not set in .env"
  exit 1
fi

# Export for different tools:
# - gh CLI prefers GH_TOKEN
# - git uses GITHUB_TOKEN
# - Octokit (MCP) uses GITHUB_TOKEN
export GH_TOKEN="$GITHUB_TOKEN"
export GITHUB_TOKEN

# Set git config for current repo (if in a git repo)
if git rev-parse --git-dir > /dev/null 2>&1; then
  if [ -n "$GITHUB_USERNAME" ] && [ -n "$GITHUB_EMAIL" ]; then
    git config user.name "$GITHUB_USERNAME"
    git config user.email "$GITHUB_EMAIL"
  fi
fi

# Export org for scripts
export GITHUB_ORG="${GITHUB_ORG:-tuulbelt}"

# Success - credentials loaded
# (Don't echo anything - this is a library script)
