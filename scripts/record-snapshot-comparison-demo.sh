#!/bin/bash
# Record Snapshot Comparison demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="snapshot-comparison"
SHORT_NAME="snapcmp"
LANGUAGE="rust"

# Create temp directory for demo
DEMO_DIR=""
SNAP_DIR=""

demo_setup() {
  DEMO_DIR=$(mktemp -d)
  SNAP_DIR="$DEMO_DIR/snapshots"
  mkdir -p "$SNAP_DIR"
  export DEMO_DIR SNAP_DIR
}

demo_cleanup() {
  rm -rf "$DEMO_DIR"
}

demo_commands() {
  echo "# Snapshot Comparison Demo"
  sleep 1

  echo ""
  echo "# 1. Create a snapshot from command output"
  sleep 0.5
  echo "$ echo 'Hello, World!' | snapcmp create greeting --dir $SNAP_DIR"
  sleep 0.5
  echo "Hello, World!" | $BIN create greeting --dir $SNAP_DIR
  sleep 1.5

  echo ""
  echo "# 2. Check output against snapshot (match)"
  sleep 0.5
  echo "$ echo 'Hello, World!' | snapcmp check greeting --dir $SNAP_DIR"
  sleep 0.5
  echo "Hello, World!" | $BIN check greeting --dir $SNAP_DIR
  sleep 1.5

  echo ""
  echo "# 3. Check with different output (mismatch with diff)"
  sleep 0.5
  echo "$ echo 'Hello, Universe!' | snapcmp check greeting --dir $SNAP_DIR --color"
  sleep 0.5
  echo "Hello, Universe!" | $BIN check greeting --dir $SNAP_DIR --color || true
  sleep 2

  echo ""
  echo "# 4. Update snapshot with new content"
  sleep 0.5
  echo "$ echo 'Hello, Universe!' | snapcmp update greeting --dir $SNAP_DIR"
  sleep 0.5
  echo "Hello, Universe!" | $BIN update greeting --dir $SNAP_DIR
  sleep 1.5

  echo ""
  echo "# 5. Create JSON snapshot"
  sleep 0.5
  echo "$ echo '{\"status\": \"ok\", \"count\": 42}' | snapcmp create api-response --dir $SNAP_DIR --type json"
  sleep 0.5
  echo "{\"status\": \"ok\", \"count\": 42}" | $BIN create api-response --dir $SNAP_DIR --type json
  sleep 1.5

  echo ""
  echo "# 6. List all snapshots"
  sleep 0.5
  echo "$ snapcmp list --dir $SNAP_DIR"
  sleep 0.5
  $BIN list --dir $SNAP_DIR
  sleep 2

  echo ""
  echo "# Done! Snapshot testing with snapcmp."
  sleep 1
}

run_demo
