//! # Snapshot Comparison
//!
//! Snapshot testing utility for regression detection with integrated diff output.
//!
//! ## Features
//!
//! - Store and compare snapshots for regression testing
//! - Integrated with output-diffing-utility for rich diff display
//! - Support for text, JSON, and binary snapshots
//! - Hash-based fast comparison with detailed diff on mismatch
//! - CLI and library interfaces
//!
//! ## Example
//!
//! ```rust,no_run
//! use snapshot_comparison::{SnapshotStore, SnapshotConfig};
//! use std::path::PathBuf;
//!
//! let store = SnapshotStore::new(PathBuf::from("snapshots"));
//! let config = SnapshotConfig::default();
//!
//! // Create a snapshot
//! store.create("my-test", b"expected output", &config).unwrap();
//!
//! // Check against snapshot
//! let result = store.check("my-test", b"expected output", &config).unwrap();
//! assert!(result.matches());
//! ```

use output_diffing_utility::{
    detect_file_type, diff_binary, diff_json, diff_text, format_binary_diff_with_color,
    format_json_diff_with_color, format_unified_diff_with_color, BinaryDiffResult, DiffConfig,
    JsonDiffResult, TextDiffResult,
};

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// Re-export FileType for convenience
pub use output_diffing_utility::FileType;

/// Configuration for snapshot operations
#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    /// File type hint (None = auto-detect)
    pub file_type: Option<FileType>,
    /// Enable colored diff output
    pub color: bool,
    /// Number of context lines in diff output
    pub context_lines: usize,
    /// Update mode: if true, update snapshot instead of failing
    pub update_mode: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            file_type: None,
            color: false,
            context_lines: 3,
            update_mode: false,
        }
    }
}

/// Error types for snapshot operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    /// Snapshot not found
    NotFound(String),
    /// IO error
    IoError(String),
    /// Invalid snapshot name (path traversal, etc.)
    InvalidName(String),
    /// Snapshot file is corrupted
    CorruptedSnapshot(String),
    /// Snapshot content is not valid UTF-8 (for text/JSON comparison)
    InvalidUtf8(String),
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnapshotError::NotFound(name) => write!(f, "Snapshot not found: {}", name),
            SnapshotError::IoError(msg) => write!(f, "IO error: {}", msg),
            SnapshotError::InvalidName(name) => write!(f, "Invalid snapshot name: {}", name),
            SnapshotError::CorruptedSnapshot(msg) => write!(f, "Corrupted snapshot: {}", msg),
            SnapshotError::InvalidUtf8(msg) => write!(f, "Invalid UTF-8: {}", msg),
        }
    }
}

impl Error for SnapshotError {}

/// Metadata for a snapshot
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotMetadata {
    /// Name of the snapshot
    pub name: String,
    /// Unix timestamp when created
    pub created_at: u64,
    /// Unix timestamp when last updated
    pub updated_at: u64,
    /// Hash of content (hex string)
    pub content_hash: String,
    /// Content size in bytes
    pub size: usize,
    /// Detected or specified file type
    pub file_type: FileType,
}

/// A stored snapshot
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Snapshot metadata
    pub metadata: SnapshotMetadata,
    /// Snapshot content
    pub content: Vec<u8>,
}

/// Result of a snapshot comparison
#[derive(Debug)]
pub enum CompareResult {
    /// Snapshot matches current content
    Match,
    /// Snapshot differs from current content
    Mismatch {
        /// The stored snapshot
        snapshot: Snapshot,
        /// The current content being compared
        actual: Vec<u8>,
        /// Detailed diff result
        diff: DiffOutput,
    },
    /// Snapshot was updated (in update mode)
    Updated {
        /// The old snapshot (before update)
        old_snapshot: Snapshot,
        /// The new content
        new_content: Vec<u8>,
    },
}

