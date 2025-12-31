# Library Usage

Integrate File-Based Semaphore into your Rust applications for robust process coordination.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
file-based-semaphore = { git = "https://github.com/tuulbelt/file-based-semaphore.git" }
```

## Basic Usage

### Non-Blocking Acquisition

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sem = Semaphore::with_defaults("/tmp/my.lock")?;

    {
        let _guard = sem.try_acquire()?;
        // Critical section - lock held
        println!("Doing exclusive work...");
    } // Guard dropped, lock automatically released

    Ok(())
}
```

### Blocking Acquisition with Timeout

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SemaphoreConfig {
        acquire_timeout: Some(Duration::from_secs(30)),
        retry_interval: Duration::from_millis(100),
        ..Default::default()
    };

    let sem = Semaphore::new("/tmp/blocking.lock", config)?;

    {
        let _guard = sem.acquire()?;
        // Got the lock after waiting
        do_work();
    }

    Ok(())
}
```

### Configuration Options

```rust
use file_based_semaphore::SemaphoreConfig;
use std::time::Duration;

let config = SemaphoreConfig {
    // Detect locks older than 1 hour as stale
    stale_timeout: Some(Duration::from_secs(3600)),

    // Wait up to 30 seconds when acquiring
    acquire_timeout: Some(Duration::from_secs(30)),

    // Poll every 100ms while waiting
    retry_interval: Duration::from_millis(100),
};
```

### Custom Lock Info

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig, LockInfo};

let sem = Semaphore::with_defaults("/tmp/tagged.lock")?;

// Add a tag to identify the lock holder
let info = LockInfo::with_tag("backup-job-2024");

{
    let _guard = sem.try_acquire_with_info(info)?;
    run_backup();
}
```

### Checking Lock Status

```rust
use file_based_semaphore::Semaphore;

let sem = Semaphore::with_defaults("/tmp/check.lock")?;

// Check if locked
if sem.is_locked() {
    println!("Lock is held");

    // Get holder information
    if let Some(info) = sem.lock_info() {
        println!("Held by PID: {}", info.pid);
        println!("Since: {}", info.timestamp);
        if let Some(tag) = info.tag {
            println!("Tag: {}", tag);
        }
    }
}
```

### Stale Lock Recovery

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::time::Duration;

let config = SemaphoreConfig {
    // Consider locks older than 5 minutes as stale
    stale_timeout: Some(Duration::from_secs(300)),
    ..Default::default()
};

let sem = Semaphore::new("/tmp/recoverable.lock", config)?;

// try_acquire will automatically clean up stale locks
let _guard = sem.try_acquire()?;
```

### Force Release (Use with Caution)

```rust
use file_based_semaphore::Semaphore;

let sem = Semaphore::with_defaults("/tmp/force.lock")?;

// Force release even if held by another process
// WARNING: This can break coordination!
sem.force_release()?;
```

## Error Handling

### SemaphoreError Types

```rust
use file_based_semaphore::{Semaphore, SemaphoreError};

let sem = Semaphore::with_defaults("/tmp/error.lock")?;

match sem.try_acquire() {
    Ok(guard) => {
        // Lock acquired
        do_work();
    }
    Err(SemaphoreError::AlreadyLocked { holder_pid, locked_since }) => {
        if let Some(pid) = holder_pid {
            println!("Lock held by PID: {}", pid);
        }
    }
    Err(SemaphoreError::InvalidPath(msg)) => {
        eprintln!("Invalid path: {}", msg);
    }
    Err(SemaphoreError::IoError(e)) => {
        eprintln!("IO error: {}", e);
    }
    Err(SemaphoreError::Timeout) => {
        eprintln!("Timed out waiting for lock");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Thread Safety

The `Semaphore` struct is `Send + Sync` and can be shared across threads:

```rust
use file_based_semaphore::Semaphore;
use std::sync::Arc;
use std::thread;

let sem = Arc::new(Semaphore::with_defaults("/tmp/shared.lock")?);

let handles: Vec<_> = (0..4).map(|i| {
    let sem = Arc::clone(&sem);
    thread::spawn(move || {
        if let Ok(guard) = sem.try_acquire() {
            println!("Thread {} got the lock", i);
            thread::sleep(Duration::from_millis(100));
        }
    })
}).collect();

for h in handles {
    h.join().unwrap();
}
```

## Best Practices

### 1. Always Use Guards

```rust
// Good: RAII guard ensures release
{
    let _guard = sem.try_acquire()?;
    do_work();
} // Automatically released

// Avoid: Manual release can be forgotten
sem.try_acquire()?;
do_work();
// Oops, forgot to release!
```

### 2. Use Appropriate Timeouts

```rust
// For short operations
SemaphoreConfig {
    acquire_timeout: Some(Duration::from_secs(5)),
    stale_timeout: Some(Duration::from_secs(60)),
    ..Default::default()
}

// For long-running jobs
SemaphoreConfig {
    acquire_timeout: Some(Duration::from_secs(300)),
    stale_timeout: Some(Duration::from_secs(3600)),
    ..Default::default()
}
```

### 3. Handle Errors Gracefully

```rust
match sem.acquire() {
    Ok(guard) => {
        // Proceed with work
    }
    Err(SemaphoreError::Timeout) => {
        // Gracefully handle timeout
        log::warn!("Could not acquire lock, skipping operation");
    }
    Err(e) => {
        return Err(e.into());
    }
}
```
