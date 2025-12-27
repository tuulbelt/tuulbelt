#!/bin/bash
# Record Structured Error Handler demo
set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../structured-error-handler" && pwd)"
DEMO_FILE="$TOOL_DIR/demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Ensure dependencies are installed
echo "Installing dependencies..."
cd "$TOOL_DIR"
npm install --silent 2>/dev/null || npm install

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --title "Structured Error Handler / serr - Tuulbelt" --command "bash -c '
echo \"# Structured Error Handler Demo\"
sleep 1

echo \"\"
echo \"# 1. Show demo with JSON output (default)\"
sleep 0.5
echo \"\$ serr demo\"
sleep 0.5
cd \"$TOOL_DIR\"
npx serr demo | head -30
sleep 2

echo \"\"
echo \"# 2. Show demo with human-readable text format\"
sleep 0.5
echo \"\$ serr demo --format text\"
sleep 0.5
npx serr demo --format text
sleep 2

echo \"\"
echo \"# 3. Parse a JSON error\"
sleep 0.5
echo '\''$ serr parse '\''\"'\"'{\"message\":\"Test error\",\"code\":\"TEST_CODE\",\"context\":[]}'\''\"'\"
sleep 0.5
npx serr parse '\''{\"message\":\"Test error\",\"code\":\"TEST_CODE\",\"context\":[]}'\'' --format text
sleep 2

echo \"\"
echo \"# 4. Validate JSON error format\"
sleep 0.5
echo '\''$ serr validate '\''\"'\"'{\"message\":\"Hello\"}'\''\"'\"
sleep 0.5
npx serr validate '\''{\"message\":\"Hello\"}'\''
sleep 1

echo \"\"
echo \"# 5. Show help\"
sleep 0.5
echo \"\$ serr --help\"
sleep 0.5
npx serr --help
sleep 2

echo \"\"
echo \"# Done! Handle structured errors with the serr command.\"
sleep 1
'"

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
