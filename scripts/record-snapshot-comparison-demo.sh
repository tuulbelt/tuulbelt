#!/bin/bash
# Record Snapshot Comparison demo
set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../snapshot-comparison" && pwd)"
DEMO_FILE="$TOOL_DIR/demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Build the release binary first
echo "Building release binary..."
cd "$TOOL_DIR"
cargo build --release 2>/dev/null
BIN="./target/release/snapcmp"

# Create temp directory for demo
DEMO_DIR=$(mktemp -d)
SNAP_DIR="$DEMO_DIR/snapshots"
mkdir -p "$SNAP_DIR"

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --title "Snapshot Comparison / snapcmp - Tuulbelt" --command "bash -c '
echo \"# Snapshot Comparison Demo\"
sleep 1

echo \"\"
echo \"# 1. Create a snapshot from command output\"
sleep 0.5
echo \"\$ echo '\''Hello, World!'\'' | snapcmp create greeting --dir $SNAP_DIR\"
sleep 0.5
cd \"$TOOL_DIR\"
echo \"Hello, World!\" | $BIN create greeting --dir $SNAP_DIR
sleep 1.5

echo \"\"
echo \"# 2. Check output against snapshot (match)\"
sleep 0.5
echo \"\$ echo '\''Hello, World!'\'' | snapcmp check greeting --dir $SNAP_DIR\"
sleep 0.5
echo \"Hello, World!\" | $BIN check greeting --dir $SNAP_DIR
sleep 1.5

echo \"\"
echo \"# 3. Check with different output (mismatch with diff)\"
sleep 0.5
echo \"\$ echo '\''Hello, Universe!'\'' | snapcmp check greeting --dir $SNAP_DIR --color\"
sleep 0.5
echo \"Hello, Universe!\" | $BIN check greeting --dir $SNAP_DIR --color || true
sleep 2

echo \"\"
echo \"# 4. Update snapshot with new content\"
sleep 0.5
echo \"\$ echo '\''Hello, Universe!'\'' | snapcmp update greeting --dir $SNAP_DIR\"
sleep 0.5
echo \"Hello, Universe!\" | $BIN update greeting --dir $SNAP_DIR
sleep 1.5

echo \"\"
echo \"# 5. Create JSON snapshot\"
sleep 0.5
echo \"\$ echo '\''{\"status\": \"ok\", \"count\": 42}'\'' | snapcmp create api-response --dir $SNAP_DIR --type json\"
sleep 0.5
echo \"{\\\"status\\\": \\\"ok\\\", \\\"count\\\": 42}\" | $BIN create api-response --dir $SNAP_DIR --type json
sleep 1.5

echo \"\"
echo \"# 6. List all snapshots\"
sleep 0.5
echo \"\$ snapcmp list --dir $SNAP_DIR\"
sleep 0.5
$BIN list --dir $SNAP_DIR
sleep 2

echo \"\"
echo \"# Done! Snapshot testing with snapcmp.\"
sleep 1
'"

echo "Demo recording saved to $DEMO_FILE"

# Cleanup temp dir
rm -rf "$DEMO_DIR"

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
