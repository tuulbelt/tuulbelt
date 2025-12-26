# Output Formats

Detailed specifications for all output formats supported by Output Diffing Utility.

## Format Overview

| Format | MIME Type | Machine-Parseable | Colorized | Use Case |
|--------|-----------|-------------------|-----------|----------|
| **Unified** | `text/plain` | No | Yes | Human review, patches |
| **JSON** | `application/json` | Yes | No | CI/CD, automation |
| **Side-by-Side** | `text/plain` | No | No | Visual review |
| **Compact** | `text/plain` | No | No | Summaries, commits |

---

## Format 1: Unified (Default)

Traditional diff format compatible with `patch` command.

### Structure

```diff
--- <file1_name>
+++ <file2_name>
@@ -<old_start>,<old_count> +<new_start>,<new_count> @@
 <context_line>
-<removed_line>
+<added_line>
 <context_line>
```

### Example (Text)

```diff
--- old.txt
+++ new.txt
@@ -1,3 +1,3 @@
 line 1
-line 2
+line 2 modified
 line 3
```

### Example (JSON)

```
JSON diff for 'old.json' vs 'new.json':
  Modified at 'user.name': "Alice" → "Bob"
  Added at 'user.email': "bob@example.com"
  Removed at 'user.age': 30

3 changes (1 addition, 1 deletion, 1 modification)
```

### Example (Binary)

```
Binary files differ
Size: 1024 bytes vs 1048 bytes
First difference at offset 0x00000010: 0xff → 0x00
Total: 1 change
```

### Color Support

When `--color always` or auto-detected terminal:

- **Red (`\x1b[31m`)**: Removed lines (`- content`)
- **Green (`\x1b[32m`)**: Added lines (`+ content`)
- **Default**: Context lines

**Example:**
```bash
output-diff --color always file1.txt file2.txt | cat -v
# Shows: ^[[31m- old line^[[0m and ^[[32m+ new line^[[0m
```

### Context Control

```bash
# No context (changes only)
output-diff --context 0 file1.txt file2.txt
@@ -2,1 +2,1 @@
-line 2
+line 2 modified

# 3 lines context
output-diff --context 3 file1.txt file2.txt
@@ -1,3 +1,3 @@
 line 1
-line 2
+line 2 modified
 line 3
```

---

## Format 2: JSON Report

Machine-parseable structured report.

### Schema

```typescript
interface DiffReport {
  format: "json";
  file1: string;
  file2: string;
  identical: boolean;
  file_type: "text" | "json" | "binary";
  differences: Difference[];
  summary: Summary;
}

interface Difference {
  type: "added" | "removed" | "modified";
  path: string;           // Location identifier
  old_value?: any;        // Present for "removed" or "modified"
  new_value?: any;        // Present for "added" or "modified"
  old_line?: number;      // Text diffs only
  new_line?: number;      // Text diffs only
}

interface Summary {
  total_changes: number;
  additions: number;
  deletions: number;
  modifications: number;
}
```

### Example (Text Diff)

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

