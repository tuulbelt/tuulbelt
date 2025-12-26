# Library Usage

Integrate Output Diffing Utility into your Rust projects as a library.

## Adding as Dependency

Since this tool is part of the Tuulbelt monorepo, you can either:

### Option 1: Local Path Dependency (Monorepo)

```toml
# Cargo.toml
[dependencies]
output-diffing-utility = { path = "../output-diffing-utility" }
```

### Option 2: Git Dependency

```toml
# Cargo.toml
[dependencies]
output-diffing-utility = { git = "https://github.com/tuulbelt/tuulbelt", branch = "main" }
```

### Option 3: Copy Source (Zero Dependencies)

Since this tool has zero runtime dependencies, you can copy `src/` directly into your project.

## Basic Usage

### Text Diff

```rust
use output_diffing_utility::{diff_text, DiffConfig, OutputFormat};

fn main() {
    let config = DiffConfig {
        context_lines: 3,
        format: OutputFormat::Unified,
        color: false,
        verbose: false,
    };

    let old_text = "line 1\nline 2\nline 3";
    let new_text = "line 1\nline 2 modified\nline 3";

    let result = diff_text(old_text, new_text, &config);

    if result.has_changes() {
        println!("Files differ!");
        println!("Additions: {}", result.additions());
        println!("Deletions: {}", result.deletions());

        for change in &result.changes {
            match change {
                output_diffing_utility::LineChange::Added { new_line_num, line } => {
                    println!("+ Line {}: {}", new_line_num, line);
                }
                output_diffing_utility::LineChange::Removed { old_line_num, line } => {
                    println!("- Line {}: {}", old_line_num, line);
                }
                output_diffing_utility::LineChange::Unchanged { .. } => {
                    // Context lines
                }
            }
        }
    } else {
        println!("Files are identical");
    }
}
```

### JSON Diff

```rust
use output_diffing_utility::{diff_json, DiffConfig, JsonChange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = DiffConfig::default();

    let old_json = r#"{"name": "Alice", "age": 30}"#;
    let new_json = r#"{"name": "Bob", "age": 30}"#;

    let result = diff_json(old_json, new_json, &config)?;

    if result.has_changes() {
        println!("JSON structures differ:");

        for change in &result.changes {
            match change {
                JsonChange::Added { path, value } => {
                    println!("  Added at '{}': {:?}", path, value);
                }
                JsonChange::Removed { path, value } => {
                    println!("  Removed at '{}': {:?}", path, value);
                }
                JsonChange::Modified { path, old_value, new_value } => {
                    println!("  Modified at '{}': {:?} → {:?}", path, old_value, new_value);
                }
            }
        }
    }

    Ok(())
}
```

### Binary Diff

```rust
use output_diffing_utility::{diff_binary, DiffConfig};

fn main() {
    let config = DiffConfig::default();

    let data1 = vec![0x00, 0x01, 0x02, 0xFF];
    let data2 = vec![0x00, 0xFF, 0x02, 0xFF];

    let result = diff_binary(&data1, &data2, &config);

    if result.is_identical() {
        println!("Binary data is identical");
    } else {
        println!("Binary files differ");
        println!("Size 1: {}", result.size1);
        println!("Size 2: {}", result.size2);

        if let Some(diff_offset) = result.first_diff_offset {
            println!("First difference at offset: 0x{:08x}", diff_offset);
        }
    }
}
```

## Advanced Usage

### Custom Formatting

```rust
use output_diffing_utility::{
    diff_text, format_unified_diff_with_color, format_compact,
    DiffConfig, DiffResult
};

fn main() {
    let config = DiffConfig::default();
    let result = diff_text("old", "new", &config);

    // Format as unified diff with color
    let unified = format_unified_diff_with_color(
        &result,
        "file1.txt",
        "file2.txt",
        true  // enable color
    );
    println!("{}", unified);

    // Format as compact
    let diff_result = DiffResult::Text(result);
    let compact = format_compact(&diff_result);
    println!("{}", compact);
}
```

