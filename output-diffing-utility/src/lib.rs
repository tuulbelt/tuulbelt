//! # Output Diffing Utility
//!
//! Semantic diff tool for JSON, text, and binary files with zero dependencies.
//!
//! ## Features
//!
//! - Zero runtime dependencies (uses only std library)
//! - Multiple diff types: text (LCS), JSON (structural), binary (byte-by-byte)
//! - Multiple output formats: unified, JSON, side-by-side, compact
//! - Cross-platform support
//! - Both library and CLI interfaces
//!
//! ## Example
//!
//! ```rust
//! use output_diffing_utility::{diff_text, DiffConfig};
//!
//! let old = "line 1\nline 2\nline 3";
//! let new = "line 1\nline 2 modified\nline 3";
//!
//! let config = DiffConfig::default();
//! let result = diff_text(old, new, &config);
//!
//! assert!(result.has_changes());
//! ```

mod json;
pub use json::{parse_json, JsonError, JsonValue};

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

/// Configuration for diff operations
#[derive(Debug, Clone)]
pub struct DiffConfig {
    /// Number of context lines to show around changes (text diffs)
    pub context_lines: usize,
    /// Output format
    pub format: OutputFormat,
    /// Enable colored output
    pub color: bool,
    /// Verbose output
    pub verbose: bool,
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            context_lines: usize::MAX, // Show all context by default
            format: OutputFormat::Unified,
            color: false,
            verbose: false,
        }
    }
}

/// Output format for diffs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Unified diff format (like diff -u)
    Unified,
    /// Structured JSON report
    Json,
    /// Side-by-side comparison
    SideBySide,
    /// Compact (only changes)
    Compact,
}

/// File type for diffing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// Text file (line-based diff)
    Text,
    /// JSON file (structural diff)
    Json,
    /// Binary file (byte-by-byte diff)
    Binary,
}

/// Detect file type from content and optionally extension
///
/// # Arguments
///
/// * `content` - File content as bytes
/// * `extension` - Optional file extension (e.g., "txt", "json")
///
/// # Returns
///
/// Returns the detected `FileType`.
///
/// # Detection Logic
///
/// 1. If extension is ".json", return FileType::Json
/// 2. If content is valid UTF-8 and contains mostly printable characters, return FileType::Text
/// 3. Otherwise, return FileType::Binary
///
/// # Example
///
/// ```rust
/// use output_diffing_utility::detect_file_type;
///
/// let text_content = b"hello world";
/// let file_type = detect_file_type(text_content, Some("txt"));
/// // Returns FileType::Text
/// ```
pub fn detect_file_type(content: &[u8], extension: Option<&str>) -> FileType {
    // Check extension first
    if let Some(ext) = extension {
        match ext.to_lowercase().as_str() {
            "json" => return FileType::Json,
            "txt" | "md" | "log" | "rs" | "ts" | "js" | "py" | "c" | "h" | "cpp" | "hpp" => {
                return FileType::Text
            }
            _ => {} // Continue to content-based detection
        }
    }

    // Check if content is valid UTF-8
    if let Ok(text) = std::str::from_utf8(content) {
        // Check if mostly printable (allow newlines, tabs, whitespace, Unicode)
        let printable_count = text
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
            .count();

        let total_chars = text.chars().count();

        // If >95% printable characters, consider it text
        if total_chars > 0 && (printable_count * 100 / total_chars) >= 95 {
            return FileType::Text;
        }
    }

    // Default to binary
    FileType::Binary
}

/// Error types for diff operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffError {
    /// File not found
    FileNotFound(String),
    /// IO error
    IoError(String),
    /// Invalid UTF-8 in text file
    InvalidUtf8(String),
    /// JSON parse error
    JsonParseError(String),
}

impl fmt::Display for DiffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffError::FileNotFound(path) => write!(f, "File not found: {}", path),
            DiffError::IoError(msg) => write!(f, "IO error: {}", msg),
            DiffError::InvalidUtf8(msg) => write!(f, "Invalid UTF-8: {}", msg),
            DiffError::JsonParseError(msg) => write!(f, "JSON parse error: {}", msg),
        }
    }
}

impl Error for DiffError {}

/// A single line change in a text diff
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineChange {
    /// Line unchanged (context)
    Unchanged {
        line: String,
        old_line_num: usize,
        new_line_num: usize,
    },
    /// Line added
    Added { line: String, new_line_num: usize },
    /// Line removed
    Removed { line: String, old_line_num: usize },
}

/// Result of a text diff
#[derive(Debug, Clone)]
pub struct TextDiffResult {
    /// Changes detected
    pub changes: Vec<LineChange>,
    /// Total lines in old file
    pub old_line_count: usize,
    /// Total lines in new file
    pub new_line_count: usize,
}

impl TextDiffResult {
    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        self.changes
            .iter()
            .any(|c| !matches!(c, LineChange::Unchanged { .. }))
    }

    /// Count additions
    pub fn additions(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| matches!(c, LineChange::Added { .. }))
            .count()
    }

    /// Count deletions
    pub fn deletions(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| matches!(c, LineChange::Removed { .. }))
            .count()
    }
}

/// Diff two text strings line-by-line using LCS algorithm
///
/// # Arguments
///
/// * `old` - The old text content
/// * `new` - The new text content
/// * `config` - Configuration options
///
/// # Returns
///
/// Returns a `TextDiffResult` containing the detected changes.
///
/// # Example
///
/// ```rust
/// use output_diffing_utility::{diff_text, DiffConfig};
///
/// let old = "line 1\nline 2";
/// let new = "line 1\nline 2 modified";
///
/// let result = diff_text(old, new, &DiffConfig::default());
/// assert!(result.has_changes());
/// assert_eq!(result.additions(), 1);
/// assert_eq!(result.deletions(), 1);
/// ```
pub fn diff_text(old: &str, new: &str, config: &DiffConfig) -> TextDiffResult {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    if config.verbose {
        eprintln!(
            "[DEBUG] Old lines: {}, New lines: {}",
            old_lines.len(),
            new_lines.len()
        );
    }

    let lcs = compute_lcs(&old_lines, &new_lines);
    let changes = build_changes_from_lcs(&old_lines, &new_lines, &lcs, config.context_lines);

    TextDiffResult {
        changes,
        old_line_count: old_lines.len(),
        new_line_count: new_lines.len(),
    }
}

/// Compute Longest Common Subsequence (LCS) using dynamic programming
///
/// Time complexity: O(m*n) where m, n are lengths of sequences
/// Space complexity: O(m*n)
fn compute_lcs<T: PartialEq>(a: &[T], b: &[T]) -> Vec<usize> {
    let m = a.len();
    let n = b.len();

    // Build DP table
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Backtrack to find LCS indices in b
    // Pre-allocate: LCS length is at most min(m, n)
    let mut lcs_indices = Vec::with_capacity(m.min(n));
    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            lcs_indices.push(j - 1);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    lcs_indices.reverse();
    lcs_indices
}

