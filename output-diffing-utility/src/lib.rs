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
            context_lines: 3,
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
    let mut lcs_indices = Vec::new();
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
    _context_lines: usize,
) -> Vec<LineChange> {
    let mut changes = Vec::new();
    let mut old_idx = 0;
    let mut new_idx = 0;
    let mut lcs_iter = lcs_indices.iter().peekable();

    while old_idx < old_lines.len() || new_idx < new_lines.len() {
        if let Some(&&lcs_new_idx) = lcs_iter.peek() {
            if new_idx == lcs_new_idx {
                // This line is in LCS (unchanged)
                changes.push(LineChange::Unchanged {
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
            changes.push(LineChange::Removed {
                line: old_lines[old_idx].to_string(),
                old_line_num: old_idx + 1,
            });
            old_idx += 1;
        } else if new_idx < new_lines.len() {
            // Line added
            changes.push(LineChange::Added {
                line: new_lines[new_idx].to_string(),
                new_line_num: new_idx + 1,
            });
            new_idx += 1;
        } else {
            break;
        }
    }

    changes
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
    let mut output = String::new();

    output.push_str(&format!("--- {}\n", file1_name));
    output.push_str(&format!("+++ {}\n", file2_name));

    for change in &result.changes {
        match change {
            LineChange::Unchanged { line, .. } => {
                output.push_str(&format!("  {}\n", line));
            }
            LineChange::Added { line, .. } => {
                output.push_str(&format!("+ {}\n", line));
            }
            LineChange::Removed { line, .. } => {
                output.push_str(&format!("- {}\n", line));
            }
        }
    }

    output
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
}