### JSON Report Generation

```rust
use output_diffing_utility::{
    diff_json, format_as_json_report, DiffResult, FileType
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Default::default();
    let result = diff_json("{\"a\": 1}", "{\"a\": 2}", &config)?;

    let diff_result = DiffResult::Json(result);
    let json_report = format_as_json_report(
        &diff_result,
        "old.json",
        "new.json",
        FileType::Json
    );

    // Parse and use the report
    let report: serde_json::Value = serde_json::from_str(&json_report)?;
    println!("Total changes: {}", report["summary"]["total_changes"]);

    Ok(())
}
```

### File Type Detection

```rust
use output_diffing_utility::{detect_file_type, FileType};
use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read("myfile.txt")?;
    let extension = Some("txt");

    let file_type = detect_file_type(&content, extension);

    match file_type {
        FileType::Text => println!("Detected as text file"),
        FileType::Json => println!("Detected as JSON file"),
        FileType::Binary => println!("Detected as binary file"),
    }

    Ok(())
}
```

### Configuration Builder Pattern

```rust
use output_diffing_utility::{DiffConfig, OutputFormat};

fn main() {
    // Create custom config
    let config = DiffConfig {
        context_lines: 5,
        format: OutputFormat::SideBySide,
        color: true,
        verbose: true,
    };

    // Or use default
    let default_config = DiffConfig::default();

    // Default values:
    // context_lines: usize::MAX (show all)
    // format: OutputFormat::Unified
    // color: false
    // verbose: false
}
```

## Testing Integration

### Assert Equal with Better Diff

```rust
#[cfg(test)]
mod tests {
    use output_diffing_utility::{diff_text, format_compact, DiffConfig, DiffResult};

    #[test]
    fn test_output_matches_expected() {
        let expected = include_str!("../expected/output.txt");
        let actual = generate_output();  // Your function

        let config = DiffConfig::default();
        let result = diff_text(expected, actual, &config);

        if result.has_changes() {
            let diff_result = DiffResult::Text(result);
            let diff = format_compact(&diff_result);
            panic!("Output differs from expected:\n{}", diff);
        }
    }

    fn generate_output() -> &'static str {
        "generated output"
    }
}
```

### Snapshot Testing

```rust
use output_diffing_utility::{diff_json, DiffConfig};
use std::fs;

fn test_api_response() -> Result<(), Box<dyn std::error::Error>> {
    // Call API
    let response = call_api()?;

    // Load snapshot
    let snapshot = fs::read_to_string("snapshots/api_response.json")?;

    // Compare
    let config = DiffConfig::default();
    let result = diff_json(&snapshot, &response, &config)?;

    if result.has_changes() {
        eprintln!("API response differs from snapshot:");
        for change in &result.changes {
            eprintln!("{:?}", change);
        }

        // Update snapshot if desired
        if std::env::var("UPDATE_SNAPSHOTS").is_ok() {
            fs::write("snapshots/api_response.json", response)?;
        } else {
            panic!("Snapshot test failed");
        }
    }

    Ok(())
}

fn call_api() -> Result<String, Box<dyn std::error::Error>> {
    Ok(r#"{"status": "ok"}"#.to_string())
}
```

## Error Handling

### Handling JSON Parse Errors

```rust
use output_diffing_utility::diff_json;

fn main() {
    let invalid_json = "{invalid}";
    let valid_json = r#"{"valid": true}"#;

    match diff_json(invalid_json, valid_json, &Default::default()) {
        Ok(result) => {
            println!("Diff result: {:?}", result);
        }
        Err(e) => {
            eprintln!("JSON parse error: {}", e.message);
            // Fall back to text diff if needed
        }
    }
}
```

## Performance Tips

### Large File Comparison

