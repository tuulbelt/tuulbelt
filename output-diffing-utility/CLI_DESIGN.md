# CLI Interface Design

Design specification for the output-diffing-utility command-line interface.

## Command Signature

```bash
output-diff [OPTIONS] <FILE1> <FILE2>
```

---

## Arguments

### Positional Arguments

- `<FILE1>` - First file to compare (required)
- `<FILE2>` - Second file to compare (required)

**Examples:**
```bash
output-diff file1.json file2.json
output-diff /path/to/old.txt /path/to/new.txt
output-diff data.bin updated-data.bin
```

---

## Options

### Format Selection

**`-f, --format <FORMAT>`**

Specify diff output format.

**Values:**
- `unified` (default) - Unified diff format (like `diff -u`)
- `json` - Structured JSON report
- `side-by-side` - Side-by-side comparison
- `compact` - Minimal output showing only changes

**Examples:**
```bash
output-diff --format unified file1.txt file2.txt
output-diff --format json data1.json data2.json
output-diff -f side-by-side old.txt new.txt
```

---

### Type Detection

**`-t, --type <TYPE>`**

Force file type interpretation (overrides auto-detection).

**Values:**
- `text` - Line-based text diff
- `json` - Structural JSON diff
- `binary` - Byte-by-byte binary diff
- `auto` (default) - Detect from extension/content

**Examples:**
```bash
output-diff --type json file.txt file2.txt  # Treat .txt as JSON
output-diff --type text data.json data2.json  # Line-based, not structural
output-diff -t binary file1 file2  # Force binary comparison
```

---

### Context Control

**`-c, --context <LINES>`**

Number of context lines to show around differences (text diffs only).

**Default:** 3 lines

**Examples:**
```bash
output-diff --context 5 file1.txt file2.txt  # 5 lines of context
output-diff -c 0 file1.txt file2.txt  # No context, only changes
```

---

### Output Options

**`--color <WHEN>`**

Control colored output.

**Values:**
- `auto` (default) - Color if terminal supports it
- `always` - Always use ANSI colors
- `never` - No colors (plain text)

**Examples:**
```bash
output-diff --color always file1.txt file2.txt
output-diff --color never file1.txt file2.txt > diff.txt
```

**`-q, --quiet`**

Suppress output, only set exit code.

**Use case:** Check if files differ without seeing the diff.

**Examples:**
```bash
if output-diff --quiet file1.txt file2.txt; then
    echo "Files are identical"
else
    echo "Files differ"
fi
```

**`-o, --output <FILE>`**

Write diff output to file instead of stdout.

**Examples:**
```bash
output-diff -o changes.diff file1.txt file2.txt
output-diff --output report.json --format json data1.json data2.json
```

---

### Performance Options

**`--progress-id <ID>`**

Enable progress reporting for large files using cli-progress-reporting.

**Dogfooding:** Integrates with CLI Progress Reporting tool.

**Examples:**
```bash
output-diff --progress-id my-diff large1.json large2.json
# Shows: [45%] 4.5MB/10MB - Comparing JSON structures (12s)
```

**`--cache`**

Enable diff result caching using file-based-semaphore.

**Dogfooding:** Uses File-Based Semaphore for concurrent cache access.

**Examples:**
```bash
output-diff --cache file1.json file2.json
# Second run: instant result from cache
```

---

### Utility Options

**`-h, --help`**

Show help message and exit.

**`-V, --version`**

Show version information and exit.

**`-v, --verbose`**

Enable verbose output (debug information).

**Examples:**
```bash
output-diff --verbose file1.json file2.json
# Output includes: file sizes, detection logic, timing info
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Files are identical |
| 1 | Files differ |
| 2 | Error occurred (file not found, parse error, etc.) |

**Examples:**
```bash
output-diff file1.txt file2.txt
echo $?  # 0 if identical, 1 if different, 2 if error
```

---

## Output Formats

### 1. Unified Diff (Default)

**Characteristics:**
- Human-readable
- Familiar to developers (like `diff -u`)
- ANSI colors (added=green, removed=red, context=white)

**Example:**
```diff
--- file1.json
+++ file2.json
@@ -3,5 +3,5 @@
   "version": "1.0.0",
   "users": [
     {
-      "name": "Alice",
+      "name": "Bob",
       "age": 30
     }
   ]
