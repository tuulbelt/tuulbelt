#!/bin/bash
# Record demo for test-port-resolver (portres)
#
# Prerequisites:
#   - asciinema installed
#   - agg installed (for GIF generation)
#   - ASCIINEMA_INSTALL_ID set (for upload)
#
# Usage:
#   ./scripts/record-test-port-resolver-demo.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TOOL_DIR="$SCRIPT_DIR/../test-port-resolver"

cd "$TOOL_DIR"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    npm ci --silent
fi

echo "Recording test-port-resolver demo..."

# Record the demo
asciinema rec "demo.cast" --overwrite --title "Test Port Resolver (portres) - Tuulbelt" --command "bash -c '
echo \"Test Port Resolver / portres - Demo\"
echo \"=====================================\"
echo
sleep 0.5

echo \"1. Get an available port:\"
sleep 0.3
echo \"$ portres get\"
npx tsx src/index.ts get -d /tmp/portres-demo
sleep 1

echo
echo \"2. Get multiple ports at once:\"
sleep 0.3
echo \"$ portres get -n 3\"
npx tsx src/index.ts get -n 3 -d /tmp/portres-demo
sleep 1

echo
echo \"3. Get port with tag:\"
sleep 0.3
echo \"$ portres get -t mytest --json\"
npx tsx src/index.ts get -t mytest --json -d /tmp/portres-demo
sleep 1

echo
echo \"4. List all allocations:\"
sleep 0.3
echo \"$ portres list\"
npx tsx src/index.ts list -d /tmp/portres-demo
sleep 1

echo
echo \"5. Show registry status:\"
sleep 0.3
echo \"$ portres status\"
npx tsx src/index.ts status -d /tmp/portres-demo
sleep 1

echo
echo \"6. Release all ports:\"
sleep 0.3
echo \"$ portres release-all\"
npx tsx src/index.ts release-all -d /tmp/portres-demo
sleep 1

echo
echo \"7. Verify registry is empty:\"
sleep 0.3
echo \"$ portres list\"
npx tsx src/index.ts list -d /tmp/portres-demo
sleep 1

echo
echo \"=====================================\"
echo \"Zero dependencies. Concurrent safe.\"
echo \"=====================================\"
sleep 1
'"

# Upload to asciinema.org
ASCIINEMA_URL=$(asciinema upload demo.cast 2>&1 | grep -oP 'https://asciinema.org/a/[a-zA-Z0-9]+' | head -1)

if [ -n "$ASCIINEMA_URL" ]; then
    echo "$ASCIINEMA_URL" > demo-url.txt
    echo "Uploaded to: $ASCIINEMA_URL"
else
    echo "Warning: Could not extract asciinema URL"
fi

# Generate GIF using agg
if command -v agg &> /dev/null; then
    echo "Generating demo.gif..."
    mkdir -p docs
    agg demo.cast docs/demo.gif --cols 80 --rows 24 --speed 1.5
    echo "Generated docs/demo.gif"
else
    echo "Warning: agg not found, skipping GIF generation"
fi

# Clean up temp registry
rm -rf /tmp/portres-demo

echo "Demo recording complete!"
