#!/bin/bash
# Record Output Diffing Utility demo
set -e

DEMO_FILE="demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Build the release binary first
echo "Building release binary..."
cargo build --release 2>/dev/null
BIN="./target/release/output-diff"

# Create test files
echo "Creating test files..."
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

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --title "Output Diffing Utility - Tuulbelt" --command "bash -c '
echo \"# Output Diffing Utility Demo\"
sleep 1

echo \"\"
echo \"# 1. Text diff (unified format)\"
sleep 0.5
echo \"\$ $BIN /tmp/old.txt /tmp/new.txt\"
sleep 0.5
$BIN /tmp/old.txt /tmp/new.txt
sleep 2

echo \"\"
echo \"# 2. JSON diff (structural)\"
sleep 0.5
echo \"\$ $BIN --format json /tmp/old.json /tmp/new.json\"
sleep 0.5
$BIN --format json /tmp/old.json /tmp/new.json | head -20
sleep 2

echo \"\"
echo \"# 3. Compact format\"
sleep 0.5
echo \"\$ $BIN --format compact /tmp/old.txt /tmp/new.txt\"
sleep 0.5
$BIN --format compact /tmp/old.txt /tmp/new.txt
sleep 2

echo \"\"
echo \"# 4. Context lines (show only 1 line around changes)\"
sleep 0.5
echo \"\$ $BIN --context 1 /tmp/old.txt /tmp/new.txt\"
sleep 0.5
$BIN --context 1 /tmp/old.txt /tmp/new.txt
sleep 2

echo \"\"
echo \"# Done!\"
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
    echo "https://asciinema.org/a/$UPLOAD_URL" > demo-url.txt
  else
    echo "Upload failed or URL not found in output:"
    echo "$UPLOAD_OUTPUT"
  fi
fi

# Convert to GIF (requires agg)
if command -v agg &> /dev/null; then
  echo "Converting to GIF..."
  mkdir -p docs
  agg "$DEMO_FILE" docs/demo.gif --theme monokai --font-size 16
  echo "GIF saved to docs/demo.gif"
fi

echo "Demo creation complete!"
