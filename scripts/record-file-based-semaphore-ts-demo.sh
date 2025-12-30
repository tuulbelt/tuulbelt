#!/bin/bash
# Record File-Based Semaphore (TypeScript) demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="file-based-semaphore-ts"
SHORT_NAME="semats"
LANGUAGE="typescript"

# Lock file for demo
LOCK_FILE="/tmp/ts-demo.lock"

demo_setup() {
  rm -f "$LOCK_FILE"
}

demo_cleanup() {
  rm -f "$LOCK_FILE"
}

demo_commands() {
  echo "# File-Based Semaphore (TypeScript) Demo"
  sleep 1

  echo ""
  echo "# 1. Acquire a lock (non-blocking)"
  sleep 0.5
  echo "$ semats try-acquire $LOCK_FILE --tag demo"
  sleep 0.3
  semats try-acquire $LOCK_FILE --tag demo
  sleep 1

  echo ""
  echo "# 2. Check lock status"
  sleep 0.5
  echo "$ semats status $LOCK_FILE"
  sleep 0.3
  semats status $LOCK_FILE || true
  sleep 1.5

  echo ""
  echo "# 3. Check status with JSON output"
  sleep 0.5
  echo "$ semats status $LOCK_FILE --json"
  sleep 0.3
  semats status $LOCK_FILE --json || true
  sleep 1.5

  echo ""
  echo "# 4. Release the lock"
  sleep 0.5
  echo "$ semats release $LOCK_FILE"
  sleep 0.3
  semats release $LOCK_FILE
  sleep 1

  echo ""
  echo "# 5. Verify lock is released"
  sleep 0.5
  echo "$ semats status $LOCK_FILE"
  sleep 0.3
  semats status $LOCK_FILE || true
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
  echo "# Clean stale lock"
  sleep 0.5
  echo "$ semats clean $LOCK_FILE"
  sleep 0.3
  semats clean $LOCK_FILE
  sleep 1.5

  echo ""
  echo "# 7. Show cross-language compatibility"
  sleep 0.5
  echo "# TypeScript semats uses same format as Rust sema!"
  sleep 0.5
  echo "$ semats try-acquire $LOCK_FILE --tag typescript-process"
  semats try-acquire $LOCK_FILE --tag typescript-process
  sleep 1
  echo ""
  echo "$ cat $LOCK_FILE"
  cat $LOCK_FILE
  sleep 1.5

  echo ""
  echo "# Done! TypeScript file-based semaphore with the semats command."
  sleep 2
}

run_demo