/// Build line changes from LCS result
fn build_changes_from_lcs(
    old_lines: &[&str],
    new_lines: &[&str],
    lcs_indices: &[usize],
    context_lines: usize,
) -> Vec<LineChange> {
    // Pre-allocate: worst case is all lines changed
    let mut all_changes = Vec::with_capacity(old_lines.len() + new_lines.len());
    let mut old_idx = 0;
    let mut new_idx = 0;
    let mut lcs_iter = lcs_indices.iter().peekable();

    // First pass: collect all changes
    while old_idx < old_lines.len() || new_idx < new_lines.len() {
        if let Some(&&lcs_new_idx) = lcs_iter.peek() {
            if new_idx == lcs_new_idx {
                // This line is in LCS (unchanged)
                all_changes.push(LineChange::Unchanged {
                    line: new_lines[new_idx].to_string(),
                    old_line_num: old_idx + 1,
                    new_line_num: new_idx + 1,
                });
                old_idx += 1;
                new_idx += 1;
                lcs_iter.next();
                continue;
            }
        }

        // Check if old_line exists in remaining LCS
        let old_line_in_lcs = old_idx < old_lines.len()
            && lcs_iter.clone().any(|&lcs_idx| {
                lcs_idx < new_lines.len() && old_lines[old_idx] == new_lines[lcs_idx]
            });

        if !old_line_in_lcs && old_idx < old_lines.len() {
            // Line removed
            all_changes.push(LineChange::Removed {
                line: old_lines[old_idx].to_string(),
                old_line_num: old_idx + 1,
            });
            old_idx += 1;
        } else if new_idx < new_lines.len() {
            // Line added
            all_changes.push(LineChange::Added {
                line: new_lines[new_idx].to_string(),
                new_line_num: new_idx + 1,
            });
            new_idx += 1;
        } else {
            break;
        }
    }

    // Second pass: filter to only include context around changes
    if context_lines == usize::MAX {
        // Show all context
        return all_changes;
    }

    // Pre-allocate: filtered result is at most the size of all_changes
    let mut filtered_changes = Vec::with_capacity(all_changes.len());
    let len = all_changes.len();

    // Find indices of actual changes (not unchanged)
    let change_indices: Vec<usize> = all_changes
        .iter()
        .enumerate()
        .filter_map(|(i, change)| {
            if !matches!(change, LineChange::Unchanged { .. }) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    if change_indices.is_empty() {
        // No changes, return empty
        return filtered_changes;
    }

    let mut included = vec![false; len];

    // Mark changes and their context
    for &change_idx in &change_indices {
        // Mark the change itself
        included[change_idx] = true;

        // Mark context before
        let start = change_idx.saturating_sub(context_lines);
        for item in included.iter_mut().take(change_idx).skip(start) {
            *item = true;
        }

        // Mark context after
        let end = (change_idx + context_lines + 1).min(len);
        for item in included.iter_mut().take(end).skip(change_idx + 1) {
            *item = true;
        }
    }

    // Build filtered list
    for (i, change) in all_changes.into_iter().enumerate() {
        if included[i] {
            filtered_changes.push(change);
        }
    }

    filtered_changes
}

/// Diff two files
pub fn diff_files(
    file1: &Path,
    file2: &Path,
    config: &DiffConfig,
) -> Result<TextDiffResult, DiffError> {
    let content1 = fs::read_to_string(file1)
        .map_err(|e| DiffError::IoError(format!("Error reading {}: {}", file1.display(), e)))?;

    let content2 = fs::read_to_string(file2)
        .map_err(|e| DiffError::IoError(format!("Error reading {}: {}", file2.display(), e)))?;

    Ok(diff_text(&content1, &content2, config))
}

/// Format diff result as unified diff
pub fn format_unified_diff(result: &TextDiffResult, file1_name: &str, file2_name: &str) -> String {
    format_unified_diff_with_color(result, file1_name, file2_name, false)
}

/// Format unified diff with optional ANSI colors
pub fn format_unified_diff_with_color(
    result: &TextDiffResult,
    file1_name: &str,
    file2_name: &str,
    color: bool,
) -> String {
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";

    let mut output = String::new();

    output.push_str(&format!("--- {}\n", file1_name));
    output.push_str(&format!("+++ {}\n", file2_name));

    for change in &result.changes {
        match change {
            LineChange::Unchanged { line, .. } => {
                output.push_str(&format!("  {}\n", line));
            }
            LineChange::Added { line, .. } => {
                if color {
                    output.push_str(&format!("{}+ {}{}\n", GREEN, line, RESET));
                } else {
                    output.push_str(&format!("+ {}\n", line));
                }
            }
            LineChange::Removed { line, .. } => {
                if color {
                    output.push_str(&format!("{}- {}{}\n", RED, line, RESET));
                } else {
                    output.push_str(&format!("- {}\n", line));
                }
            }
        }
    }

    output
}

// ============================================================================
// Binary Diff
// ============================================================================

/// A single byte difference in binary files
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteDifference {
    /// Offset in the file
    pub offset: usize,
    /// Old byte value (None if byte was added)
    pub old_byte: Option<u8>,
    /// New byte value (None if byte was removed)
    pub new_byte: Option<u8>,
}

/// Result of a binary diff
#[derive(Debug, Clone)]
pub struct BinaryDiffResult {
    /// Byte differences detected
    pub differences: Vec<ByteDifference>,
    /// Total bytes in old file
    pub old_size: usize,
    /// Total bytes in new file
    pub new_size: usize,
}

impl BinaryDiffResult {
    /// Check if files are identical
    pub fn is_identical(&self) -> bool {
        self.differences.is_empty()
    }

    /// Count byte additions
    pub fn additions(&self) -> usize {
        self.differences
            .iter()
            .filter(|d| d.old_byte.is_none())
            .count()
    }

    /// Count byte deletions
    pub fn deletions(&self) -> usize {
        self.differences
            .iter()
            .filter(|d| d.new_byte.is_none())
            .count()
    }

    /// Count byte modifications
    pub fn modifications(&self) -> usize {
        self.differences
            .iter()
            .filter(|d| d.old_byte.is_some() && d.new_byte.is_some())
            .count()
    }
}

/// Diff two byte arrays
///
/// # Arguments
///
/// * `old` - The old file contents
/// * `new` - The new file contents
/// * `config` - Configuration options
///
/// # Returns
///
/// Returns a `BinaryDiffResult` containing byte-level differences.
///
/// # Example
///
/// ```rust
/// use output_diffing_utility::{diff_binary, DiffConfig};
///
/// let old = b"hello";
/// let new = b"hallo";
///
/// let result = diff_binary(old, new, &DiffConfig::default());
/// assert!(!result.is_identical());
/// assert_eq!(result.modifications(), 1);
/// ```
pub fn diff_binary(old: &[u8], new: &[u8], config: &DiffConfig) -> BinaryDiffResult {
    let max_len = old.len().max(new.len());
    let mut differences = Vec::new();

    if config.verbose {
        eprintln!("[DEBUG] Old size: {}, New size: {}", old.len(), new.len());
    }

    for i in 0..max_len {
        let old_byte = old.get(i).copied();
        let new_byte = new.get(i).copied();

        if old_byte != new_byte {
            differences.push(ByteDifference {
                offset: i,
                old_byte,
                new_byte,
            });
        }
    }

    BinaryDiffResult {
        differences,
        old_size: old.len(),
        new_size: new.len(),
    }
}

/// Diff two binary files
pub fn diff_binary_files(
    file1: &Path,
    file2: &Path,
    config: &DiffConfig,
) -> Result<BinaryDiffResult, DiffError> {
    let content1 = fs::read(file1)
        .map_err(|e| DiffError::IoError(format!("Error reading {}: {}", file1.display(), e)))?;

    let content2 = fs::read(file2)
        .map_err(|e| DiffError::IoError(format!("Error reading {}: {}", file2.display(), e)))?;

    Ok(diff_binary(&content1, &content2, config))
}

/// Format binary diff result as hex dump
pub fn format_binary_diff(result: &BinaryDiffResult) -> String {
    format_binary_diff_with_color(result, false)
}

/// Format binary diff with optional ANSI colors
pub fn format_binary_diff_with_color(result: &BinaryDiffResult, color: bool) -> String {
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const CYAN: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";

    let mut output = String::new();

    output.push_str(&format!(
        "Binary files differ: {} bytes old, {} bytes new\n",
        result.old_size, result.new_size
    ));

    if result.differences.is_empty() {
        output.push_str("Files are identical\n");
        return output;
    }

    output.push_str(&format!("\n{} differences:\n\n", result.differences.len()));

    if color {
        output.push_str(&format!("{}Offset     | Old  | New{}\n", CYAN, RESET));
    } else {
        output.push_str("Offset     | Old  | New\n");
    }
    output.push_str("-----------|------|------\n");

    for diff in &result.differences {
        let old_hex = diff
            .old_byte
            .map(|b| format!("{:02x}", b))
            .unwrap_or_else(|| "--  ".to_string());

        let new_hex = diff
            .new_byte
            .map(|b| format!("{:02x}", b))
            .unwrap_or_else(|| "--  ".to_string());

        if color {
            // Color the offset in cyan, old bytes in red, new bytes in green
            let old_colored = if diff.old_byte.is_some() {
                format!("{}{}{}  ", RED, old_hex, RESET)
            } else {
                format!("{}--  {}", RED, RESET)
            };

            let new_colored = if diff.new_byte.is_some() {
                format!("{}{}  {}", GREEN, new_hex, RESET)
            } else {
                format!("{}--  {}", GREEN, RESET)
            };

            output.push_str(&format!(
                "{}0x{:08x}{} | {} | {}\n",
                CYAN, diff.offset, RESET, old_colored, new_colored
            ));
        } else {
            output.push_str(&format!(
                "0x{:08x} | {}   | {}\n",
                diff.offset, old_hex, new_hex
            ));
        }
    }

    output
}

// ============================================================================
// JSON Diff
// ============================================================================

/// A change in JSON structure
#[derive(Debug, Clone, PartialEq)]
pub enum JsonChange {
    /// Value added at path
    Added { path: String, value: JsonValue },
    /// Value removed from path
    Removed { path: String, value: JsonValue },
    /// Value modified at path
    Modified {
        path: String,
        old_value: JsonValue,
        new_value: JsonValue,
    },
    /// Type changed at path
    TypeChanged {
        path: String,
        old_value: JsonValue,
        new_value: JsonValue,
    },
}

/// Result of a JSON diff operation
#[derive(Debug, Clone, PartialEq)]
pub struct JsonDiffResult {
    /// List of detected changes
    pub changes: Vec<JsonChange>,
}

impl JsonDiffResult {
    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    /// Count additions
    pub fn additions(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| matches!(c, JsonChange::Added { .. }))
            .count()
    }

    /// Count deletions
    pub fn deletions(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| matches!(c, JsonChange::Removed { .. }))
            .count()
    }

    /// Count modifications
    pub fn modifications(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| {
                matches!(
                    c,
                    JsonChange::Modified { .. } | JsonChange::TypeChanged { .. }
                )
            })
            .count()
    }
}

