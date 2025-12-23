#!/bin/bash
#
# Post-Bash Hook: Audit and log bash commands
#
# This hook runs after Claude Code executes a bash command and:
# 1. Logs all commands to an audit file
# 2. Warns about potentially dangerous commands
# 3. Tracks command execution for debugging

# Read tool input from stdin (JSON format)
TOOL_INPUT=$(cat)

# Extract command from the tool input JSON
COMMAND=$(echo "$TOOL_INPUT" | jq -r '.toolUse.input.command // empty' 2>/dev/null)

# If no command found, exit silently
if [ -z "$COMMAND" ]; then
  exit 0
fi

# Create audit log directory if it doesn't exist
AUDIT_DIR="${HOME}/.claude/logs"
mkdir -p "$AUDIT_DIR"

# Audit log file with date
AUDIT_LOG="${AUDIT_DIR}/bash-audit-$(date +%Y-%m).log"

# Log the command with timestamp
{
  echo "=== $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="
  echo "Command: $COMMAND"
  echo "Working Directory: $(pwd)"
  echo "---"
} >> "$AUDIT_LOG"

# Check for potentially dangerous commands
DANGEROUS_PATTERNS=(
  'rm -rf /'
  'rm -rf ~'
  'rm -rf \*'
  ':(){:|:&};:'           # Fork bomb
  'dd if=/dev/zero'
  'mkfs\.'
  'DROP DATABASE'
  'DROP TABLE'
  '> /dev/sda'
  'chmod -R 777'
  'chown -R'
)

for pattern in "${DANGEROUS_PATTERNS[@]}"; do
  if echo "$COMMAND" | grep -qF "$pattern"; then
    echo "⚠️  WARNING: Potentially dangerous command detected!" >&2
    echo "Command: $COMMAND" >&2
    echo "" >&2
    echo "This command matches a dangerous pattern: $pattern" >&2
    echo "Execution was allowed, but please review carefully." >&2
    echo "" >&2

    # Log warning to audit file
    {
      echo "!!! DANGEROUS COMMAND WARNING !!!"
      echo "Pattern: $pattern"
      echo ""
    } >> "$AUDIT_LOG"

    break
  fi
done

# Check for commands that modify important files
SENSITIVE_OPERATIONS=(
  'git push --force'
  'git reset --hard'
  'git clean -fd'
  'npm publish'
  'cargo publish'
)

for operation in "${SENSITIVE_OPERATIONS[@]}"; do
  if echo "$COMMAND" | grep -qF "$operation"; then
    echo "ℹ️  NOTICE: Sensitive operation detected" >&2
    echo "Command: $COMMAND" >&2
    echo "Operation: $operation" >&2
    echo "" >&2

    # Log to audit file
    {
      echo "SENSITIVE OPERATION: $operation"
      echo ""
    } >> "$AUDIT_LOG"

    break
  fi
done

# Rotate audit log if it's too large (>10MB)
if [ -f "$AUDIT_LOG" ]; then
  LOG_SIZE=$(stat -f%z "$AUDIT_LOG" 2>/dev/null || stat -c%s "$AUDIT_LOG" 2>/dev/null || echo 0)
  if [ "$LOG_SIZE" -gt 10485760 ]; then
    # Rotate: keep last 1000 lines
    tail -n 1000 "$AUDIT_LOG" > "${AUDIT_LOG}.tmp"
    mv "${AUDIT_LOG}.tmp" "$AUDIT_LOG"

    echo "Audit log rotated (was ${LOG_SIZE} bytes)" >> "$AUDIT_LOG"
  fi
fi

# Exit successfully (don't block commands)
exit 0
