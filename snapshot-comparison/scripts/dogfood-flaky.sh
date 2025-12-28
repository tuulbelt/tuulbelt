#!/bin/bash
# Validate snapshot-comparison tests using Test Flakiness Detector
#
# Usage: ./scripts/dogfood-flaky.sh [runs]
#   runs: Number of test runs (default: 10)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
RUNS="${1:-10}"

# Check if we're in the monorepo context
DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"
if [ ! -d "$DETECTOR_DIR" ]; then
    echo "Not in monorepo context - test-flakiness-detector not available"
    echo "Skipping dogfooding validation"
    exit 0
fi

# Check if detector has node_modules
if [ ! -d "$DETECTOR_DIR/node_modules" ]; then
    echo "📦 Installing Test Flakiness Detector dependencies..."
    (cd "$DETECTOR_DIR" && npm install --silent)
fi

echo "🔬 Validating snapshot-comparison test determinism..."
echo "Running cargo test $RUNS times..."
echo ""

cd "$DETECTOR_DIR"
set +e  # Temporarily disable exit on error to capture exit code
npx flaky \
    --test "cd '$TOOL_DIR' && cargo test 2>&1" \
    --runs "$RUNS" \
    --verbose

EXIT_CODE=$?
set -e  # Re-enable exit on error

echo ""
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Dogfooding validation complete - no flaky tests!"
else
    echo "❌ Flakiness detected or tests failed!"
    exit $EXIT_CODE
fi
