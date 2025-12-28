#!/bin/bash
#
# Dogfooding: Demonstrate File-Based Semaphore integration
#
# This script shows how output-diffing-utility can compose with file-based-semaphore
# to protect concurrent diff operations with cache locking.
#
# Composability Demo:
#   - Rust tool (semaphore) protects Rust tool (diff) via CLI
#   - Simulates concurrent diff processes accessing shared cache
#   - No tight coupling, just CLI composition
#   - Demonstrates same-language composition (Rust → Rust)
#
# Usage:
#   ./scripts/dogfood-semaphore.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
SEMAPHORE_DIR="$TOOL_DIR/../file-based-semaphore"
TEMP_DIR="/tmp/output-diff-semaphore-demo-$$"
LOCK_FILE="/tmp/diff-cache-demo-$$.lock"
CACHE_DIR="/tmp/diff-cache-demo-$$"

echo "🔬 Dogfooding: File-Based Semaphore + Output Diffing Utility"
echo ""
echo "This demonstrates concurrent-safe caching:"
echo "  Rust (semaphore) protects Rust (diff) cache access"
echo ""

# Verify File-Based Semaphore exists
if [ ! -d "$SEMAPHORE_DIR" ]; then
    echo "⚠️  File-Based Semaphore not found at: $SEMAPHORE_DIR"
    echo ""
    echo "This tool works standalone, but concurrent cache protection unavailable."
    echo "Running diff without cache locking..."
    echo ""

    # Graceful fallback: still works without semaphore
    mkdir -p "$TEMP_DIR"
    echo '{"status": "old"}' > "$TEMP_DIR/data1.json"
    echo '{"status": "new"}' > "$TEMP_DIR/data2.json"

    cd "$TOOL_DIR"
    cargo run --release --quiet --bin odiff -- "$TEMP_DIR/data1.json" "$TEMP_DIR/data2.json"

    rm -rf "$TEMP_DIR"
    exit 0
fi

# Build semaphore if needed
if [ ! -f "$SEMAPHORE_DIR/target/release/file-semaphore" ]; then
    echo "🔨 Building File-Based Semaphore..."
    (cd "$SEMAPHORE_DIR" && cargo build --release 2>&1 | grep -v "Compiling\|Finished" || true)
fi

SEMAPHORE_BIN="$SEMAPHORE_DIR/target/release/file-semaphore"

if [ ! -f "$SEMAPHORE_BIN" ]; then
    echo "❌ Failed to build semaphore binary"
    exit 1
fi

echo "✅ File-Based Semaphore available"
echo ""

# Create test files and cache directory
mkdir -p "$TEMP_DIR"
mkdir -p "$CACHE_DIR"
echo "📝 Creating test files..."

echo '{"users": 100, "active": 85}' > "$TEMP_DIR/stats-old.json"
echo '{"users": 150, "active": 120, "premium": 30}' > "$TEMP_DIR/stats-new.json"

echo "✅ Test files and cache directory created"
echo ""

# Function to run diff with cache locking
run_diff_with_lock() {
    local process_id="$1"
    local color="$2"

    echo "${color}[Process $process_id] Attempting to acquire lock...${NC}"

    # Try to acquire lock
    if "$SEMAPHORE_BIN" try "$LOCK_FILE" 2>/dev/null; then
        echo "${color}[Process $process_id] ✅ Lock acquired${NC}"

        # Simulate cache check
        CACHE_KEY="stats-old-vs-stats-new"
        CACHE_FILE="$CACHE_DIR/$CACHE_KEY.json"

        if [ -f "$CACHE_FILE" ]; then
            echo "${color}[Process $process_id] 📦 Cache hit! Reading cached diff...${NC}"
            cat "$CACHE_FILE"
        else
            echo "${color}[Process $process_id] 💾 Cache miss. Computing diff...${NC}"

            # Run actual diff
            cd "$TOOL_DIR"
            DIFF_OUTPUT=$(cargo run --release --quiet --bin odiff -- --format json "$TEMP_DIR/stats-old.json" "$TEMP_DIR/stats-new.json")

            # Save to cache
            echo "$DIFF_OUTPUT" > "$CACHE_FILE"
            echo "${color}[Process $process_id] ✅ Diff computed and cached${NC}"
            echo "$DIFF_OUTPUT"
        fi

        # Release lock
        "$SEMAPHORE_BIN" release "$LOCK_FILE" 2>/dev/null
        echo "${color}[Process $process_id] 🔓 Lock released${NC}"
    else
        echo "${color}[Process $process_id] ⏳ Lock held by another process, waiting...${NC}"
        sleep 1

        # Try again
        if "$SEMAPHORE_BIN" try "$LOCK_FILE" 2>/dev/null; then
            echo "${color}[Process $process_id] ✅ Lock acquired (after waiting)${NC}"
            echo "${color}[Process $process_id] 📦 Reading cached diff...${NC}"

            # Cache should exist now
            CACHE_FILE="$CACHE_DIR/stats-old-vs-stats-new.json"
            if [ -f "$CACHE_FILE" ]; then
                cat "$CACHE_FILE"
            fi

            "$SEMAPHORE_BIN" release "$LOCK_FILE" 2>/dev/null
            echo "${color}[Process $process_id] 🔓 Lock released${NC}"
        fi
    fi

    echo ""
}

# ANSI color codes
NC='\033[0m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'

echo "🔄 Simulating concurrent diff processes with cache protection..."
echo ""

# Run two processes concurrently
run_diff_with_lock "1" "$BLUE" &
PID1=$!

sleep 0.2  # Small delay to ensure process 1 acquires lock first

run_diff_with_lock "2" "$GREEN" &
PID2=$!

# Wait for both to complete
wait $PID1
wait $PID2

echo "✅ Both processes completed"
echo ""

# Check lock status
echo "🔍 Final lock status:"
"$SEMAPHORE_BIN" status "$LOCK_FILE" || echo "Lock file removed (clean state)"
echo ""

# Show cache contents
echo "📦 Cache contents:"
ls -lh "$CACHE_DIR"
echo ""

# Cleanup
rm -rf "$TEMP_DIR" "$CACHE_DIR"
rm -f "$LOCK_FILE"

echo "✨ Composition Demo Complete!"
echo ""
echo "💡 Key Takeaways:"
echo "   - File-based locking protects concurrent cache access"
echo "   - First process computes diff, second reads from cache"
echo "   - Rust-to-Rust composition via CLI (no library coupling)"
echo "   - Atomic lock operations prevent race conditions"
echo "   - Each tool remains independently usable"
echo "   - Graceful fallback if semaphore not available"
echo ""
echo "🔗 Use cases:"
echo "   • Concurrent CI jobs diffing same files"
echo "   • Multiple developers comparing configurations"
echo "   • Parallel test suites with shared fixtures"
echo ""
echo "🔗 See DOGFOODING_STRATEGY.md for implementation details"