impl CompareResult {
    /// Check if the comparison was a match
    pub fn matches(&self) -> bool {
        matches!(self, CompareResult::Match)
    }

    /// Check if the snapshot was updated
    pub fn was_updated(&self) -> bool {
        matches!(self, CompareResult::Updated { .. })
    }

    /// Get the diff output if there was a mismatch
    pub fn diff(&self) -> Option<&DiffOutput> {
        match self {
            CompareResult::Mismatch { diff, .. } => Some(diff),
            _ => None,
        }
    }
}

/// Diff output from odiff integration
#[derive(Debug)]
pub enum DiffOutput {
    /// Text diff result
    Text(TextDiffResult),
    /// JSON diff result
    Json(JsonDiffResult),
    /// Binary diff result
    Binary(BinaryDiffResult),
}

impl DiffOutput {
    /// Format the diff as a human-readable string
    pub fn format(&self, snapshot_name: &str, color: bool) -> String {
        match self {
            DiffOutput::Text(result) => format_unified_diff_with_color(
                result,
                &format!("{}.snap", snapshot_name),
                "actual",
                color,
            ),
            DiffOutput::Json(result) => format_json_diff_with_color(result, color),
            DiffOutput::Binary(result) => format_binary_diff_with_color(result, color),
        }
    }

    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        match self {
            DiffOutput::Text(r) => r.has_changes(),
            DiffOutput::Json(r) => r.has_changes(),
            DiffOutput::Binary(r) => !r.is_identical(),
        }
    }
}

/// Manages snapshot storage and comparison
#[derive(Debug, Clone)]
pub struct SnapshotStore {
    /// Base directory for storing snapshots
    base_dir: PathBuf,
}

impl SnapshotStore {
    /// Create a new snapshot store at the given directory
    ///
    /// # Arguments
    ///
    /// * `base_dir` - Directory to store snapshots
    ///
    /// # Example
    ///
    /// ```rust
    /// use snapshot_comparison::SnapshotStore;
    /// use std::path::PathBuf;
    ///
    /// let store = SnapshotStore::new(PathBuf::from("snapshots"));
    /// ```
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Get the base directory
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// Get the path to a snapshot file
    fn snapshot_path(&self, name: &str) -> Result<PathBuf, SnapshotError> {
        Self::validate_name(name)?;
        Ok(self.base_dir.join(format!("{}.snap", name)))
    }

    /// Validate a snapshot name
    fn validate_name(name: &str) -> Result<(), SnapshotError> {
        if name.is_empty() {
            return Err(SnapshotError::InvalidName(
                "Snapshot name cannot be empty".to_string(),
            ));
        }

        // Security: prevent path traversal
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            return Err(SnapshotError::InvalidName(format!(
                "Snapshot name contains invalid characters: {}",
                name
            )));
        }

        // Security: prevent null bytes
        if name.contains('\0') {
            return Err(SnapshotError::InvalidName(
                "Snapshot name contains null bytes".to_string(),
            ));
        }

        // Only allow safe characters
        let valid = name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.');

        if !valid {
            return Err(SnapshotError::InvalidName(format!(
                "Snapshot name contains invalid characters: {}. Use alphanumeric, -, _, or .",
                name
            )));
        }

