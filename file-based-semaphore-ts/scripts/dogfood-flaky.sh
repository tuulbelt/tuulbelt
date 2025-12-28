#!/bin/bash
# Dogfood: Validate semats test reliability using test-flakiness-detector
#
# Usage: ./scripts/dogfood-flaky.sh [runs]
#   runs: Number of test runs (default: 10)
#
# This validates that all 160 tests are deterministic across multiple runs.
# Flaky tests could hide race conditions in our concurrent lock handling.

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
RUNS="${1:-10}"

echo "=========================================="
echo "Dogfooding: Test Flakiness Detection"
echo "Tool: file-based-semaphore-ts (semats)"
echo "Runs: $RUNS"
echo "=========================================="
echo

# Check for test-flakiness-detector
DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"

if [ ! -d "$DETECTOR_DIR" ]; then
    echo "⚠️  Not in monorepo context (test-flakiness-detector not found)"
    echo "   Skipping dogfood validation - tool works standalone"
    exit 0
fi

# Ensure flaky is available
if ! command -v flaky &> /dev/null; then
    echo "Installing test-flakiness-detector..."
    cd "$DETECTOR_DIR"
    npm install --silent
    npm link --silent
    cd "$TOOL_DIR"
fi

echo "Running flakiness detection..."
echo

cd "$DETECTOR_DIR"
set +e  # Temporarily disable exit on error to capture exit code
npx flaky \
    --test "cd '$TOOL_DIR' && npm test 2>&1" \
    --runs "$RUNS" \
    --verbose

EXIT_CODE=$?
set -e  # Re-enable exit on error

echo
echo "=========================================="
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Dogfood validation complete - no flaky tests!"
else
    echo "❌ Flakiness detected or tests failed!"
    exit $EXIT_CODE
fi
echo "=========================================="
