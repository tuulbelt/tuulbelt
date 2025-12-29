#!/bin/bash
# Setup GitHub authentication from local .env file
# This script ensures we use project-specific credentials, not global ones

set -e

# Load .env file
if [ ! -f .env ]; then
  echo "ERROR: .env file not found"
  echo "Copy .env.example to .env and fill in your credentials"
  exit 1
fi

# Source .env to get variables
set -a
source .env
set +a

# Validate required variables
if [ -z "$GITHUB_TOKEN" ] || [ "$GITHUB_TOKEN" = "your_github_token_here" ]; then
  echo "ERROR: GITHUB_TOKEN not set in .env"
  exit 1
fi

if [ -z "$GITHUB_USERNAME" ] || [ "$GITHUB_USERNAME" = "your_github_username" ]; then
  echo "ERROR: GITHUB_USERNAME not set in .env"
  exit 1
fi

if [ -z "$GITHUB_EMAIL" ] || [ "$GITHUB_EMAIL" = "your_github_email@example.com" ]; then
  echo "ERROR: GITHUB_EMAIL not set in .env"
  exit 1
fi

# Export credentials for gh CLI
# CRITICAL: gh CLI prefers GH_TOKEN over GITHUB_TOKEN, and both override keyring cache
export GITHUB_TOKEN
export GH_TOKEN="$GITHUB_TOKEN"  # gh CLI respects this more than GITHUB_TOKEN
export GITHUB_ORG

# Export git author/committer info (takes precedence over git config)
# Matches .envrc behavior for consistency
export GIT_AUTHOR_NAME="$GITHUB_USERNAME"
export GIT_AUTHOR_EMAIL="$GITHUB_EMAIL"
export GIT_COMMITTER_NAME="$GITHUB_USERNAME"
export GIT_COMMITTER_EMAIL="$GITHUB_EMAIL"

# Set local git config (not global) - env vars above will take precedence
git config user.name "$GITHUB_USERNAME"
git config user.email "$GITHUB_EMAIL"

# Verify authentication
echo ""
echo "✓ GitHub authentication configured:"
echo "  Username: $GITHUB_USERNAME"
echo "  Email: $GITHUB_EMAIL"
echo "  Organization: $GITHUB_ORG"
echo ""
echo "Current gh auth status:"
gh auth status