```

---

### 2. JSON Report

**Characteristics:**
- Machine-parseable
- Structured data
- Can be piped to other tools

**Example:**
```json
{
  "format": "json",
  "file1": "file1.json",
  "file2": "file2.json",
  "identical": false,
  "file_type": "json",
  "differences": [
    {
      "path": "users[0].name",
      "type": "modified",
      "old_value": "Alice",
      "new_value": "Bob"
    }
  ],
  "summary": {
    "total_changes": 1,
    "additions": 0,
    "deletions": 0,
    "modifications": 1
  }
}
```

---

### 3. Side-by-Side

**Characteristics:**
- Visual comparison
- Shows both files in parallel
- Terminal width aware

**Example:**
```
file1.json                  | file2.json
----------------------------+----------------------------
{                           | {
  "version": "1.0.0",       |   "version": "1.0.0",
  "users": [                |   "users": [
    {                       |     {
      "name": "Alice",   <  |       "name": "Bob",
      "age": 30             |       "age": 30
    }                       |     }
  ]                         |   ]
}                           | }
```

---

### 4. Compact

**Characteristics:**
- Minimal output
- Only shows paths/locations that changed
- Good for large diffs with few changes

**Example:**
```
Modified: users[0].name
  - Alice
  + Bob

1 change (0 additions, 0 deletions, 1 modification)
```

---

## Dogfooding Integration Examples

### Example 1: Multi-Tool Composition

```bash
#!/bin/bash
# Diff workflow using multiple Tuulbelt tools

# 1. Normalize paths (cross-platform-path-normalizer)
FILE1=$(npx tsx ../cross-platform-path-normalizer/src/index.ts --absolute "$1" | jq -r .path)
FILE2=$(npx tsx ../cross-platform-path-normalizer/src/index.ts --absolute "$2" | jq -r .path)

# 2. Initialize progress (cli-progress-reporting)
PROGRESS_ID="diff-$(date +%s)"
npx tsx ../cli-progress-reporting/src/index.ts init --total 100 --id "$PROGRESS_ID" --message "Comparing files"

# 3. Run diff with progress
output-diff "$FILE1" "$FILE2" --progress-id "$PROGRESS_ID" --format json

# 4. Finish progress
npx tsx ../cli-progress-reporting/src/index.ts finish --id "$PROGRESS_ID" --message "Diff complete"
```

### Example 2: Cached Diff with Semaphore

```bash
# Multiple processes can safely use cached diffs
output-diff --cache large-file1.json large-file2.json &
output-diff --cache large-file1.json large-file2.json &
# File-Based Semaphore ensures no cache corruption
```

---

## Library API Design

For use as a Rust library:

```rust
use output_diffing_utility::{diff, DiffConfig, DiffFormat, FileType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = DiffConfig {
        format: DiffFormat::Json,
        file_type: FileType::Auto,
        context_lines: 3,
        color: false,
        cache_enabled: true,
        progress_id: Some("my-diff".to_string()),
    };

    let result = diff("file1.json", "file2.json", config)?;

    println!("{}", result.format_output());

    std::process::exit(if result.identical { 0 } else { 1 });
}
```

---

## CLI Help Output

```
output-diff 0.1.0
Semantic diff tool for JSON, text, and binary files

USAGE:
    output-diff [OPTIONS] <FILE1> <FILE2>

ARGS:
    <FILE1>    First file to compare
    <FILE2>    Second file to compare

OPTIONS:
    -f, --format <FORMAT>       Output format [default: unified] [possible: unified, json, side-by-side, compact]
    -t, --type <TYPE>           Force file type [default: auto] [possible: text, json, binary, auto]
    -c, --context <LINES>       Context lines around diffs [default: 3]
        --color <WHEN>          Colored output [default: auto] [possible: auto, always, never]
    -q, --quiet                 Suppress output, only exit code
    -o, --output <FILE>         Write output to file
        --progress-id <ID>      Enable progress reporting (dogfooding: cli-progress-reporting)
        --cache                 Enable diff caching (dogfooding: file-based-semaphore)
    -v, --verbose               Verbose output
    -h, --help                  Print help
    -V, --version               Print version

EXIT CODES:
    0    Files are identical
    1    Files differ
    2    Error occurred

EXAMPLES:
    # Basic diff
    output-diff file1.json file2.json

    # JSON report
    output-diff --format json data1.json data2.json

    # With progress for large files
    output-diff --progress-id my-task large1.json large2.json

    # Cached diff
    output-diff --cache file1.json file2.json

DOGFOODING:
    This tool integrates with other Tuulbelt tools:
    - cli-progress-reporting: Show progress for large files
    - cross-platform-path-normalizer: Accept paths in any format
    - file-based-semaphore: Protect concurrent cache access
    - test-flakiness-detector: Validate test suite reliability

For more information, see: https://github.com/tuulbelt/output-diffing-utility
```

---

**Status:** CLI design complete
**Next:** Scaffold tool from template
