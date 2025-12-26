# CLI Usage

Complete command-line reference for `output-diff`.

## Synopsis

```bash
output-diff [OPTIONS] <FILE1> <FILE2>
```

## Arguments

### `<FILE1>` (Required)

First file to compare.

**Type:** File path (absolute or relative)

**Example:**
```bash
output-diff /path/to/old.txt new.txt
```

### `<FILE2>` (Required)

Second file to compare.

**Type:** File path (absolute or relative)

**Example:**
```bash
output-diff file1.json file2.json
```

## Options

### Format Selection

#### `-f, --format <FORMAT>`

Output format for diff results.

**Values:**
- `unified` (default) - Traditional diff format
- `json` - Machine-parseable JSON report
- `side-by-side` - Visual column comparison
- `compact` - Minimal output (changes only)

**Examples:**
```bash
# Unified diff (default)
output-diff file1.txt file2.txt

# JSON report
output-diff --format json data1.json data2.json

# Side-by-side
output-diff -f side-by-side old.txt new.txt

# Compact
output-diff --format compact file1.txt file2.txt
```

### Type Detection

#### `-t, --type <TYPE>`

Force file type interpretation (overrides auto-detection).

**Values:**
- `auto` (default) - Detect from extension and content
- `text` - Line-based text diff
- `json` - Structural JSON diff
- `binary` - Byte-by-byte binary diff

**Use Cases:**
- Compare `.json` files as plain text
- Force binary comparison for unknown extensions
- Override misdetection

**Examples:**
```bash
# Treat JSON as plain text
output-diff --type text config.json config2.json

# Force binary mode
output-diff --type binary file1.dat file2.dat

# Auto-detect (default)
output-diff --type auto file1.txt file2.txt
```

### Context Control

#### `-c, --context <LINES>`

Number of unchanged lines to show around changes (text diffs only).

**Default:** Show all context (equivalent to `usize::MAX`)

**Range:** `0` to unlimited

**Examples:**
```bash
# 3 lines of context
output-diff --context 3 file1.txt file2.txt

# No context (changes only)
output-diff -c 0 file1.txt file2.txt

# 10 lines of context
output-diff --context 10 large1.txt large2.txt

# All context (default)
output-diff file1.txt file2.txt
```

### Color Control

#### `--color <WHEN>`

Control ANSI color output (unified format only).

**Values:**
- `auto` (default) - Color if stdout is a terminal
- `always` - Always use ANSI colors
- `never` - No colors (plain text)

**Examples:**
```bash
# Auto-detect terminal
output-diff file1.txt file2.txt

# Force color for piping
output-diff --color always file1.txt file2.txt | less -R

# Disable color for file output
output-diff --color never file1.txt file2.txt > diff.txt
```

### Output Control

#### `-q, --quiet`

Suppress all output; only set exit code.

**Use Case:** Shell scripts checking file equivalence without showing diff.

**Examples:**
```bash
# Check if files differ
if output-diff --quiet file1.txt file2.txt; then
    echo "Identical"
else
    echo "Different"
fi

# One-liner
output-diff -q expected.txt actual.txt && echo "✅ Pass" || echo "❌ Fail"
```

#### `-o, --output <FILE>`

Write diff output to file instead of stdout.

**Examples:**
```bash
# Save unified diff
output-diff --output changes.diff file1.txt file2.txt

# Save JSON report
output-diff -f json -o report.json data1.json data2.json

# Save side-by-side
output-diff --format side-by-side --output review.txt old.txt new.txt
```

#### `-v, --verbose`

Enable verbose debug output (written to stderr).

**Output Includes:**
- File sizes
- Detected file types
- Processing time
- Internal decisions (auto-detection logic)

**Examples:**
```bash
# Verbose mode
output-diff --verbose file1.txt file2.txt
# Output to stderr:
# [DEBUG] File 1 type: Text
# [DEBUG] File 2 type: Text
# [DEBUG] Using unified diff format

# Identical files with verbose
output-diff -v identical1.txt identical2.txt
# [INFO] Files are identical
```

### Utility Options

#### `-h, --help`

Show help message and exit.

**Example:**
```bash
output-diff --help
```

#### `-V, --version`

Show version information and exit.

