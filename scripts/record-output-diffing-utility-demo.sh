#!/bin/bash
# Record Output Diffing Utility demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="output-diffing-utility"
SHORT_NAME="odiff"
LANGUAGE="rust"

demo_setup() {
  cat > /tmp/old.txt <<EOF
Hello World
This is line 2
Line 3 remains
Line 4 is here
EOF

  cat > /tmp/new.txt <<EOF
Hello World
This is modified line 2
Line 3 remains
Line 5 is different
EOF

  cat > /tmp/old.json <<EOF
{
  "user": "alice",
  "age": 30,
  "active": true
}
EOF

  cat > /tmp/new.json <<EOF
{
  "user": "alice",
  "age": 31,
  "active": false
}
EOF
}

demo_cleanup() {
  rm -f /tmp/old.txt /tmp/new.txt /tmp/old.json /tmp/new.json
}

demo_commands() {
  echo "# Output Diffing Utility Demo"
  sleep 1

  echo ""
  echo "# 1. Text diff with color (unified format)"
  sleep 0.5
  echo "$ odiff --color always /tmp/old.txt /tmp/new.txt"
  sleep 0.5
  $BIN --color always /tmp/old.txt /tmp/new.txt
  sleep 2

  echo ""
  echo "# 2. JSON diff with color (structural)"
  sleep 0.5
  echo "$ odiff --color always /tmp/old.json /tmp/new.json"
  sleep 0.5
  $BIN --color always /tmp/old.json /tmp/new.json
  sleep 2

  echo ""
  echo "# 3. JSON output format (structured data)"
  sleep 0.5
  echo "$ odiff --format json /tmp/old.json /tmp/new.json"
  sleep 0.5
  $BIN --format json /tmp/old.json /tmp/new.json | head -20
  sleep 2

  echo ""
  echo "# 4. Compact format"
  sleep 0.5
  echo "$ odiff --format compact /tmp/old.txt /tmp/new.txt"
  sleep 0.5
  $BIN --format compact /tmp/old.txt /tmp/new.txt
  sleep 2

  echo ""
  echo "# Done! Diff outputs with the odiff command."
  sleep 1
}

run_demo
