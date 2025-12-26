# Output Diffing Utility - Specification

**Version:** 1.0
**Status:** Stable

This document specifies the diff output formats and behavior for interoperability and documentation purposes.

## Overview

The Output Diffing Utility performs semantic comparison of files and produces human-readable or machine-parseable diff reports. It supports three file types (text, JSON, binary) and four output formats (unified, JSON report, side-by-side, compact).

## Supported File Types

### 1. Text Files

**Detection:**
- Extension: `.txt`, `.md`, `.rs`, `.js`, `.ts`, `.py`, etc.
- Content: Valid UTF-8 encoding
- Fallback: If not binary or JSON

**Diff Algorithm:** Line-based comparison using Longest Common Subsequence (LCS)

**Output:** Line changes with context (unchanged lines around changes)

### 2. JSON Files

**Detection:**
- Extension: `.json`
- Content: Valid JSON syntax (parsed successfully)

**Diff Algorithm:** Structural comparison of JSON values

**Output:** Path-based changes (e.g., `user.name` changed from "Alice" to "Bob")

### 3. Binary Files

**Detection:**
- Extension: `.bin`, `.exe`, `.dat`, etc.
- Content: Contains null bytes or non-UTF-8 sequences
- Fallback: If UTF-8 validation fails

**Diff Algorithm:** Byte-by-byte comparison

**Output:** Hex diff showing offset, old byte, new byte

## Output Formats

### Format 1: Unified Diff (Default)

**MIME Type:** `text/plain`
**Extension:** `.diff`, `.patch`
**Description:** Human-readable diff format similar to `diff -u`

**Structure:**
```diff
--- file1.txt
+++ file2.txt
@@ -<old_start>,<old_count> +<new_start>,<new_count> @@
 context line
-removed line
+added line
 context line
```

**Features:**
- ANSI color support (red=removed, green=added, default=context)
- Configurable context lines (default: show all)
- Exit code: 0 if identical, 1 if different

**Example:**
```diff
--- old.txt
+++ new.txt
@@ -1,3 +1,3 @@
 line 1
-line 2
+line 2 modified
 line 3
```

### Format 2: JSON Report

**MIME Type:** `application/json`
**Extension:** `.json`
**Description:** Machine-parseable structured diff report

**Schema:**
```json
{
  "format": "json",
  "file1": string,
  "file2": string,
  "identical": boolean,
  "file_type": "text" | "json" | "binary",
  "differences": [
    {
      "type": "added" | "removed" | "modified",
      "path": string,           // JSON: "user.name", Text: "Line 5", Binary: "0x00000010"
      "old_value"?: any,        // Present for "removed" or "modified"
      "new_value"?: any,        // Present for "added" or "modified"
      "old_line"?: number,      // Text only
      "new_line"?: number       // Text only
    }
  ],
  "summary": {
    "total_changes": number,
    "additions": number,
    "deletions": number,
    "modifications": number
  }
}
```

**Example (Text):**
```json
{
  "format": "json",
  "file1": "old.txt",
  "file2": "new.txt",
  "identical": false,
  "file_type": "text",
  "differences": [
    {
      "type": "removed",
      "path": "Line 2",
      "old_value": "line 2",
      "old_line": 2
    },
    {
      "type": "added",
      "path": "Line 2",
      "new_value": "line 2 modified",
      "new_line": 2
    }
  ],
  "summary": {
    "total_changes": 2,
    "additions": 1,
    "deletions": 1,
    "modifications": 0
  }
}
```

**Example (JSON):**
```json
{
  "format": "json",
  "file1": "data1.json",
  "file2": "data2.json",
  "identical": false,
  "file_type": "json",
  "differences": [
    {
      "type": "modified",
      "path": "user.name",
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

### Format 3: Side-by-Side

**MIME Type:** `text/plain`
**Extension:** `.txt`
**Description:** Visual comparison showing files in parallel columns

**Structure:**
```
<file1 name>                 | <file2 name>
-----------------------------+-----------------------------
<file1 line 1>               | <file2 line 1>
<file1 line 2 (if differs)> <
                            > | <file2 line 2 (if differs)>
```

**Features:**
- Terminal width aware (default: 120 columns)
- `<` marker for lines only in file1
- `>` marker for lines only in file2
- Synchronized line numbers where possible

**Example:**
```
old.txt                      | new.txt
-----------------------------+-----------------------------
line 1                       | line 1
line 2                      <
                            > | line 2 modified
line 3                       | line 3
```

### Format 4: Compact

**MIME Type:** `text/plain`
**Extension:** `.txt`
**Description:** Minimal output showing only changed items

**Structure (Text):**
```
Line <N>:
  - <old content>
  + <new content>

<summary line>
```

**Structure (JSON):**
```
Modified: <path>
  - <old value>
  + <new value>

