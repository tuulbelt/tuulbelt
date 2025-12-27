#!/bin/bash
# Record CLI Progress Reporting demo
set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../cli-progress-reporting" && pwd)"
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
asciinema rec "$DEMO_FILE" --overwrite --title "CLI Progress Reporting / prog - Tuulbelt" --command "bash -c '
echo \"# CLI Progress Reporting Demo\"
sleep 1

echo \"\"
echo \"# Initialize progress tracking\"
sleep 0.5
echo \"\$ prog init --total 100 --message \\\"Processing files\\\"\"
sleep 0.5
cd \"$TOOL_DIR\"
npx tsx src/index.ts init --total 100 --message \"Processing files\"
sleep 1

echo \"\"
echo \"# Update progress in a loop\"
sleep 0.5
for i in {1..5}; do
  echo \"\$ prog increment --amount 20\"
  npx tsx src/index.ts increment --amount 20
  sleep 0.8
done
sleep 1

echo \"\"
echo \"# Finish with completion message\"
sleep 0.5
echo \"\$ prog finish --message \\\"Complete!\\\"\"
sleep 0.5
npx tsx src/index.ts finish --message \"Complete!\"
sleep 2

echo \"\"
echo \"# Show help\"
sleep 0.5
echo \"\$ prog --help\"
sleep 0.5
npx tsx src/index.ts --help
sleep 2

echo \"\"
echo \"# Done! Track progress with the prog command.\"
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