**Example:**
```bash
output-diff --version
# Output: output-diff 0.1.0
```

## Exit Codes

| Code | Meaning | Use Case |
|------|---------|----------|
| `0` | Files are identical | Success in testing |
| `1` | Files differ | Expected in diffing |
| `2` | Error occurred | Invalid args, file not found, parse error |

**Examples:**
```bash
# Check exit code
output-diff file1.txt file2.txt
echo $?  # 0, 1, or 2

# Use in conditional
output-diff --quiet expected.txt actual.txt
if [ $? -eq 0 ]; then
    echo "Test passed"
elif [ $? -eq 1 ]; then
    echo "Test failed - files differ"
else
    echo "Error running diff"
fi
```

## Option Combinations

### Common Combinations

```bash
# JSON report with verbose logging
output-diff --format json --verbose data1.json data2.json 2> debug.log

# Compact diff with no context
output-diff --format compact --context 0 file1.txt file2.txt

# Side-by-side with color for review
output-diff --format side-by-side --color always file1.txt file2.txt | less -R

# Silent check, show diff only on failure
output-diff --quiet file1.txt file2.txt || output-diff file1.txt file2.txt

# Force text mode with JSON output format
output-diff --type text --format json config1.json config2.json

# Save colored diff to file
output-diff --color always file1.txt file2.txt > /dev/null  # Can't save colors to file
# Better:
output-diff --color never --output diff.txt file1.txt file2.txt
```

### Testing Workflows

```bash
# Test suite validation
#!/bin/bash
for test in tests/*.expected; do
    actual="${test%.expected}.actual"
    if ! output-diff --quiet "$test" "$actual"; then
        echo "❌ Failed: $test"
        output-diff --format compact "$test" "$actual"
        exit 1
    fi
done
echo "✅ All tests passed"
```

### CI/CD Integration

```bash
# Generate diff report in CI
output-diff \
    --format json \
    --output diff-report.json \
    baseline.json \
    current.json

# Parse report
cat diff-report.json | jq '.summary.total_changes'

# Fail if differences found
if [ $? -eq 1 ]; then
    echo "::error::Baseline differs from current"
    exit 1
fi
```

## Error Messages

### File Not Found

```
Error reading /tmp/missing.txt: No such file or directory
Exit code: 2
```

**Solution:** Check file paths are correct.

### Invalid UTF-8

```
Error: File 1 is not valid UTF-8
Exit code: 2
```

**Solution:** Use `--type binary` for non-UTF-8 files.

### JSON Parse Error

```
Error parsing JSON: expected value at line 1 column 5
Exit code: 2
```

**Solution:** Fix JSON syntax or use `--type text` to diff as plain text.

### Invalid Arguments

```
Error: Expected exactly 2 file arguments, got 1
Use --help for usage information
Exit code: 2
```

**Solution:** Provide both file paths.

```
Error: Invalid format 'xml'. Use: unified, json, side-by-side, or compact
Exit code: 2
```

**Solution:** Use valid format name.

## Tips & Tricks

### Diff Directories (Workaround)

```bash
# Output-diff compares files, not directories
# Use find + while loop for directory comparison:
find dir1 -type f | while read -r file; do
    rel_path="${file#dir1/}"
    if [ -f "dir2/$rel_path" ]; then
        output-diff --quiet "$file" "dir2/$rel_path" || echo "Differs: $rel_path"
    else
        echo "Missing in dir2: $rel_path"
    fi
done
```

### Ignore Whitespace (Workaround)

```bash
# No built-in whitespace ignore, but can preprocess:
diff <(tr -s ' ' < file1.txt) <(tr -s ' ' < file2.txt)
```

### Large File Optimization

```bash
# Use context limit for large files
output-diff --context 5 large1.txt large2.txt

# Or compact format
output-diff --format compact large1.txt large2.txt
```

## See Also

- [Getting Started](/tools/output-diffing-utility/getting-started) - Installation and quick start
- [Output Formats](/tools/output-diffing-utility/output-formats) - Format specifications
- [Examples](/tools/output-diffing-utility/examples) - Real-world examples
- [SPEC.md](https://github.com/tuulbelt/tuulbelt/blob/main/output-diffing-utility/SPEC.md) - Technical specification
