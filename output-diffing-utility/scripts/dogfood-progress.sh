#!/bin/bash
#
# Dogfooding: Demonstrate CLI Progress Reporting integration
#
# This script shows how output-diffing-utility can compose with cli-progress-reporting
# to provide real-time progress updates when diffing large files.
#
# Composability Demo:
#   - Rust tool (output-diff) → TypeScript tool (cli-progress) → Back to user
#   - No tight coupling, just CLI composition
#   - Graceful fallback if progress tool not available
#
# Usage:
#   ./scripts/dogfood-progress.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOL_DIR="$(dirname "$SCRIPT_DIR")"
PROGRESS_DIR="$TOOL_DIR/../cli-progress-reporting"
TEMP_DIR="/tmp/output-diff-progress-demo-$$"

echo "🔬 Dogfooding: CLI Progress Reporting + Output Diffing Utility"
echo ""
echo "This demonstrates cross-language tool composition:"
echo "  Rust (output-diff) → TypeScript (cli-progress) → User"
echo ""

# Verify CLI Progress Reporting exists
if [ ! -d "$PROGRESS_DIR" ]; then
    echo "⚠️  CLI Progress Reporting not found at: $PROGRESS_DIR"
    echo ""
    echo "This tool works standalone, but progress tracking is unavailable."
    echo "Running diff without progress updates..."
    echo ""

    # Graceful fallback: still works without progress tool
    mkdir -p "$TEMP_DIR"
    echo '{"name": "Alice", "age": 30, "city": "New York"}' > "$TEMP_DIR/v1.json"
    echo '{"name": "Bob", "age": 25, "city": "San Francisco"}' > "$TEMP_DIR/v2.json"

    cd "$TOOL_DIR"
    cargo run --release --quiet --bin odiff -- "$TEMP_DIR/v1.json" "$TEMP_DIR/v2.json"

    rm -rf "$TEMP_DIR"
    exit 0
fi

# Verify npm dependencies are installed
if [ ! -d "$PROGRESS_DIR/node_modules" ]; then
    echo "📦 Installing CLI Progress Reporting dependencies..."
    (cd "$PROGRESS_DIR" && npm install --silent)
fi

echo "✅ CLI Progress Reporting available"
echo ""

# Create test files
mkdir -p "$TEMP_DIR"
echo "📝 Creating test files..."

# Create two large-ish JSON files to diff
cat > "$TEMP_DIR/old.json" <<'EOF'
{
  "users": [
    {"id": 1, "name": "Alice", "age": 30, "email": "alice@example.com"},
    {"id": 2, "name": "Bob", "age": 25, "email": "bob@example.com"},
    {"id": 3, "name": "Charlie", "age": 35, "email": "charlie@example.com"}
  ],
  "metadata": {
    "version": "1.0",
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
EOF

cat > "$TEMP_DIR/new.json" <<'EOF'
{
  "users": [
    {"id": 1, "name": "Alice", "age": 31, "email": "alice@example.com"},
    {"id": 2, "name": "Bob", "age": 25, "email": "bob@newdomain.com"},
    {"id": 3, "name": "Charlie", "age": 35, "email": "charlie@example.com"},
    {"id": 4, "name": "David", "age": 28, "email": "david@example.com"}
  ],
  "metadata": {
    "version": "2.0",
    "timestamp": "2024-06-15T12:30:00Z"
  }
}
EOF

echo "✅ Test files created"
echo ""

# Initialize progress tracker
echo "🔧 Initializing progress tracker..."
PROGRESS_ID="diff-demo-$$"

cd "$PROGRESS_DIR"
npx prog init \
    --total 100 \
    --id "$PROGRESS_ID" \
    --message "Comparing JSON files" > /dev/null

echo "✅ Progress tracker initialized (ID: $PROGRESS_ID)"
echo ""

# Simulate progress updates during diff
echo "🔄 Running diff with progress updates..."
echo ""

# Update: Starting
npx prog set \
    --current 0 \
    --total 100 \
    --id "$PROGRESS_ID" \
    --message "Reading old.json" > /dev/null

# Update: Reading files
npx prog set \
    --current 25 \
    --total 100 \
    --id "$PROGRESS_ID" \
    --message "Reading new.json" > /dev/null

# Update: Parsing
npx prog set \
    --current 50 \
    --total 100 \
    --id "$PROGRESS_ID" \
    --message "Parsing JSON structures" > /dev/null

# Update: Computing diff
npx prog set \
    --current 75 \
    --total 100 \
    --id "$PROGRESS_ID" \
    --message "Computing structural diff" > /dev/null

# Run actual diff
cd "$TOOL_DIR"
echo "📊 Diff Results:"
echo "---"
cargo run --release --quiet --bin odiff -- "$TEMP_DIR/old.json" "$TEMP_DIR/new.json"
echo "---"
echo ""

# Update: Complete
cd "$PROGRESS_DIR"
npx prog finish \
    --id "$PROGRESS_ID" \
    --message "Diff complete" > /dev/null

echo "✅ Progress tracker finished"
echo ""

# Show final progress state
echo "📊 Final Progress State:"
npx prog get --id "$PROGRESS_ID"
echo ""

# Cleanup
rm -rf "$TEMP_DIR"

echo "✨ Composition Demo Complete!"
echo ""
echo "💡 Key Takeaways:"
echo "   - Rust tool composed with TypeScript tool via CLI"
echo "   - No runtime dependencies or tight coupling"
echo "   - Progress updates work across language boundaries"
echo "   - Graceful fallback if progress tool unavailable"
echo "   - Each tool remains independently usable"
echo ""
echo "🔗 See DOGFOODING_STRATEGY.md for implementation details"
