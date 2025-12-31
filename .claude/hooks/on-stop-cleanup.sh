#!/bin/bash
#
# On-Stop Hook: Cleanup when Claude Code session ends
#
# This hook runs when a Claude Code session stops and:
# 1. Cleans up temporary files
# 2. Archives session logs
# 3. Optionally commits work in progress

# Define directories
# Dynamically detect repository root (works in both CLI and Web)
TUULBELT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$HOME/tuulbelt")"
CLAUDE_DIR="${HOME}/.claude"
LOGS_DIR="${CLAUDE_DIR}/logs"

# Create logs directory if it doesn't exist
mkdir -p "$LOGS_DIR"

# Cleanup temporary files
echo "Cleaning up temporary files..."

# Remove temporary files in Tuulbelt directory
find "$TUULBELT_ROOT" -name "*.tmp" -type f -delete 2>/dev/null || true
find "$TUULBELT_ROOT" -name ".plan.md" -type f -delete 2>/dev/null || true
find "$TUULBELT_ROOT" -name "*.swp" -type f -delete 2>/dev/null || true
find "$TUULBELT_ROOT" -name "*~" -type f -delete 2>/dev/null || true

# Remove empty directories
find "$TUULBELT_ROOT" -type d -empty -delete 2>/dev/null || true

# Archive session logs if they exist
if [ -f "${CLAUDE_DIR}/session.log" ]; then
  TIMESTAMP=$(date +%Y%m%d_%H%M%S)
  LOG_ARCHIVE="${LOGS_DIR}/session_${TIMESTAMP}.log"

  # Copy session log to archive
  cp "${CLAUDE_DIR}/session.log" "$LOG_ARCHIVE"

  # Compress if gzip is available
  if command -v gzip &> /dev/null; then
    gzip "$LOG_ARCHIVE"
    echo "Session log archived: ${LOG_ARCHIVE}.gz"
  else
    echo "Session log archived: $LOG_ARCHIVE"
  fi

  # Clear the session log
  > "${CLAUDE_DIR}/session.log"
fi

# Clean up old log archives (keep last 30 days)
find "$LOGS_DIR" -name "session_*.log*" -type f -mtime +30 -delete 2>/dev/null || true
find "$LOGS_DIR" -name "bash-audit-*.log" -type f -mtime +90 -delete 2>/dev/null || true

# Check if we're in a git repository
if [ -d "$TUULBELT_ROOT/.git" ]; then
  cd "$TUULBELT_ROOT" || exit 0

  # Web workflow: Auto-commit tracking file if it changed
  if [ "${CLAUDE_CODE_REMOTE}" = "true" ] && [ -f ".claude/web-session-tracking.json" ]; then
    if ! git diff --quiet .claude/web-session-tracking.json 2>/dev/null; then
      echo "Auto-committing Web session tracking file..."
      git add .claude/web-session-tracking.json
      git commit -m "chore: update Web session tracking on session stop" 2>/dev/null || true
      echo "✓ Tracking file committed"
      echo ""
    fi
  fi

  # Check if there are uncommitted changes
  if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "Uncommitted changes detected in Tuulbelt repository."
    echo ""
    echo "Modified files:"
    git status --short
    echo ""
    echo "Consider committing these changes:"
    echo "  git add ."
    echo "  git commit -m 'chore: work in progress from session'"
    echo ""
  fi

  # Web workflow: Show final session status
  if [ "${CLAUDE_CODE_REMOTE}" = "true" ] && [ -f "scripts/web/show-status.sh" ]; then
    echo "Final Web session status:"
    echo ""
    ./scripts/web/show-status.sh --no-color 2>/dev/null || true
    echo ""
  fi
fi

# Display summary
echo ""
echo "Session cleanup complete!"
echo "- Temporary files removed"
echo "- Session logs archived"
echo "- Old logs cleaned up (>30 days)"
echo ""

exit 0
