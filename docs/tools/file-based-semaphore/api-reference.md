# API Reference

Complete API documentation for File-Based Semaphore.

## Types

### `Semaphore`

Main semaphore struct for coordinating access.

```rust
pub struct Semaphore {
    path: PathBuf,
    config: SemaphoreConfig,
}
```

### `SemaphoreConfig`

Configuration for semaphore behavior.

```rust
pub struct SemaphoreConfig {
    /// Stale lock timeout. Locks older than this are auto-cleaned.
    /// Default: Some(Duration::from_secs(3600))
    pub stale_timeout: Option<Duration>,

    /// Maximum time to wait when acquiring.
    /// Default: None (wait forever)
    pub acquire_timeout: Option<Duration>,

    /// Interval between acquisition retries.
    /// Default: Duration::from_millis(100)
    pub retry_interval: Duration,
}
```

### `SemaphoreGuard`

RAII guard that releases the lock when dropped.

```rust
pub struct SemaphoreGuard<'a> {
    semaphore: &'a Semaphore,
}
```

### `LockInfo`

Information about a lock holder.

```rust
pub struct LockInfo {
    /// Process ID of lock holder
    pub pid: u32,

    /// Unix timestamp when lock was acquired
    pub timestamp: u64,

    /// Optional description/tag
    pub tag: Option<String>,
}
```

### `SemaphoreError`

Error types for semaphore operations.

```rust
pub enum SemaphoreError {
    /// Lock already held by another process
    AlreadyLocked {
        holder_pid: Option<u32>,
        locked_since: Option<u64>,
    },

    /// Lock file path is invalid
    InvalidPath(String),

    /// IO error during lock operations
    IoError(std::io::Error),

    /// Lock was not held when trying to release
    NotLocked,

    /// Timeout waiting for lock
    Timeout,

    /// Failed to parse lock file contents
    ParseError(String),
}
```

## Semaphore Methods

### `new`

Create a new semaphore with configuration.

```rust
pub fn new(path: impl AsRef<Path>, config: SemaphoreConfig) -> Result<Self, SemaphoreError>
```

**Parameters:**
- `path` - Path to the lock file
- `config` - Configuration options

**Returns:** `Result<Semaphore, SemaphoreError>`

**Errors:**
- `InvalidPath` - Path is empty or parent directory doesn't exist

### `with_defaults`

Create a semaphore with default configuration.

```rust
pub fn with_defaults(path: impl AsRef<Path>) -> Result<Self, SemaphoreError>
```

### `try_acquire`

Non-blocking lock acquisition.

```rust
pub fn try_acquire(&self) -> Result<SemaphoreGuard<'_>, SemaphoreError>
```

**Returns:** `Result<SemaphoreGuard, SemaphoreError>`

**Errors:**
- `AlreadyLocked` - Lock is held by another process
- `IoError` - Filesystem error

### `try_acquire_with_info`

Non-blocking acquisition with custom lock info.

```rust
pub fn try_acquire_with_info(&self, info: LockInfo) -> Result<SemaphoreGuard<'_>, SemaphoreError>
```

### `acquire`

Blocking lock acquisition (uses config timeout).

```rust
pub fn acquire(&self) -> Result<SemaphoreGuard<'_>, SemaphoreError>
```

**Errors:**
- `Timeout` - Timed out waiting for lock
- `IoError` - Filesystem error

### `acquire_with_info`

Blocking acquisition with custom lock info.

```rust
pub fn acquire_with_info(&self, info: LockInfo) -> Result<SemaphoreGuard<'_>, SemaphoreError>
```

### `acquire_timeout`

Blocking acquisition with specific timeout.

```rust
pub fn acquire_timeout(&self, timeout: Duration) -> Result<SemaphoreGuard<'_>, SemaphoreError>
```

### `is_locked`

Check if the lock is currently held.

```rust
pub fn is_locked(&self) -> bool
```

### `lock_info`

Get information about the current lock holder.

```rust
pub fn lock_info(&self) -> Option<LockInfo>
```

**Returns:** `Some(LockInfo)` if locked, `None` if free

### `force_release`

Force release a lock (even if held by another process).

```rust
pub fn force_release(&self) -> Result<(), SemaphoreError>
```

**Warning:** Use with caution - can break coordination.

### `path`

Get the path to the lock file.

```rust
pub fn path(&self) -> &Path
```

### `is_process_running`

Check if a process is still running (Unix only).

```rust
#[cfg(unix)]
pub fn is_process_running(pid: u32) -> bool
```

## LockInfo Methods

### `new`

Create lock info for current process.

```rust
pub fn new() -> Self
```

### `with_tag`

Create lock info with a custom tag.

```rust
pub fn with_tag(tag: impl Into<String>) -> Self
```

### `serialize`

Serialize to lock file format.

```rust
pub fn serialize(&self) -> String
```

### `parse`

Parse from lock file contents.

```rust
pub fn parse(content: &str) -> Result<Self, SemaphoreError>
```

### `is_stale`

Check if lock is stale based on timeout.

```rust
pub fn is_stale(&self, timeout: Duration) -> bool
```

## SemaphoreConfig Default

```rust
impl Default for SemaphoreConfig {
    fn default() -> Self {
        Self {
            stale_timeout: Some(Duration::from_secs(3600)),
            acquire_timeout: None,
            retry_interval: Duration::from_millis(100),
        }
    }
}
```

## Error Display

All errors implement `Display` and `Error`:

```rust
use file_based_semaphore::SemaphoreError;

let err = SemaphoreError::Timeout;
println!("{}", err); // "Timeout waiting for lock"

let err = SemaphoreError::AlreadyLocked {
    holder_pid: Some(12345),
    locked_since: Some(1703520000),
};
println!("{}", err); // "Lock already held by PID 12345 since 1703520000"
```

## Thread Safety

- `Semaphore` is `Send + Sync`
- `SemaphoreGuard` is `Send` but not `Sync`
- Multiple threads can share a `Semaphore` via `Arc`

## Example: Complete Usage

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig, LockInfo, SemaphoreError};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure
    let config = SemaphoreConfig {
        stale_timeout: Some(Duration::from_secs(300)),
        acquire_timeout: Some(Duration::from_secs(30)),
        retry_interval: Duration::from_millis(50),
    };

    // Create
    let sem = Semaphore::new("/tmp/example.lock", config)?;

    // Check status
    println!("Locked: {}", sem.is_locked());

    // Acquire with tag
    let info = LockInfo::with_tag("example-operation");

    match sem.acquire_with_info(info) {
        Ok(guard) => {
            println!("Lock acquired!");

            // Read lock info
            if let Some(info) = sem.lock_info() {
                println!("Held by PID: {}", info.pid);
            }

            // Work...
            drop(guard); // Explicit release (or let it drop)
        }
        Err(SemaphoreError::Timeout) => {
            println!("Timed out");
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    Ok(())
}
```
