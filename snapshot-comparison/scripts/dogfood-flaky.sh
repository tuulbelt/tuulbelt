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

# Check if flaky command is available
if ! command -v flaky &> /dev/null; then
    # Try to use npx
    if command -v npx &> /dev/null && [ -f "$DETECTOR_DIR/package.json" ]; then
        cd "$DETECTOR_DIR"
        npm install --silent 2>/dev/null
        FLAKY_CMD="npx tsx src/index.ts"
    else
        echo "Test flakiness detector not installed"
        echo "Run 'npm install' in $DETECTOR_DIR first"
        exit 0
    fi
else
    FLAKY_CMD="flaky"
fi

echo "Validating snapshot-comparison test determinism..."
echo "Running cargo test $RUNS times..."
echo ""

cd "$TOOL_DIR"

$FLAKY_CMD \
    --test "cargo test 2>&1" \
    --runs "$RUNS" \
    --verbose

echo ""
echo "Dogfooding validation complete!"
