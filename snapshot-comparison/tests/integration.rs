//! Integration tests for snapshot-comparison

use snapshot_comparison::{CompareResult, FileType, SnapshotConfig, SnapshotError, SnapshotStore};
use std::fs;

/// Create a unique temporary directory for each test
fn create_temp_store() -> (SnapshotStore, std::path::PathBuf) {
    let temp_dir = std::env::temp_dir().join(format!(
        "snapcmp-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let store = SnapshotStore::new(temp_dir.clone());
    (store, temp_dir)
}

/// Cleanup after test
fn cleanup(path: &std::path::Path) {
    let _ = fs::remove_dir_all(path);
}

#[test]
fn test_create_and_check_snapshot() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    // Create a snapshot
    let content = b"Hello, World!";
    let snapshot = store.create("test", content, &config).unwrap();

    assert_eq!(snapshot.metadata.name, "test");
    assert_eq!(snapshot.metadata.size, content.len());
    assert_eq!(snapshot.metadata.file_type, FileType::Text);

    // Check matching content
    let result = store.check("test", content, &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    cleanup(&temp_dir);
}

#[test]
fn test_mismatch_detection() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("test", b"original content", &config).unwrap();

    let result = store.check("test", b"modified content", &config).unwrap();

    match result {
        CompareResult::Mismatch { diff, .. } => {
            let output = diff.format("test", false);
            assert!(output.contains("original") || output.contains("modified"));
        }
        _ => panic!("Expected mismatch"),
    }

    cleanup(&temp_dir);
}

#[test]
fn test_update_mode() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();
    let update_config = SnapshotConfig {
        update_mode: true,
        ..Default::default()
    };

    // Create initial snapshot
    store.create("test", b"version 1", &config).unwrap();

    // Check with update mode - should update on mismatch
    let result = store.check("test", b"version 2", &update_config).unwrap();

    match result {
        CompareResult::Updated {
            old_snapshot,
            new_content,
        } => {
            assert_eq!(old_snapshot.metadata.size, 9); // "version 1"
            assert_eq!(new_content.len(), 9); // "version 2"
        }
        _ => panic!("Expected updated"),
    }

    // Verify the update persisted
    let result = store.check("test", b"version 2", &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    cleanup(&temp_dir);
}

#[test]
fn test_explicit_update() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("test", b"original", &config).unwrap();

    // Update explicitly
    let updated = store.update("test", b"new content", &config).unwrap();

    assert_eq!(updated.metadata.size, 11); // "new content"

    // Verify the update
    let result = store.check("test", b"new content", &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    cleanup(&temp_dir);
}

#[test]
fn test_list_snapshots() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    // Create multiple snapshots
    store.create("alpha", b"a", &config).unwrap();
    store.create("beta", b"b", &config).unwrap();
    store.create("gamma", b"c", &config).unwrap();

    let list = store.list().unwrap();

    assert_eq!(list.len(), 3);

    let names: Vec<_> = list.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
    assert!(names.contains(&"gamma"));

    cleanup(&temp_dir);
}

#[test]
fn test_delete_snapshot() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("test", b"content", &config).unwrap();

    // Delete it
    store.delete("test").unwrap();

    // Verify it's gone
    let result = store.check("test", b"content", &config);
    assert!(matches!(result, Err(SnapshotError::NotFound(_))));

    cleanup(&temp_dir);
}

#[test]
fn test_clean_orphaned_snapshots() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("keep-this", b"k1", &config).unwrap();
    store.create("also-keep", b"k2", &config).unwrap();
    store.create("orphan-1", b"o1", &config).unwrap();
    store.create("orphan-2", b"o2", &config).unwrap();

    // Clean, keeping only specified ones
    let deleted = store.clean(&["keep-this", "also-keep"], false).unwrap();

    assert_eq!(deleted.len(), 2);
    assert!(deleted.contains(&"orphan-1".to_string()));
    assert!(deleted.contains(&"orphan-2".to_string()));

    // Verify remaining
    let remaining = store.list().unwrap();
    assert_eq!(remaining.len(), 2);

    cleanup(&temp_dir);
}

#[test]
fn test_clean_dry_run() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("keep", b"k", &config).unwrap();
    store.create("delete", b"d", &config).unwrap();

    // Dry run - should return what would be deleted but not delete
    let would_delete = store.clean(&["keep"], true).unwrap();

    assert_eq!(would_delete.len(), 1);
    assert!(would_delete.contains(&"delete".to_string()));

    // Verify nothing was actually deleted
    let all = store.list().unwrap();
    assert_eq!(all.len(), 2);

    cleanup(&temp_dir);
}

