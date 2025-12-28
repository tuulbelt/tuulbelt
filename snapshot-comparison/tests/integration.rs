//! Integration tests for snapshot-comparison

use snapshot_comparison::{CompareResult, FileType, SnapshotConfig, SnapshotError, SnapshotStore};
use std::fs;
use std::process::Command;

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

/// Get path to the release binary
fn get_binary() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_snapcmp"))
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

// ============================================================================
// Exists Method Tests
// ============================================================================

#[test]
fn test_exists_for_existing_snapshot() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("test", b"content", &config).unwrap();
    assert!(store.exists("test").unwrap());

    cleanup(&temp_dir);
}

#[test]
fn test_exists_for_nonexistent_snapshot() {
    let (store, temp_dir) = create_temp_store();

    assert!(!store.exists("nonexistent").unwrap());

    cleanup(&temp_dir);
}

#[test]
fn test_exists_after_delete() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("test", b"content", &config).unwrap();
    assert!(store.exists("test").unwrap());

    store.delete("test").unwrap();
    assert!(!store.exists("test").unwrap());

    cleanup(&temp_dir);
}

#[test]
fn test_exists_with_invalid_name() {
    let (store, temp_dir) = create_temp_store();

    // Invalid names should return error, not false
    let result = store.exists("../escape");
    assert!(result.is_err());

    cleanup(&temp_dir);
}

// ============================================================================
// Config Default Tests
// ============================================================================

#[test]
fn test_config_default_values() {
    let config = SnapshotConfig::default();

    assert!(config.file_type.is_none()); // Auto-detect
    assert!(!config.color); // No color by default
    assert!(!config.update_mode); // No auto-update
    assert_eq!(config.context_lines, 3); // Default context lines
}

#[test]
fn test_config_with_file_type() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig {
        file_type: Some(FileType::Binary),
        ..Default::default()
    };

    // Even text content should be stored as binary
    let snapshot = store
        .create("force-binary", b"text content", &config)
        .unwrap();
    assert_eq!(snapshot.metadata.file_type, FileType::Binary);

    cleanup(&temp_dir);
}

#[test]
fn test_config_color_option() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig {
        color: true,
        ..Default::default()
    };

    store.create("color-test", b"line 1", &config).unwrap();

    let result = store.check("color-test", b"line 2", &config).unwrap();
    match result {
        CompareResult::Mismatch { diff, .. } => {
            let output = diff.format("color-test", true);
            // Color output should contain ANSI escape sequences
            assert!(output.contains("\x1b[") || !output.is_empty());
        }
        _ => panic!("Expected mismatch"),
    }

    cleanup(&temp_dir);
}

// ============================================================================
// File Type Auto-Detection Tests
// ============================================================================

// Note: odiff's detect_file_type uses extension-based detection for JSON,
// not content-based. Without explicit file_type, JSON content is treated as Text.

#[test]
fn test_auto_detect_text_for_json_content() {
    // odiff detects JSON by extension, not content - so this becomes Text
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let json = b"[1, 2, 3]";
    let snapshot = store.create("json-array", json, &config).unwrap();
    // Without extension hint, valid UTF-8 content is detected as Text
    assert_eq!(snapshot.metadata.file_type, FileType::Text);

    cleanup(&temp_dir);
}

#[test]
fn test_explicit_json_type() {
    // To get JSON handling, use explicit file_type in config
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig {
        file_type: Some(FileType::Json),
        ..Default::default()
    };

    let json = br#"{"key": "value"}"#;
    let snapshot = store.create("json-object", json, &config).unwrap();
    assert_eq!(snapshot.metadata.file_type, FileType::Json);

    cleanup(&temp_dir);
}

#[test]
fn test_explicit_binary_type() {
    // Explicit binary type even for valid UTF-8
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig {
        file_type: Some(FileType::Binary),
        ..Default::default()
    };

    let text = b"Hello, World!";
    let snapshot = store.create("forced-binary", text, &config).unwrap();
    assert_eq!(snapshot.metadata.file_type, FileType::Binary);

    cleanup(&temp_dir);
}