/// Diff two JSON strings structurally
///
/// # Arguments
///
/// * `old` - The old JSON content
/// * `new` - The new JSON content
/// * `config` - Configuration options
///
/// # Returns
///
/// Returns a `Result<JsonDiffResult, JsonError>` containing the structural differences.
///
/// # Example
///
/// ```rust
/// use output_diffing_utility::{diff_json, DiffConfig};
///
/// let old = r#"{"name": "Alice", "age": 30}"#;
/// let new = r#"{"name": "Bob", "age": 30}"#;
///
/// let result = diff_json(old, new, &DiffConfig::default()).unwrap();
/// assert!(result.has_changes());
/// assert_eq!(result.modifications(), 1);
/// ```
pub fn diff_json(old: &str, new: &str, config: &DiffConfig) -> Result<JsonDiffResult, JsonError> {
    let old_value = parse_json(old)?;
    let new_value = parse_json(new)?;

    if config.verbose {
        eprintln!("[DEBUG] Comparing JSON structures");
    }

    let mut changes = Vec::new();
    compare_json_values("", &old_value, &new_value, &mut changes);

    Ok(JsonDiffResult { changes })
}

/// Recursively compare two JSON values and collect changes
fn compare_json_values(
    path: &str,
    old: &JsonValue,
    new: &JsonValue,
    changes: &mut Vec<JsonChange>,
) {
    use JsonValue::*;

    match (old, new) {
        // Both null - no change
        (Null, Null) => {}

        // Both bool - compare values
        (Bool(a), Bool(b)) => {
            if a != b {
                changes.push(JsonChange::Modified {
                    path: path.to_string(),
                    old_value: old.clone(),
                    new_value: new.clone(),
                });
            }
        }

        // Both number - compare values
        (Number(a), Number(b)) => {
            if (a - b).abs() > f64::EPSILON {
                changes.push(JsonChange::Modified {
                    path: path.to_string(),
                    old_value: old.clone(),
                    new_value: new.clone(),
                });
            }
        }

        // Both string - compare values
        (String(a), String(b)) => {
            if a != b {
                changes.push(JsonChange::Modified {
                    path: path.to_string(),
                    old_value: old.clone(),
                    new_value: new.clone(),
                });
            }
        }

        // Both array - compare elements
        (Array(old_arr), Array(new_arr)) => {
            let max_len = old_arr.len().max(new_arr.len());

            for i in 0..max_len {
                let element_path = if path.is_empty() {
                    format!("[{}]", i)
                } else {
                    format!("{}[{}]", path, i)
                };

                match (old_arr.get(i), new_arr.get(i)) {
                    (Some(old_elem), Some(new_elem)) => {
                        compare_json_values(&element_path, old_elem, new_elem, changes);
                    }
                    (Some(old_elem), None) => {
                        changes.push(JsonChange::Removed {
                            path: element_path,
                            value: old_elem.clone(),
                        });
                    }
                    (None, Some(new_elem)) => {
                        changes.push(JsonChange::Added {
                            path: element_path,
                            value: new_elem.clone(),
                        });
                    }
                    (None, None) => unreachable!(),
                }
            }
        }

        // Both object - compare keys and values
        (Object(old_obj), Object(new_obj)) => {
            // Collect all unique keys
            let mut all_keys = std::collections::HashSet::new();
            for (key, _) in old_obj {
                all_keys.insert(key.as_str());
            }
            for (key, _) in new_obj {
                all_keys.insert(key.as_str());
            }

            for key in all_keys {
                let field_path = if path.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", path, key)
                };

                let old_val = old_obj.iter().find(|(k, _)| k == key).map(|(_, v)| v);
                let new_val = new_obj.iter().find(|(k, _)| k == key).map(|(_, v)| v);

                match (old_val, new_val) {
                    (Some(old_v), Some(new_v)) => {
                        compare_json_values(&field_path, old_v, new_v, changes);
                    }
                    (Some(old_v), None) => {
                        changes.push(JsonChange::Removed {
                            path: field_path,
                            value: old_v.clone(),
                        });
                    }
                    (None, Some(new_v)) => {
                        changes.push(JsonChange::Added {
                            path: field_path,
                            value: new_v.clone(),
                        });
                    }
                    (None, None) => unreachable!(),
                }
            }
        }

        // Type mismatch
        _ => {
            changes.push(JsonChange::TypeChanged {
                path: path.to_string(),
                old_value: old.clone(),
                new_value: new.clone(),
            });
        }
    }
}

