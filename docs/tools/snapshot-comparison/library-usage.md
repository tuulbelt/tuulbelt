# Library Usage

Use Snapshot Comparison as a Rust library in your applications.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
snapshot-comparison = { git = "https://github.com/tuulbelt/tuulbelt.git" }
```

Or for monorepo use:

```toml
[dependencies]
snapshot-comparison = { path = "../snapshot-comparison" }
```

## Basic Usage

### Creating a Snapshot Store

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig};
use std::path::PathBuf;

fn main() {
    // Create a store in the snapshots directory
    let store = SnapshotStore::new(PathBuf::from("snapshots"));

    // Use default configuration
    let config = SnapshotConfig::default();

    // Store is ready to use
}
```

### Creating Snapshots

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = SnapshotStore::new(PathBuf::from("snapshots"));
    let config = SnapshotConfig::default();

    // Create a text snapshot
    let content = b"Hello, World!";
    let snapshot = store.create("greeting", content, &config)?;

    println!("Created: {} ({} bytes)",
        snapshot.metadata.name,
        snapshot.metadata.size
    );

    Ok(())
}
```

### Checking Against Snapshots

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = SnapshotStore::new(PathBuf::from("snapshots"));
    let config = SnapshotConfig::default();

    let current_output = b"Hello, World!";

    match store.check("greeting", current_output, &config)? {
        CompareResult::Match => {
            println!("Output matches snapshot");
        }
        CompareResult::Mismatch { diff, .. } => {
            eprintln!("Output differs from snapshot:");
            eprintln!("{}", diff.format("greeting", false));
        }
        CompareResult::Updated { .. } => {
            println!("Snapshot was updated");
        }
    }

    Ok(())
}
```

## Configuration

### SnapshotConfig

```rust
use snapshot_comparison::{SnapshotConfig, FileType};

// Default configuration
let default_config = SnapshotConfig::default();

// Custom configuration
let config = SnapshotConfig {
    // Explicit file type (None = auto-detect)
    file_type: Some(FileType::Json),

    // Enable colored diff output
    color: true,

    // Number of context lines in diffs
    context_lines: 5,

    // Auto-update on mismatch
    update_mode: true,
};
```

### File Types

```rust
use snapshot_comparison::FileType;

// Available file types
let text = FileType::Text;    // Plain text
let json = FileType::Json;    // JSON data
let binary = FileType::Binary; // Binary data
```

## Advanced Usage

### Update Mode

Automatically update snapshots on mismatch:

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult};

fn check_with_update(store: &SnapshotStore, name: &str, content: &[u8]) {
    let config = SnapshotConfig {
        update_mode: true,
        ..Default::default()
    };

    match store.check(name, content, &config).unwrap() {
        CompareResult::Match => println!("Match"),
        CompareResult::Updated { old_snapshot, new_content } => {
            println!("Updated from {} to {} bytes",
                old_snapshot.metadata.size,
                new_content.len()
            );
        }
        _ => unreachable!("update_mode prevents Mismatch"),
    }
}
```

### Listing Snapshots

```rust
use snapshot_comparison::SnapshotStore;
use std::path::PathBuf;

fn list_all_snapshots(store: &SnapshotStore) {
    let snapshots = store.list().unwrap();

    println!("Found {} snapshots:", snapshots.len());
    for meta in &snapshots {
        println!("  {} ({} bytes, {:?})",
            meta.name,
            meta.size,
            meta.file_type
        );
    }
}
```

### Cleaning Orphaned Snapshots

```rust
use snapshot_comparison::SnapshotStore;

fn cleanup_old_snapshots(store: &SnapshotStore, keep: &[&str]) {
    // Dry run first
    let would_delete = store.clean(keep, true).unwrap();
    println!("Would delete: {:?}", would_delete);

    // Actually delete
    let deleted = store.clean(keep, false).unwrap();
    println!("Deleted {} snapshots", deleted.len());
}
```

### Error Handling

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, SnapshotError};

fn safe_check(store: &SnapshotStore, name: &str, content: &[u8]) {
    let config = SnapshotConfig::default();

    match store.check(name, content, &config) {
        Ok(result) => println!("Check succeeded"),
        Err(SnapshotError::NotFound(name)) => {
            println!("Snapshot '{}' not found", name);
        }
        Err(SnapshotError::InvalidName(name)) => {
            println!("Invalid snapshot name: {}", name);
        }
        Err(SnapshotError::CorruptedSnapshot(msg)) => {
            println!("Snapshot corrupted: {}", msg);
        }
        Err(SnapshotError::IoError(msg)) => {
            println!("IO error: {}", msg);
        }
        Err(SnapshotError::InvalidUtf8(msg)) => {
            println!("Invalid UTF-8: {}", msg);
        }
    }
}
```

## Working with Diffs

### DiffOutput

When a mismatch occurs, you get a `DiffOutput` with rich diff information. This is powered by [output-diffing-utility](/tools/output-diffing-utility/) which is integrated as a library dependency:

- **Text**: LCS-based unified diff with context lines
- **JSON**: Semantic structural comparison
- **Binary**: Hex dump with byte-level highlighting

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult};

fn show_diff(store: &SnapshotStore, name: &str, content: &[u8]) {
    let config = SnapshotConfig::default();

    if let Ok(CompareResult::Mismatch { diff, .. }) = store.check(name, content, &config) {
        // Format without colors
        let plain = diff.format(name, false);
        println!("{}", plain);

        // Format with ANSI colors
        let colored = diff.format(name, true);
        println!("{}", colored);

        // Check if there are changes
        if diff.has_changes() {
            println!("Changes detected!");
        }
    }
}
```

## Testing Patterns

### Test Helper Function

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult};
use std::path::PathBuf;

fn assert_snapshot(name: &str, actual: &[u8]) {
    let store = SnapshotStore::new(PathBuf::from("tests/snapshots"));
    let config = SnapshotConfig::default();

    match store.check(name, actual, &config) {
        Ok(CompareResult::Match) => {} // Test passed
        Ok(CompareResult::Mismatch { diff, .. }) => {
            panic!("Snapshot mismatch:\n{}", diff.format(name, false));
        }
        Err(e) => panic!("Snapshot error: {}", e),
        _ => unreachable!(),
    }
}

#[test]
fn test_api_response() {
    let response = get_api_response();
    assert_snapshot("api-response", response.as_bytes());
}
```

### Update Mode for Development

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult};
use std::path::PathBuf;

// Set to true during development, false for CI
const UPDATE_SNAPSHOTS: bool = false;

fn check_snapshot(name: &str, actual: &[u8]) {
    let store = SnapshotStore::new(PathBuf::from("tests/snapshots"));
    let config = SnapshotConfig {
        update_mode: UPDATE_SNAPSHOTS,
        ..Default::default()
    };

    match store.check(name, actual, &config) {
        Ok(CompareResult::Match) => {}
        Ok(CompareResult::Updated { .. }) => {
            println!("Updated snapshot: {}", name);
        }
        Ok(CompareResult::Mismatch { diff, .. }) => {
            panic!("Snapshot mismatch:\n{}", diff.format(name, false));
        }
        Err(e) => panic!("Snapshot error: {}", e),
    }
}
```
