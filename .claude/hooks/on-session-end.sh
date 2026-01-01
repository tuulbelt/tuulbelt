#!/bin/bash
# Session end hook - runs automatically when Claude Code session ends
# Saves session state for Web environment

set -e

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Commit CLI tracking file if changed
if [ -f ".claude/cli-workspace-tracking.json" ]; then
  if ! git diff --quiet HEAD -- .claude/cli-workspace-tracking.json 2>/dev/null || \
     git ls-files --others --exclude-standard | grep -q "^.claude/cli-workspace-tracking.json$" 2>/dev/null; then
    git add .claude/cli-workspace-tracking.json 2>/dev/null || true
    git commit -m "chore(cli): update workspace tracking" 2>/dev/null || true
  fi
fi

# Commit Web tracking file if changed
if [ -f ".claude/web-session-tracking.json" ]; then
  if ! git diff --quiet HEAD -- .claude/web-session-tracking.json 2>/dev/null || \
     git ls-files --others --exclude-standard | grep -q "^.claude/web-session-tracking.json$" 2>/dev/null; then
    git add .claude/web-session-tracking.json 2>/dev/null || true
    git commit -m "chore(web): update session tracking" 2>/dev/null || true
  fi
fi

# Environment-specific cleanup
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web: Save session state
  if [ -f "scripts/web/save-session.sh" ]; then
    ./scripts/web/save-session.sh
  fi
else
  # CLI: No cleanup needed (worktrees persist)
  echo "✓ Session ended (worktrees preserved)"
fi
