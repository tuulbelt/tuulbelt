# Diff Algorithms Research

Research notes for implementing semantic diff functionality with zero runtime dependencies.

## Requirements

- **Zero external dependencies** (only Rust std library + serde for JSON parsing in dev)
- **Multiple format support** (JSON, text, binary)
- **Clear output** (human-readable and machine-parseable)
- **Performance** (efficient for files up to 100MB)

---

## 1. Text Diffing Algorithms

### Option A: Line-Based Diff (LCS Algorithm)

**Algorithm:** Longest Common Subsequence (LCS) with backtracking

**Pros:**
- Simple to implement from scratch
- Familiar output format (similar to Unix `diff`)
- Good for line-oriented files (code, logs, config)
- Time complexity: O(n*m) where n, m are line counts

**Cons:**
- Not great for word-level or character-level diffs
- Memory intensive for very large files

**Implementation Approach:**
```rust
// Build LCS table
// Backtrack to find diff operations
// Output: - (deleted), + (added), unchanged
```

**Example Output:**
```diff
  line 1
  line 2
- line 3 (old)
+ line 3 (new)
  line 4
```

**Verdict:** ✅ **Primary choice for text diffing**

---

### Option B: Myers Diff Algorithm

**Algorithm:** Eugene Myers' O(ND) algorithm (used by Git)

**Pros:**
- Produces minimal diffs (optimal edit distance)
- Industry-standard (Git, SVN use variations)
- Efficient for files with small differences

**Cons:**
- Complex to implement from scratch
- Requires understanding of edit graphs
- May need external libraries for full implementation

**Verdict:** ❌ **Too complex for zero-dependency tool, LCS is sufficient**

---

## 2. JSON Diffing Algorithms

### Option A: Structural Tree Diff

**Algorithm:** Recursive tree comparison with path tracking

**Approach:**
1. Parse both JSON files to `serde_json::Value`
2. Recursively compare tree structures
3. Track paths to differences (e.g., `data.users[2].name`)
4. Report added/removed/modified fields

**Pros:**
- Semantic comparison (understands JSON structure)
- Clear output showing exactly which fields differ
- Can handle nested objects/arrays
- Native Rust serde_json in std (dev dependency only)

**Cons:**
- Must parse entire JSON into memory
- Large files (>100MB) may be slow

**Implementation Approach:**
```rust
use serde_json::Value;

fn diff_json(a: &Value, b: &Value, path: &str) -> Vec<Difference> {
    match (a, b) {
        (Value::Object(a_obj), Value::Object(b_obj)) => {
            // Compare object keys and recurse
        }
        (Value::Array(a_arr), Value::Array(b_arr)) => {
            // Compare array elements
        }
        _ if a != b => {
            // Leaf values differ
            vec![Difference { path: path.to_string(), old: a, new: b }]
        }
        _ => vec![]
    }
}
```

**Example Output:**
```json
{
  "differences": [
    {
      "path": "data.users[2].name",
      "old": "Alice",
      "new": "Bob"
    },
    {
      "path": "data.count",
      "old": 10,
      "new": 11
    }
  ]
}
```

**Verdict:** ✅ **Primary choice for JSON diffing**

---

### Option B: JSON Patch (RFC 6902)

**Algorithm:** Generate JSON Patch operations

**Pros:**
- Standardized format (RFC 6902)
- Can be applied to transform A → B
- Machine-readable

**Cons:**
- More complex to generate correctly
- Less human-readable
- Array handling is tricky (indices change on insert/delete)

**Verdict:** ❌ **Too complex for v0.1.0, tree diff is more intuitive**

---

## 3. Binary Diffing Algorithms

### Option A: Byte-by-Byte Comparison

**Algorithm:** Compare files byte-by-byte, report differences with offsets

**Approach:**
1. Read both files as byte arrays
2. Compare byte-by-byte
3. Output offset, old byte (hex), new byte (hex)

**Pros:**
- Simple to implement
- Exact differences reported
- Works for any binary format

