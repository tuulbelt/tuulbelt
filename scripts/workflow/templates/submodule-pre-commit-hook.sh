#!/bin/bash
# Reject commits to main branch (submodule protection)

BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$BRANCH" = "main" ]; then
  echo ""
  echo "❌ ERROR: Direct commits to main branch are not allowed in submodule"
  echo ""
  echo "This submodule is managed via feature branches."
  echo "The meta repository workflow handles submodule branching automatically."
  echo ""
  exit 1
fi

exit 0
