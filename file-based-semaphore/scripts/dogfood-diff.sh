#!/bin/bash
# Dogfooding: Output Diffing Utility integration
# HIGH-VALUE: Proves test outputs deterministic

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
DIFF_DIR="$TOOL_DIR/../output-diffing-utility"
TEMP_DIR="/tmp/semaphore-diff-$$"

echo "🔬 Validating Semaphore output determinism"
echo ""

[ ! -d "$DIFF_DIR" ] || [ ! -f "$DIFF_DIR/target/release/output-diff" ] && echo "⚠️  Using standard diff" && USE_STANDARD=1

mkdir -p "$TEMP_DIR"
cd "$TOOL_DIR"

cargo test > "$TEMP_DIR/run1.txt" 2>&1
cargo test > "$TEMP_DIR/run2.txt" 2>&1

if [ "$USE_STANDARD" = "1" ]; then
  diff "$TEMP_DIR/run1.txt" "$TEMP_DIR/run2.txt" && echo "✅ IDENTICAL!" || echo "⚠️  Differences"
else
  cd "$DIFF_DIR"
  ./target/release/output-diff "$TEMP_DIR/run1.txt" "$TEMP_DIR/run2.txt" && echo "✅ IDENTICAL!" || echo "⚠️  Differences"
fi

rm -rf "$TEMP_DIR"
