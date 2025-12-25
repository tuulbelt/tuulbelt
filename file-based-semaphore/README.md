# File-Based Semaphore

[![Tests](https://github.com/tuulbelt/tuulbelt/actions/workflows/test-all-tools.yml/badge.svg)](https://github.com/tuulbelt/tuulbelt/actions/workflows/test-all-tools.yml)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success)
![Tests](https://img.shields.io/badge/tests-46%20passing-success)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Cross-platform file-based semaphore for process coordination.

## Problem

When multiple processes need to coordinate access to a shared resource, you need a locking mechanism that:
- Works across different programming languages
- Survives process crashes (with stale lock detection)
- Doesn't require a running daemon or server
- Uses only standard filesystem operations

File-based semaphores provide exactly this: a portable, reliable locking primitive using lock files.

## Features

- **Zero runtime dependencies** (uses only Rust standard library)
- **Cross-platform support** (Linux, macOS, Windows)
- **Both library and CLI interfaces**
- **Stale lock detection** with configurable timeout
- **RAII-style guards** for automatic release
- **Atomic operations** using exclusive file creation

## Installation

### From Source

```bash
git clone https://github.com/tuulbelt/file-based-semaphore.git
cd file-based-semaphore
cargo build --release
```

The binary will be at `target/release/file-semaphore`.

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
file-based-semaphore = { git = "https://github.com/tuulbelt/file-based-semaphore.git" }
```

## Usage

### CLI

```bash
# Try to acquire a lock (non-blocking)
file-semaphore try /tmp/my.lock

# Acquire with timeout (blocks up to 10 seconds)
file-semaphore acquire /tmp/my.lock --timeout 10

# Check lock status
file-semaphore status /tmp/my.lock

# Release a lock
file-semaphore release /tmp/my.lock

# Wait for a lock to be released
file-semaphore wait /tmp/my.lock --timeout 30

# Get JSON output
file-semaphore status /tmp/my.lock --json
```

### Library

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SemaphoreConfig {
        stale_timeout: Some(Duration::from_secs(60)),
        acquire_timeout: Some(Duration::from_secs(5)),
        ..Default::default()
    };

    let semaphore = Semaphore::new("/tmp/my-lock.lock", config)?;

    // RAII-style: lock auto-released when guard drops
    {
        let _guard = semaphore.try_acquire()?;
        // Critical section - only one process can be here
        do_work();
    } // Guard dropped, lock released

    Ok(())
}
```

## API

### `Semaphore::new(path, config) -> Result<Semaphore, SemaphoreError>`

Create a new semaphore with the given lock file path.

### `semaphore.try_acquire() -> Result<SemaphoreGuard, SemaphoreError>`

Try to acquire the lock without blocking. Returns immediately.

### `semaphore.acquire() -> Result<SemaphoreGuard, SemaphoreError>`

Acquire the lock, blocking until available or timeout.

### `semaphore.is_locked() -> bool`

Check if the lock is currently held.

### `semaphore.lock_info() -> Option<LockInfo>`

Get information about the current lock holder.

### `semaphore.force_release() -> Result<(), SemaphoreError>`

Force release a lock (even if held by another process).

### `SemaphoreConfig`

```rust
SemaphoreConfig {
    stale_timeout: Some(Duration::from_secs(3600)),  // Detect stale locks
    acquire_timeout: Some(Duration::from_secs(30)), // Max wait time
    retry_interval: Duration::from_millis(100),     // Polling interval
}
```

## Lock File Format

Lock files contain simple key-value pairs:

```
pid=12345
timestamp=1703520000
tag=optional-description
```

This format is:
- Human-readable for debugging
- Easy to parse from any language
- Forward-compatible (unknown keys are ignored)

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (lock acquired/released, or lock is free) |
| 1 | Lock already held or timeout |
| 2 | Invalid arguments |
| 3 | IO or system error |

## Examples

### Bash Script Coordination

```bash
#!/bin/bash

# Acquire lock before running exclusive task
if file-semaphore try /tmp/deploy.lock --tag "deploy-$(date +%s)"; then
    echo "Lock acquired, deploying..."
    ./deploy.sh
    file-semaphore release /tmp/deploy.lock
else
    echo "Another deployment in progress"
    exit 1
fi
```

### Concurrent Process Guard

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::time::Duration;

// Multiple worker processes competing for exclusive access
let sem = Semaphore::new("/tmp/worker.lock", SemaphoreConfig {
    stale_timeout: Some(Duration::from_secs(300)), // 5 min stale timeout
    acquire_timeout: Some(Duration::from_secs(60)), // Wait up to 1 min
    ..Default::default()
})?;

match sem.acquire() {
    Ok(guard) => {
        process_exclusive_work();
        // Guard dropped here, lock released
    }
    Err(e) => eprintln!("Could not acquire lock: {}", e),
}
```

## Testing

```bash
cargo test              # Run all tests
cargo test -- --nocapture  # Show output
```

### Dogfooding

We validate this tool's test reliability using **Test Flakiness Detector** (another Tuulbelt tool):

```bash
# Run the dogfood script
./scripts/dogfood.sh

# Or with custom run count
./scripts/dogfood.sh 20
```

This runs all 46 tests 10 times (460 total executions) to detect any non-deterministic behavior.

**Expected output:**
```
✅ NO FLAKINESS DETECTED
All 10 test runs passed consistently.
The test suite is deterministic and reliable! 🎉
```

> **Note:** Rust tools can be validated via CLI by Test Flakiness Detector even though they can't import TypeScript libraries directly. This cross-language validation demonstrates that Tuulbelt tools compose well together.

## Design Decisions

### Why file-based locking?

1. **Portable**: Works everywhere with a filesystem
2. **No daemon**: Doesn't require a running service
3. **Crash-safe**: Stale detection handles crashed processes
4. **Language-agnostic**: Any language can read/write lock files

### Why not flock/lockf?

- Platform-specific behavior (especially on NFS)
- Doesn't survive process crashes reliably
- This approach is simpler and more portable

### Why O_EXCL instead of advisory locks?

- `O_CREAT | O_EXCL` is atomic on POSIX and Windows
- Works on network filesystems
- Clear semantics: file exists = lock held

## Limitations

- **Network filesystems**: Works but with caveats on NFS (use NFSv4+ with proper locking)
- **Race window**: Small window between stale detection and acquisition
- **Clock skew**: Stale detection relies on system time; ensure synchronized clocks

## Demo

![Demo](docs/demo.gif)

**[▶ View interactive recording on asciinema.org](#)**

### Try it Yourself

```bash
# Build and run
cargo build --release
./target/release/file-semaphore try /tmp/demo.lock --tag "demo-session"
./target/release/file-semaphore status /tmp/demo.lock --json
./target/release/file-semaphore release /tmp/demo.lock
```

### Run Examples

```bash
# Basic usage
cargo run --example basic

# Concurrent access demo
cargo run --example concurrent

# Stale lock recovery
cargo run --example stale_recovery
```

> **Note:** StackBlitz is not available for Rust projects. Clone the repository and run locally with `cargo build --release`.

> Demo recordings are automatically generated via GitHub Actions.

## Related Tools

- [Test Flakiness Detector](../test-flakiness-detector/) - Validates this tool's test suite reliability
- [CLI Progress Reporting](../cli-progress-reporting/) - Could use this tool for exclusive log access

## License

MIT - see [LICENSE](LICENSE)
