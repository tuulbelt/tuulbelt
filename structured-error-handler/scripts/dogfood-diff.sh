#!/bin/bash
# Validate Structured Error Handler serialization determinism
#
# Usage: ./scripts/dogfood-diff.sh
#
# This script uses Output Diffing Utility to verify that error serialization
# produces identical output across runs (excluding timestamps).

set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIFFER_DIR="$TOOL_DIR/../output-diffing-utility"
TMP_DIR=$(mktemp -d)

cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "🔬 Validating Structured Error Handler serialization determinism"
echo "   Tool directory: $TOOL_DIR"
echo ""

# Check if Output Diffing Utility is available
if [ ! -d "$DIFFER_DIR" ] || [ ! -f "$DIFFER_DIR/target/release/output-diffing-utility" ]; then
    echo "⚠️  Output Diffing Utility not found (standalone mode)"
    echo "   Running basic determinism check instead..."
    echo ""

    cd "$TOOL_DIR"

    # Run demo twice and compare (masking timestamps)
    npx tsx src/index.ts demo --format json 2>/dev/null | sed 's/"timestamp":"[^"]*"/"timestamp":"MASKED"/g' > "$TMP_DIR/run1.json"
    npx tsx src/index.ts demo --format json 2>/dev/null | sed 's/"timestamp":"[^"]*"/"timestamp":"MASKED"/g' > "$TMP_DIR/run2.json"

    if diff -q "$TMP_DIR/run1.json" "$TMP_DIR/run2.json" > /dev/null; then
        echo "✅ Serialization is deterministic (basic check)"
    else
        echo "❌ Serialization differs between runs!"
        diff "$TMP_DIR/run1.json" "$TMP_DIR/run2.json"
        exit 1
    fi
    exit 0
fi

# Run demo twice with timestamp masking
cd "$TOOL_DIR"
echo "📝 Generating serialized outputs..."

npx tsx src/index.ts demo --format json 2>/dev/null | sed 's/"timestamp":"[^"]*"/"timestamp":"MASKED"/g' > "$TMP_DIR/run1.json"
npx tsx src/index.ts demo --format json 2>/dev/null | sed 's/"timestamp":"[^"]*"/"timestamp":"MASKED"/g' > "$TMP_DIR/run2.json"

echo "🔍 Comparing outputs with Output Diffing Utility..."
echo ""

# Use output-diffing-utility to compare
cd "$DIFFER_DIR"
./target/release/output-diffing-utility "$TMP_DIR/run1.json" "$TMP_DIR/run2.json" --format summary

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Serialization is deterministic!"
else
    echo ""
    echo "❌ Serialization differs between runs!"
    ./target/release/output-diffing-utility "$TMP_DIR/run1.json" "$TMP_DIR/run2.json" --format unified
    exit 1
fi
