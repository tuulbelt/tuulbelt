#!/bin/bash
# Dogfood: Validate test reliability using Test Flakiness Detector
#
# Usage: ./scripts/dogfood-flaky.sh [runs]
#
# This validates that running tests multiple times produces consistent
# results. Uses the test-flakiness-detector from Tuulbelt.

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
FLAKY_DIR="$TOOL_DIR/../test-flakiness-detector"
RUNS="${1:-10}"

echo "=========================================="
echo "Dogfooding: Test Flakiness Detection"
echo "Tool: [TOOL_NAME]"  # TODO: Replace with actual tool name
echo "Validator: test-flakiness-detector (flaky)"
echo "=========================================="
echo

# Check for test-flakiness-detector
if [ ! -d "$FLAKY_DIR" ]; then
    echo "⚠️  Not in meta repo context (test-flakiness-detector not found)"
    echo "   Skipping dogfood validation - tool works standalone"
    exit 0
fi

cd "$FLAKY_DIR"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "Installing test-flakiness-detector dependencies..."
    npm ci --silent
fi

# Link CLI if not already linked
npm link 2>/dev/null || true

cd "$TOOL_DIR"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "Installing [TOOL_NAME] dependencies..."  # TODO: Replace with actual tool name
    npm ci --silent
fi

echo "Running flakiness detection ($RUNS runs)..."
echo

# Run flakiness detector
# Note: Using 'npm test' as the test command
flaky \
    --test "npm test" \
    --runs "$RUNS" \
    --verbose

echo
echo "=========================================="
echo "Flakiness detection complete!"
echo "=========================================="
