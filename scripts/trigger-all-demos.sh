#!/bin/bash
# Trigger demo generation for all Tuulbelt tools
# This manually dispatches the create-demo.yml workflow in each standalone tool repository
#
# Usage:
#   ./scripts/trigger-all-demos.sh
#
# Requirements:
#   - gh CLI installed
#   - GITHUB_TOKEN in .env with workflow dispatch permissions
#   - Must have write access to all tool repositories

set -e

# Load GitHub token from .env
if [ -f .env ]; then
  set -a
  source .env
  set +a
else
  echo "ERROR: .env file not found. Create .env with GITHUB_TOKEN."
  exit 1
fi

if [ -z "$GITHUB_TOKEN" ]; then
  echo "ERROR: GITHUB_TOKEN not set in .env"
  exit 1
fi

# List of all tool repositories
TOOLS=(
  "cli-progress-reporting"
  "cross-platform-path-normalizer"
  "config-file-merger"
  "structured-error-handler"
  "file-based-semaphore"
  "file-based-semaphore-ts"
  "output-diffing-utility"
  "snapshot-comparison"
  "test-flakiness-detector"
  "port-resolver"
)

echo "Triggering demo generation for all tools..."
echo ""

for tool in "${TOOLS[@]}"; do
  echo "→ $tool"

  # Trigger the workflow
  if gh workflow run create-demo.yml --repo "tuulbelt/$tool"; then
    echo "  ✓ Workflow triggered"
  else
    echo "  ✗ Failed to trigger workflow"
  fi

  echo ""
done

echo "All workflow dispatches sent."
echo ""
echo "Monitor progress at:"
for tool in "${TOOLS[@]}"; do
  echo "  https://github.com/tuulbelt/$tool/actions"
done