### Example (JSON Diff)

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
    },
    {
      "type": "added",
      "path": "user.email",
      "new_value": "bob@example.com"
    },
    {
      "type": "removed",
      "path": "user.age",
      "old_value": 30
    }
  ],
  "summary": {
    "total_changes": 3,
    "additions": 1,
    "deletions": 1,
    "modifications": 1
  }
}
```

### Example (Binary Diff)

```json
{
  "format": "json",
  "file1": "file1.bin",
  "file2": "file2.bin",
  "identical": false,
  "file_type": "binary",
  "differences": [
    {
      "type": "modified",
      "path": "0x00000010",
      "old_value": "ff",
      "new_value": "00"
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

### Parsing Example

```bash
# Extract summary using jq
output-diff --format json file1.json file2.json | jq '.summary'

# Count changes
output-diff --format json file1.json file2.json | jq '.summary.total_changes'

# List modified paths
output-diff --format json file1.json file2.json | \
  jq '.differences[] | select(.type == "modified") | .path'
```

---

## Format 3: Side-by-Side

Visual comparison in parallel columns.

### Structure

```
<file1_name>                 | <file2_name>
-----------------------------+-----------------------------
<file1_line1>                | <file2_line1>
<file1_line2 (differs)>     <
                            > | <file2_line2 (differs)>
<file1_line3>                | <file2_line3>
```

### Example

```
old.txt                      | new.txt
-----------------------------+-----------------------------
line 1                       | line 1
line 2                      <
                            > | line 2 modified
line 3                       | line 3
line 4                       | line 4
```

### Width Control

Default width: 120 columns

```rust
use output_diffing_utility::{format_side_by_side, DiffResult};

let formatted = format_side_by_side(&diff_result, "old.txt", "new.txt", 160);
// Uses 160-column width
```

### Line Markers

- `<` - Line only in file1 (removed)
- `>` - Line only in file2 (added)
- `|` - Line in both files (may differ)
- No marker - Lines are identical

---

## Format 4: Compact

Minimal output showing only changed items.

### Structure (Text)

```
Line <N>:
  - <old_content>
  + <new_content>

<summary>
```

### Example (Text)

```
Line 2:
  - line 2
  + line 2 modified

Line 5:
  + new line added

Line 7:
  - removed line

3 changes (1 addition, 1 deletion, 1 modification)
```

### Structure (JSON)

```
Modified: <path>
  - <old_value>
  + <new_value>

Added: <path>
  + <new_value>

Removed: <path>
  - <old_value>

<summary>
```

### Example (JSON)

```
Modified: user.name
  - "Alice"
  + "Bob"

Added: user.email
  + "bob@example.com"

Removed: user.age
  - 30

3 changes (1 addition, 1 deletion, 1 modification)
```

### Structure (Binary)

```
Binary files differ

<summary>
```

### Example (Binary)

```
Binary files differ

1 change
```

---

## Choosing a Format

### Use **Unified** when:
- Reviewing changes manually
- Creating patches to apply with `patch` command
- Terminal output with color highlighting
- Traditional diff workflows

### Use **JSON** when:
- Parsing diff results programmatically
- CI/CD pipeline integration
- Storing diff results in databases
- Generating automated reports

### Use **Side-by-Side** when:
- Visual code review
- Comparing short files
- Terminal-based review sessions
- Presenting changes to stakeholders

### Use **Compact** when:
- Summarizing changes in commits
- Showing only what changed (no context)
- Large files with few changes
- Quick overview needed

---

## Format Comparison

### Same Input, All Formats

**Input files:**
```
# old.txt
line 1
line 2
line 3

# new.txt
line 1
line 2 modified
line 3
```

**Unified:**
```diff
--- old.txt
+++ new.txt
@@ -1,3 +1,3 @@
 line 1
-line 2
+line 2 modified
 line 3
```

**JSON:**
```json
{
  "format": "json",
  "file1": "old.txt",
  "file2": "new.txt",
  "identical": false,
  "differences": [
    {"type": "removed", "path": "Line 2", "old_value": "line 2"},
    {"type": "added", "path": "Line 2", "new_value": "line 2 modified"}
  ],
  "summary": {"total_changes": 2, "additions": 1, "deletions": 1}
}
```

**Side-by-Side:**
```
old.txt                      | new.txt
-----------------------------+-----------------------------
line 1                       | line 1
line 2                      <
                            > | line 2 modified
line 3                       | line 3
```

**Compact:**
```
Line 2:
  - line 2
  + line 2 modified

2 changes (1 addition, 1 deletion)
```

---

## Technical Details

### Unified Diff Hunk Headers

```diff
@@ -<old_start>,<old_count> +<new_start>,<new_count> @@
```

**Fields:**
- `old_start` - Starting line number in old file
- `old_count` - Number of lines in old file
- `new_start` - Starting line number in new file
- `new_count` - Number of lines in new file

**Example:**
```diff
@@ -10,5 +10,6 @@  # Lines 10-14 in old → lines 10-15 in new
```

### JSON Path Format

**JSON paths** use dot notation for objects and brackets for arrays:

```
user.name          # Object property
users[0].name      # Array index + property
data.items[5]      # Nested array
root["key.with.dots"]  # Keys containing dots (future)
```

### Binary Offset Format

**Hex offsets** with 8-digit zero-padded format:

```
0x00000000  # Byte 0
0x00000010  # Byte 16 (decimal)
0x000003e8  # Byte 1000 (decimal)
```

---

## See Also

- [CLI Usage](/tools/output-diffing-utility/cli-usage) - Format selection options
- [Examples](/tools/output-diffing-utility/examples) - Real-world format usage
- [SPEC.md](https://github.com/tuulbelt/tuulbelt/blob/main/output-diffing-utility/SPEC.md) - Full technical specification
