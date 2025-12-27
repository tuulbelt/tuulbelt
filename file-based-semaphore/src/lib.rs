//! # File-Based Semaphore
//!
//! Cross-platform file-based semaphore for process coordination.
//!
//! ## Features
//!
//! - Zero runtime dependencies (uses only Rust standard library)
//! - Cross-platform support (Linux, macOS, Windows)
//! - Both library and CLI interfaces
//! - Stale lock detection and automatic cleanup
//! - RAII-style guards for automatic release
//!
//! ## Example
//!
//! ```rust,no_run
//! use file_based_semaphore::{Semaphore, SemaphoreConfig};
//! use std::time::Duration;
//!
//! let config = SemaphoreConfig {
//!     stale_timeout: Some(Duration::from_secs(60)),
//!     ..Default::default()
//! };
//!
//! let semaphore = Semaphore::new("/tmp/my-lock.lock", config).unwrap();
//!
//! // Try to acquire the lock
//! {
//!     if let Ok(guard) = semaphore.try_acquire() {
//!         // Lock acquired, do work...
//!         // Lock automatically released when guard is dropped
//!     }
//! }; // Explicit scope ensures guard is dropped before semaphore
//! ```

use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Error type for semaphore operations
#[derive(Debug)]
pub enum SemaphoreError {
    /// Lock file already exists and is held by another process
    AlreadyLocked {
        /// PID of the process holding the lock (if available)
        holder_pid: Option<u32>,
        /// When the lock was acquired (if available)
        locked_since: Option<u64>,
    },
    /// Lock file path is invalid
    InvalidPath(String),
    /// IO error during lock operations
    IoError(io::Error),
    /// Lock was not held when trying to release
    NotLocked,
    /// Timeout waiting for lock
    Timeout,
    /// Failed to parse lock file contents
    ParseError(String),
}

impl fmt::Display for SemaphoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemaphoreError::AlreadyLocked {
                holder_pid,
                locked_since,
            } => {
                write!(f, "Lock already held")?;
                if let Some(pid) = holder_pid {
                    write!(f, " by PID {}", pid)?;
                }
                if let Some(since) = locked_since {
                    write!(f, " since {}", since)?;
                }
                Ok(())
            }
            SemaphoreError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            SemaphoreError::IoError(e) => write!(f, "IO error: {}", e),
            SemaphoreError::NotLocked => write!(f, "Lock not held"),
            SemaphoreError::Timeout => write!(f, "Timeout waiting for lock"),
            SemaphoreError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for SemaphoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SemaphoreError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for SemaphoreError {
    fn from(err: io::Error) -> Self {
        SemaphoreError::IoError(err)
    }
}

/// Configuration for semaphore behavior
#[derive(Debug, Clone)]
pub struct SemaphoreConfig {
    /// Timeout for stale lock detection. If a lock is older than this duration,
    /// it can be forcibly removed. `None` means no stale detection.
    pub stale_timeout: Option<Duration>,
    /// Retry interval when waiting for a lock
    pub retry_interval: Duration,
    /// Maximum time to wait when acquiring a lock. `None` means wait forever.
    pub acquire_timeout: Option<Duration>,
}

impl Default for SemaphoreConfig {
    fn default() -> Self {
        Self {
            stale_timeout: Some(Duration::from_secs(3600)), // 1 hour default
            retry_interval: Duration::from_millis(100),
            acquire_timeout: None,
        }
    }
}

/// Lock file contents
#[derive(Debug, Clone)]
pub struct LockInfo {
    /// Process ID that holds the lock
    pub pid: u32,
    /// Unix timestamp when lock was acquired
    pub timestamp: u64,
    /// Optional description/tag
    pub tag: Option<String>,
}

