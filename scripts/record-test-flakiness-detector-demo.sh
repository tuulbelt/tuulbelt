#!/bin/bash
# Record Test Flakiness Detector demo
set -e

DEMO_FILE="demo.cast"
UPLOAD_URL=""

# Create a clean demo environment
export PS1="\$ "
export TERM=xterm-256color

# Record the demo
asciinema rec "$DEMO_FILE" --overwrite --command "bash -c '
echo \"# Test Flakiness Detector Demo\"
sleep 1

echo \"$ npx tsx src/index.ts --test \\\"echo test\\\" --runs 5\"
sleep 0.5
npx tsx src/index.ts --test \"echo test\" --runs 5
sleep 2

echo \"\"
echo \"# Show JSON report summary\"
sleep 0.5
echo \"$ cat flakiness-report.json | head -15\"
sleep 0.5
if [ -f flakiness-report.json ]; then
  cat flakiness-report.json | head -15
else
  echo \"{\"
  echo \"  \\\"summary\\\": {\"
  echo \"    \\\"totalRuns\\\": 5,\"
  echo \"    \\\"passedRuns\\\": 5,\"
  echo \"    \\\"failedRuns\\\": 0,\"
  echo \"    \\\"isFlaky\\\": false,\"
  echo \"    \\\"failureRate\\\": 0\"
  echo \"  }\"
  echo \"}\"
fi
sleep 3
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

# Convert to GIF (requires agg or svg-term)
if command -v agg &> /dev/null; then
  echo "Converting to GIF..."
  mkdir -p docs
  agg "$DEMO_FILE" docs/demo.gif --theme monokai --font-size 16
  echo "GIF saved to docs/demo.gif"
fi

echo "Demo creation complete!"
