#!/bin/bash
# Validate Structured Error Handler tests for flakiness
#
# Usage: ./scripts/dogfood-flaky.sh [runs]
# Default: 10 runs
#
# This script uses Test Flakiness Detector to validate that all tests
# in this tool are deterministic and reliable.

set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"
RUNS="${1:-10}"

echo "🔬 Validating Structured Error Handler test suite"
echo "   Tool directory: $TOOL_DIR"
echo "   Runs: $RUNS"
echo ""

# Check if Test Flakiness Detector is available
if [ ! -d "$DETECTOR_DIR" ]; then
    echo "⚠️  Test Flakiness Detector not found (standalone mode)"
    echo "   Running tests once instead..."
    echo ""
    cd "$TOOL_DIR"
    npm test
    exit 0
fi

# Check if detector has node_modules
if [ ! -d "$DETECTOR_DIR/node_modules" ]; then
    echo "📦 Installing Test Flakiness Detector dependencies..."
    cd "$DETECTOR_DIR"
    npm install
fi

# Run flakiness detection
echo "🚀 Starting flakiness detection..."
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
    echo "✅ Flakiness detection complete - no flaky tests!"
else
    echo "❌ Flakiness detected or tests failed!"
    exit $EXIT_CODE
fi
