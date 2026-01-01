#!/bin/bash
# Setup git credential helper for Claude Code Web environment
# Uses GITHUB_USERNAME and GITHUB_TOKEN environment variables

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Auto-detect if we should disable colors
# In Web environment, default to no colors (terminal doesn't interpret ANSI codes)
NO_COLOR=false
if [ "${CLAUDE_CODE_REMOTE}" = "true" ]; then
  NO_COLOR=true
fi

# Colors (disabled in non-interactive terminals)
if [ "$NO_COLOR" = true ]; then
  RED=''
  GREEN=''
  YELLOW=''
  NC=''
else
  RED='\033[0;31m'
  GREEN='\033[0;32m'
  YELLOW='\033[1;33m'
  NC='\033[0m'
fi

echo "🔐 Setting up git credentials for Claude Code Web..."
echo ""

# Check if we're in Web environment
if [ "${CLAUDE_CODE_REMOTE}" != "true" ]; then
  echo -e "${YELLOW}⚠️  Not in Claude Code Web environment${NC}"
  echo "CLAUDE_CODE_REMOTE=${CLAUDE_CODE_REMOTE:-not set}"
  echo "Credential helper not needed in CLI environment"
  exit 0
fi

echo -e "${GREEN}✓${NC} Detected Claude Code Web environment"

# Check for required environment variables
if [ -z "$GITHUB_USERNAME" ]; then
  echo -e "${RED}✗${NC} GITHUB_USERNAME not set"
  exit 1
fi

if [ -z "$GITHUB_TOKEN" ]; then
  echo -e "${RED}✗${NC} GITHUB_TOKEN not set"
  exit 1
fi

echo -e "${GREEN}✓${NC} GITHUB_USERNAME: $GITHUB_USERNAME"
echo -e "${GREEN}✓${NC} GITHUB_TOKEN: ${GITHUB_TOKEN:0:7}... (redacted)"
echo ""

# Configure credential helper globally
echo "Configuring git credential helper..."
git config --global credential.helper '!f() { echo "username=${GITHUB_USERNAME}"; echo "password=${GITHUB_TOKEN}"; }; f'

echo -e "${GREEN}✓${NC} Credential helper configured"
echo ""

# Test configuration
echo "Testing credential helper..."
TEST_OUTPUT=$(git config --get credential.helper)
if [ -n "$TEST_OUTPUT" ]; then
  echo -e "${GREEN}✓${NC} Credential helper active"
  echo "  Config: $TEST_OUTPUT"
else
  echo -e "${RED}✗${NC} Credential helper not configured"
  exit 1
fi

echo ""
echo -e "${GREEN}🎉 Credential setup complete!${NC}"
echo ""
echo "Git is now configured to use GITHUB_TOKEN for HTTPS authentication"
echo "This applies to meta repo and all submodules"