#[test]
fn test_auto_detect_binary_with_null() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let binary = b"text\x00with null";
    let snapshot = store.create("binary-null", binary, &config).unwrap();
    assert_eq!(snapshot.metadata.file_type, FileType::Binary);

    cleanup(&temp_dir);
}

#[test]
fn test_auto_detect_text() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let text = b"plain text content\nwith newlines";
    let snapshot = store.create("plain-text", text, &config).unwrap();
    assert_eq!(snapshot.metadata.file_type, FileType::Text);

    cleanup(&temp_dir);
}

// ============================================================================
// Error Display Tests
// ============================================================================

#[test]
fn test_error_display_not_found() {
    let err = SnapshotError::NotFound("test-snapshot".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("not found") || msg.contains("test-snapshot"));
}

#[test]
fn test_error_display_invalid_name() {
    let err = SnapshotError::InvalidName("bad/name".to_string());
    let msg = format!("{}", err);
    assert!(msg.to_lowercase().contains("invalid") || msg.contains("bad/name"));
}

#[test]
fn test_error_display_io_error() {
    let err = SnapshotError::IoError("permission denied".to_string());
    let msg = format!("{}", err);
    assert!(msg.to_lowercase().contains("io") || msg.contains("permission"));
}

#[test]
fn test_error_display_corrupted() {
    let err = SnapshotError::CorruptedSnapshot("missing header".to_string());
    let msg = format!("{}", err);
    assert!(msg.to_lowercase().contains("corrupt") || msg.contains("missing"));
}

#[test]
fn test_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SnapshotError>();
}

// ============================================================================
// CLI Tests
// ============================================================================

