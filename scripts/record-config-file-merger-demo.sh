#!/bin/bash
# Record Config File Merger demo
set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../config-file-merger" && pwd)"
DEMO_FILE="$TOOL_DIR/demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Ensure dependencies are installed and CLI is linked
echo "Installing dependencies..."
cd "$TOOL_DIR"
npm install --silent 2>/dev/null || npm install

# Link the CLI globally so short name works
echo "Linking CLI globally..."
npm link --force 2>/dev/null || npm link

# Create demo config files
mkdir -p "$TOOL_DIR/demo-files"
echo '{"host": "localhost", "port": 8080, "debug": false}' > "$TOOL_DIR/demo-files/config.json"
echo '{"timeout": 30000, "retries": 3}' > "$TOOL_DIR/demo-files/defaults.json"

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --title "Config File Merger / cfgmerge - Tuulbelt" --command "bash -c '
echo \"# Config File Merger Demo\"
sleep 1

echo \"\"
echo \"# 1. Merge from a config file\"
sleep 0.5
echo \"\$ cfgmerge --file demo-files/config.json\"
sleep 0.5
cd \"$TOOL_DIR\"
cfgmerge --file demo-files/config.json
sleep 2

echo \"\"
echo \"# 2. Override with CLI arguments\"
sleep 0.5
echo \"\$ cfgmerge --file demo-files/config.json --args \\\"port=3000,debug=true\\\"\"
sleep 0.5
cfgmerge --file demo-files/config.json --args \"port=3000,debug=true\"
sleep 2

echo \"\"
echo \"# 3. Track sources (where each value came from)\"
sleep 0.5
echo \"\$ cfgmerge --file demo-files/config.json --args \\\"port=443\\\" --track-sources\"
sleep 0.5
cfgmerge --file demo-files/config.json --args \"port=443\" --track-sources
sleep 2

echo \"\"
echo \"# 4. Set environment variables and merge\"
sleep 0.5
echo \"\$ export APP_HOST=production.example.com\"
export APP_HOST=production.example.com
echo \"\$ cfgmerge --file demo-files/config.json --env --prefix APP_ --track-sources\"
sleep 0.5
cfgmerge --file demo-files/config.json --env --prefix APP_ --track-sources
sleep 2

echo \"\"
echo \"# 5. Combine defaults, file, env, and CLI\"
sleep 0.5
echo \"\$ cfgmerge --defaults demo-files/defaults.json --file demo-files/config.json --env --prefix APP_ --args \\\"debug=true\\\" --track-sources\"
sleep 0.5
cfgmerge --defaults demo-files/defaults.json --file demo-files/config.json --env --prefix APP_ --args \"debug=true\" --track-sources
sleep 2

echo \"\"
echo \"# 6. Show help\"
sleep 0.5
echo \"\$ cfgmerge --help\"
sleep 0.5
cfgmerge --help
sleep 2

echo \"\"
echo \"# Done! Merge configuration with the cfgmerge command.\"
sleep 1
'"

# Clean up demo files
rm -rf "$TOOL_DIR/demo-files"

echo "Demo recording saved to $DEMO_FILE"

# Upload to asciinema.org if install ID is provided
if [ -n "$ASCIINEMA_INSTALL_ID" ]; then
  echo "Uploading to asciinema.org..."

  # Set up install ID in asciinema config
  mkdir -p ~/.config/asciinema
  echo "$ASCIINEMA_INSTALL_ID" > ~/.config/asciinema/install-id

  # Upload the recording
  UPLOAD_OUTPUT=$(asciinema upload "$DEMO_FILE" 2>&1)
  UPLOAD_URL=$(echo "$UPLOAD_OUTPUT" | grep -oP 'https://asciinema.org/a/\K[a-zA-Z0-9]+' || echo "")

  if [ -n "$UPLOAD_URL" ]; then
    echo "Demo uploaded: https://asciinema.org/a/$UPLOAD_URL"
    echo "https://asciinema.org/a/$UPLOAD_URL" > "$TOOL_DIR/demo-url.txt"
  else
    echo "Upload failed or URL not found in output:"
    echo "$UPLOAD_OUTPUT"
  fi
fi

# Convert to GIF (requires agg)
if command -v agg &> /dev/null; then
  echo "Converting to GIF..."
  mkdir -p "$TOOL_DIR/docs"
  agg "$DEMO_FILE" "$TOOL_DIR/docs/demo.gif" --theme monokai --font-size 16
  echo "GIF saved to $TOOL_DIR/docs/demo.gif"
fi

echo "Demo creation complete!"