/// Format JSON diff result as human-readable output
pub fn format_json_diff(result: &JsonDiffResult) -> String {
    format_json_diff_with_color(result, false)
}

/// Format JSON diff with optional ANSI colors
pub fn format_json_diff_with_color(result: &JsonDiffResult, color: bool) -> String {
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const CYAN: &str = "\x1b[36m";
    const RESET: &str = "\x1b[0m";

    let mut output = String::new();

    if result.changes.is_empty() {
        output.push_str("No differences found\n");
        return output;
    }

    output.push_str(&format!(
        "JSON diff: {} change{}\n\n",
        result.changes.len(),
        if result.changes.len() == 1 { "" } else { "s" }
    ));

    for change in &result.changes {
        match change {
            JsonChange::Added { path, value } => {
                if color {
                    output.push_str(&format!(
                        "{}+ Added at '{}{}{}': {}{}\n",
                        GREEN,
                        CYAN,
                        path,
                        GREEN,
                        format_json_value(value),
                        RESET
                    ));
                } else {
                    output.push_str(&format!(
                        "+ Added at '{}': {}\n",
                        path,
                        format_json_value(value)
                    ));
                }
            }
            JsonChange::Removed { path, value } => {
                if color {
                    output.push_str(&format!(
                        "{}- Removed at '{}{}{}': {}{}\n",
                        RED,
                        CYAN,
                        path,
                        RED,
                        format_json_value(value),
                        RESET
                    ));
                } else {
                    output.push_str(&format!(
                        "- Removed at '{}': {}\n",
                        path,
                        format_json_value(value)
                    ));
                }
            }
            JsonChange::Modified {
                path,
                old_value,
                new_value,
            } => {
                if color {
                    output.push_str(&format!(
                        "{}~ Modified at '{}{}{}':\n  {}Old: {}{}\n  {}New: {}{}\n",
                        YELLOW,
                        CYAN,
                        path,
                        RESET,
                        RED,
                        format_json_value(old_value),
                        RESET,
                        GREEN,
                        format_json_value(new_value),
                        RESET
                    ));
                } else {
                    output.push_str(&format!(
                        "~ Modified at '{}':\n  Old: {}\n  New: {}\n",
                        path,
                        format_json_value(old_value),
                        format_json_value(new_value)
                    ));
                }
            }
            JsonChange::TypeChanged {
                path,
                old_value,
                new_value,
            } => {
                if color {
                    output.push_str(&format!(
                        "{}! Type changed at '{}{}{}':\n  {}Old: {} ({}){}\n  {}New: {} ({}){}\n",
                        YELLOW,
                        CYAN,
                        path,
                        RESET,
                        RED,
                        format_json_value(old_value),
                        json_type_name(old_value),
                        RESET,
                        GREEN,
                        format_json_value(new_value),
                        json_type_name(new_value),
                        RESET
                    ));
                } else {
                    output.push_str(&format!(
                        "! Type changed at '{}':\n  Old: {} ({})\n  New: {} ({})\n",
                        path,
                        format_json_value(old_value),
                        json_type_name(old_value),
                        format_json_value(new_value),
                        json_type_name(new_value)
                    ));
                }
            }
        }
    }

    output
}

/// Format a JSON value as a compact string
fn format_json_value(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else if arr.len() <= 3 {
                let items: Vec<String> = arr.iter().map(format_json_value).collect();
                format!("[{}]", items.join(", "))
            } else {
                format!("[...{} items...]", arr.len())
            }
        }
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else if obj.len() <= 3 {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, format_json_value(v)))
                    .collect();
                format!("{{{}}}", items.join(", "))
            } else {
                format!("{{...{} fields...}}", obj.len())
            }
        }
    }
}

/// Get the type name of a JSON value
fn json_type_name(value: &JsonValue) -> &'static str {
    match value {
        JsonValue::Null => "null",
        JsonValue::Bool(_) => "boolean",
        JsonValue::Number(_) => "number",
        JsonValue::String(_) => "string",
        JsonValue::Array(_) => "array",
        JsonValue::Object(_) => "object",
    }
}

// ============================================================================
// Additional Output Formats
// ============================================================================

/// Unified diff result type for formatting
#[derive(Debug)]
pub enum DiffResult {
    /// Text diff result
    Text(TextDiffResult),
    /// Binary diff result
    Binary(BinaryDiffResult),
    /// JSON diff result
    Json(JsonDiffResult),
}

impl DiffResult {
    /// Check if files are identical
    pub fn is_identical(&self) -> bool {
        match self {
            DiffResult::Text(r) => !r.has_changes(),
            DiffResult::Binary(r) => r.is_identical(),
            DiffResult::Json(r) => !r.has_changes(),
        }
    }

    /// Get additions count
    pub fn additions(&self) -> usize {
        match self {
            DiffResult::Text(r) => r.additions(),
            DiffResult::Binary(r) => r.additions(),
            DiffResult::Json(r) => r.additions(),
        }
    }

    /// Get deletions count
    pub fn deletions(&self) -> usize {
        match self {
            DiffResult::Text(r) => r.deletions(),
            DiffResult::Binary(r) => r.deletions(),
            DiffResult::Json(r) => r.deletions(),
        }
    }

    /// Get modifications count
    pub fn modifications(&self) -> usize {
        match self {
            DiffResult::Text(_) => 0, // Text doesn't track mods separately
            DiffResult::Binary(r) => r.modifications(),
            DiffResult::Json(r) => r.modifications(),
        }
    }

    /// Get total changes count
    pub fn total_changes(&self) -> usize {
        self.additions() + self.deletions() + self.modifications()
    }
}

