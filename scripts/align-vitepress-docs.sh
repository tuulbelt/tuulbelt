#!/bin/bash
# Align VitePress documentation with standalone repository URLs
# Migration cleanup: monorepo → standalone repos

set -e

TOOLS=(
  "test-flakiness-detector"
  "cli-progress-reporting"
  "cross-platform-path-normalizer"
  "file-based-semaphore"
  "output-diffing-utility"
  "structured-error-handler"
  "config-file-merger"
  "snapshot-comparison"
  "file-based-semaphore-ts"
  "port-resolver"
)

echo "========================================="
echo "VitePress Documentation Alignment Script"
echo "========================================="
echo ""
echo "This script updates VitePress docs to use standalone repository URLs"
echo "instead of monorepo references."
echo ""

TOTAL_UPDATES=0

for tool in "${TOOLS[@]}"; do
  VITEPRESS_DOC="docs/tools/$tool/index.md"

  if [ ! -f "$VITEPRESS_DOC" ]; then
    echo "❌ ERROR: $VITEPRESS_DOC not found"
    continue
  fi

  echo "Processing: $tool"

  UPDATES=0

  # 1. Fix Repository link
  # FROM: [tuulbelt/tuulbelt/<tool>](https://github.com/tuulbelt/tuulbelt/tree/main/<tool>)
  # TO:   [tuulbelt/<tool>](https://github.com/tuulbelt/<tool>)
  if grep -q "tuulbelt/tuulbelt/$tool](https://github.com/tuulbelt/tuulbelt/tree/main/$tool)" "$VITEPRESS_DOC"; then
    sed -i "s|tuulbelt/tuulbelt/$tool](https://github.com/tuulbelt/tuulbelt/tree/main/$tool)|tuulbelt/$tool](https://github.com/tuulbelt/$tool)|g" "$VITEPRESS_DOC"
    echo "  ✓ Updated repository link"
    ((UPDATES++))
  fi

  # 2. Fix clone command URL
  # FROM: git clone https://github.com/tuulbelt/tuulbelt.git
  # TO:   git clone https://github.com/tuulbelt/<tool>.git
  if grep -q "git clone https://github.com/tuulbelt/tuulbelt.git" "$VITEPRESS_DOC"; then
    sed -i "s|git clone https://github.com/tuulbelt/tuulbelt.git|git clone https://github.com/tuulbelt/$tool.git|g" "$VITEPRESS_DOC"
    echo "  ✓ Updated clone URL (all occurrences)"
    ((UPDATES++))
  fi

  # 3. Fix clone directory path
  # FROM: cd tuulbelt/<tool>
  # TO:   cd <tool>
  if grep -q "cd tuulbelt/$tool" "$VITEPRESS_DOC"; then
    sed -i "s|cd tuulbelt/$tool|cd $tool|g" "$VITEPRESS_DOC"
    echo "  ✓ Updated clone directory path"
    ((UPDATES++))
  fi

  # 4. Fix StackBlitz links (preserve query parameters)
  # FROM: stackblitz.com/github/tuulbelt/tuulbelt/tree/main/<tool>
  # TO:   stackblitz.com/github/tuulbelt/<tool>
  if grep -q "stackblitz.com/github/tuulbelt/tuulbelt/tree/main/$tool" "$VITEPRESS_DOC"; then
    sed -i "s|stackblitz.com/github/tuulbelt/tuulbelt/tree/main/$tool|stackblitz.com/github/tuulbelt/$tool|g" "$VITEPRESS_DOC"
    echo "  ✓ Updated StackBlitz link"
    ((UPDATES++))
  fi

  if [ $UPDATES -eq 0 ]; then
    echo "  ℹ No updates needed (already aligned)"
  else
    echo "  📝 Total changes: $UPDATES"
    TOTAL_UPDATES=$((TOTAL_UPDATES + UPDATES))
  fi

  echo ""
done

echo "========================================="
echo "Summary"
echo "========================================="
echo "Tools processed: ${#TOOLS[@]}"
echo "Total updates: $TOTAL_UPDATES"
echo ""

if [ $TOTAL_UPDATES -gt 0 ]; then
  echo "✅ Alignment complete! Please review changes with:"
  echo "   git diff docs/tools/*/index.md"
  echo ""
  echo "Next steps:"
  echo "1. Manually verify edge cases (snapshot-comparison, duplicates)"
  echo "2. Test VitePress build: npm run docs:build"
  echo "3. Commit changes: git add docs/ && git commit -m 'docs: align VitePress with standalone repo URLs'"
else
  echo "ℹ All documentation already aligned."
fi
