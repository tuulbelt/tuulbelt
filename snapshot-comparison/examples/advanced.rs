//! Advanced usage patterns for snapshot-comparison
//!
//! This example demonstrates:
//! - Colored diff output
//! - Binary snapshot handling
//! - Orphan cleanup
//! - Context lines configuration
//! - Error handling patterns
//!
//! Run this example:
//!   cargo run --example advanced

use snapshot_comparison::{CompareResult, FileType, SnapshotConfig, SnapshotError, SnapshotStore};
use std::fs;

fn main() {
    println!("Snapshot Comparison - Advanced Usage Patterns\n");

    // Create a temporary directory
    let temp_dir = std::env::temp_dir().join("snapcmp-advanced");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    let store = SnapshotStore::new(temp_dir.clone());

    // Example 1: Colored diff output
    println!("Example 1: Colored diff output");
    {
        let config = SnapshotConfig::default();
        store
            .create("colortest", b"line 1\nline 2\nline 3", &config)
            .unwrap();

        let color_config = SnapshotConfig {
            color: true,
            ..Default::default()
        };

        let changed = b"line 1\nmodified line 2\nline 3\nline 4";
        if let CompareResult::Mismatch { diff, .. } =
            store.check("colortest", changed, &color_config).unwrap()
        {
            println!("  Colored diff (ANSI codes):");
            // Note: Colors only show in terminal, not in documentation
            println!("{}", diff.format("colortest", true));
        }
    }
    println!();

    // Example 2: Context lines configuration
    println!("Example 2: Context lines (unified diff)");
    {
        let content: Vec<u8> = (1..=20)
            .map(|i| format!("line {}\n", i))
            .collect::<String>()
            .into_bytes();
        let config = SnapshotConfig::default();
        store.create("context", &content, &config).unwrap();

        // Modify one line in the middle
        let mut modified = content.clone();
        let changed_line = b"CHANGED LINE 10\n";
        let start = modified.windows(8).position(|w| w == b"line 10\n").unwrap();
        modified.splice(start..start + 8, changed_line.iter().copied());

        // Show with different context sizes
        for ctx in [1, 3, 5] {
            let ctx_config = SnapshotConfig {
                context_lines: ctx,
                ..Default::default()
            };
            if let CompareResult::Mismatch { diff, .. } =
                store.check("context", &modified, &ctx_config).unwrap()
            {
                println!("  With {} context lines:", ctx);
                let output = diff.format("context", false);
                let line_count = output.lines().count();
                println!("    Diff output is {} lines", line_count);
            }
        }
    }
    println!();

    // Example 3: Binary snapshot handling
    println!("Example 3: Binary snapshots");
    {
        let binary_config = SnapshotConfig {
            file_type: Some(FileType::Binary),
            ..Default::default()
        };

        // Create binary content with null bytes
        let binary_data: Vec<u8> = (0..=255).collect();
        store
            .create("binary", &binary_data, &binary_config)
            .unwrap();

        // Check with modified binary
        let mut modified = binary_data.clone();
        modified[100] = 0xFF;
        modified[200] = 0x00;

        if let CompareResult::Mismatch { diff, .. } =
            store.check("binary", &modified, &binary_config).unwrap()
        {
            println!("  Binary diff output:");
            println!("{}", diff.format("binary", false));
        }
    }
    println!();

    // Example 4: Orphan cleanup
    println!("Example 4: Orphan cleanup");
    {
        let config = SnapshotConfig::default();

        // Create several snapshots
        store.create("keep-1", b"content 1", &config).unwrap();
        store.create("keep-2", b"content 2", &config).unwrap();
        store.create("orphan-1", b"orphan 1", &config).unwrap();
        store.create("orphan-2", b"orphan 2", &config).unwrap();

        println!(
            "  Before cleanup: {} snapshots",
            store.list().unwrap().len()
        );

        // Clean up orphans, keeping only specific ones
        let keep_list = vec!["keep-1", "keep-2"];
        let deleted = store.clean(&keep_list, false).unwrap();

        println!("  Deleted {} orphaned snapshots:", deleted.len());
        for name in &deleted {
            println!("    - {}", name);
        }
        println!("  After cleanup: {} snapshots", store.list().unwrap().len());
    }
    println!();

    // Example 5: Dry-run cleanup
    println!("Example 5: Dry-run cleanup");
    {
        let config = SnapshotConfig::default();

        // Create more snapshots
        store.create("active", b"active", &config).unwrap();
        store.create("stale", b"stale", &config).unwrap();

        // Dry-run shows what WOULD be deleted without deleting
        let would_delete = store.clean(&["active"], true).unwrap();
        println!("  Would delete {} snapshots (dry-run):", would_delete.len());
        for name in &would_delete {
            println!("    - {}", name);
        }

        // Verify nothing was actually deleted
        let all = store.list().unwrap();
        println!("  Still have {} snapshots", all.len());
    }
    println!();

    // Example 6: Error handling
    println!("Example 6: Error handling");
    {
        // Not found error
        let config = SnapshotConfig::default();
        match store.check("nonexistent", b"content", &config) {
            Err(SnapshotError::NotFound(name)) => {
                println!("  NotFound: {}", name);
            }
            _ => println!("  Unexpected result"),
        }

        // Invalid name error
        match store.create("invalid/name", b"content", &config) {
            Err(SnapshotError::InvalidName(name)) => {
                println!("  InvalidName: {}", name);
            }
            _ => println!("  Unexpected result"),
        }

        // Path traversal attempt
        match store.create("../escape", b"content", &config) {
            Err(SnapshotError::InvalidName(name)) => {
                println!("  Path traversal blocked: {}", name);
            }
            _ => println!("  Unexpected result"),
        }
    }
    println!();

    // Example 7: Programmatic snapshot management
    println!("Example 7: Batch operations");
    {
        let config = SnapshotConfig::default();

        // Create test fixtures
        let fixtures = vec![
            ("api-response-1", br#"{"status": "ok"}"#.to_vec()),
            ("api-response-2", br#"{"status": "error"}"#.to_vec()),
            ("api-response-3", br#"{"status": "pending"}"#.to_vec()),
        ];

        // Batch create
        for (name, content) in &fixtures {
            store.create(name, content, &config).unwrap();
        }
        println!("  Created {} test fixtures", fixtures.len());

        // Batch check
        let mut matches = 0;
        let mut mismatches = 0;
        for (name, content) in &fixtures {
            match store.check(name, content, &config).unwrap() {
                CompareResult::Match => matches += 1,
                CompareResult::Mismatch { .. } => mismatches += 1,
                _ => {}
            }
        }
        println!(
            "  Batch check: {} matches, {} mismatches",
            matches, mismatches
        );
    }
    println!();

    // Cleanup
    println!("Cleaning up...");
    fs::remove_dir_all(&temp_dir).expect("Failed to cleanup");
    println!("Done!");
}
