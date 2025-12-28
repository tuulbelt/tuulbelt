#!/bin/bash
# Dogfooding: Test Flakiness Detector validation
#
# Usage: ./scripts/dogfood-flaky.sh [runs]
# Default: 10 runs
#
# HIGH-VALUE: Validates 135 config merger tests are deterministic
#
# Why This Matters:
#   - Config merging involves type coercion and precedence rules
#   - Must be 100% deterministic for reliable application config
#   - Tests must pass consistently across environments

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"
RUNS="${1:-10}"

echo "🔬 Validating Config File Merger test suite"
echo "   135 tests × $RUNS runs = $((135 * RUNS)) executions"
echo ""

# Check if Test Flakiness Detector is available
if [ ! -d "$DETECTOR_DIR" ]; then
    echo "⚠️  Test Flakiness Detector not found (standalone mode)"
    echo "   Not in monorepo context, skipping dogfooding"
    echo ""
    echo "   Running tests once instead..."
    cd "$TOOL_DIR"
    npm test
    exit 0
fi

# Check if detector has node_modules
if [ ! -d "$DETECTOR_DIR/node_modules" ]; then
    echo "📦 Installing Test Flakiness Detector dependencies..."
    (cd "$DETECTOR_DIR" && npm install --silent)
fi

echo "✅ Test Flakiness Detector available"
echo "🎯 Running flakiness detection..."
echo ""

cd "$DETECTOR_DIR"
set +e  # Temporarily disable exit on error to capture exit code
npx flaky \
    --test "cd '$TOOL_DIR' && npm test 2>&1" \
    --runs "$RUNS" \
    --verbose

EXIT_CODE=$?
set -e  # Re-enable exit on error

echo ""
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ NO FLAKINESS DETECTED!"
    echo ""
    echo "   All $RUNS runs passed consistently"
    echo "   Config merging is deterministic"
else
    echo "❌ FLAKINESS DETECTED!"
    echo ""
    echo "   ⚠️  Fix flaky tests before using in production"
    exit 1
fi

echo ""
echo "🔗 See DOGFOODING_STRATEGY.md for details"