/// Format diff result as JSON report
pub fn format_as_json_report(
    result: &DiffResult,
    file1_name: &str,
    file2_name: &str,
    file_type: FileType,
) -> String {
    let type_str = match file_type {
        FileType::Text => "text",
        FileType::Json => "json",
        FileType::Binary => "binary",
    };

    let mut output = String::from("{\n");
    output.push_str("  \"format\": \"json\",\n");
    output.push_str(&format!(
        "  \"file1\": \"{}\",\n",
        escape_json_string(file1_name)
    ));
    output.push_str(&format!(
        "  \"file2\": \"{}\",\n",
        escape_json_string(file2_name)
    ));
    output.push_str(&format!("  \"identical\": {},\n", result.is_identical()));
    output.push_str(&format!("  \"file_type\": \"{}\",\n", type_str));

    // Differences array
    output.push_str("  \"differences\": [\n");

    match result {
        DiffResult::Text(text_result) => {
            let mut first = true;
            for change in &text_result.changes {
                match change {
                    LineChange::Added { new_line_num, line } => {
                        if !first {
                            output.push_str(",\n");
                        }
                        output.push_str(&format!(
                            "    {{\n      \"type\": \"added\",\n      \"line\": {},\n      \"content\": \"{}\"\n    }}",
                            new_line_num,
                            escape_json_string(line)
                        ));
                        first = false;
                    }
                    LineChange::Removed { old_line_num, line } => {
                        if !first {
                            output.push_str(",\n");
                        }
                        output.push_str(&format!(
                            "    {{\n      \"type\": \"removed\",\n      \"line\": {},\n      \"content\": \"{}\"\n    }}",
                            old_line_num,
                            escape_json_string(line)
                        ));
                        first = false;
                    }
                    LineChange::Unchanged { .. } => {}
                }
            }
        }
        DiffResult::Binary(binary_result) => {
            let mut first = true;
            for diff in &binary_result.differences {
                if !first {
                    output.push_str(",\n");
                }
                output.push_str(&format!(
                    "    {{\n      \"offset\": {},\n      \"old_byte\": {},\n      \"new_byte\": {}\n    }}",
                    diff.offset,
                    diff.old_byte.map(|b| format!("{}", b)).unwrap_or_else(|| "null".to_string()),
                    diff.new_byte.map(|b| format!("{}", b)).unwrap_or_else(|| "null".to_string())
                ));
                first = false;
            }
        }
        DiffResult::Json(json_result) => {
            let mut first = true;
            for change in &json_result.changes {
                if !first {
                    output.push_str(",\n");
                }
                match change {
                    JsonChange::Added { path, value } => {
                        output.push_str(&format!(
                            "    {{\n      \"type\": \"added\",\n      \"path\": \"{}\",\n      \"value\": \"{}\"\n    }}",
                            escape_json_string(path),
                            escape_json_string(&format_json_value(value))
                        ));
                    }
                    JsonChange::Removed { path, value } => {
                        output.push_str(&format!(
                            "    {{\n      \"type\": \"removed\",\n      \"path\": \"{}\",\n      \"value\": \"{}\"\n    }}",
                            escape_json_string(path),
                            escape_json_string(&format_json_value(value))
                        ));
                    }
                    JsonChange::Modified {
                        path,
                        old_value,
                        new_value,
                    } => {
                        output.push_str(&format!(
                            "    {{\n      \"type\": \"modified\",\n      \"path\": \"{}\",\n      \"old_value\": \"{}\",\n      \"new_value\": \"{}\"\n    }}",
                            escape_json_string(path),
                            escape_json_string(&format_json_value(old_value)),
                            escape_json_string(&format_json_value(new_value))
                        ));
                    }
                    JsonChange::TypeChanged {
                        path,
                        old_value,
                        new_value,
                    } => {
                        output.push_str(&format!(
                            "    {{\n      \"type\": \"type_changed\",\n      \"path\": \"{}\",\n      \"old_value\": \"{}\",\n      \"old_type\": \"{}\",\n      \"new_value\": \"{}\",\n      \"new_type\": \"{}\"\n    }}",
                            escape_json_string(path),
                            escape_json_string(&format_json_value(old_value)),
                            json_type_name(old_value),
                            escape_json_string(&format_json_value(new_value)),
                            json_type_name(new_value)
                        ));
                    }
                }
                first = false;
            }
        }
    }

    output.push_str("\n  ],\n");

    // Summary
    output.push_str("  \"summary\": {\n");
    output.push_str(&format!(
        "    \"total_changes\": {},\n",
        result.total_changes()
    ));
    output.push_str(&format!("    \"additions\": {},\n", result.additions()));
    output.push_str(&format!("    \"deletions\": {},\n", result.deletions()));
    output.push_str(&format!(
        "    \"modifications\": {}\n",
        result.modifications()
    ));
    output.push_str("  }\n");
    output.push_str("}\n");

    output
}

/// Escape string for JSON output
fn escape_json_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            c if c.is_control() => format!("\\u{:04x}", c as u32),
            c => c.to_string(),
        })
        .collect()
}

/// Format diff result in compact mode (only changes)
pub fn format_compact(result: &DiffResult) -> String {
    let mut output = String::new();

    match result {
        DiffResult::Text(text_result) => {
            if !text_result.has_changes() {
                output.push_str("No differences\n");
                return output;
            }

            for change in &text_result.changes {
                match change {
                    LineChange::Added { new_line_num, line } => {
                        output.push_str(&format!("+ Line {}: {}\n", new_line_num, line));
                    }
                    LineChange::Removed { old_line_num, line } => {
                        output.push_str(&format!("- Line {}: {}\n", old_line_num, line));
                    }
                    LineChange::Unchanged { .. } => {}
                }
            }

            output.push_str(&format!(
                "\n{} change{} ({} addition{}, {} deletion{})\n",
                text_result.additions() + text_result.deletions(),
                if text_result.additions() + text_result.deletions() == 1 {
                    ""
                } else {
                    "s"
                },
                text_result.additions(),
                if text_result.additions() == 1 {
                    ""
                } else {
                    "s"
                },
                text_result.deletions(),
                if text_result.deletions() == 1 {
                    ""
                } else {
                    "s"
                }
            ));
        }
        DiffResult::Binary(binary_result) => {
            if binary_result.is_identical() {
                output.push_str("No differences\n");
                return output;
            }

            for diff in &binary_result.differences {
                output.push_str(&format!(
                    "0x{:08x}: {} → {}\n",
                    diff.offset,
                    diff.old_byte
                        .map(|b| format!("{:02x}", b))
                        .unwrap_or_else(|| "--".to_string()),
                    diff.new_byte
                        .map(|b| format!("{:02x}", b))
                        .unwrap_or_else(|| "--".to_string())
                ));
            }

            output.push_str(&format!(
                "\n{} byte{} differ\n",
                binary_result.differences.len(),
                if binary_result.differences.len() == 1 {
                    ""
                } else {
                    "s"
                }
            ));
        }
        DiffResult::Json(json_result) => {
            if !json_result.has_changes() {
                output.push_str("No differences\n");
                return output;
            }

            for change in &json_result.changes {
                match change {
                    JsonChange::Added { path, value } => {
                        output.push_str(&format!("+ {}: {}\n", path, format_json_value(value)));
                    }
                    JsonChange::Removed { path, value } => {
                        output.push_str(&format!("- {}: {}\n", path, format_json_value(value)));
                    }
                    JsonChange::Modified {
                        path,
                        old_value,
                        new_value,
                    } => {
                        output.push_str(&format!(
                            "~ {}: {} → {}\n",
                            path,
                            format_json_value(old_value),
                            format_json_value(new_value)
                        ));
                    }
                    JsonChange::TypeChanged {
                        path,
                        old_value,
                        new_value,
                    } => {
                        output.push_str(&format!(
                            "! {}: {} ({}) → {} ({})\n",
                            path,
                            format_json_value(old_value),
                            json_type_name(old_value),
                            format_json_value(new_value),
                            json_type_name(new_value)
                        ));
                    }
                }
            }

            output.push_str(&format!(
                "\n{} change{} ({} addition{}, {} deletion{}, {} modification{})\n",
                json_result.changes.len(),
                if json_result.changes.len() == 1 {
                    ""
                } else {
                    "s"
                },
                json_result.additions(),
                if json_result.additions() == 1 {
                    ""
                } else {
                    "s"
                },
                json_result.deletions(),
                if json_result.deletions() == 1 {
                    ""
                } else {
                    "s"
                },
                json_result.modifications(),
                if json_result.modifications() == 1 {
                    ""
                } else {
                    "s"
                }
            ));
        }
    }

    output
}

