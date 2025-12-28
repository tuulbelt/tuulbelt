//! Basic usage example for snapshot-comparison
//!
//! This example demonstrates:
//! - Creating snapshots from content
//! - Checking content against snapshots
//! - Updating snapshots
//! - Listing and deleting snapshots
//!
//! Run this example:
//!   cargo run --example basic

use snapshot_comparison::{CompareResult, FileType, SnapshotConfig, SnapshotStore};
use std::fs;

fn main() {
    println!("Snapshot Comparison - Basic Usage Examples\n");

    // Create a temporary directory for snapshots
    let temp_dir = std::env::temp_dir().join("snapcmp-example");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    let store = SnapshotStore::new(temp_dir.clone());
    let config = SnapshotConfig::default();

    // Example 1: Create a snapshot
    println!("Example 1: Create a snapshot");
    let content = b"Hello, World!\nThis is test content.";
    let snapshot = store.create("hello", content, &config).unwrap();
    println!("  Created snapshot '{}'", snapshot.metadata.name);
    println!("  Size: {} bytes", snapshot.metadata.size);
    println!("  Type: {:?}", snapshot.metadata.file_type);
    println!();

    // Example 2: Check matching content
    println!("Example 2: Check matching content");
    let same_content = b"Hello, World!\nThis is test content.";
    match store.check("hello", same_content, &config).unwrap() {
        CompareResult::Match => println!("  ✓ Content matches snapshot"),
        CompareResult::Mismatch { .. } => println!("  ✗ Content differs"),
        CompareResult::Updated { .. } => println!("  ↻ Snapshot was updated"),
    }
    println!();

    // Example 3: Check different content (mismatch)
    println!("Example 3: Check different content");
    let different_content = b"Hello, World!\nThis is DIFFERENT content.";
    match store.check("hello", different_content, &config).unwrap() {
        CompareResult::Match => println!("  ✓ Content matches"),
        CompareResult::Mismatch { diff, .. } => {
            println!("  ✗ Content differs from snapshot");
            println!("  Diff output:\n{}", diff.format("hello", false));
        }
        CompareResult::Updated { .. } => println!("  ↻ Updated"),
    }
    println!();

    // Example 4: Update mode (auto-update on mismatch)
    println!("Example 4: Update mode");
    let update_config = SnapshotConfig {
        update_mode: true,
        ..Default::default()
    };
    let new_content = b"Completely new content here.";
    match store.check("hello", new_content, &update_config).unwrap() {
        CompareResult::Match => println!("  ✓ Content matches"),
        CompareResult::Mismatch { .. } => println!("  ✗ Mismatch (not in update mode)"),
        CompareResult::Updated {
            old_snapshot,
            new_content,
        } => {
            println!("  ↻ Snapshot updated!");
            println!("  Previous size: {} bytes", old_snapshot.metadata.size);
            println!("  New size: {} bytes", new_content.len());
        }
    }
    println!();

    // Example 5: Create a JSON snapshot
    println!("Example 5: JSON snapshot");
    let json_config = SnapshotConfig {
        file_type: Some(FileType::Json),
        ..Default::default()
    };
    let json_content = br#"{"name": "test", "value": 42}"#;
    let json_snapshot = store.create("config", json_content, &json_config).unwrap();
    println!("  Created JSON snapshot: {}", json_snapshot.metadata.name);
    println!("  Type: {:?}", json_snapshot.metadata.file_type);
    println!();

    // Example 6: List all snapshots
    println!("Example 6: List snapshots");
    let snapshots = store.list().unwrap();
    println!("  Found {} snapshots:", snapshots.len());
    for s in &snapshots {
        println!("    - {} ({} bytes, {:?})", s.name, s.size, s.file_type);
    }
    println!();

    // Example 7: Delete a snapshot
    println!("Example 7: Delete snapshot");
    store.delete("config").unwrap();
    println!("  Deleted snapshot 'config'");

    let remaining = store.list().unwrap();
    println!("  Remaining: {} snapshots", remaining.len());
    println!();

    // Cleanup
    println!("Cleaning up...");
    fs::remove_dir_all(&temp_dir).expect("Failed to cleanup");
    println!("Done!");
}