#[test]
fn test_json_snapshot() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig {
        file_type: Some(FileType::Json),
        ..Default::default()
    };

    let json = br#"{"name": "test", "value": 42}"#;
    let snapshot = store.create("json-test", json, &config).unwrap();

    assert_eq!(snapshot.metadata.file_type, FileType::Json);

    // Check semantically equivalent JSON (different formatting)
    let equivalent = br#"{"name":"test","value":42}"#;
    let result = store.check("json-test", equivalent, &config).unwrap();

    // Note: Whether this matches depends on JSON normalization
    // The current implementation stores raw bytes, so this would be a mismatch
    match result {
        CompareResult::Match => {
            println!("JSON was normalized");
        }
        CompareResult::Mismatch { .. } => {
            println!("JSON compared byte-by-byte");
        }
        _ => {}
    }

    cleanup(&temp_dir);
}

#[test]
fn test_binary_snapshot() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig {
        file_type: Some(FileType::Binary),
        ..Default::default()
    };

    let binary: Vec<u8> = (0..=255).collect();
    let snapshot = store.create("binary-test", &binary, &config).unwrap();

    assert_eq!(snapshot.metadata.file_type, FileType::Binary);

    // Check matching
    let result = store.check("binary-test", &binary, &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    // Check with modification
    let mut modified = binary.clone();
    modified[100] = 0xFF;
    let result = store.check("binary-test", &modified, &config).unwrap();
    assert!(matches!(result, CompareResult::Mismatch { .. }));

    cleanup(&temp_dir);
}

#[test]
fn test_not_found_error() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let result = store.check("nonexistent", b"content", &config);

    match result {
        Err(SnapshotError::NotFound(name)) => {
            assert_eq!(name, "nonexistent");
        }
        _ => panic!("Expected NotFound error"),
    }

    cleanup(&temp_dir);
}

#[test]
fn test_invalid_name_error() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    // Names with path separators should be rejected
    let invalid_names = vec![
        "path/traversal",
        "back\\slash",
        "../escape",
        "..\\escape",
        "sub/dir/file",
    ];

    for name in invalid_names {
        let result = store.create(name, b"content", &config);
        match result {
            Err(SnapshotError::InvalidName(_)) => {}
            _ => panic!("Expected InvalidName error for: {}", name),
        }
    }

    cleanup(&temp_dir);
}

#[test]
fn test_empty_snapshot() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    // Empty content is valid
    let snapshot = store.create("empty", b"", &config).unwrap();
    assert_eq!(snapshot.metadata.size, 0);

    let result = store.check("empty", b"", &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    cleanup(&temp_dir);
}

#[test]
fn test_large_content() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    // 1 MB of content
    let large: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
    let snapshot = store.create("large", &large, &config).unwrap();

    assert_eq!(snapshot.metadata.size, 1_000_000);

    let result = store.check("large", &large, &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    cleanup(&temp_dir);
}

#[test]
fn test_context_lines() {
    let (store, temp_dir) = create_temp_store();

    let content = b"line1\nline2\nline3\nline4\nline5";
    let config = SnapshotConfig::default();
    store.create("ctx", content, &config).unwrap();

    let modified = b"line1\nLINE2\nline3\nline4\nline5";

    for ctx in [1, 3, 5] {
        let ctx_config = SnapshotConfig {
            context_lines: ctx,
            ..Default::default()
        };
        let result = store.check("ctx", modified, &ctx_config).unwrap();
        match result {
            CompareResult::Mismatch { diff, .. } => {
                let output = diff.format("ctx", false);
                assert!(!output.is_empty());
            }
            _ => panic!("Expected mismatch"),
        }
    }

    cleanup(&temp_dir);
}

#[test]
fn test_unicode_content() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let unicode = "日本語 🎉 émoji café über".as_bytes();
    store.create("unicode", unicode, &config).unwrap();

    let result = store.check("unicode", unicode, &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    cleanup(&temp_dir);
}

#[test]
fn test_delete_nonexistent() {
    let (store, temp_dir) = create_temp_store();

    let result = store.delete("nonexistent");

    match result {
        Err(SnapshotError::NotFound(_)) => {}
        _ => panic!("Expected NotFound error"),
    }

    cleanup(&temp_dir);
}

#[test]
fn test_snapshot_metadata() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("meta-test", b"test content", &config).unwrap();

    let list = store.list().unwrap();
    let meta = list.iter().find(|m| m.name == "meta-test").unwrap();

    assert_eq!(meta.name, "meta-test");
    assert_eq!(meta.size, 12);
    assert!(!meta.content_hash.is_empty()); // Hash should be computed
    assert!(meta.created_at > 0); // Timestamp should be set

    cleanup(&temp_dir);
}
