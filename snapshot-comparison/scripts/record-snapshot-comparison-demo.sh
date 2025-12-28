#!/bin/bash
# Record demo for snapshot-comparison
#
# Prerequisites:
#   - asciinema installed
#   - agg (asciinema gif generator) installed
#   - cargo build --release completed

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$TOOL_DIR"

# Build if needed
if [ ! -f "target/release/snapcmp" ]; then
    echo "Building snapshot-comparison..."
    cargo build --release --quiet
fi

# Create temp directory for demo
DEMO_DIR=$(mktemp -d)
trap "rm -rf $DEMO_DIR" EXIT
mkdir -p "$DEMO_DIR/snapshots"

echo "Recording demo..."

asciinema rec "demo.cast" --overwrite --title "Snapshot Comparison - Tuulbelt" --command "bash -c '
cd $DEMO_DIR

echo \"=== Snapshot Comparison Demo ===\"
echo \"\"
sleep 0.5

echo \"# Create a snapshot from command output\"
echo \"echo \\\"Hello, World!\\\" | snapcmp create hello -d snapshots\"
echo \"Hello, World!\" | $TOOL_DIR/target/release/snapcmp create hello -d snapshots
sleep 1

echo \"\"
echo \"# Check matching content\"
echo \"echo \\\"Hello, World!\\\" | snapcmp check hello -d snapshots\"
echo \"Hello, World!\" | $TOOL_DIR/target/release/snapcmp check hello -d snapshots
sleep 1

echo \"\"
echo \"# Check different content (mismatch)\"
echo \"echo \\\"Hello, Universe!\\\" | snapcmp check hello -d snapshots\"
echo \"Hello, Universe!\" | $TOOL_DIR/target/release/snapcmp check hello -d snapshots 2>&1 || true
sleep 1.5

echo \"\"
echo \"# Update the snapshot\"
echo \"echo \\\"Hello, Universe!\\\" | snapcmp update hello -d snapshots\"
echo \"Hello, Universe!\" | $TOOL_DIR/target/release/snapcmp update hello -d snapshots
sleep 1

echo \"\"
echo \"# List all snapshots\"
echo \"snapcmp list -d snapshots\"
$TOOL_DIR/target/release/snapcmp list -d snapshots
sleep 1

echo \"\"
echo \"# Create a JSON snapshot\"
echo \"echo \\\"{\\\\\"name\\\\\": \\\\\"test\\\\\"}\\\" | snapcmp create config -t json -d snapshots\"
echo \"{\\\"name\\\": \\\"test\\\"}\" | $TOOL_DIR/target/release/snapcmp create config -t json -d snapshots
sleep 1

echo \"\"
echo \"# List snapshots again\"
echo \"snapcmp list -d snapshots\"
$TOOL_DIR/target/release/snapcmp list -d snapshots
sleep 1

echo \"\"
echo \"# Clean orphaned snapshots (dry-run)\"
echo \"snapcmp clean --keep hello --dry-run -d snapshots\"
$TOOL_DIR/target/release/snapcmp clean --keep hello --dry-run -d snapshots
sleep 1

echo \"\"
echo \"# Delete a snapshot\"
echo \"snapcmp delete config -d snapshots\"
$TOOL_DIR/target/release/snapcmp delete config -d snapshots
sleep 1

echo \"\"
echo \"=== Demo Complete ===\"
sleep 0.5
'"

echo "Demo recorded to demo.cast"

# Generate GIF if agg is available
if command -v agg &> /dev/null; then
    echo "Generating demo.gif..."
    agg demo.cast demo.gif --cols 100 --rows 30 --font-size 14

    # Copy to docs if directory exists
    if [ -d "../docs/public/snapshot-comparison" ]; then
        cp demo.gif ../docs/public/snapshot-comparison/demo.gif
        echo "Copied to docs/public/snapshot-comparison/demo.gif"
    fi

    echo "GIF generated: demo.gif"
else
    echo "Note: Install 'agg' to generate demo.gif"
fi

echo "Done!"
