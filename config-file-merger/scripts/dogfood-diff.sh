#!/bin/bash
# Validate Config File Merger output determinism
#
# Usage: ./scripts/dogfood-diff.sh
#
# HIGH-VALUE: Proves config merging produces identical output for identical input
#
# Why This Matters:
#   - Type coercion must be deterministic (42 always becomes number)
#   - Precedence rules must be consistent
#   - Source tracking must be reliable

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
DIFFER_DIR="$TOOL_DIR/../output-diffing-utility"
TMP_DIR=$(mktemp -d)

cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "🔬 Validating Config File Merger output determinism"
echo ""

# Create test config file
echo '{"host": "localhost", "port": 8080, "debug": false}' > "$TMP_DIR/config.json"

# Check if Output Diffing Utility is available
if [ ! -d "$DIFFER_DIR" ] || [ ! -f "$DIFFER_DIR/target/release/output-diffing-utility" ]; then
    echo "⚠️  Output Diffing Utility not found (standalone mode)"
    echo "   Not in monorepo context, using basic diff instead..."
    echo ""

    cd "$TOOL_DIR"

    # Run merge twice and compare
    npx cfgmerge --file "$TMP_DIR/config.json" --args "port=3000,debug=true" --track-sources > "$TMP_DIR/run1.json" 2>/dev/null
    npx cfgmerge --file "$TMP_DIR/config.json" --args "port=3000,debug=true" --track-sources > "$TMP_DIR/run2.json" 2>/dev/null

    if diff -q "$TMP_DIR/run1.json" "$TMP_DIR/run2.json" > /dev/null; then
        echo "✅ Config merging is deterministic (basic check)"
        echo ""
        echo "   Identical output produced for identical input"
    else
        echo "❌ Config merging differs between runs!"
        diff "$TMP_DIR/run1.json" "$TMP_DIR/run2.json"
        exit 1
    fi
    exit 0
fi

# Run merge twice
cd "$TOOL_DIR"
echo "📝 Generating merged configs..."

npx cfgmerge --file "$TMP_DIR/config.json" --args "port=3000,debug=true" --track-sources > "$TMP_DIR/run1.json" 2>/dev/null
npx cfgmerge --file "$TMP_DIR/config.json" --args "port=3000,debug=true" --track-sources > "$TMP_DIR/run2.json" 2>/dev/null

echo "🔍 Comparing outputs with Output Diffing Utility..."
echo ""

# Use output-diffing-utility to compare
cd "$DIFFER_DIR"
./target/release/output-diffing-utility "$TMP_DIR/run1.json" "$TMP_DIR/run2.json" --format summary

EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    echo ""
    echo "✅ Config merging is deterministic!"
    echo ""
    echo "   Type coercion: consistent"
    echo "   Precedence: reliable"
    echo "   Source tracking: stable"
else
    echo ""
    echo "❌ Config merging differs between runs!"
    ./target/release/output-diffing-utility "$TMP_DIR/run1.json" "$TMP_DIR/run2.json" --format unified
    exit 1
fi

echo ""
echo "🔗 See DOGFOODING_STRATEGY.md for details"
