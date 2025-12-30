#!/bin/bash
# Record Config File Merger demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="config-file-merger"
SHORT_NAME="cfgmerge"
LANGUAGE="typescript"

demo_setup() {
  mkdir -p "$TOOL_DIR/demo-files"
  echo '{"host": "localhost", "port": 8080, "debug": false}' > "$TOOL_DIR/demo-files/config.json"
  echo '{"timeout": 30000, "retries": 3}' > "$TOOL_DIR/demo-files/defaults.json"
}

demo_cleanup() {
  rm -rf "$TOOL_DIR/demo-files"
}

demo_commands() {
  echo "# Config File Merger Demo"
  sleep 1

  echo ""
  echo "# 1. Merge from a config file"
  sleep 0.5
  echo "$ cfgmerge --file demo-files/config.json"
  sleep 0.5
  cfgmerge --file demo-files/config.json
  sleep 2

  echo ""
  echo "# 2. Override with CLI arguments"
  sleep 0.5
  echo "$ cfgmerge --file demo-files/config.json --args \"port=3000,debug=true\""
  sleep 0.5
  cfgmerge --file demo-files/config.json --args "port=3000,debug=true"
  sleep 2

  echo ""
  echo "# 3. Track sources (where each value came from)"
  sleep 0.5
  echo "$ cfgmerge --file demo-files/config.json --args \"port=443\" --track-sources"
  sleep 0.5
  cfgmerge --file demo-files/config.json --args "port=443" --track-sources
  sleep 2

  echo ""
  echo "# 4. Set environment variables and merge"
  sleep 0.5
  echo "$ export APP_HOST=production.example.com"
  export APP_HOST=production.example.com
  echo "$ cfgmerge --file demo-files/config.json --env --prefix APP_ --track-sources"
  sleep 0.5
  cfgmerge --file demo-files/config.json --env --prefix APP_ --track-sources
  sleep 2

  echo ""
  echo "# 5. Combine defaults, file, env, and CLI"
  sleep 0.5
  echo "$ cfgmerge --defaults demo-files/defaults.json --file demo-files/config.json --env --prefix APP_ --args \"debug=true\" --track-sources"
  sleep 0.5
  cfgmerge --defaults demo-files/defaults.json --file demo-files/config.json --env --prefix APP_ --args "debug=true" --track-sources
  sleep 2

  echo ""
  echo "# 6. Show help"
  sleep 0.5
  echo "$ cfgmerge --help"
  sleep 0.5
  cfgmerge --help
  sleep 2

  echo ""
  echo "# Done! Merge configuration with the cfgmerge command."
  sleep 1
}

run_demo
