# Getting Started

Get up and running with Output Diffing Utility in minutes.

## Installation

### From Source (Recommended)

```bash
# Clone the monorepo
git clone https://github.com/tuulbelt/output-diffing-utility.git
cd output-diffing-utility

# Build release binary
cargo build --release

# Binary location
./target/release/odiff
```

### Add to PATH

```bash
# Option 1: Symlink to /usr/local/bin
sudo ln -s "$(pwd)/target/release/odiff" /usr/local/bin/output-diff

# Option 2: Copy to ~/.local/bin
mkdir -p ~/.local/bin
cp target/release/odiff ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc or ~/.zshrc

# Verify installation
odiff --version
```

## Quick Test

### Compare Two Text Files

```bash
# Create test files
echo -e "line 1\nline 2\nline 3" > file1.txt
echo -e "line 1\nline 2 modified\nline 3" > file2.txt

# Run diff
odiff file1.txt file2.txt
```

**Expected output:**
```diff
--- file1.txt
+++ file2.txt
@@ -1,3 +1,3 @@
 line 1
-line 2
+line 2 modified
 line 3
```

**Exit code:** `1` (files differ)

### Compare JSON Files

```bash
# Create JSON test files
echo '{"name": "Alice", "age": 30}' > data1.json
echo '{"name": "Bob", "age": 30}' > data2.json

# Run JSON diff
odiff data1.json data2.json
```

**Expected output:**
```
JSON diff for 'data1.json' vs 'data2.json':
  Modified at 'name': "Alice" → "Bob"

1 change (1 modification)
```

### Try Different Formats

```bash
# JSON report format
odiff --format json data1.json data2.json

# Side-by-side format
odiff --format side-by-side file1.txt file2.txt

# Compact format
odiff --format compact file1.txt file2.txt
```

## Basic Usage Patterns

### Check if Files Differ (Shell Scripts)

```bash
if odiff --quiet expected.txt actual.txt; then
    echo "✅ Test passed"
else
    echo "❌ Test failed - files differ"
    odiff expected.txt actual.txt  # Show diff
    exit 1
fi
```

### Save Diff to File

```bash
# Save unified diff
odiff file1.txt file2.txt > changes.diff

# Or use --output flag
odiff --output changes.diff file1.txt file2.txt

# Save JSON report
odiff --format json --output report.json data1.json data2.json
```

### Colored Output for Review

```bash
# Force color even when piping
odiff --color always file1.txt file2.txt | less -R

# Disable color for file output
odiff --color never file1.txt file2.txt > changes.txt
```

### Limit Context Lines

```bash
# Show only 1 line before/after changes
odiff --context 1 large1.txt large2.txt

# Show no context (only changes)
odiff --context 0 file1.txt file2.txt

# Show all context (default)
odiff file1.txt file2.txt
```

## Common Workflows

### Workflow 1: Testing Expected Output

```bash
#!/bin/bash
# test-output.sh

# Run command that produces output
./my-tool --config config.json > actual.txt

# Compare with expected
if odiff --quiet expected.txt actual.txt; then
    echo "✅ Output matches expected"
    exit 0
else
    echo "❌ Output differs from expected:"
    odiff expected.txt actual.txt
    exit 1
fi
```

### Workflow 2: Reviewing Config Changes

```bash
# Compare staging vs production config
odiff \
    --format side-by-side \
    configs/staging.json \
    configs/production.json
```

### Workflow 3: Validating Data Exports

```bash
# Compare database exports
odiff \
    --format json \
    --output validation-report.json \
    export-2025-01-01.json \
    export-2025-01-02.json

# Check if changes occurred
if [ $? -eq 1 ]; then
    echo "Data changed between exports"
    cat validation-report.json | jq '.summary'
fi
```

## Troubleshooting

### Issue: "File is not valid UTF-8"

**Cause:** File contains binary data or invalid UTF-8 sequences.

**Solution:** Force binary mode:
```bash
odiff --type binary file1.dat file2.dat
```

### Issue: "Error parsing JSON"

**Cause:** JSON syntax error in input file.

**Solution:** Force text mode to see line-level diff:
```bash
odiff --type text invalid1.json invalid2.json
```

### Issue: "No differences shown for identical files"

**Cause:** Files are identical (exit code 0).

**Solution:** Check exit code or use `--verbose`:
```bash
odiff --verbose file1.txt file2.txt
# Output: "[INFO] Files are identical"
```

### Issue: "Diff too large to review"

**Cause:** Many changes make output overwhelming.

**Solution:** Use compact format or limit context:
```bash
odiff --format compact large1.txt large2.txt
# or
odiff --context 0 large1.txt large2.txt
```

## Next Steps

- [CLI Usage](/tools/output-diffing-utility/cli-usage) - Complete option reference
- [Library Usage](/tools/output-diffing-utility/library-usage) - Integrate in Rust projects
- [Examples](/tools/output-diffing-utility/examples) - Real-world examples
- [Output Formats](/tools/output-diffing-utility/output-formats) - Format details

## Need Help?

- Report issues: [GitHub Issues](https://github.com/tuulbelt/tuulbelt/issues)
- Read the spec: [SPEC.md](https://github.com/tuulbelt/tuulbelt/blob/main/output-diffing-utility/SPEC.md)