impl LockInfo {
    /// Create a new lock info for the current process
    pub fn new() -> Self {
        Self {
            pid: process::id(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            tag: None,
        }
    }

    /// Create lock info with a custom tag
    pub fn with_tag(tag: impl Into<String>) -> Self {
        let mut info = Self::new();
        info.tag = Some(tag.into());
        info
    }

    /// Serialize to string for writing to lock file
    ///
    /// Note: Newlines in tags are sanitized to spaces to prevent injection attacks.
    pub fn serialize(&self) -> String {
        let mut content = format!("pid={}\ntimestamp={}\n", self.pid, self.timestamp);
        if let Some(ref tag) = self.tag {
            // Sanitize newlines to prevent injection of fake keys
            let sanitized_tag = tag.replace('\n', " ").replace('\r', " ");
            content.push_str(&format!("tag={}\n", sanitized_tag));
        }
        content
    }

    /// Parse lock info from file contents
    pub fn parse(content: &str) -> Result<Self, SemaphoreError> {
        let mut pid = None;
        let mut timestamp = None;
        let mut tag = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key {
                    "pid" => {
                        pid = value.parse().ok();
                    }
                    "timestamp" => {
                        timestamp = value.parse().ok();
                    }
                    "tag" => {
                        tag = Some(value.to_string());
                    }
                    _ => {} // Ignore unknown keys for forward compatibility
                }
            }
        }

        let pid = pid.ok_or_else(|| SemaphoreError::ParseError("missing pid".to_string()))?;
        let timestamp =
            timestamp.ok_or_else(|| SemaphoreError::ParseError("missing timestamp".to_string()))?;

        Ok(Self {
            pid,
            timestamp,
            tag,
        })
    }

    /// Check if the lock is stale based on the given timeout
    pub fn is_stale(&self, timeout: Duration) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now.saturating_sub(self.timestamp) > timeout.as_secs()
    }
}

impl Default for LockInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// A guard that releases the lock when dropped
#[derive(Debug)]
pub struct SemaphoreGuard<'a> {
    semaphore: &'a Semaphore,
}

impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        // Best-effort release on drop
        let _ = self.semaphore.release_internal();
    }
}

/// File-based semaphore for cross-platform process coordination
#[derive(Debug)]
pub struct Semaphore {
    /// Path to the lock file
    path: PathBuf,
    /// Configuration
    config: SemaphoreConfig,
}