<summary line>
```

**Structure (Binary):**
```
Binary files differ

<summary line>
```

**Example:**
```
Line 2:
  - line 2
  + line 2 modified

1 change (1 addition, 1 deletion, 0 modifications)
```

## CLI Options

### File Type Selection

**`-t, --type <TYPE>`**

Force file type interpretation (overrides auto-detection).

**Values:** `text`, `json`, `binary`, `auto` (default)

**Example:**
```bash
output-diff --type text data.json data2.json  # Treat JSON as plain text
```

### Format Selection

**`-f, --format <FORMAT>`**

Specify output format.

**Values:** `unified` (default), `json`, `side-by-side`, `compact`

**Example:**
```bash
output-diff --format json file1.txt file2.txt
```

### Context Control

**`-c, --context <LINES>`**

Number of unchanged lines to show around changes (text diffs only).

**Default:** Show all context
**Range:** 0 to unlimited

**Example:**
```bash
output-diff --context 3 file1.txt file2.txt  # 3 lines before/after changes
output-diff --context 0 file1.txt file2.txt  # No context, only changes
```

### Color Control

**`--color <WHEN>`**

Control ANSI color output (unified format only).

**Values:** `auto` (default), `always`, `never`

**Behavior:**
- `auto`: Color if stdout is a terminal
- `always`: Always use color (for piping to `less -R`)
- `never`: No color (for redirecting to files)

**Example:**
```bash
output-diff --color always file1.txt file2.txt | less -R
```

### Output Control

**`-q, --quiet`**

Suppress all output; only set exit code.

**Use case:** Shell scripts checking file equivalence.

**Example:**
```bash
if output-diff --quiet file1.txt file2.txt; then
    echo "Files are identical"
fi
```

**`-o, --output <FILE>`**

Write diff output to file instead of stdout.

**Example:**
```bash
output-diff --output changes.diff file1.txt file2.txt
```

**`-v, --verbose`**

Enable verbose debug output (to stderr).

**Output:** File sizes, detected types, timing information.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Files are identical |
| 1 | Files differ |
| 2 | Error occurred (file not found, parse error, invalid arguments) |

## Algorithm Details

### Text Diff (LCS)

**Algorithm:** Dynamic Programming-based Longest Common Subsequence

**Time Complexity:** O(n × m) where n, m are line counts
**Space Complexity:** O(n × m)

**Steps:**
1. Split files into lines
2. Compute LCS table via DP
3. Backtrack to identify changes
4. Filter to context range (if specified)

### JSON Diff

**Algorithm:** Recursive structural comparison

**Time Complexity:** O(n) where n is total JSON elements
**Space Complexity:** O(depth) for recursion stack

**Steps:**
1. Parse JSON to internal representation
2. Recursively compare structures
3. Generate path-based change list
4. Classify as added/removed/modified

### Binary Diff

**Algorithm:** Byte-by-byte comparison with hex output

**Time Complexity:** O(min(n, m))
**Space Complexity:** O(1)

**Steps:**
1. Compare byte-by-byte
2. Stop at first difference (early exit)
3. Report offset and hex values

## Error Handling

**Invalid UTF-8:**
```
Error: File 1 is not valid UTF-8
Exit code: 2
```

**JSON Parse Error:**
```
Error parsing JSON: expected value at line 1 column 5
Exit code: 2
```

**File Not Found:**
```
Error reading /tmp/nonexistent.txt: No such file or directory
Exit code: 2
```

**Invalid Arguments:**
```
Error: Expected exactly 2 file arguments, got 1
Exit code: 2
```

## Library API

### Rust

```rust
use output_diffing_utility::{diff_text, diff_json, diff_binary, DiffConfig};

let config = DiffConfig {
    context_lines: 3,
    format: OutputFormat::Unified,
    color: false,
    verbose: false,
};

let result = diff_text("old text", "new text", &config);

if result.has_changes() {
    println!("Additions: {}", result.additions());
    println!("Deletions: {}", result.deletions());
}
```

## Interoperability

This tool outputs standard formats compatible with:

- **Unified diff:** Can be applied with `patch` command (GNU diff compatible)
- **JSON report:** Can be consumed by CI/CD tools, testing frameworks
- **Side-by-side:** Human-readable review format
- **Compact:** Ideal for commit messages, PR descriptions

## Versioning

**Current Version:** 1.0
**Stability:** Stable (output formats will not change in breaking ways)

**Semantic Versioning:**
- MAJOR: Breaking changes to output format or CLI interface
- MINOR: New formats or options (backwards compatible)
- PATCH: Bug fixes

## References

- Myers diff algorithm: https://neil.fraser.name/writing/diff/
- Unified diff format: https://www.gnu.org/software/diffutils/manual/html_node/Unified-Format.html
- JSON RFC: https://www.rfc-editor.org/rfc/rfc8259
