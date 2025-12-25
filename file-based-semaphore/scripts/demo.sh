#!/bin/bash
#
# Demo script for file-based-semaphore
#
# This script demonstrates the CLI functionality.
# Can be recorded with asciinema: asciinema rec demo.cast
#

set -e

DEMO_DIR="/tmp/semaphore-demo-$$"
LOCK_FILE="$DEMO_DIR/demo.lock"

# Setup
mkdir -p "$DEMO_DIR"
cd "$(dirname "$0")/.."

echo "=== File-Based Semaphore Demo ==="
echo ""
echo "Building release binary..."
cargo build --release 2>/dev/null

BIN="./target/release/file-semaphore"

echo ""
echo "1. Try to acquire a lock (non-blocking)"
echo "   $ $BIN try $LOCK_FILE"
$BIN try "$LOCK_FILE"
echo ""

echo "2. Check lock status (shows process not running - expected for CLI)"
echo "   $ $BIN status $LOCK_FILE"
$BIN status "$LOCK_FILE" || true
echo ""

echo "3. Release the lock"
echo "   $ $BIN release $LOCK_FILE"
$BIN release "$LOCK_FILE"
echo ""

echo "4. Check status after release"
echo "   $ $BIN status $LOCK_FILE"
$BIN status "$LOCK_FILE" || true
echo ""

echo "5. Demonstrate stale lock detection"
echo "   Creating lock with fake (dead) PID and old timestamp..."
OLD_TIMESTAMP=$(($(date +%s) - 7200))  # 2 hours ago
echo "pid=99999" > "$LOCK_FILE"
echo "timestamp=$OLD_TIMESTAMP" >> "$LOCK_FILE"
echo "   $ cat $LOCK_FILE"
cat "$LOCK_FILE"
echo ""
echo "   $ $BIN status $LOCK_FILE"
$BIN status "$LOCK_FILE" || true
echo ""

echo "6. Try to acquire stale lock (using --stale 3600 = 1 hour threshold)"
echo "   $ $BIN try $LOCK_FILE --stale 3600"
$BIN try "$LOCK_FILE" --stale 3600 || echo "   (Lock acquisition result shown above)"
echo ""

# Cleanup
rm -rf "$DEMO_DIR"

echo "=== Demo Complete ==="
echo ""
echo "The file-based-semaphore provides:"
echo "  - Atomic locking with O_CREAT | O_EXCL"
echo "  - Blocking and non-blocking acquisition"
echo "  - Stale lock detection"
echo "  - Cross-platform support (Linux, macOS, Windows)"