impl Semaphore {
    /// Create a new semaphore with the given lock file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the lock file
    /// * `config` - Configuration options
    ///
    /// # Returns
    ///
    /// Returns `Ok(Semaphore)` on success, or `Err(SemaphoreError)` if the path is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_based_semaphore::{Semaphore, SemaphoreConfig};
    ///
    /// let semaphore = Semaphore::new("/tmp/my-lock.lock", SemaphoreConfig::default()).unwrap();
    /// ```
    pub fn new(path: impl AsRef<Path>, config: SemaphoreConfig) -> Result<Self, SemaphoreError> {
        let path = path.as_ref().to_path_buf();

        // Validate path
        if path.as_os_str().is_empty() {
            return Err(SemaphoreError::InvalidPath("path cannot be empty".into()));
        }

        // Check if parent directory exists (or is root)
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                return Err(SemaphoreError::InvalidPath(format!(
                    "parent directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        Ok(Self { path, config })
    }

    /// Create a semaphore with default configuration
    pub fn with_defaults(path: impl AsRef<Path>) -> Result<Self, SemaphoreError> {
        Self::new(path, SemaphoreConfig::default())
    }

    /// Get the path to the lock file
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if the lock is currently held
    pub fn is_locked(&self) -> bool {
        self.path.exists()
    }

    /// Get information about the current lock holder (if any)
    pub fn lock_info(&self) -> Option<LockInfo> {
        if !self.path.exists() {
            return None;
        }

        let mut file = File::open(&self.path).ok()?;
        let mut content = String::new();
        file.read_to_string(&mut content).ok()?;
        LockInfo::parse(&content).ok()
    }

    /// Try to acquire the lock without blocking
    ///
    /// # Returns
    ///
    /// Returns `Ok(SemaphoreGuard)` if the lock was acquired, or `Err(SemaphoreError)` if
    /// the lock is already held.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use file_based_semaphore::{Semaphore, SemaphoreConfig};
    ///
    /// let semaphore = Semaphore::new("/tmp/try-lock.lock", SemaphoreConfig::default()).unwrap();
    ///
    /// {
    ///     match semaphore.try_acquire() {
    ///         Ok(guard) => {
    ///             // Lock acquired
    ///             println!("Got the lock!");
    ///         }
    ///         Err(e) => {
    ///             println!("Could not acquire lock: {}", e);
    ///         }
    ///     }
    /// };
    /// ```
    pub fn try_acquire(&self) -> Result<SemaphoreGuard<'_>, SemaphoreError> {
        self.try_acquire_with_info(LockInfo::new())
    }

    /// Try to acquire the lock with custom lock info
    pub fn try_acquire_with_info(
        &self,
        info: LockInfo,
    ) -> Result<SemaphoreGuard<'_>, SemaphoreError> {
        // Check for stale lock first
        if let Some(stale_timeout) = self.config.stale_timeout {
            if let Some(existing_info) = self.lock_info() {
                if existing_info.is_stale(stale_timeout) {
                    // Try to remove stale lock
                    let _ = fs::remove_file(&self.path);
                }
            }
        }

        // Try to create lock file exclusively
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.path)
        {
            Ok(mut file) => {
                // Write lock info
                file.write_all(info.serialize().as_bytes())?;
                file.sync_all()?;
                Ok(SemaphoreGuard { semaphore: self })
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                // Lock is held by someone else
                let lock_info = self.lock_info();
                Err(SemaphoreError::AlreadyLocked {
                    holder_pid: lock_info.as_ref().map(|i| i.pid),
                    locked_since: lock_info.as_ref().map(|i| i.timestamp),
                })
            }
            Err(e) => Err(SemaphoreError::IoError(e)),
        }
    }

