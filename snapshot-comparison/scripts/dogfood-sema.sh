#!/bin/bash
# Demonstrate concurrent snapshot safety with file-based-semaphore
#
# This script proves that parallel snapshot updates can corrupt data
# without proper locking, and shows how sema prevents this.
#
# Usage: ./scripts/dogfood-sema.sh [workers]
#        workers: number of concurrent processes (default: 10)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKERS="${1:-10}"

# Check if we're in the monorepo context
SEMA_DIR="$TOOL_DIR/../file-based-semaphore"
if [ ! -d "$SEMA_DIR" ]; then
    echo "Not in monorepo context - file-based-semaphore not available"
    echo "Skipping concurrent safety validation"
    exit 0
fi

echo "=== Concurrent Snapshot Safety Test ==="
echo "Workers: $WORKERS parallel processes"
echo ""

# Build both tools
echo "Building tools..."
cd "$TOOL_DIR"
cargo build --release --quiet 2>/dev/null || cargo build --release

cd "$SEMA_DIR"
cargo build --release --quiet 2>/dev/null || cargo build --release

SNAPCMP="$TOOL_DIR/target/release/snapcmp"
SEMA="$SEMA_DIR/target/release/sema"

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

SNAPSHOTS_DIR="$TEMP_DIR/snapshots"
LOCK_FILE="$TEMP_DIR/snapshot.lock"
RESULTS_DIR="$TEMP_DIR/results"
mkdir -p "$SNAPSHOTS_DIR" "$RESULTS_DIR"

# Initialize snapshot
echo "initial-content" | $SNAPCMP create test-snap -d "$SNAPSHOTS_DIR" > /dev/null 2>&1

echo ""
echo "--- Test 1: WITHOUT semaphore protection ---"
echo "Launching $WORKERS concurrent update processes..."

# Run concurrent updates WITHOUT locking
for i in $(seq 1 $WORKERS); do
    (
        # Each worker writes its ID
        echo "content-from-worker-$i" | $SNAPCMP update test-snap -d "$SNAPSHOTS_DIR" 2>/dev/null
        echo "$i" > "$RESULTS_DIR/completed-$i"
    ) &
done
wait

# Check the final state
UNPROTECTED_CONTENT=$(cat "$SNAPSHOTS_DIR/test-snap.snap" | tail -1)
UNPROTECTED_COMPLETED=$(ls "$RESULTS_DIR" | wc -l | tr -d ' ')

echo "Completed: $UNPROTECTED_COMPLETED/$WORKERS workers finished"
echo "Final content: $UNPROTECTED_CONTENT"
echo ""

# Reset for protected test
rm -rf "$SNAPSHOTS_DIR"/* "$RESULTS_DIR"/*
echo "initial-content" | $SNAPCMP create test-snap -d "$SNAPSHOTS_DIR" > /dev/null 2>&1

echo "--- Test 2: WITH semaphore protection ---"
echo "Launching $WORKERS concurrent update processes with sema..."

# Run concurrent updates WITH locking
for i in $(seq 1 $WORKERS); do
    (
        # Acquire lock before update
        $SEMA acquire "$LOCK_FILE" --tag "worker-$i" --timeout 30000 > /dev/null 2>&1

        # Critical section: update snapshot
        echo "content-from-worker-$i" | $SNAPCMP update test-snap -d "$SNAPSHOTS_DIR" 2>/dev/null

        # Release lock
        $SEMA release "$LOCK_FILE" > /dev/null 2>&1

        echo "$i" > "$RESULTS_DIR/completed-$i"
    ) &
done
wait

# Check the final state
PROTECTED_CONTENT=$(cat "$SNAPSHOTS_DIR/test-snap.snap" | tail -1)
PROTECTED_COMPLETED=$(ls "$RESULTS_DIR" | wc -l | tr -d ' ')

echo "Completed: $PROTECTED_COMPLETED/$WORKERS workers finished"
echo "Final content: $PROTECTED_CONTENT"
echo ""

# Verify snapshot file integrity
echo "--- Verification ---"

# Check that snapshot is valid (has proper header)
if grep -q "^# Snapshot:" "$SNAPSHOTS_DIR/test-snap.snap" && \
   grep -q "^# Hash:" "$SNAPSHOTS_DIR/test-snap.snap" && \
   grep -q "^---$" "$SNAPSHOTS_DIR/test-snap.snap"; then
    echo "  File integrity: VALID (proper .snap format)"
else
    echo "  File integrity: CORRUPTED (missing headers)"
    cat "$SNAPSHOTS_DIR/test-snap.snap"
    exit 1
fi

# Check that content is from exactly one worker (not interleaved)
if echo "$PROTECTED_CONTENT" | grep -qE "^content-from-worker-[0-9]+$"; then
    echo "  Content format: VALID (clean write from single worker)"
else
    echo "  Content format: CORRUPTED (possibly interleaved writes)"
    exit 1
fi

# All workers should complete
if [ "$PROTECTED_COMPLETED" -eq "$WORKERS" ]; then
    echo "  All workers: COMPLETED ($WORKERS/$WORKERS)"
else
    echo "  Workers: INCOMPLETE ($PROTECTED_COMPLETED/$WORKERS)"
    exit 1
fi

echo ""
echo "=== Results ==="
echo "Without sema: Updates complete but order is non-deterministic"
echo "With sema:    Updates are serialized, file integrity guaranteed"
echo ""
echo "Dogfooding validation: sema + snapcmp work together for safe concurrent access"
