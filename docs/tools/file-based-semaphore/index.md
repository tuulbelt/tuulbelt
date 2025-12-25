# File-Based Semaphore

Cross-platform file-based semaphore for process coordination. Lock resources reliably across processes with zero dependencies.

## Overview

File-Based Semaphore provides cross-platform process coordination using lock files. It handles stale lock detection, supports both blocking and non-blocking acquisition, and works across different programming languages. Built in Rust for performance and reliability.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** Rust

**Repository:** [tuulbelt/tuulbelt/file-based-semaphore](https://github.com/tuulbelt/tuulbelt/tree/main/file-based-semaphore)

## Features

### <img src="/icons/lock.svg" class="inline-icon" alt=""> Atomic Locking

Acquire locks atomically using `O_CREAT | O_EXCL`. No race conditions in lock acquisition.

### <img src="/icons/clock.svg" class="inline-icon" alt=""> Stale Detection

Detect and recover from stale locks with configurable timeouts. Handle crashed processes gracefully.

### <img src="/icons/layers.svg" class="inline-icon" alt=""> RAII Guards

Use Rust's ownership system for automatic lock release. Guards ensure locks are released even on panic.

### <img src="/icons/link.svg" class="inline-icon" alt=""> Cross-Platform

Works on Linux, macOS, and Windows. Uses portable file operations only.

### <img src="/icons/tool.svg" class="inline-icon" alt=""> CLI & Library

Use as a command-line tool for shell scripts or integrate as a Rust library in your applications.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Rust standard library. No external crates required at runtime.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/file-based-semaphore

# Build
cargo build --release

# CLI: Try to acquire a lock
./target/release/file-semaphore try /tmp/my.lock

# CLI: Check status
./target/release/file-semaphore status /tmp/my.lock

# CLI: Release
./target/release/file-semaphore release /tmp/my.lock
```

```rust
// Library usage
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::time::Duration;

let config = SemaphoreConfig {
    stale_timeout: Some(Duration::from_secs(60)),
    ..Default::default()
};

let sem = Semaphore::new("/tmp/my.lock", config)?;

{
    let _guard = sem.try_acquire()?;
    // Critical section - only one process here
    do_exclusive_work();
} // Lock auto-released
```

## Use Cases

- **Build Coordination:** Prevent concurrent builds from conflicting
- **Deployment Scripts:** Ensure only one deployment runs at a time
- **Test Isolation:** Coordinate access to shared test resources
- **Service Health Checks:** Single instance for health monitoring
- **Batch Job Coordination:** Prevent duplicate batch job runs

## Why File-Based Semaphore?

Traditional locking approaches have limitations:

1. **Database Locks:** Require running database server
2. **Advisory Locks (flock):** Platform-specific behavior, don't survive crashes well
3. **In-Memory Locks:** Don't work across processes
4. **Redis/ZooKeeper:** Require external services

File-based semaphores solve these by using simple lock files that:
- Work across any language that can create files
- Survive process crashes with stale detection
- Need no running daemons or services
- Use atomic filesystem operations

## Dogfooding

We use **[Test Flakiness Detector](/tools/test-flakiness-detector/)** to validate test reliability:

```bash
# From test-flakiness-detector directory
npx tsx src/index.ts --test "cd ../file-based-semaphore && cargo test" --runs 10
```

This validates that all 46 tests are deterministic across multiple runs.

## Demo

![File-Based Semaphore Demo](/file-based-semaphore/demo.gif)

**[▶ View interactive recording on asciinema.org](#)**

### Try it Locally

```bash
# Clone and build
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/file-based-semaphore
cargo build --release

# Run examples
cargo run --example basic
cargo run --example concurrent
cargo run --example stale_recovery

# Run full demo script
./scripts/demo.sh
```

> **Note:** StackBlitz is not available for Rust projects. Clone the repository to try it locally.

> Demo recordings are automatically generated via GitHub Actions.

## Next Steps

- [Getting Started Guide](/tools/file-based-semaphore/getting-started) - Installation and setup
- [CLI Usage](/tools/file-based-semaphore/cli-usage) - Command-line interface
- [Library Usage](/tools/file-based-semaphore/library-usage) - Rust library API
- [Examples](/tools/file-based-semaphore/examples) - Real-world usage patterns
- [Protocol Spec](/tools/file-based-semaphore/protocol-spec) - Lock file format specification
- [API Reference](/tools/file-based-semaphore/api-reference) - Complete API documentation

## License

MIT License - see repository for details.
