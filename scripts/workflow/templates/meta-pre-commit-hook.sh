#!/bin/bash
# Reject commits to main branch (meta repo protection)

BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$BRANCH" = "main" ]; then
  echo ""
  echo "❌ ERROR: Direct commits to main branch are not allowed"
  echo ""
  echo "Please create a feature workspace:"
  echo "  CLI: /work-init <feature-name>"
  echo "  Web: Automatically handled by session hooks"
  echo ""
  echo "Example:"
  echo "  /work-init feature/my-feature"
  echo ""
  exit 1
fi

exit 0