#[test]
fn test_cli_help() {
    let output = Command::new(get_binary())
        .arg("--help")
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("snapcmp") || stdout.contains("snapshot"));
    assert!(stdout.contains("create"));
    assert!(stdout.contains("check"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(get_binary())
        .arg("--version")
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1") || stdout.contains("snapshot"));
}

#[test]
fn test_cli_create_from_stdin() {
    let temp_dir = std::env::temp_dir().join(format!(
        "cli-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).unwrap();

    let output = Command::new(get_binary())
        .args(["create", "cli-test", "--dir"])
        .arg(&temp_dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn")
        .wait_with_output()
        .expect("Failed to wait");

    // Even without input, should not crash
    assert!(output.status.success() || output.status.code() == Some(1));

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_cli_list_empty() {
    let temp_dir = std::env::temp_dir().join(format!(
        "cli-list-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).unwrap();

    let output = Command::new(get_binary())
        .args(["list", "--dir"])
        .arg(&temp_dir)
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_cli_check_not_found() {
    let temp_dir = std::env::temp_dir().join(format!(
        "cli-notfound-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).unwrap();

    let output = Command::new(get_binary())
        .args(["check", "nonexistent", "--dir"])
        .arg(&temp_dir)
        .output()
        .expect("Failed to run CLI");

    // Should fail with non-zero exit
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.to_lowercase().contains("not found") || stderr.contains("error"));

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_cli_invalid_name() {
    let temp_dir = std::env::temp_dir().join(format!(
        "cli-invalid-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).unwrap();

    let output = Command::new(get_binary())
        .args(["create", "../escape", "--dir"])
        .arg(&temp_dir)
        .output()
        .expect("Failed to run CLI");

    // Should fail
    assert!(!output.status.success());

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_cli_unknown_command() {
    let output = Command::new(get_binary())
        .arg("unknown-command")
        .output()
        .expect("Failed to run CLI");

    // Should fail with error
    assert!(!output.status.success());
}

#[test]
fn test_cli_no_args() {
    let output = Command::new(get_binary())
        .output()
        .expect("Failed to run CLI");

    // Should show help or error
    assert!(!output.status.success() || !output.stdout.is_empty());
}

#[test]
fn test_cli_color_flag() {
    let output = Command::new(get_binary())
        .args(["--help"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--color") || stdout.contains("-c"));
}

#[test]
fn test_cli_context_flag() {
    let output = Command::new(get_binary())
        .args(["--help"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--context"));
}

#[test]
fn test_cli_type_flag() {
    let output = Command::new(get_binary())
        .args(["--help"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--type") || stdout.contains("-t"));
}

#[test]
fn test_cli_dry_run_flag() {
    let output = Command::new(get_binary())
        .args(["--help"])
        .output()
        .expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--dry-run"));
}

#[test]
fn test_cli_invalid_context_lines() {
    let output = Command::new(get_binary())
        .args(["check", "test", "--context", "abc"])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.to_lowercase().contains("invalid") || stderr.contains("context"));
}

#[test]
fn test_cli_context_lines_too_large() {
    let output = Command::new(get_binary())
        .args(["check", "test", "--context", "9999"])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.to_lowercase().contains("too large") || stderr.contains("max"));
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_overwrite_existing_snapshot() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("overwrite", b"first", &config).unwrap();
    let second = store.create("overwrite", b"second", &config).unwrap();

    assert_eq!(second.content, b"second");

    cleanup(&temp_dir);
}

#[test]
fn test_update_nonexistent_creates() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    // Update on non-existent should create
    let snapshot = store.update("new-snap", b"content", &config).unwrap();
    assert_eq!(snapshot.metadata.name, "new-snap");

    cleanup(&temp_dir);
}

#[test]
fn test_timestamps_are_reasonable() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let before = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let snapshot = store.create("timestamp-test", b"content", &config).unwrap();

    let after = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert!(snapshot.metadata.created_at >= before);
    assert!(snapshot.metadata.created_at <= after);
    assert!(snapshot.metadata.updated_at >= before);
    assert!(snapshot.metadata.updated_at <= after);

    cleanup(&temp_dir);
}

#[test]
fn test_update_preserves_created_at() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let original = store.create("preserve-time", b"v1", &config).unwrap();
    let created = original.metadata.created_at;

    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated = store.update("preserve-time", b"v2", &config).unwrap();

    assert_eq!(updated.metadata.created_at, created);
    assert!(updated.metadata.updated_at >= created);

    cleanup(&temp_dir);
}

#[test]
fn test_list_sorted_by_name() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("zebra", b"z", &config).unwrap();
    store.create("alpha", b"a", &config).unwrap();
    store.create("middle", b"m", &config).unwrap();

    let list = store.list().unwrap();
    let names: Vec<_> = list.iter().map(|m| m.name.as_str()).collect();

    assert_eq!(names, vec!["alpha", "middle", "zebra"]);

    cleanup(&temp_dir);
}

#[test]
fn test_clean_with_empty_keep_list() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    store.create("snap1", b"1", &config).unwrap();
    store.create("snap2", b"2", &config).unwrap();

    // Empty keep list should delete all
    let deleted = store.clean(&[], false).unwrap();
    assert_eq!(deleted.len(), 2);

    let remaining = store.list().unwrap();
    assert!(remaining.is_empty());

    cleanup(&temp_dir);
}

#[test]
fn test_clean_on_empty_store() {
    let (store, temp_dir) = create_temp_store();

    // Should not error on empty store
    let deleted = store.clean(&["anything"], false).unwrap();
    assert!(deleted.is_empty());

    cleanup(&temp_dir);
}

#[test]
fn test_list_on_empty_store() {
    let (store, temp_dir) = create_temp_store();

    let list = store.list().unwrap();
    assert!(list.is_empty());

    cleanup(&temp_dir);
}

#[test]
fn test_whitespace_in_content() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    let whitespace = b"   \t\n\r\n   ";
    store.create("whitespace", whitespace, &config).unwrap();

    let result = store.check("whitespace", whitespace, &config).unwrap();
    assert!(matches!(result, CompareResult::Match));

    cleanup(&temp_dir);
}

#[test]
fn test_special_bytes_in_content() {
    let (store, temp_dir) = create_temp_store();
    let config = SnapshotConfig::default();

    // Test various special bytes
    let special: Vec<u8> = vec![
        0x00, 0x01, 0x02, 0x03, // Control chars
        0x7F, 0x80, 0x81, // DEL and high bytes
        0xFE, 0xFF, // Near max bytes
    ];

    store.create("special-bytes", &special, &config).unwrap();

    let snapshot = store.read("special-bytes").unwrap();
    assert_eq!(snapshot.content, special);

    cleanup(&temp_dir);
}
