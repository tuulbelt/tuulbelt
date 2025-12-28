#!/bin/bash
# Record File-Based Semaphore (TypeScript) demo
set -e

TOOL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../file-based-semaphore-ts" && pwd)"
DEMO_FILE="$TOOL_DIR/demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Build/install first
echo "Installing dependencies..."
cd "$TOOL_DIR"
npm install 2>/dev/null
npm link 2>/dev/null || true

# Create temp lock file path
LOCK_FILE="/tmp/ts-demo.lock"
rm -f "$LOCK_FILE"

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --title "File-Based Semaphore (TypeScript) / semats - Tuulbelt" --command "bash -c '
echo \"# File-Based Semaphore (TypeScript) Demo\"
sleep 1

echo \"\"
echo \"# 1. Acquire a lock (non-blocking)\"
sleep 0.5
echo \"\$ semats try-acquire $LOCK_FILE --tag demo\"
sleep 0.3
cd \"$TOOL_DIR\"
npx tsx src/index.ts try-acquire $LOCK_FILE --tag demo
sleep 1

echo \"\"
echo \"# 2. Check lock status\"
sleep 0.5
echo \"\$ semats status $LOCK_FILE\"
sleep 0.3
npx tsx src/index.ts status $LOCK_FILE || true
sleep 1.5

echo \"\"
echo \"# 3. Check status with JSON output\"
sleep 0.5
echo \"\$ semats status $LOCK_FILE --json\"
sleep 0.3
npx tsx src/index.ts status $LOCK_FILE --json || true
sleep 1.5

echo \"\"
echo \"# 4. Release the lock\"
sleep 0.5
echo \"\$ semats release $LOCK_FILE\"
sleep 0.3
npx tsx src/index.ts release $LOCK_FILE
sleep 1

echo \"\"
echo \"# 5. Verify lock is released\"
sleep 0.5
echo \"\$ semats status $LOCK_FILE\"
sleep 0.3
npx tsx src/index.ts status $LOCK_FILE || true
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
echo \"# Clean stale lock\"
sleep 0.5
echo \"\$ semats clean $LOCK_FILE\"
sleep 0.3
npx tsx src/index.ts clean $LOCK_FILE
sleep 1.5

echo \"\"
echo \"# 7. Show cross-language compatibility\"
sleep 0.5
echo \"# TypeScript semats uses same format as Rust sema!\"
sleep 0.5
echo \"\$ semats try-acquire $LOCK_FILE --tag typescript-process\"
npx tsx src/index.ts try-acquire $LOCK_FILE --tag typescript-process
sleep 1
echo \"\"
echo \"\$ cat $LOCK_FILE\"
cat $LOCK_FILE
sleep 1.5

echo \"\"
echo \"# Done! TypeScript file-based semaphore with the semats command.\"
sleep 2
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
