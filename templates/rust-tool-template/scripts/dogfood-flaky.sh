#!/bin/bash
#
# Dogfooding: Use Test Flakiness Detector to validate this tool's tests
#
# This script runs the test-flakiness-detector tool against this tool's test suite
# to verify all tests are deterministic and non-flaky.
#
# Usage:
#   ./scripts/dogfood.sh [runs]
#
# Examples:
#   ./scripts/dogfood.sh        # Default: 10 runs
#   ./scripts/dogfood.sh 20     # Custom: 20 runs
#
# Prerequisites:
#   - test-flakiness-detector must be available in parent directory (meta repo)
#   - Node.js and npm must be installed
#   - Rust and cargo must be installed
#

set -e

# Configuration
RUNS="${1:-10}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"
TEMP_OUTPUT="/tmp/dogfood-output-$$.json"

echo "Dogfooding: Using Test Flakiness Detector to validate $(basename "$TOOL_DIR") tests"
echo ""

# Verify Test Flakiness Detector exists
if [ ! -d "$DETECTOR_DIR" ]; then
    echo "Test Flakiness Detector not found at: $DETECTOR_DIR"
    echo "   Make sure test-flakiness-detector tool is available in parent directory"
    echo ""
    echo "   This is expected if running standalone (outside meta repo)."
    echo "   Dogfooding is optional and only works in meta repo context."
    exit 0
fi

# Verify npm dependencies are installed
if [ ! -d "$DETECTOR_DIR/node_modules" ]; then
    echo "Installing Test Flakiness Detector dependencies..."
    (cd "$DETECTOR_DIR" && npm install --silent)
fi

echo "Test Flakiness Detector location: $DETECTOR_DIR"
echo "Test command: cargo test"
echo "Number of runs: $RUNS"
echo ""
echo "Running flakiness detection (this may take a minute)..."
echo ""

START_TIME=$(date +%s)

# Run the flakiness detector and save output
cd "$DETECTOR_DIR"
npx flaky \
    --test "cd '$TOOL_DIR' && cargo test 2>&1" \
    --runs "$RUNS" > "$TEMP_OUTPUT" 2>&1

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

# Extract just the JSON part (starts with {)
JSON_RESULT=$(grep -A9999 '^{' "$TEMP_OUTPUT" | head -1000)

# Parse key values
TOTAL_RUNS=$(echo "$JSON_RESULT" | grep -oE '"totalRuns"\s*:\s*[0-9]+' | grep -oE '[0-9]+' | head -1)
PASSED_RUNS=$(echo "$JSON_RESULT" | grep -oE '"passedRuns"\s*:\s*[0-9]+' | grep -oE '[0-9]+' | head -1)
FAILED_RUNS=$(echo "$JSON_RESULT" | grep -oE '"failedRuns"\s*:\s*[0-9]+' | grep -oE '[0-9]+' | head -1)

# Cleanup
rm -f "$TEMP_OUTPUT"

# Fallback if parsing failed
if [ -z "$TOTAL_RUNS" ]; then
    echo "Could not parse JSON output."
    exit 1
fi

echo "Flakiness detection completed in ${ELAPSED} seconds"
echo ""
echo "Results:"
echo "   Total runs: $TOTAL_RUNS"
echo "   Passed runs: $PASSED_RUNS"
echo "   Failed runs: $FAILED_RUNS"
echo ""

# Check for flakiness
if [ "$FAILED_RUNS" = "0" ]; then
    echo "NO FLAKINESS DETECTED"
    echo ""
    echo "All $TOTAL_RUNS test runs passed consistently."
    echo "The test suite is deterministic and reliable!"
    echo ""
    echo "This validates that:"
    echo "   - All tests are deterministic"
    echo "   - No race conditions in concurrent tests"
    echo "   - No shared state between tests"
    echo "   - File cleanup is thorough"
    exit 0
elif [ "$PASSED_RUNS" = "0" ]; then
    echo "ALL TESTS FAILED"
    echo ""
    echo "All test runs failed consistently."
    echo "This indicates a genuine test failure, not flakiness."
    exit 1
else
    echo "FLAKY TESTS DETECTED!"
    echo ""
    echo "The test suite has intermittent failures:"
    echo "   $PASSED_RUNS/$TOTAL_RUNS runs passed"
    echo "   $FAILED_RUNS/$TOTAL_RUNS runs failed"
    echo ""
    echo "Fix these flaky tests before releasing!"
    exit 1
fi
