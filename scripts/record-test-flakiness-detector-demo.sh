#!/bin/bash
# Record Test Flakiness Detector demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="test-flakiness-detector"
SHORT_NAME="flaky"
LANGUAGE="typescript"

demo_commands() {
  echo "# Test Flakiness Detector Demo"
  sleep 1

  echo ""
  echo "# Basic usage: Run flaky test detection (5 runs)"
  sleep 0.5
  echo "$ flaky --test \"echo test\" --runs 5"
  sleep 0.5
  flaky --test "echo test" --runs 5
  sleep 2

  echo ""
  echo "# With threshold: Ignore failures <10% (recommended for CI)"
  sleep 0.5
  echo "$ flaky --test \"npm test\" --runs 20 --threshold 10"
  sleep 0.5
  echo "Only tests with >10% failure rate will be flagged as flaky"
  sleep 1.5

  echo ""
  echo "# Show help (includes --threshold flag)"
  sleep 0.5
  echo "$ flaky --help"
  sleep 0.5
  flaky --help
  sleep 2

  echo ""
  echo "# Done! Detect flaky tests with configurable thresholds."
  sleep 1
}

run_demo