**Cons:**
- Not semantic (doesn't understand file structure)
- Large output for files with many differences

**Implementation Approach:**
```rust
fn diff_binary(a: &[u8], b: &[u8]) -> Vec<ByteDiff> {
    let max_len = a.len().max(b.len());
    let mut diffs = Vec::new();

    for i in 0..max_len {
        let a_byte = a.get(i);
        let b_byte = b.get(i);

        if a_byte != b_byte {
            diffs.push(ByteDiff {
                offset: i,
                old: a_byte.map(|b| format!("{:02x}", b)),
                new: b_byte.map(|b| format!("{:02x}", b)),
            });
        }
    }

    diffs
}
```

**Example Output:**
```
Offset | Old  | New
-------|------|------
0x0010 | 4a   | 4b
0x0011 | 6f   | 61
0x002a | --   | ff  (added)
```

**Verdict:** ✅ **Primary choice for binary diffing**

---

### Option B: Binary Delta (BSDIFF)

**Algorithm:** Compute binary delta (patch) using suffix arrays

**Pros:**
- Compact representation of differences
- Used for software updates (bsdiff/bspatch)

**Cons:**
- Extremely complex to implement from scratch
- Requires external libraries
- Violates zero-dependency principle

**Verdict:** ❌ **Too complex, not zero-dependency**

---

## 4. Output Format Options

### Option A: Human-Readable Text

**Format:** Color-coded diff output (like `diff -u`)

**Pros:**
- Familiar to developers
- Easy to read in terminal
- Can use ANSI colors

**Cons:**
- Hard to parse programmatically

**Example:**
```diff
--- file1.json
+++ file2.json
@@ -3,2 +3,2 @@
-  "name": "Alice"
+  "name": "Bob"
```

---

### Option B: JSON Report

**Format:** Structured JSON with differences

**Pros:**
- Machine-parseable
- Can be piped to other tools
- Supports complex structures

**Cons:**
- Less readable for humans

**Example:**
```json
{
  "format": "json",
  "file1": "file1.json",
  "file2": "file2.json",
  "differences": [
    { "path": "name", "old": "Alice", "new": "Bob" }
  ]
}
```

---

### Recommendation: Both Formats

- **Default:** Human-readable text output
- **Flag:** `--json` for structured JSON output
- **Flag:** `--format <unified|json|compact>` for different styles

---

## 5. Implementation Plan

### Phase 1: Core Algorithms (MVP)

1. **Text Diff** - LCS-based line diff
2. **JSON Diff** - Structural tree comparison
3. **Binary Diff** - Byte-by-byte comparison

### Phase 2: Output Formats

1. **Unified diff** format (default)
2. **JSON report** format (--json flag)
3. **Side-by-side** format (--side-by-side flag)

### Phase 3: Optimizations

1. **Streaming** - Don't load entire file into memory for binary
2. **Chunking** - Process large files in chunks
3. **Caching** - Hash-based early exit (files identical)

---

## 6. File Type Detection

**Strategy:** Detect file type automatically or via flag

```rust
fn detect_file_type(path: &Path) -> FileType {
    // 1. Check extension
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => FileType::Json,
        Some("txt") | Some("log") | Some("md") => FileType::Text,
        _ => {
            // 2. Check content (read first 512 bytes)
            if is_valid_utf8(content) {
                FileType::Text
            } else {
                FileType::Binary
            }
        }
    }
}
```

---

## 7. Edge Cases to Handle

1. **Empty files** - Both empty = no diff, one empty = all added/removed
2. **Identical files** - Early exit with SHA-256 hash comparison
3. **File size mismatch** - Large files: show warning, process anyway
4. **Encoding issues** - Invalid UTF-8 in "text" files → treat as binary
5. **Missing files** - Clear error message, exit code 2

---

## References

- Myers Diff Algorithm: "An O(ND) Difference Algorithm and Its Variations" (1986)
- LCS: Introduction to Algorithms (CLRS), Chapter 15
- JSON Patch: RFC 6902 https://datatracker.ietf.org/doc/html/rfc6902
- Git Diff: https://git-scm.com/docs/git-diff
- Unix Diff: https://man7.org/linux/man-pages/man1/diff.1.html

---

## Decision Summary

| Format | Algorithm | Implementation Complexity | Performance |
|--------|-----------|--------------------------|-------------|
| Text | LCS Line Diff | Medium | O(n*m) lines |
| JSON | Tree Diff | Low | O(n) nodes |
| Binary | Byte-by-Byte | Low | O(n) bytes |

**All chosen algorithms:**
- ✅ Zero external dependencies (use std lib + serde for parsing)
- ✅ Reasonable performance (suitable for files up to 100MB)
- ✅ Semantic output (human-understandable)
- ✅ Composable (JSON output can be piped to other tools)

---

**Status:** Research complete, ready for design phase
**Next:** Design CLI interface and output format
