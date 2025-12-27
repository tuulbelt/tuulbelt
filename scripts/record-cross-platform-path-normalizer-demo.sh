#!/bin/bash
# Record Cross-Platform Path Normalizer demo
set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../cross-platform-path-normalizer" && pwd)"
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
asciinema rec "$DEMO_FILE" --overwrite --title "Cross-Platform Path Normalizer / normpath - Tuulbelt" --command "bash -c '
echo \"# Cross-Platform Path Normalizer Demo\"
sleep 1

echo \"\"
echo \"# Normalize a Windows path\"
sleep 0.5
echo \"\$ normpath \\\"C:\\\\Users\\\\Documents\\\\file.txt\\\"\"
sleep 0.5
cd \"$TOOL_DIR\"
npx tsx src/index.ts \"C:\\Users\\Documents\\file.txt\"
sleep 1.5

echo \"\"
echo \"# Convert Windows path to Unix format\"
sleep 0.5
echo \"\$ normpath --format unix \\\"C:\\\\Users\\\\Documents\\\\file.txt\\\"\"
sleep 0.5
npx tsx src/index.ts --format unix \"C:\\Users\\Documents\\file.txt\"
sleep 1.5

echo \"\"
echo \"# Convert Unix path to Windows format\"
sleep 0.5
echo \"\$ normpath --format windows \\\"/home/user/project/src\\\"\"
sleep 0.5
npx tsx src/index.ts --format windows \"/home/user/project/src\"
sleep 1.5

echo \"\"
echo \"# Handle UNC network paths\"
sleep 0.5
echo \"\$ normpath --format unix \\\"\\\\\\\\\\\\\\\\server\\\\\\\\share\\\\\\\\folder\\\"\"
sleep 0.5
npx tsx src/index.ts --format unix \"\\\\\\\\server\\\\share\\\\folder\"
sleep 1.5

echo \"\"
echo \"# Show help\"
sleep 0.5
echo \"\$ normpath --help\"
sleep 0.5
npx tsx src/index.ts --help
sleep 2

echo \"\"
echo \"# Done! Normalize paths with the normpath command.\"
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

# Convert to GIF (requires agg or svg-term)
if command -v agg &> /dev/null; then
  echo "Converting to GIF..."
  mkdir -p "$TOOL_DIR/docs"
  agg "$DEMO_FILE" "$TOOL_DIR/docs/demo.gif" --theme monokai --font-size 16
  echo "GIF saved to $TOOL_DIR/docs/demo.gif"
fi

echo "Demo creation complete!"
