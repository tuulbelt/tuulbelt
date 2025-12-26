# API Reference

Complete Rust API documentation for Output Diffing Utility.

## Core Functions

### `diff_text`

Compare two text strings line-by-line.

```rust
pub fn diff_text(
    old: &str,
    new: &str,
    config: &DiffConfig
) -> TextDiffResult
```

**Parameters:**
- `old` - Original text content
- `new` - New text content
- `config` - Diff configuration

**Returns:** `TextDiffResult` containing line changes

**Example:**
```rust
let result = diff_text("old\ntext", "new\ntext", &DiffConfig::default());
assert!(result.has_changes());
```

---

### `diff_json`

Compare two JSON strings structurally.

```rust
pub fn diff_json(
    old_json: &str,
    new_json: &str,
    config: &DiffConfig
) -> Result<JsonDiffResult, JsonError>
```

**Parameters:**
- `old_json` - Original JSON string
- `new_json` - New JSON string
- `config` - Diff configuration

**Returns:** `Result<JsonDiffResult, JsonError>`

**Errors:** Returns `JsonError` if JSON parsing fails

**Example:**
```rust
let result = diff_json(r#"{"a":1}"#, r#"{"a":2}"#, &DiffConfig::default())?;
assert_eq!(result.modifications(), 1);
```

---

### `diff_binary`

Compare two byte arrays.

```rust
pub fn diff_binary(
    data1: &[u8],
    data2: &[u8],
    config: &DiffConfig
) -> BinaryDiffResult
```

**Parameters:**
- `data1` - Original binary data
- `data2` - New binary data
- `config` - Diff configuration

**Returns:** `BinaryDiffResult` with comparison details

**Example:**
```rust
let result = diff_binary(&[0x00, 0x01], &[0x00, 0xFF], &DiffConfig::default());
assert!(!result.is_identical());
assert_eq!(result.first_diff_offset, Some(1));
```

---

### `detect_file_type`

Auto-detect file type from content and extension.

```rust
pub fn detect_file_type(
    content: &[u8],
    extension: Option<&str>
) -> FileType
```

**Parameters:**
- `content` - File content bytes
- `extension` - Optional file extension (without dot)

**Returns:** `FileType` (Text, Json, or Binary)

**Detection Logic:**
1. Check extension first (.json → JSON, .txt → Text)
2. Try UTF-8 validation
3. Try JSON parsing
4. Check for null bytes (→ Binary)
5. Default to Text

**Example:**
```rust
let file_type = detect_file_type(b"{\"key\": \"value\"}", Some("json"));
assert_eq!(file_type, FileType::Json);
```

---

## Format Functions

### `format_unified_diff`

Format text diff as unified diff (no color).

```rust
pub fn format_unified_diff(
    result: &TextDiffResult,
    file1_name: &str,
    file2_name: &str
) -> String
```

---

### `format_unified_diff_with_color`

Format text diff as unified diff with ANSI colors.

```rust
pub fn format_unified_diff_with_color(
    result: &TextDiffResult,
    file1_name: &str,
    file2_name: &str,
    color: bool
) -> String
```

**Color Codes:**
- `\x1b[31m` - Red (removed lines)
- `\x1b[32m` - Green (added lines)
- `\x1b[0m` - Reset (context lines)

---

### `format_json_diff`

Format JSON diff in human-readable format.

```rust
pub fn format_json_diff(result: &JsonDiffResult) -> String
```

**Example Output:**
```
JSON diff:
  Modified at 'user.name': "Alice" → "Bob"
  Added at 'user.email': "bob@example.com"

2 changes (1 addition, 0 deletions, 1 modification)
```

---

### `format_binary_diff`

Format binary diff with hex output.

```rust
pub fn format_binary_diff(result: &BinaryDiffResult) -> String
```

**Example Output:**
```
Binary files differ
Size: 1024 bytes vs 1024 bytes
First difference at offset 0x00000010: 0xff → 0x00
```

---

### `format_as_json_report`

Generate machine-parseable JSON report.

```rust
pub fn format_as_json_report(
    result: &DiffResult,
    file1: &str,
    file2: &str,
    file_type: FileType
) -> String
```

**Returns:** JSON string conforming to SPEC.md schema

---

### `format_compact`

Format diff in compact style (changes only).

```rust
pub fn format_compact(result: &DiffResult) -> String
```

---

### `format_side_by_side`

Format diff in side-by-side columns.

```rust
pub fn format_side_by_side(
    result: &DiffResult,
    file1: &str,
    file2: &str,
    width: usize
) -> String
```

**Parameters:**
- `width` - Total terminal width (default: 120)

