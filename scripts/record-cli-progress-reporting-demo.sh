#!/bin/bash
# Record CLI Progress Reporting demo
set -e

DEMO_FILE="demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --command "bash -c '
echo \"# CLI Progress Reporting Demo\"
sleep 1

echo \"$ npx tsx src/index.ts init --total 100 --message \\\"Processing files\\\"\"
sleep 0.5
npx tsx src/index.ts init --total 100 --message "Processing files"
sleep 1

echo \"\"
echo \"$ # Update progress in a loop\"
sleep 0.5
for i in {1..5}; do
  npx tsx src/index.ts increment --amount 20
  sleep 0.8
done
sleep 1

echo \"\"
echo \"$ npx tsx src/index.ts finish --message \\\"Complete!\\\"\"
sleep 0.5
npx tsx src/index.ts finish --message "Complete!"
sleep 2
'"

echo "Demo recording saved to $DEMO_FILE"

# Upload to asciinema.org if install ID is provided
if [ -n "$ASCIINEMA_INSTALL_ID" ]; then
  echo "Uploading to asciinema.org..."
  UPLOAD_URL=$(asciinema upload "$DEMO_FILE" 2>&1 | grep -oP 'https://asciinema.org/a/\K[a-zA-Z0-9]+' || echo "")

  if [ -n "$UPLOAD_URL" ]; then
    echo "Demo uploaded: https://asciinema.org/a/$UPLOAD_URL"
    echo "https://asciinema.org/a/$UPLOAD_URL" > demo-url.txt
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