/// Format diff result in side-by-side mode
pub fn format_side_by_side(
    result: &DiffResult,
    file1_name: &str,
    file2_name: &str,
    width: usize,
) -> String {
    let half_width = (width - 3) / 2;
    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "{:<width$} | {}\n",
        file1_name,
        file2_name,
        width = half_width
    ));
    output.push_str(&format!(
        "{:-<width$}-+-{:-<width$}\n",
        "",
        "",
        width = half_width
    ));

    match result {
        DiffResult::Text(text_result) => {
            for change in &text_result.changes {
                match change {
                    LineChange::Unchanged { line, .. } => {
                        let truncated = truncate_to_width(line, half_width);
                        output.push_str(&format!(
                            "{:<width$} | {}\n",
                            truncated,
                            truncated,
                            width = half_width
                        ));
                    }
                    LineChange::Added { line, .. } => {
                        let truncated = truncate_to_width(line, half_width);
                        output.push_str(&format!(
                            "{:<width$} > {}\n",
                            "",
                            truncated,
                            width = half_width
                        ));
                    }
                    LineChange::Removed { line, .. } => {
                        let truncated = truncate_to_width(line, half_width);
                        output.push_str(&format!(
                            "{:<width$} < {}\n",
                            truncated,
                            "",
                            width = half_width
                        ));
                    }
                }
            }
        }
        DiffResult::Binary(_) => {
            output.push_str("Side-by-side view not supported for binary files\n");
        }
        DiffResult::Json(_) => {
            output.push_str("Side-by-side view not supported for JSON diffs\n");
            output.push_str("Use --format compact or --format json instead\n");
        }
    }

    output
}

