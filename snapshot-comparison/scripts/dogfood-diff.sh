#!/bin/bash
# Verify snapshot-comparison produces deterministic CLI output
#
# Usage: ./scripts/dogfood-diff.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check if we're in the monorepo context
ODIFF_DIR="$TOOL_DIR/../output-diffing-utility"
if [ ! -d "$ODIFF_DIR" ]; then
    echo "Not in monorepo context - output-diffing-utility not available"
    echo "Skipping output consistency validation"
    exit 0
fi

# Build the tool
echo "Building snapshot-comparison..."
cd "$TOOL_DIR"
cargo build --release --quiet

SNAPCMP="$TOOL_DIR/target/release/snapcmp"
ODIFF="$ODIFF_DIR/target/release/odiff"

# Build odiff if needed
if [ ! -f "$ODIFF" ]; then
    echo "Building output-diffing-utility..."
    cd "$ODIFF_DIR"
    cargo build --release --quiet
fi

cd "$TOOL_DIR"

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "Verifying output consistency..."

# Test 1: Create snapshot and check help output consistency
echo "  Testing help output..."
$SNAPCMP --help > "$TEMP_DIR/help1.txt"
$SNAPCMP --help > "$TEMP_DIR/help2.txt"
if ! diff -q "$TEMP_DIR/help1.txt" "$TEMP_DIR/help2.txt" > /dev/null; then
    echo "  ✗ Help output differs between runs"
    $ODIFF text "$TEMP_DIR/help1.txt" "$TEMP_DIR/help2.txt"
    exit 1
fi
echo "  ✓ Help output is deterministic"

# Test 2: Create and check snapshot
echo "  Testing create/check cycle..."
mkdir -p "$TEMP_DIR/snapshots"
echo "test content" | $SNAPCMP create test1 -d "$TEMP_DIR/snapshots" 2>&1 > "$TEMP_DIR/create1.txt"
echo "test content" | $SNAPCMP check test1 -d "$TEMP_DIR/snapshots" 2>&1 > "$TEMP_DIR/check1.txt"
echo "test content" | $SNAPCMP check test1 -d "$TEMP_DIR/snapshots" 2>&1 > "$TEMP_DIR/check2.txt"
if ! diff -q "$TEMP_DIR/check1.txt" "$TEMP_DIR/check2.txt" > /dev/null; then
    echo "  ✗ Check output differs between runs"
    $ODIFF text "$TEMP_DIR/check1.txt" "$TEMP_DIR/check2.txt"
    exit 1
fi
echo "  ✓ Create/check output is deterministic"

# Test 3: List output consistency
echo "  Testing list output..."
$SNAPCMP list -d "$TEMP_DIR/snapshots" > "$TEMP_DIR/list1.txt"
$SNAPCMP list -d "$TEMP_DIR/snapshots" > "$TEMP_DIR/list2.txt"
if ! diff -q "$TEMP_DIR/list1.txt" "$TEMP_DIR/list2.txt" > /dev/null; then
    echo "  ✗ List output differs between runs"
    $ODIFF text "$TEMP_DIR/list1.txt" "$TEMP_DIR/list2.txt"
    exit 1
fi
echo "  ✓ List output is deterministic"

echo ""
echo "✅ All outputs are identical across runs"
echo "Dogfooding validation complete!"
