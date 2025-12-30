#!/bin/bash
# Record Structured Error Handler demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="structured-error-handler"
SHORT_NAME="serr"
LANGUAGE="typescript"

demo_commands() {
  echo "# Structured Error Handler Demo"
  sleep 1

  echo ""
  echo "# 1. Show demo with JSON output (default)"
  sleep 0.5
  echo "$ serr demo"
  sleep 0.5
  serr demo | head -30
  sleep 2

  echo ""
  echo "# 2. Show demo with human-readable text format"
  sleep 0.5
  echo "$ serr demo --format text"
  sleep 0.5
  serr demo --format text
  sleep 2

  echo ""
  echo "# 3. Parse a JSON error"
  sleep 0.5
  echo '$ serr parse '\''{"message":"Test error","code":"TEST_CODE","context":[]}'\'
  sleep 0.5
  serr parse '{"message":"Test error","code":"TEST_CODE","context":[]}' --format text
  sleep 2

  echo ""
  echo "# 4. Validate JSON error format"
  sleep 0.5
  echo '$ serr validate '\''{"message":"Hello"}'\'
  sleep 0.5
  serr validate '{"message":"Hello"}'
  sleep 1

  echo ""
  echo "# 5. Show help"
  sleep 0.5
  echo "$ serr --help"
  sleep 0.5
  serr --help
  sleep 2

  echo ""
  echo "# Done! Handle structured errors with the serr command."
  sleep 1
}

run_demo