/// Truncate string to fit within width
fn truncate_to_width(s: &str, width: usize) -> String {
    if s.len() <= width {
        s.to_string()
    } else {
        format!("{}...", &s[..width.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_text_identical() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2", "line 1\nline 2", &config);
        assert!(!result.has_changes());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_diff_text_one_line_added() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2", "line 1\nline 2\nline 3", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 1);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_diff_text_one_line_removed() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2\nline 3", "line 1\nline 2", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 1);
    }

    #[test]
    fn test_diff_text_one_line_modified() {
        let config = DiffConfig::default();
        let result = diff_text(
            "line 1\nline 2\nline 3",
            "line 1\nline 2 modified\nline 3",
            &config,
        );
        assert!(result.has_changes());
        assert_eq!(result.additions(), 1);
        assert_eq!(result.deletions(), 1);
    }

    #[test]
    fn test_diff_text_empty_files() {
        let config = DiffConfig::default();
        let result = diff_text("", "", &config);
        assert!(!result.has_changes());
    }

    #[test]
    fn test_diff_text_empty_to_content() {
        let config = DiffConfig::default();
        let result = diff_text("", "line 1\nline 2", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 2);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_format_unified_diff() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2", "line 1\nline 2 modified", &config);
        let formatted = format_unified_diff(&result, "old.txt", "new.txt");

        assert!(formatted.contains("--- old.txt"));
        assert!(formatted.contains("+++ new.txt"));
        assert!(formatted.contains("- line 2"));
        assert!(formatted.contains("+ line 2 modified"));
    }

    #[test]
    fn test_compute_lcs_simple() {
        let a = vec!["a", "b", "c"];
        let b = vec!["a", "b", "c"];
        let lcs = compute_lcs(&a, &b);
        assert_eq!(lcs, vec![0, 1, 2]);
    }

    #[test]
    fn test_compute_lcs_with_changes() {
        let a = vec!["a", "b", "c"];
        let b = vec!["a", "x", "c"];
        let lcs = compute_lcs(&a, &b);
        // LCS is "a", "c" at indices 0, 2 in b
        assert_eq!(lcs, vec![0, 2]);
    }

    // Edge case tests
    #[test]
    fn test_diff_text_content_to_empty() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2", "", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 2);
    }

    #[test]
    fn test_diff_text_completely_different() {
        let config = DiffConfig::default();
        let result = diff_text("a\nb\nc", "x\ny\nz", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 3);
        assert_eq!(result.deletions(), 3);
    }

    #[test]
    fn test_diff_text_whitespace_changes() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2", "line 1\nline  2", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 1);
        assert_eq!(result.deletions(), 1);
    }

    #[test]
    fn test_diff_text_large_common_prefix() {
        let config = DiffConfig::default();
        let mut old_lines = vec!["same"; 100];
        old_lines.push("different1");
        let mut new_lines = vec!["same"; 100];
        new_lines.push("different2");

        let old_text = old_lines.join("\n");
        let new_text = new_lines.join("\n");

        let result = diff_text(&old_text, &new_text, &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 1);
        assert_eq!(result.deletions(), 1);
    }

    #[test]
    fn test_diff_text_multiline_insert() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 4", "line 1\nline 2\nline 3\nline 4", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 2);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_diff_text_multiline_delete() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2\nline 3\nline 4", "line 1\nline 4", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 2);
    }

    #[test]
    fn test_diff_text_trailing_newlines() {
        let config = DiffConfig::default();
        // In Rust, lines() ignores trailing newlines, so these are identical
        let result = diff_text("line 1\nline 2\n", "line 1\nline 2", &config);
        assert!(!result.has_changes());
    }

    #[test]
    fn test_diff_text_unicode() {
        let config = DiffConfig::default();
        let result = diff_text("café\nrésumé", "café\nresume", &config);
        assert!(result.has_changes());
        assert_eq!(result.additions(), 1);
        assert_eq!(result.deletions(), 1);
    }

    #[test]
    fn test_diff_text_very_long_lines() {
        let config = DiffConfig::default();
        let long_line1 = "a".repeat(10000);
        let long_line2 = "b".repeat(10000);
        let result = diff_text(&long_line1, &long_line2, &config);
        assert!(result.has_changes());
    }

    #[test]
    fn test_format_unified_diff_additions_only() {
        let config = DiffConfig::default();
        let result = diff_text("line 1", "line 1\nline 2", &config);
        let formatted = format_unified_diff(&result, "old.txt", "new.txt");

        assert!(formatted.contains("--- old.txt"));
        assert!(formatted.contains("+++ new.txt"));
        assert!(formatted.contains("+ line 2"));
    }

    #[test]
    fn test_format_unified_diff_deletions_only() {
        let config = DiffConfig::default();
        let result = diff_text("line 1\nline 2", "line 1", &config);
        let formatted = format_unified_diff(&result, "old.txt", "new.txt");

        assert!(formatted.contains("--- old.txt"));
        assert!(formatted.contains("+++ new.txt"));
        assert!(formatted.contains("- line 2"));
    }

    #[test]
    fn test_compute_lcs_empty_sequences() {
        let a: Vec<&str> = vec![];
        let b: Vec<&str> = vec![];
        let lcs = compute_lcs(&a, &b);
        assert_eq!(lcs, vec![]);
    }

    #[test]
    fn test_compute_lcs_one_empty() {
        let a = vec!["a", "b", "c"];
        let b: Vec<&str> = vec![];
        let lcs = compute_lcs(&a, &b);
        assert_eq!(lcs, vec![]);
    }

    #[test]
    fn test_compute_lcs_completely_different() {
        let a = vec!["a", "b", "c"];
        let b = vec!["x", "y", "z"];
        let lcs = compute_lcs(&a, &b);
        assert_eq!(lcs, vec![]);
    }

    // Binary diff tests
    #[test]
    fn test_diff_binary_identical() {
        let config = DiffConfig::default();
        let result = diff_binary(b"hello", b"hello", &config);
        assert!(result.is_identical());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 0);
        assert_eq!(result.modifications(), 0);
    }

    #[test]
    fn test_diff_binary_one_byte_changed() {
        let config = DiffConfig::default();
        let result = diff_binary(b"hello", b"hallo", &config);
        assert!(!result.is_identical());
        assert_eq!(result.modifications(), 1);
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 0);
        assert_eq!(result.differences.len(), 1);
        assert_eq!(result.differences[0].offset, 1);
        assert_eq!(result.differences[0].old_byte, Some(b'e'));
        assert_eq!(result.differences[0].new_byte, Some(b'a'));
    }

    #[test]
    fn test_diff_binary_bytes_added() {
        let config = DiffConfig::default();
        let result = diff_binary(b"hello", b"hello world", &config);
        assert!(!result.is_identical());
        assert_eq!(result.additions(), 6); // " world"
        assert_eq!(result.deletions(), 0);
        assert_eq!(result.modifications(), 0);
    }

    #[test]
    fn test_diff_binary_bytes_removed() {
        let config = DiffConfig::default();
        let result = diff_binary(b"hello world", b"hello", &config);
        assert!(!result.is_identical());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 6); // " world"
        assert_eq!(result.modifications(), 0);
    }

    #[test]
    fn test_diff_binary_empty_files() {
        let config = DiffConfig::default();
        let result = diff_binary(b"", b"", &config);
        assert!(result.is_identical());
        assert_eq!(result.old_size, 0);
        assert_eq!(result.new_size, 0);
    }

    #[test]
    fn test_diff_binary_empty_to_content() {
        let config = DiffConfig::default();
        let result = diff_binary(b"", b"hello", &config);
        assert!(!result.is_identical());
        assert_eq!(result.additions(), 5);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_diff_binary_content_to_empty() {
        let config = DiffConfig::default();
        let result = diff_binary(b"hello", b"", &config);
        assert!(!result.is_identical());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 5);
    }

    #[test]
    fn test_diff_binary_completely_different() {
        let config = DiffConfig::default();
        let result = diff_binary(b"abc", b"xyz", &config);
        assert!(!result.is_identical());
        assert_eq!(result.modifications(), 3);
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_diff_binary_null_bytes() {
        let config = DiffConfig::default();
        let result = diff_binary(b"\x00\x01\x02", b"\x00\x00\x02", &config);
        assert!(!result.is_identical());
        assert_eq!(result.modifications(), 1);
        assert_eq!(result.differences[0].offset, 1);
        assert_eq!(result.differences[0].old_byte, Some(0x01));
        assert_eq!(result.differences[0].new_byte, Some(0x00));
    }

    #[test]
    fn test_diff_binary_large_files() {
        let config = DiffConfig::default();
        let old = vec![0u8; 10000];
        let mut new = vec![0u8; 10000];
        new[5000] = 1;
        let result = diff_binary(&old, &new, &config);
        assert!(!result.is_identical());
        assert_eq!(result.modifications(), 1);
        assert_eq!(result.differences[0].offset, 5000);
    }

    #[test]
    fn test_format_binary_diff_identical() {
        let config = DiffConfig::default();
        let result = diff_binary(b"test", b"test", &config);
        let formatted = format_binary_diff(&result);
        assert!(formatted.contains("Files are identical"));
    }

    #[test]
    fn test_format_binary_diff_with_changes() {
        let config = DiffConfig::default();
        let result = diff_binary(b"hello", b"hallo", &config);
        let formatted = format_binary_diff(&result);
        assert!(formatted.contains("1 differences"));
        assert!(formatted.contains("0x00000001"));
        assert!(formatted.contains("65")); // 'e' = 0x65
        assert!(formatted.contains("61")); // 'a' = 0x61
    }

    #[test]
    fn test_binary_diff_mixed_changes() {
        let config = DiffConfig::default();
        let result = diff_binary(b"abcd", b"aXcdef", &config);
        assert!(!result.is_identical());
        // Position 1: b->X (modification)
        // Position 3: d->d (same, not counted)
        // Positions 4-5: e, f (additions)
        assert_eq!(result.modifications(), 1);
        assert_eq!(result.additions(), 2);
    }

    // File type detection tests
    #[test]
    fn test_detect_file_type_text_from_extension() {
        assert_eq!(detect_file_type(b"hello", Some("txt")), FileType::Text);
        assert_eq!(detect_file_type(b"hello", Some("md")), FileType::Text);
        assert_eq!(detect_file_type(b"hello", Some("rs")), FileType::Text);
        assert_eq!(detect_file_type(b"hello", Some("py")), FileType::Text);
    }

    #[test]
    fn test_detect_file_type_json_from_extension() {
        assert_eq!(detect_file_type(b"{}", Some("json")), FileType::Json);
        assert_eq!(detect_file_type(b"[]", Some("JSON")), FileType::Json); // Case insensitive
    }

    #[test]
    fn test_detect_file_type_text_from_content() {
        assert_eq!(
            detect_file_type(b"hello world\nhow are you?", None),
            FileType::Text
        );
        assert_eq!(
            detect_file_type(b"Line 1\nLine 2\nLine 3", None),
            FileType::Text
        );
    }

    #[test]
    fn test_detect_file_type_binary_from_content() {
        // Binary data (null bytes)
        assert_eq!(
            detect_file_type(b"\x00\x01\x02\x03", None),
            FileType::Binary
        );

        // Mixed binary/text (mostly binary)
        assert_eq!(
            detect_file_type(b"abc\x00\x01\x02\x03\x04\x05\x06", None),
            FileType::Binary
        );
    }

    #[test]
    fn test_detect_file_type_unicode_text() {
        // UTF-8 encoded text should be detected as text
        assert_eq!(
            detect_file_type("café résumé".as_bytes(), None),
            FileType::Text
        );
    }

    #[test]
    fn test_detect_file_type_empty_file() {
        // Empty content with no extension - default to binary
        assert_eq!(detect_file_type(b"", None), FileType::Binary);

        // Empty with text extension - use extension
        assert_eq!(detect_file_type(b"", Some("txt")), FileType::Text);
    }

    #[test]
    fn test_detect_file_type_mixed_content() {
        // Mostly printable (>95%)
        let content = b"hello world\nthis is text\x01"; // 1 non-printable in 29 chars
        assert_eq!(detect_file_type(content, None), FileType::Text);
    }

    #[test]
    fn test_detect_file_type_unknown_extension() {
        // Unknown extension falls back to content detection
        assert_eq!(
            detect_file_type(b"hello world", Some("xyz")),
            FileType::Text
        );
        assert_eq!(
            detect_file_type(b"\x00\x01\x02", Some("xyz")),
            FileType::Binary
        );
    }

    // JSON diff tests
    #[test]
    fn test_json_diff_identical() {
        let config = DiffConfig::default();
        let json = r#"{"name": "Alice", "age": 30}"#;
        let result = diff_json(json, json, &config).unwrap();
        assert!(!result.has_changes());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 0);
        assert_eq!(result.modifications(), 0);
    }

    #[test]
    fn test_json_diff_modified_string() {
        let config = DiffConfig::default();
        let old = r#"{"name": "Alice"}"#;
        let new = r#"{"name": "Bob"}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1);
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_json_diff_modified_number() {
        let config = DiffConfig::default();
        let old = r#"{"age": 30}"#;
        let new = r#"{"age": 31}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1);
    }

    #[test]
    fn test_json_diff_modified_bool() {
        let config = DiffConfig::default();
        let old = r#"{"active": true}"#;
        let new = r#"{"active": false}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1);
    }

    #[test]
    fn test_json_diff_added_field() {
        let config = DiffConfig::default();
        let old = r#"{"name": "Alice"}"#;
        let new = r#"{"name": "Alice", "age": 30}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.additions(), 1);
        assert_eq!(result.deletions(), 0);
        assert_eq!(result.modifications(), 0);
    }

    #[test]
    fn test_json_diff_removed_field() {
        let config = DiffConfig::default();
        let old = r#"{"name": "Alice", "age": 30}"#;
        let new = r#"{"name": "Alice"}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 1);
        assert_eq!(result.modifications(), 0);
    }

    #[test]
    fn test_json_diff_type_changed() {
        let config = DiffConfig::default();
        let old = r#"{"value": 42}"#;
        let new = r#"{"value": "42"}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        // Type change counts as modification
        assert_eq!(result.modifications(), 1);
    }

    #[test]
    fn test_json_diff_nested_object() {
        let config = DiffConfig::default();
        let old = r#"{"user": {"name": "Alice", "age": 30}}"#;
        let new = r#"{"user": {"name": "Bob", "age": 30}}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1);

        // Check path is correct
        assert_eq!(result.changes.len(), 1);
        if let JsonChange::Modified { path, .. } = &result.changes[0] {
            assert_eq!(path, "user.name");
        } else {
            panic!("Expected Modified change");
        }
    }

    #[test]
    fn test_json_diff_array_elements() {
        let config = DiffConfig::default();
        let old = r#"{"items": [1, 2, 3]}"#;
        let new = r#"{"items": [1, 5, 3]}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1);

        // Check path is correct
        if let JsonChange::Modified { path, .. } = &result.changes[0] {
            assert_eq!(path, "items[1]");
        } else {
            panic!("Expected Modified change");
        }
    }

    #[test]
    fn test_json_diff_array_added_element() {
        let config = DiffConfig::default();
        let old = r#"{"items": [1, 2]}"#;
        let new = r#"{"items": [1, 2, 3]}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.additions(), 1);
        assert_eq!(result.deletions(), 0);
    }

    #[test]
    fn test_json_diff_array_removed_element() {
        let config = DiffConfig::default();
        let old = r#"{"items": [1, 2, 3]}"#;
        let new = r#"{"items": [1, 2]}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.additions(), 0);
        assert_eq!(result.deletions(), 1);
    }

    #[test]
    fn test_json_diff_root_array() {
        let config = DiffConfig::default();
        let old = r#"[1, 2, 3]"#;
        let new = r#"[1, 5, 3]"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1);

        // Root array paths should start with [index]
        if let JsonChange::Modified { path, .. } = &result.changes[0] {
            assert_eq!(path, "[1]");
        } else {
            panic!("Expected Modified change");
        }
    }

    #[test]
    fn test_json_diff_complex_nested() {
        let config = DiffConfig::default();
        let old = r#"{
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ]
        }"#;
        let new = r#"{
            "users": [
                {"name": "Alice", "age": 31},
                {"name": "Bob", "age": 25}
            ]
        }"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1);

        if let JsonChange::Modified { path, .. } = &result.changes[0] {
            assert_eq!(path, "users[0].age");
        } else {
            panic!("Expected Modified change");
        }
    }

    #[test]
    fn test_json_diff_multiple_changes() {
        let config = DiffConfig::default();
        let old = r#"{"name": "Alice", "age": 30}"#;
        let new = r#"{"name": "Bob", "age": 31, "city": "NYC"}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        // 2 modifications (name, age) + 1 addition (city)
        assert_eq!(result.modifications(), 2);
        assert_eq!(result.additions(), 1);
        assert_eq!(result.changes.len(), 3);
    }

    #[test]
    fn test_json_diff_empty_objects() {
        let config = DiffConfig::default();
        let result = diff_json("{}", "{}", &config).unwrap();
        assert!(!result.has_changes());
    }

    #[test]
    fn test_json_diff_empty_arrays() {
        let config = DiffConfig::default();
        let result = diff_json("[]", "[]", &config).unwrap();
        assert!(!result.has_changes());
    }

    #[test]
    fn test_json_diff_null_values() {
        let config = DiffConfig::default();
        let old = r#"{"value": null}"#;
        let new = r#"{"value": 42}"#;
        let result = diff_json(old, new, &config).unwrap();
        assert!(result.has_changes());
        assert_eq!(result.modifications(), 1); // Type change
    }

    #[test]
    fn test_format_json_diff_no_changes() {
        let result = JsonDiffResult { changes: vec![] };
        let formatted = format_json_diff(&result);
        assert!(formatted.contains("No differences found"));
    }

    #[test]
    fn test_format_json_diff_with_changes() {
        let result = JsonDiffResult {
            changes: vec![JsonChange::Modified {
                path: "name".to_string(),
                old_value: JsonValue::String("Alice".to_string()),
                new_value: JsonValue::String("Bob".to_string()),
            }],
        };
        let formatted = format_json_diff(&result);
        assert!(formatted.contains("JSON diff: 1 change"));
        assert!(formatted.contains("Modified at 'name'"));
        assert!(formatted.contains("Alice"));
        assert!(formatted.contains("Bob"));
    }

    #[test]
    fn test_format_json_diff_added() {
        let result = JsonDiffResult {
            changes: vec![JsonChange::Added {
                path: "city".to_string(),
                value: JsonValue::String("NYC".to_string()),
            }],
        };
        let formatted = format_json_diff(&result);
        assert!(formatted.contains("Added at 'city'"));
        assert!(formatted.contains("NYC"));
    }

    #[test]
    fn test_format_json_diff_removed() {
        let result = JsonDiffResult {
            changes: vec![JsonChange::Removed {
                path: "age".to_string(),
                value: JsonValue::Number(30.0),
            }],
        };
        let formatted = format_json_diff(&result);
        assert!(formatted.contains("Removed at 'age'"));
        assert!(formatted.contains("30"));
    }

    #[test]
    fn test_format_json_diff_type_changed() {
        let result = JsonDiffResult {
            changes: vec![JsonChange::TypeChanged {
                path: "value".to_string(),
                old_value: JsonValue::Number(42.0),
                new_value: JsonValue::String("42".to_string()),
            }],
        };
        let formatted = format_json_diff(&result);
        assert!(formatted.contains("Type changed at 'value'"));
        assert!(formatted.contains("number"));
        assert!(formatted.contains("string"));
    }
}
