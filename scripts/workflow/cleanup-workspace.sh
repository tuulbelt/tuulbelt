#!/bin/bash
# Remove workspace after PR merge

set -e

# Show help
if [ "$1" = "--help" ] || [ "$1" = "-h" ] || [ -z "$1" ]; then
  cat <<EOF
/work-cleanup - Remove workspace after PR merge

Usage:
  /work-cleanup <feature-name> [--force]
  /work-cleanup --help

Parameters:
  feature-name   Branch name to cleanup (e.g., feature/my-feature)
  --force        Skip PR merge verification (dangerous)

Examples:
  /work-cleanup feature/component-prop-validator
  /work-cleanup fix/docs-typo --force

Warnings:
  - Using --force may lose work if PRs are rejected
  - Always verify PRs are merged before cleanup
  - Backup important changes before using --force

EOF
  exit 0
fi

FEATURE_NAME="$1"
FORCE_FLAG="${2:-}"

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Detect environment and delegate
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment
  "$SCRIPT_DIR/../web/cleanup-web-session.sh" "$FEATURE_NAME" "$FORCE_FLAG"
else
  # CLI environment
  "$SCRIPT_DIR/../cli/cleanup-cli-workspace.sh" "$FEATURE_NAME" "$FORCE_FLAG"
fi