```rust
use output_diffing_utility::{diff_text, DiffConfig};

fn main() {
    // Limit context for large files
    let config = DiffConfig {
        context_lines: 3,  // Instead of usize::MAX
        ..Default::default()
    };

    let large_text1 = "...";
    let large_text2 = "...";

    let result = diff_text(large_text1, large_text2, &config);
    // Faster processing with limited context
}
```

### Binary Early Exit

```rust
use output_diffing_utility::diff_binary;

fn main() {
    let data1 = vec![0; 1_000_000];  // 1 MB
    let mut data2 = vec![0; 1_000_000];
    data2[100] = 1;  // One byte differs

    let result = diff_binary(&data1, &data2, &Default::default());

    // Exits early at byte 100, doesn't compare full 1 MB
    assert!(result.first_diff_offset == Some(100));
}
```

## API Types Reference

### `DiffConfig`

```rust
pub struct DiffConfig {
    pub context_lines: usize,      // Context around changes
    pub format: OutputFormat,       // Output format
    pub color: bool,                // Enable ANSI colors
    pub verbose: bool,              // Debug logging
}
```

### `OutputFormat`

```rust
pub enum OutputFormat {
    Unified,      // Default: diff -u style
    Json,         // JSON report
    SideBySide,   // Column comparison
    Compact,      // Minimal output
}
```

### `FileType`

```rust
pub enum FileType {
    Text,    // Line-based diff
    Json,    // Structural diff
    Binary,  // Byte diff
}
```

### `TextDiffResult`

```rust
pub struct TextDiffResult {
    pub changes: Vec<LineChange>,
}

pub enum LineChange {
    Added { new_line_num: usize, line: String },
    Removed { old_line_num: usize, line: String },
    Unchanged { old_line_num: usize, new_line_num: usize, line: String },
}
```

### `JsonDiffResult`

```rust
pub struct JsonDiffResult {
    pub changes: Vec<JsonChange>,
}

pub enum JsonChange {
    Added { path: String, value: JsonValue },
    Removed { path: String, value: JsonValue },
    Modified { path: String, old_value: JsonValue, new_value: JsonValue },
}
```

### `BinaryDiffResult`

```rust
pub struct BinaryDiffResult {
    pub size1: usize,
    pub size2: usize,
    pub first_diff_offset: Option<usize>,
}
```

## Complete Example

```rust
use output_diffing_utility::{
    diff_text, diff_json, detect_file_type,
    format_unified_diff_with_color, DiffConfig, FileType
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = "data1.json";
    let file2 = "data2.json";

    // Read files
    let content1 = fs::read(file1)?;
    let content2 = fs::read(file2)?;

    // Detect type
    let type1 = detect_file_type(&content1, Some("json"));
    let type2 = detect_file_type(&content2, Some("json"));

    let config = DiffConfig {
        context_lines: 3,
        color: true,
        ..Default::default()
    };

    match (type1, type2) {
        (FileType::Json, FileType::Json) => {
            let text1 = std::str::from_utf8(&content1)?;
            let text2 = std::str::from_utf8(&content2)?;

            let result = diff_json(text1, text2, &config)?;

            if result.has_changes() {
                println!("JSON differences found:");
                for change in &result.changes {
                    println!("{:?}", change);
                }
            } else {
                println!("JSON files are identical");
            }
        }
        _ => {
            let text1 = std::str::from_utf8(&content1)?;
            let text2 = std::str::from_utf8(&content2)?;

            let result = diff_text(text1, text2, &config);
            let formatted = format_unified_diff_with_color(
                &result, file1, file2, true
            );
            println!("{}", formatted);
        }
    }

    Ok(())
}
```

## See Also

- [CLI Usage](/tools/output-diffing-utility/cli-usage) - Command-line interface
- [API Reference](/tools/output-diffing-utility/api-reference) - Full API docs
- [Examples](/tools/output-diffing-utility/examples) - More examples
