#!/bin/bash
# Record File-Based Semaphore demo
set -e

DEMO_FILE="demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Build the release binary first
echo "Building release binary..."
cargo build --release 2>/dev/null
BIN="./target/release/file-semaphore"

# Create temp lock file path
LOCK_FILE="/tmp/demo.lock"
rm -f "$LOCK_FILE"

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --title "File-Based Semaphore - Tuulbelt" --command "bash -c '
echo \"# File-Based Semaphore Demo\"
sleep 1

echo \"\"
echo \"# 1. Acquire a lock (non-blocking)\"
sleep 0.5
echo \"\$ $BIN try $LOCK_FILE --tag demo\"
sleep 0.3
$BIN try $LOCK_FILE --tag demo
sleep 1

echo \"\"
echo \"# 2. Check lock status\"
sleep 0.5
echo \"\$ $BIN status $LOCK_FILE\"
sleep 0.3
$BIN status $LOCK_FILE || true
sleep 1.5

echo \"\"
echo \"# 3. Check status with JSON output\"
sleep 0.5
echo \"\$ $BIN status $LOCK_FILE --json\"
sleep 0.3
$BIN status $LOCK_FILE --json || true
sleep 1.5

echo \"\"
echo \"# 4. Release the lock\"
sleep 0.5
echo \"\$ $BIN release $LOCK_FILE\"
sleep 0.3
$BIN release $LOCK_FILE
sleep 1

echo \"\"
echo \"# 5. Verify lock is released\"
sleep 0.5
echo \"\$ $BIN status $LOCK_FILE\"
sleep 0.3
$BIN status $LOCK_FILE || true
sleep 1.5

echo \"\"
echo \"# 6. Demo stale lock recovery\"
sleep 0.5
echo \"# Creating a stale lock (old timestamp, dead PID)...\"
OLD_TS=\$(($(date +%s) - 7200))
echo \"pid=99999\" > $LOCK_FILE
echo \"timestamp=\$OLD_TS\" >> $LOCK_FILE
sleep 0.5
echo \"\$ cat $LOCK_FILE\"
cat $LOCK_FILE
sleep 1

echo \"\"
echo \"# Acquiring stale lock (--stale 3600 = 1 hour threshold)\"
sleep 0.5
echo \"\$ $BIN try $LOCK_FILE --stale 3600\"
sleep 0.3
$BIN try $LOCK_FILE --stale 3600
sleep 1.5

echo \"\"
echo \"# Done! File-based semaphore provides:\"
echo \"#   - Atomic locking (O_CREAT | O_EXCL)\"
echo \"#   - Stale lock detection\"
echo \"#   - Cross-platform support\"
sleep 3
'"

# Cleanup
rm -f "$LOCK_FILE"

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

# Convert to GIF (requires agg or svg-term)
if command -v agg &> /dev/null; then
  echo "Converting to GIF..."
  mkdir -p docs
  agg "$DEMO_FILE" docs/demo.gif --theme monokai --font-size 16
  echo "GIF saved to docs/demo.gif"
fi

echo "Demo creation complete!"