    /// Acquire the lock, blocking until it becomes available
    ///
    /// Uses the `acquire_timeout` from config if set, otherwise waits forever.
    ///
    /// # Returns
    ///
    /// Returns `Ok(SemaphoreGuard)` when the lock is acquired, or `Err(SemaphoreError)`
    /// on timeout or other error.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use file_based_semaphore::{Semaphore, SemaphoreConfig};
    /// use std::time::Duration;
    ///
    /// let config = SemaphoreConfig {
    ///     acquire_timeout: Some(Duration::from_secs(5)),
    ///     ..Default::default()
    /// };
    ///
    /// let semaphore = Semaphore::new("/tmp/blocking-lock.lock", config).unwrap();
    ///
    /// {
    ///     match semaphore.acquire() {
    ///         Ok(guard) => {
    ///             // Lock acquired
    ///             println!("Got the lock!");
    ///         }
    ///         Err(e) => {
    ///             println!("Failed to acquire lock: {}", e);
    ///         }
    ///     }
    /// };
    /// ```
    pub fn acquire(&self) -> Result<SemaphoreGuard<'_>, SemaphoreError> {
        self.acquire_with_info(LockInfo::new())
    }

    /// Acquire the lock with custom lock info, blocking until available
    pub fn acquire_with_info(&self, info: LockInfo) -> Result<SemaphoreGuard<'_>, SemaphoreError> {
        let start = std::time::Instant::now();

        loop {
            match self.try_acquire_with_info(info.clone()) {
                Ok(guard) => return Ok(guard),
                Err(SemaphoreError::AlreadyLocked { .. }) => {
                    // Check timeout
                    if let Some(timeout) = self.config.acquire_timeout {
                        if start.elapsed() >= timeout {
                            return Err(SemaphoreError::Timeout);
                        }
                    }
                    // Sleep and retry
                    thread::sleep(self.config.retry_interval);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Acquire with a specific timeout (overrides config)
    pub fn acquire_timeout(&self, timeout: Duration) -> Result<SemaphoreGuard<'_>, SemaphoreError> {
        let start = std::time::Instant::now();
        let info = LockInfo::new();

        loop {
            match self.try_acquire_with_info(info.clone()) {
                Ok(guard) => return Ok(guard),
                Err(SemaphoreError::AlreadyLocked { .. }) => {
                    if start.elapsed() >= timeout {
                        return Err(SemaphoreError::Timeout);
                    }
                    thread::sleep(self.config.retry_interval);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Release the lock manually (called internally by SemaphoreGuard)
    fn release_internal(&self) -> Result<(), SemaphoreError> {
        if !self.path.exists() {
            return Err(SemaphoreError::NotLocked);
        }

        fs::remove_file(&self.path)?;
        Ok(())
    }

    /// Force release a lock (even if held by another process)
    ///
    /// **Use with caution!** This can cause issues if another process thinks it holds the lock.
    pub fn force_release(&self) -> Result<(), SemaphoreError> {
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    /// Check if a process with the given PID is still running
    ///
    /// This is a best-effort check that may not work on all platforms.
    #[cfg(unix)]
    pub fn is_process_running(pid: u32) -> bool {
        // On Linux, check if /proc/{pid} exists
        Path::new(&format!("/proc/{}", pid)).exists()
    }

    #[cfg(not(unix))]
    pub fn is_process_running(_pid: u32) -> bool {
        // On other platforms, we can't easily check
        true
    }
}

/// Result type alias for semaphore operations
pub type SemaphoreResult<T> = Result<T, SemaphoreError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn temp_lock_path(name: &str) -> PathBuf {
        let mut path = env::temp_dir();
        path.push(format!(
            "file-semaphore-test-{}-{}.lock",
            name,
            process::id()
        ));
        path
    }

    fn cleanup(path: &Path) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_create_semaphore() {
        let path = temp_lock_path("create");
        cleanup(&path);

        let result = Semaphore::with_defaults(&path);
        assert!(result.is_ok());

        cleanup(&path);
    }

    #[test]
    fn test_invalid_empty_path() {
        let result = Semaphore::with_defaults("");
        assert!(result.is_err());
        match result.unwrap_err() {
            SemaphoreError::InvalidPath(_) => {}
            e => panic!("Expected InvalidPath, got {:?}", e),
        }
    }

    #[test]
    fn test_try_acquire_and_release() {
        let path = temp_lock_path("acquire-release");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();

        // Should not be locked initially
        assert!(!sem.is_locked());

        // Acquire lock
        {
            let guard = sem.try_acquire().unwrap();
            assert!(sem.is_locked());
            drop(guard);
        }

        // Should be released after guard drops
        assert!(!sem.is_locked());

        cleanup(&path);
    }

    #[test]
    fn test_double_acquire_fails() {
        let path = temp_lock_path("double-acquire");
        cleanup(&path);

        let sem1 = Semaphore::with_defaults(&path).unwrap();
        let sem2 = Semaphore::with_defaults(&path).unwrap();

        // First acquire should succeed
        let _guard = sem1.try_acquire().unwrap();

        // Second acquire should fail
        let result = sem2.try_acquire();
        assert!(result.is_err());
        match result.unwrap_err() {
            SemaphoreError::AlreadyLocked { holder_pid, .. } => {
                assert_eq!(holder_pid, Some(process::id()));
            }
            e => panic!("Expected AlreadyLocked, got {:?}", e),
        }

        cleanup(&path);
    }

    #[test]
    fn test_lock_info() {
        let path = temp_lock_path("lock-info");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();

        // No lock info when not locked
        assert!(sem.lock_info().is_none());

        {
            let _guard = sem.try_acquire().unwrap();
            let info = sem.lock_info().unwrap();
            assert_eq!(info.pid, process::id());
            assert!(info.timestamp > 0);
        }

        cleanup(&path);
    }

    #[test]
    fn test_lock_info_with_tag() {
        let path = temp_lock_path("lock-tag");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        let info = LockInfo::with_tag("test-operation");

        {
            let _guard = sem.try_acquire_with_info(info).unwrap();
            let read_info = sem.lock_info().unwrap();
            assert_eq!(read_info.tag, Some("test-operation".to_string()));
        }

        cleanup(&path);
    }

    #[test]
    fn test_lock_info_parse() {
        let content = "pid=12345\ntimestamp=1234567890\ntag=my-tag\n";
        let info = LockInfo::parse(content).unwrap();
        assert_eq!(info.pid, 12345);
        assert_eq!(info.timestamp, 1234567890);
        assert_eq!(info.tag, Some("my-tag".to_string()));
    }

    #[test]
    fn test_lock_info_parse_missing_pid() {
        let content = "timestamp=1234567890\n";
        let result = LockInfo::parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_lock_info_serialize() {
        let info = LockInfo {
            pid: 12345,
            timestamp: 1234567890,
            tag: Some("test".to_string()),
        };
        let serialized = info.serialize();
        assert!(serialized.contains("pid=12345"));
        assert!(serialized.contains("timestamp=1234567890"));
        assert!(serialized.contains("tag=test"));
    }

    #[test]
    fn test_stale_lock_detection() {
        let info = LockInfo {
            pid: 12345,
            timestamp: 0, // Very old
            tag: None,
        };

        assert!(info.is_stale(Duration::from_secs(1)));
    }

    #[test]
    fn test_force_release() {
        let path = temp_lock_path("force-release");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        let _guard = sem.try_acquire().unwrap();

        // Create another semaphore instance and force release
        let sem2 = Semaphore::with_defaults(&path).unwrap();
        sem2.force_release().unwrap();

        assert!(!sem.is_locked());

        cleanup(&path);
    }

    #[test]
    fn test_acquire_with_timeout() {
        let path = temp_lock_path("timeout");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        let _guard = sem.try_acquire().unwrap();

        // Try to acquire with short timeout
        let sem2 = Semaphore::with_defaults(&path).unwrap();
        let start = std::time::Instant::now();
        let result = sem2.acquire_timeout(Duration::from_millis(200));

        assert!(result.is_err());
        match result.unwrap_err() {
            SemaphoreError::Timeout => {}
            e => panic!("Expected Timeout, got {:?}", e),
        }

        // Should have waited approximately the timeout duration
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(150));

        cleanup(&path);
    }

    #[test]
    fn test_error_display() {
        let err = SemaphoreError::AlreadyLocked {
            holder_pid: Some(12345),
            locked_since: Some(1234567890),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("12345"));
        assert!(msg.contains("1234567890"));

        let err = SemaphoreError::Timeout;
        assert_eq!(format!("{}", err), "Timeout waiting for lock");

        let err = SemaphoreError::NotLocked;
        assert_eq!(format!("{}", err), "Lock not held");
    }

    #[test]
    fn test_config_default() {
        let config = SemaphoreConfig::default();
        assert!(config.stale_timeout.is_some());
        assert!(config.acquire_timeout.is_none());
    }

    #[test]
    fn test_release_not_locked() {
        let path = temp_lock_path("release-not-locked");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        let result = sem.release_internal();

        assert!(result.is_err());
        match result.unwrap_err() {
            SemaphoreError::NotLocked => {}
            e => panic!("Expected NotLocked, got {:?}", e),
        }
    }

    #[test]
    fn test_stale_lock_cleanup() {
        let path = temp_lock_path("stale-cleanup");
        cleanup(&path);

        // Manually create a stale lock file
        let old_info = LockInfo {
            pid: 99999,
            timestamp: 0, // Very old
            tag: None,
        };
        let mut file = File::create(&path).unwrap();
        file.write_all(old_info.serialize().as_bytes()).unwrap();

        let config = SemaphoreConfig {
            stale_timeout: Some(Duration::from_secs(1)),
            ..Default::default()
        };

        let sem = Semaphore::new(&path, config).unwrap();

        // Should be able to acquire because lock is stale
        let result = sem.try_acquire();
        assert!(result.is_ok());

        cleanup(&path);
    }

    // ============================================================
    // Additional edge case tests
    // ============================================================

    #[test]
    fn test_empty_tag() {
        let path = temp_lock_path("empty-tag");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        let info = LockInfo::with_tag("");

        {
            let _guard = sem.try_acquire_with_info(info).unwrap();
            let read_info = sem.lock_info().unwrap();
            assert_eq!(read_info.tag, Some("".to_string()));
        }

        cleanup(&path);
    }

    #[test]
    fn test_very_long_tag() {
        let path = temp_lock_path("long-tag");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        let long_tag = "x".repeat(10000); // 10KB tag
        let info = LockInfo::with_tag(&long_tag);

        {
            let _guard = sem.try_acquire_with_info(info).unwrap();
            let read_info = sem.lock_info().unwrap();
            assert_eq!(read_info.tag, Some(long_tag));
        }

        cleanup(&path);
    }

    #[test]
    fn test_special_chars_in_tag() {
        let path = temp_lock_path("special-chars");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        // Test various special characters (but not newlines which break format)
        let special_tag = "tag=with=equals&special<chars>\"quotes\"";
        let info = LockInfo::with_tag(special_tag);

        {
            let _guard = sem.try_acquire_with_info(info).unwrap();
            let read_info = sem.lock_info().unwrap();
            assert_eq!(read_info.tag, Some(special_tag.to_string()));
        }

        cleanup(&path);
    }

    #[test]
    fn test_unicode_tag() {
        let path = temp_lock_path("unicode-tag");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        let unicode_tag = "日本語タグ🔒emoji✨";
        let info = LockInfo::with_tag(unicode_tag);

        {
            let _guard = sem.try_acquire_with_info(info).unwrap();
            let read_info = sem.lock_info().unwrap();
            assert_eq!(read_info.tag, Some(unicode_tag.to_string()));
        }

        cleanup(&path);
    }

    #[test]
    fn test_tag_newline_injection_prevention() {
        // Security test: newlines in tags should be sanitized to prevent
        // injection of fake keys into the lock file format
        let malicious_tag = "mytag\npid=99999\ntimestamp=0";
        let info = LockInfo::with_tag(malicious_tag);
        let serialized = info.serialize();

        // Count lines starting with "pid=" - should be exactly one (the real one)
        let pid_lines: Vec<_> = serialized.lines().filter(|l| l.starts_with("pid=")).collect();
        assert_eq!(pid_lines.len(), 1, "Should only have one pid= line, got {:?}", pid_lines);

        // Count lines starting with "timestamp=" - should be exactly one
        let ts_lines: Vec<_> = serialized.lines().filter(|l| l.starts_with("timestamp=")).collect();
        assert_eq!(ts_lines.len(), 1, "Should only have one timestamp= line");

        // The tag should contain the sanitized version (spaces instead of newlines)
        assert!(serialized.contains("tag=mytag pid=99999 timestamp=0"),
            "Sanitized tag should have spaces: {}", serialized);

        // Verify parsing doesn't pick up fake values
        let parsed = LockInfo::parse(&serialized).unwrap();
        assert_eq!(parsed.pid, info.pid); // Original PID preserved
        assert_ne!(parsed.pid, 99999); // Fake PID not used
        assert!(parsed.tag.unwrap().contains("mytag"));
    }

    #[test]
    fn test_tag_carriage_return_injection_prevention() {
        // Also test carriage returns
        let malicious_tag = "mytag\r\npid=99999";
        let info = LockInfo::with_tag(malicious_tag);
        let serialized = info.serialize();

        // Should only have one line starting with pid=
        let pid_lines: Vec<_> = serialized.lines().filter(|l| l.starts_with("pid=")).collect();
        assert_eq!(pid_lines.len(), 1, "Should only have one pid= line");

        // Tag should have sanitized whitespace (both \r and \n replaced)
        assert!(serialized.contains("tag=mytag"));
        // Verify no literal newlines or carriage returns in the serialized output
        // (except for the actual line separators between key=value pairs)
        let tag_line = serialized.lines().find(|l| l.starts_with("tag=")).unwrap();
        assert!(!tag_line.contains('\r'), "Tag should not contain carriage return");
    }

    #[test]
    fn test_path_method() {
        let path = temp_lock_path("path-method");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();
        assert_eq!(sem.path(), path.as_path());

        cleanup(&path);
    }

    #[test]
    fn test_lock_info_without_tag() {
        let path = temp_lock_path("no-tag");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();

        {
            let _guard = sem.try_acquire().unwrap();
            let read_info = sem.lock_info().unwrap();
            assert!(read_info.tag.is_none());
        }

        cleanup(&path);
    }

    #[test]
    fn test_lock_info_parse_extra_fields() {
        // Test forward compatibility - unknown fields should be ignored
        let content = "pid=12345\ntimestamp=1234567890\ntag=test\nunknown_field=value\n";
        let info = LockInfo::parse(content).unwrap();
        assert_eq!(info.pid, 12345);
        assert_eq!(info.timestamp, 1234567890);
        assert_eq!(info.tag, Some("test".to_string()));
    }

    #[test]
    fn test_lock_info_parse_empty_lines() {
        let content = "\npid=12345\n\ntimestamp=1234567890\n\n";
        let info = LockInfo::parse(content).unwrap();
        assert_eq!(info.pid, 12345);
        assert_eq!(info.timestamp, 1234567890);
    }

    #[test]
    fn test_lock_info_parse_whitespace() {
        let content = "  pid=12345  \n  timestamp=1234567890  \n";
        let info = LockInfo::parse(content).unwrap();
        assert_eq!(info.pid, 12345);
        assert_eq!(info.timestamp, 1234567890);
    }

    #[test]
    fn test_invalid_parent_directory() {
        let result = Semaphore::with_defaults("/nonexistent/dir/file.lock");
        assert!(result.is_err());
        match result.unwrap_err() {
            SemaphoreError::InvalidPath(_) => {}
            e => panic!("Expected InvalidPath, got {:?}", e),
        }
    }

    #[test]
    fn test_lock_info_default() {
        let info = LockInfo::default();
        assert_eq!(info.pid, process::id());
        assert!(info.timestamp > 0);
        assert!(info.tag.is_none());
    }

    #[test]
    fn test_is_stale_not_stale() {
        let info = LockInfo::new();
        // Just created, should not be stale with 1 hour timeout
        assert!(!info.is_stale(Duration::from_secs(3600)));
    }

    #[test]
    fn test_semaphore_result_type() {
        // Test that SemaphoreResult type alias works
        fn returns_result() -> SemaphoreResult<()> {
            Ok(())
        }
        assert!(returns_result().is_ok());
    }

    #[test]
    fn test_error_source() {
        use std::error::Error;

        let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
        let sem_err = SemaphoreError::IoError(io_err);
        assert!(sem_err.source().is_some());

        let already_locked = SemaphoreError::AlreadyLocked {
            holder_pid: None,
            locked_since: None,
        };
        assert!(already_locked.source().is_none());
    }

    #[test]
    fn test_acquire_timeout_method() {
        let path = temp_lock_path("acquire-timeout-method");
        cleanup(&path);

        let sem = Semaphore::with_defaults(&path).unwrap();

        // Should acquire immediately since lock is free
        let result = sem.acquire_timeout(Duration::from_millis(100));
        assert!(result.is_ok());

        cleanup(&path);
    }
}