        Ok(())
    }

    /// Compute hash of content using FNV-1a algorithm
    fn compute_hash(content: &[u8]) -> String {
        // FNV-1a hash - simple, fast, good distribution
        const FNV_OFFSET: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;

        let mut hash = FNV_OFFSET;
        for &byte in content {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        format!("{:016x}", hash)
    }

    /// Create a new snapshot
    ///
    /// # Arguments
    ///
    /// * `name` - Snapshot identifier
    /// * `content` - Content to store
    /// * `config` - Snapshot configuration
    ///
    /// # Returns
    ///
    /// Returns the created `Snapshot` or an error.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use snapshot_comparison::{SnapshotStore, SnapshotConfig};
    /// use std::path::PathBuf;
    ///
    /// let store = SnapshotStore::new(PathBuf::from("snapshots"));
    /// let snapshot = store.create("my-test", b"expected output", &SnapshotConfig::default()).unwrap();
    /// ```
    pub fn create(
        &self,
        name: &str,
        content: &[u8],
        config: &SnapshotConfig,
    ) -> Result<Snapshot, SnapshotError> {
        let path = self.snapshot_path(name)?;

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                SnapshotError::IoError(format!("Failed to create directory: {}", e))
            })?;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let file_type = config
            .file_type
            .unwrap_or_else(|| detect_file_type(content, None));

        let metadata = SnapshotMetadata {
            name: name.to_string(),
            created_at: now,
            updated_at: now,
            content_hash: Self::compute_hash(content),
            size: content.len(),
            file_type,
        };

        let snapshot = Snapshot {
            metadata: metadata.clone(),
            content: content.to_vec(),
        };

        self.write_snapshot(&path, &snapshot)?;

        Ok(snapshot)
    }

    /// Write a snapshot to disk
    fn write_snapshot(&self, path: &Path, snapshot: &Snapshot) -> Result<(), SnapshotError> {
        let mut file = fs::File::create(path).map_err(|e| {
            SnapshotError::IoError(format!("Failed to create snapshot file: {}", e))
        })?;

        // Write header
        writeln!(file, "# Snapshot: {}", snapshot.metadata.name)
            .map_err(|e| SnapshotError::IoError(e.to_string()))?;
        writeln!(file, "# Created: {}", snapshot.metadata.created_at)
            .map_err(|e| SnapshotError::IoError(e.to_string()))?;
        writeln!(file, "# Updated: {}", snapshot.metadata.updated_at)
            .map_err(|e| SnapshotError::IoError(e.to_string()))?;
        writeln!(file, "# Hash: {}", snapshot.metadata.content_hash)
            .map_err(|e| SnapshotError::IoError(e.to_string()))?;
        writeln!(file, "# Size: {}", snapshot.metadata.size)
            .map_err(|e| SnapshotError::IoError(e.to_string()))?;
        writeln!(
            file,
            "# Type: {}",
            match snapshot.metadata.file_type {
                FileType::Text => "text",
                FileType::Json => "json",
                FileType::Binary => "binary",
            }
        )
        .map_err(|e| SnapshotError::IoError(e.to_string()))?;
        writeln!(file, "---").map_err(|e| SnapshotError::IoError(e.to_string()))?;

        // Write content
        file.write_all(&snapshot.content).map_err(|e| {
            SnapshotError::IoError(format!("Failed to write snapshot content: {}", e))
        })?;

        Ok(())
    }

    /// Read a snapshot from disk
    ///
    /// # Arguments
    ///
    /// * `name` - Snapshot identifier
    ///
    /// # Returns
    ///
    /// Returns the `Snapshot` or an error if not found.
    pub fn read(&self, name: &str) -> Result<Snapshot, SnapshotError> {
        let path = self.snapshot_path(name)?;

        if !path.exists() {
            return Err(SnapshotError::NotFound(name.to_string()));
        }

        let file = fs::File::open(&path)
            .map_err(|e| SnapshotError::IoError(format!("Failed to open snapshot: {}", e)))?;

        let mut reader = BufReader::new(file);
        let mut header = HashMap::new();
        let mut content = Vec::new();

        // Parse header lines until we hit the separator
        loop {
            let mut line = String::new();
            let bytes_read = reader
                .read_line(&mut line)
                .map_err(|e| SnapshotError::IoError(e.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            if line.trim() == "---" {
                // Read all remaining content after the separator
                reader
                    .read_to_end(&mut content)
                    .map_err(|e| SnapshotError::IoError(e.to_string()))?;
                break;
            } else if let Some(stripped) = line.strip_prefix("# ") {
                if let Some((key, value)) = stripped.trim().split_once(": ") {
                    header.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Parse metadata
        let snapshot_name = header
            .get("Snapshot")
            .cloned()
            .unwrap_or_else(|| name.to_string());
        let created_at = header
            .get("Created")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let updated_at = header
            .get("Updated")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let content_hash = header
            .get("Hash")
            .cloned()
            .unwrap_or_else(|| Self::compute_hash(&content));
        let size = header
            .get("Size")
            .and_then(|s| s.parse().ok())
            .unwrap_or(content.len());
        let file_type = header
            .get("Type")
            .map(|s| match s.as_str() {
                "json" => FileType::Json,
                "binary" => FileType::Binary,
                _ => FileType::Text,
            })
            .unwrap_or_else(|| detect_file_type(&content, None));

        let metadata = SnapshotMetadata {
            name: snapshot_name,
            created_at,
            updated_at,
            content_hash,
            size,
            file_type,
        };

        Ok(Snapshot { metadata, content })
    }

    /// Check current content against a stored snapshot
    ///
    /// # Arguments
    ///
    /// * `name` - Snapshot identifier
    /// * `actual` - Current content to compare
    /// * `config` - Snapshot configuration
    ///
    /// # Returns
    ///
    /// Returns `CompareResult::Match` if content matches, `CompareResult::Mismatch` with diff otherwise.
    /// If `config.update_mode` is true and content differs, updates the snapshot and returns `CompareResult::Updated`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use snapshot_comparison::{SnapshotStore, SnapshotConfig};
    /// use std::path::PathBuf;
    ///
    /// let store = SnapshotStore::new(PathBuf::from("snapshots"));
    /// let result = store.check("my-test", b"actual output", &SnapshotConfig::default()).unwrap();
    ///
    /// if result.matches() {
    ///     println!("Snapshot matches!");
    /// } else if let Some(diff) = result.diff() {
    ///     println!("Mismatch:\n{}", diff.format("my-test", true));
    /// }
    /// ```
    pub fn check(
        &self,
        name: &str,
        actual: &[u8],
        config: &SnapshotConfig,
    ) -> Result<CompareResult, SnapshotError> {
        let snapshot = self.read(name)?;

        // Fast path: hash comparison
        let actual_hash = Self::compute_hash(actual);
        if snapshot.metadata.content_hash == actual_hash {
            return Ok(CompareResult::Match);
        }

        // Content differs - compute detailed diff using odiff
        let diff_config = DiffConfig {
            context_lines: config.context_lines,
            color: config.color,
            ..Default::default()
        };

        let file_type = config.file_type.unwrap_or(snapshot.metadata.file_type);

        let diff = match file_type {
            FileType::Text => {
                let expected_str = std::str::from_utf8(&snapshot.content)
                    .map_err(|e| SnapshotError::InvalidUtf8(e.to_string()))?;
                let actual_str = std::str::from_utf8(actual)
                    .map_err(|e| SnapshotError::InvalidUtf8(e.to_string()))?;
                DiffOutput::Text(diff_text(expected_str, actual_str, &diff_config))
            }
            FileType::Json => {
                let expected_str = std::str::from_utf8(&snapshot.content)
                    .map_err(|e| SnapshotError::InvalidUtf8(e.to_string()))?;
                let actual_str = std::str::from_utf8(actual)
                    .map_err(|e| SnapshotError::InvalidUtf8(e.to_string()))?;
                match diff_json(expected_str, actual_str, &diff_config) {
                    Ok(result) => DiffOutput::Json(result),
                    Err(_) => {
                        // Fall back to text diff if JSON parsing fails
                        DiffOutput::Text(diff_text(expected_str, actual_str, &diff_config))
                    }
                }
            }
            FileType::Binary => {
                DiffOutput::Binary(diff_binary(&snapshot.content, actual, &diff_config))
            }
        };

        // Update mode: update snapshot instead of failing
        if config.update_mode {
            let old_snapshot = snapshot.clone();
            self.update(name, actual, config)?;
            return Ok(CompareResult::Updated {
                old_snapshot,
                new_content: actual.to_vec(),
            });
        }

        Ok(CompareResult::Mismatch {
            snapshot,
            actual: actual.to_vec(),
            diff,
        })
    }

    /// Update an existing snapshot
    ///
    /// # Arguments
    ///
    /// * `name` - Snapshot identifier
    /// * `content` - New content
    /// * `config` - Snapshot configuration
    ///
    /// # Returns
    ///
    /// Returns the updated `Snapshot` or an error.
    pub fn update(
        &self,
        name: &str,
        content: &[u8],
        config: &SnapshotConfig,
    ) -> Result<Snapshot, SnapshotError> {
        let path = self.snapshot_path(name)?;

        // Read existing snapshot to preserve created_at
        let created_at = if path.exists() {
            self.read(name)
                .map(|s| s.metadata.created_at)
                .unwrap_or_else(|_| {
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0)
                })
        } else {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let file_type = config
            .file_type
            .unwrap_or_else(|| detect_file_type(content, None));

        let metadata = SnapshotMetadata {
            name: name.to_string(),
            created_at,
            updated_at: now,
            content_hash: Self::compute_hash(content),
            size: content.len(),
            file_type,
        };

        let snapshot = Snapshot {
            metadata,
            content: content.to_vec(),
        };

        self.write_snapshot(&path, &snapshot)?;

        Ok(snapshot)
    }

    /// Delete a snapshot
    ///
    /// # Arguments
    ///
    /// * `name` - Snapshot identifier
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if deleted, or an error if not found.
    pub fn delete(&self, name: &str) -> Result<(), SnapshotError> {
        let path = self.snapshot_path(name)?;

        if !path.exists() {
            return Err(SnapshotError::NotFound(name.to_string()));
        }

        fs::remove_file(&path)
            .map_err(|e| SnapshotError::IoError(format!("Failed to delete snapshot: {}", e)))?;

        Ok(())
    }

    /// Check if a snapshot exists
    ///
    /// # Arguments
    ///
    /// * `name` - Snapshot identifier
    ///
    /// # Returns
    ///
    /// Returns `true` if snapshot exists.
    pub fn exists(&self, name: &str) -> Result<bool, SnapshotError> {
        let path = self.snapshot_path(name)?;
        Ok(path.exists())
    }

    /// List all snapshots in the store
    ///
    /// # Returns
    ///
    /// Returns a vector of snapshot metadata.
    pub fn list(&self) -> Result<Vec<SnapshotMetadata>, SnapshotError> {
        if !self.base_dir.exists() {
            return Ok(vec![]);
        }

        let entries = fs::read_dir(&self.base_dir)
            .map_err(|e| SnapshotError::IoError(format!("Failed to read directory: {}", e)))?;

        let mut snapshots = Vec::new();

        for entry in entries {
            let entry = entry
                .map_err(|e| SnapshotError::IoError(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.extension().map(|e| e == "snap").unwrap_or(false) {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    match self.read(name) {
                        Ok(snapshot) => snapshots.push(snapshot.metadata),
                        Err(_) => continue, // Skip corrupted snapshots
                    }
                }
            }
        }

        // Sort by name
        snapshots.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(snapshots)
    }

    /// Clean orphaned or stale snapshots
    ///
    /// # Arguments
    ///
    /// * `valid_names` - List of snapshot names that should be kept
    /// * `dry_run` - If true, only report what would be deleted
    ///
    /// # Returns
    ///
    /// Returns a list of snapshot names that were (or would be) deleted.
    pub fn clean(&self, valid_names: &[&str], dry_run: bool) -> Result<Vec<String>, SnapshotError> {
        let all_snapshots = self.list()?;
        let valid_set: std::collections::HashSet<&str> = valid_names.iter().copied().collect();

        let mut deleted = Vec::new();

        for snapshot in all_snapshots {
            if !valid_set.contains(snapshot.name.as_str()) {
                if !dry_run {
                    self.delete(&snapshot.name)?;
                }
                deleted.push(snapshot.name);
            }
        }

        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "snapcmp-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_create_and_read_snapshot() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        let content = b"hello world";
        let snapshot = store.create("test-1", content, &config).unwrap();

        assert_eq!(snapshot.metadata.name, "test-1");
        assert_eq!(snapshot.content, content);
        assert_eq!(snapshot.metadata.size, content.len());

        // Read back
        let read_snapshot = store.read("test-1").unwrap();
        assert_eq!(read_snapshot.metadata.name, "test-1");
        assert_eq!(read_snapshot.content, content);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_check_matching_snapshot() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        let content = b"hello world";
        store.create("test-match", content, &config).unwrap();

        let result = store.check("test-match", content, &config).unwrap();
        assert!(result.matches());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_check_mismatching_snapshot() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store.create("test-mismatch", b"expected", &config).unwrap();

        let result = store.check("test-mismatch", b"actual", &config).unwrap();
        assert!(!result.matches());
        assert!(result.diff().is_some());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_update_snapshot() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store
            .create("test-update", b"old content", &config)
            .unwrap();
        let updated = store
            .update("test-update", b"new content", &config)
            .unwrap();

        assert_eq!(updated.content, b"new content");

        let read = store.read("test-update").unwrap();
        assert_eq!(read.content, b"new content");

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_delete_snapshot() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store.create("test-delete", b"content", &config).unwrap();
        assert!(store.exists("test-delete").unwrap());

        store.delete("test-delete").unwrap();
        assert!(!store.exists("test-delete").unwrap());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_list_snapshots() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store.create("snap-a", b"a", &config).unwrap();
        store.create("snap-b", b"b", &config).unwrap();
        store.create("snap-c", b"c", &config).unwrap();

        let list = store.list().unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].name, "snap-a");
        assert_eq!(list[1].name, "snap-b");
        assert_eq!(list[2].name, "snap-c");

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_clean_snapshots() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store.create("keep-1", b"keep", &config).unwrap();
        store.create("keep-2", b"keep", &config).unwrap();
        store.create("orphan-1", b"orphan", &config).unwrap();
        store.create("orphan-2", b"orphan", &config).unwrap();

        let deleted = store.clean(&["keep-1", "keep-2"], false).unwrap();
        assert_eq!(deleted.len(), 2);
        assert!(deleted.contains(&"orphan-1".to_string()));
        assert!(deleted.contains(&"orphan-2".to_string()));

        let remaining = store.list().unwrap();
        assert_eq!(remaining.len(), 2);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_update_mode() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());

        let config = SnapshotConfig::default();
        store.create("test-update-mode", b"old", &config).unwrap();

        let update_config = SnapshotConfig {
            update_mode: true,
            ..Default::default()
        };

        let result = store
            .check("test-update-mode", b"new", &update_config)
            .unwrap();
        assert!(result.was_updated());

        // Verify update was applied
        let read = store.read("test-update-mode").unwrap();
        assert_eq!(read.content, b"new");

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_json_snapshot() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig {
            file_type: Some(FileType::Json),
            ..Default::default()
        };

        let old_json = br#"{"name": "Alice", "age": 30}"#;
        let new_json = br#"{"name": "Bob", "age": 30}"#;

        store.create("test-json", old_json, &config).unwrap();

        let result = store.check("test-json", new_json, &config).unwrap();
        assert!(!result.matches());

        if let Some(diff) = result.diff() {
            let formatted = diff.format("test-json", false);
            assert!(
                formatted.contains("name")
                    || formatted.contains("Alice")
                    || formatted.contains("Bob")
            );
        }

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_binary_snapshot() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig {
            file_type: Some(FileType::Binary),
            ..Default::default()
        };

        let old_binary = &[0x00, 0x01, 0x02, 0x03];
        let new_binary = &[0x00, 0xFF, 0x02, 0x03];

        store.create("test-binary", old_binary, &config).unwrap();

        let result = store.check("test-binary", new_binary, &config).unwrap();
        assert!(!result.matches());

        if let Some(DiffOutput::Binary(binary_diff)) = result.diff() {
            assert!(!binary_diff.is_identical());
            assert_eq!(binary_diff.differences.len(), 1);
        } else {
            panic!("Expected binary diff");
        }

        fs::remove_dir_all(dir).unwrap();
    }

    // Security tests
    #[test]
    fn test_path_traversal_prevention() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        // These should all fail
        assert!(store.create("../evil", b"hack", &config).is_err());
        assert!(store.create("foo/../bar", b"hack", &config).is_err());
        assert!(store.create("foo/bar", b"hack", &config).is_err());
        assert!(store.create("foo\\bar", b"hack", &config).is_err());
        assert!(store.create("foo\0bar", b"hack", &config).is_err());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_empty_name_rejected() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        assert!(store.create("", b"content", &config).is_err());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_snapshot_not_found() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());

        let result = store.read("nonexistent");
        assert!(matches!(result, Err(SnapshotError::NotFound(_))));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_diff_output_format() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store
            .create("test-format", b"line 1\nline 2\nline 3", &config)
            .unwrap();

        let result = store
            .check("test-format", b"line 1\nmodified\nline 3", &config)
            .unwrap();

        if let Some(diff) = result.diff() {
            let formatted = diff.format("test-format", false);
            assert!(formatted.contains("line 2") || formatted.contains("modified"));
        }

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_hash_consistency() {
        let content = b"hello world";
        let hash1 = SnapshotStore::compute_hash(content);
        let hash2 = SnapshotStore::compute_hash(content);
        assert_eq!(hash1, hash2);

        let different_content = b"hello world!";
        let hash3 = SnapshotStore::compute_hash(different_content);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_empty_content() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store.create("empty", b"", &config).unwrap();
        let snapshot = store.read("empty").unwrap();
        assert!(snapshot.content.is_empty());
        assert_eq!(snapshot.metadata.size, 0);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_large_content() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        let large_content: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
        store.create("large", &large_content, &config).unwrap();
        let snapshot = store.read("large").unwrap();
        assert_eq!(snapshot.content, large_content);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_unicode_content() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        let unicode_content = "Hello 世界 🌍 café résumé".as_bytes();
        store.create("unicode", unicode_content, &config).unwrap();
        let snapshot = store.read("unicode").unwrap();
        assert_eq!(snapshot.content, unicode_content);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_clean_dry_run() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        store.create("keep", b"keep", &config).unwrap();
        store.create("orphan", b"orphan", &config).unwrap();

        let deleted = store.clean(&["keep"], true).unwrap();
        assert_eq!(deleted.len(), 1);
        assert!(deleted.contains(&"orphan".to_string()));

        // Verify nothing was actually deleted
        let remaining = store.list().unwrap();
        assert_eq!(remaining.len(), 2);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_special_characters_in_name() {
        let dir = temp_dir();
        let store = SnapshotStore::new(dir.clone());
        let config = SnapshotConfig::default();

        // These should work
        store.create("test-name", b"content", &config).unwrap();
        store.create("test_name", b"content", &config).unwrap();
        store.create("test.name", b"content", &config).unwrap();
        store.create("Test123", b"content", &config).unwrap();

        // These should fail
        assert!(store.create("test name", b"content", &config).is_err());
        assert!(store.create("test@name", b"content", &config).is_err());
        assert!(store.create("test#name", b"content", &config).is_err());

        fs::remove_dir_all(dir).unwrap();
    }
}
