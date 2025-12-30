#!/bin/bash
# Record File-Based Semaphore demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="file-based-semaphore"
SHORT_NAME="sema"
LANGUAGE="rust"

# Lock file for demo
LOCK_FILE="/tmp/demo.lock"

demo_setup() {
  rm -f "$LOCK_FILE"
}

demo_cleanup() {
  rm -f "$LOCK_FILE"
}

demo_commands() {
  echo "# File-Based Semaphore Demo"
  sleep 1

  echo ""
  echo "# 1. Acquire a lock (non-blocking)"
  sleep 0.5
  echo "$ sema try $LOCK_FILE --tag demo"
  sleep 0.3
  $BIN try $LOCK_FILE --tag demo
  sleep 1

  echo ""
  echo "# 2. Check lock status"
  sleep 0.5
  echo "$ sema status $LOCK_FILE"
  sleep 0.3
  $BIN status $LOCK_FILE || true
  sleep 1.5

  echo ""
  echo "# 3. Check status with JSON output"
  sleep 0.5
  echo "$ sema status $LOCK_FILE --json"
  sleep 0.3
  $BIN status $LOCK_FILE --json || true
  sleep 1.5

  echo ""
  echo "# 4. Release the lock"
  sleep 0.5
  echo "$ sema release $LOCK_FILE"
  sleep 0.3
  $BIN release $LOCK_FILE
  sleep 1

  echo ""
  echo "# 5. Verify lock is released"
  sleep 0.5
  echo "$ sema status $LOCK_FILE"
  sleep 0.3
  $BIN status $LOCK_FILE || true
  sleep 1.5

  echo ""
  echo "# 6. Demo stale lock recovery"
  sleep 0.5
  echo "# Creating a stale lock (old timestamp, dead PID)..."
  OLD_TS=$(($(date +%s) - 7200))
  echo "pid=99999" > $LOCK_FILE
  echo "timestamp=$OLD_TS" >> $LOCK_FILE
  sleep 0.5
  echo "$ cat $LOCK_FILE"
  cat $LOCK_FILE
  sleep 1

  echo ""
  echo "# Acquiring stale lock (--stale 3600 = 1 hour threshold)"
  sleep 0.5
  echo "$ sema try $LOCK_FILE --stale 3600"
  sleep 0.3
  $BIN try $LOCK_FILE --stale 3600
  sleep 1.5

  echo ""
  echo "# Done! File-based semaphore with the sema command."
  sleep 2
}

run_demo