---

## Types

### `DiffConfig`

Configuration for diff operations.

```rust
pub struct DiffConfig {
    pub context_lines: usize,
    pub format: OutputFormat,
    pub color: bool,
    pub verbose: bool,
}
```

**Methods:**
```rust
impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            context_lines: usize::MAX,  // Show all context
            format: OutputFormat::Unified,
            color: false,
            verbose: false,
        }
    }
}
```

---

### `OutputFormat`

Output format enumeration.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Unified,
    Json,
    SideBySide,
    Compact,
}
```

---

### `FileType`

File type enumeration.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Text,
    Json,
    Binary,
}
```

---

### `DiffResult`

Unified diff result enum.

```rust
#[derive(Debug)]
pub enum DiffResult {
    Text(TextDiffResult),
    Binary(BinaryDiffResult),
    Json(JsonDiffResult),
}
```

**Methods:**
```rust
impl DiffResult {
    pub fn is_identical(&self) -> bool;
    pub fn additions(&self) -> usize;
    pub fn deletions(&self) -> usize;
    pub fn modifications(&self) -> usize;
    pub fn total_changes(&self) -> usize;
}
```

---

### `TextDiffResult`

Text diff result.

```rust
pub struct TextDiffResult {
    pub changes: Vec<LineChange>,
}
```

**Methods:**
```rust
impl TextDiffResult {
    pub fn has_changes(&self) -> bool;
    pub fn additions(&self) -> usize;
    pub fn deletions(&self) -> usize;
}
```

---

### `LineChange`

Individual line change.

```rust
#[derive(Debug, Clone)]
pub enum LineChange {
    Added {
        new_line_num: usize,
        line: String,
    },
    Removed {
        old_line_num: usize,
        line: String,
    },
    Unchanged {
        old_line_num: usize,
        new_line_num: usize,
        line: String,
    },
}
```

---

### `JsonDiffResult`

JSON diff result.

```rust
pub struct JsonDiffResult {
    pub changes: Vec<JsonChange>,
}
```

**Methods:**
```rust
impl JsonDiffResult {
    pub fn has_changes(&self) -> bool;
    pub fn additions(&self) -> usize;
    pub fn deletions(&self) -> usize;
    pub fn modifications(&self) -> usize;
}
```

---

### `JsonChange`

JSON structure change.

```rust
#[derive(Debug, Clone)]
pub enum JsonChange {
    Added {
        path: String,
        value: JsonValue,
    },
    Removed {
        path: String,
        value: JsonValue,
    },
    Modified {
        path: String,
        old_value: JsonValue,
        new_value: JsonValue,
    },
}
```

---

### `JsonValue`

JSON value representation.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}
```

---

### `BinaryDiffResult`

Binary diff result.

```rust
pub struct BinaryDiffResult {
    pub size1: usize,
    pub size2: usize,
    pub first_diff_offset: Option<usize>,
}
```

**Methods:**
```rust
impl BinaryDiffResult {
    pub fn is_identical(&self) -> bool {
        self.first_diff_offset.is_none() && self.size1 == self.size2
    }
}
```

---

### `JsonError`

JSON parsing error.

```rust
#[derive(Debug)]
pub struct JsonError {
    pub message: String,
}
```

---

## Complete Example

```rust
use output_diffing_utility::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure
    let config = DiffConfig {
        context_lines: 3,
        format: OutputFormat::Unified,
        color: true,
        verbose: false,
    };

    // Detect type
    let content = std::fs::read("file.json")?;
    let file_type = detect_file_type(&content, Some("json"));

    // Diff based on type
    match file_type {
        FileType::Json => {
            let text = std::str::from_utf8(&content)?;
            let result = diff_json(text, "{\"new\": true}", &config)?;
            let formatted = format_json_diff(&result);
            println!("{}", formatted);
        }
        FileType::Text => {
            let text = std::str::from_utf8(&content)?;
            let result = diff_text(text, "new text", &config);
            let formatted = format_unified_diff_with_color(
                &result, "old", "new", config.color
            );
            println!("{}", formatted);
        }
        FileType::Binary => {
            let other = std::fs::read("other.bin")?;
            let result = diff_binary(&content, &other, &config);
            let formatted = format_binary_diff(&result);
            println!("{}", formatted);
        }
    }

    Ok(())
}
```

## See Also

- [Library Usage](/tools/output-diffing-utility/library-usage) - Integration guide
- [Examples](/tools/output-diffing-utility/examples) - Code examples
- [SPEC.md](https://github.com/tuulbelt/tuulbelt/blob/main/output-diffing-utility/SPEC.md) - Technical specification
