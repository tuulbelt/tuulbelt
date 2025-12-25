//! CLI Integration Tests
//!
//! Tests for the file-semaphore CLI binary.

use std::fs;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

/// Helper to get the binary path
fn binary_path() -> String {
    // Use the debug binary during tests
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove deps
    path.push("file-semaphore");
    path.to_string_lossy().to_string()
}

/// Helper to run the CLI with args
fn run_cli(args: &[&str]) -> Output {
    Command::new(binary_path())
        .args(args)
        .output()
        .expect("Failed to execute command")
}

/// Helper to create a unique temp lock file path
fn temp_lock_path(name: &str) -> String {
    format!("/tmp/test-cli-{}-{}.lock", name, std::process::id())
}

/// Cleanup helper
fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
}

// ============================================================================
// Help and Version Tests
// ============================================================================

#[test]
fn test_cli_help() {
    let output = run_cli(&["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Help goes to stderr
    assert!(stderr.contains("file-semaphore") || stdout.contains("file-semaphore"));
    assert!(stderr.contains("USAGE") || stdout.contains("USAGE"));
    assert!(stderr.contains("COMMANDS") || stdout.contains("COMMANDS"));
    assert!(output.status.success());
}

#[test]
fn test_cli_help_short() {
    let output = run_cli(&["-h"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("USAGE"));
    assert!(output.status.success());
}

#[test]
fn test_cli_version() {
    let output = run_cli(&["--version"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("file-semaphore"));
    assert!(stdout.contains("0.1.0") || stdout.contains("0."));
    assert!(output.status.success());
}

#[test]
fn test_cli_version_short() {
    let output = run_cli(&["-V"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("file-semaphore"));
    assert!(output.status.success());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_cli_no_args() {
    let output = run_cli(&[]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("missing command") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_missing_path() {
    let output = run_cli(&["try"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("missing lock path") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_unknown_command() {
    let output = run_cli(&["invalid", "/tmp/test.lock"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("unknown command") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_unknown_option() {
    let output = run_cli(&["try", "/tmp/test.lock", "--invalid-option"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("unknown option") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_timeout_missing_value() {
    let output = run_cli(&["try", "/tmp/test.lock", "--timeout"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("--timeout requires a value") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_timeout_invalid_value() {
    let output = run_cli(&["try", "/tmp/test.lock", "--timeout", "abc"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("invalid timeout") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_stale_missing_value() {
    let output = run_cli(&["try", "/tmp/test.lock", "--stale"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("--stale requires a value") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_cli_tag_missing_value() {
    let output = run_cli(&["try", "/tmp/test.lock", "--tag"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("--tag requires a value") || stderr.contains("Error"));
    assert_eq!(output.status.code(), Some(2));
}

// ============================================================================
// Try Command Tests
// ============================================================================

#[test]
fn test_cli_try_acquire_success() {
    let path = temp_lock_path("try-success");
    cleanup(&path);

    let output = run_cli(&["try", &path]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Lock acquired"));
    assert!(output.status.success());

    // Verify lock file exists
    assert!(std::path::Path::new(&path).exists());

    cleanup(&path);
}

#[test]
fn test_cli_try_acquire_with_tag() {
    let path = temp_lock_path("try-tag");
    cleanup(&path);

    let output = run_cli(&["try", &path, "--tag", "my-test-tag"]);
    assert!(output.status.success());

    // Verify tag in lock file
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("tag=my-test-tag"));

    cleanup(&path);
}

#[test]
fn test_cli_try_acquire_quiet() {
    let path = temp_lock_path("try-quiet");
    cleanup(&path);

    let output = run_cli(&["try", &path, "--quiet"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.is_empty() || stdout.trim().is_empty());
    assert!(output.status.success());

    cleanup(&path);
}

#[test]
fn test_cli_try_acquire_quiet_short() {
    let path = temp_lock_path("try-quiet-short");
    cleanup(&path);

    let output = run_cli(&["try", &path, "-q"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.is_empty() || stdout.trim().is_empty());
    assert!(output.status.success());

    cleanup(&path);
}

#[test]
fn test_cli_try_acquire_already_locked() {
    let path = temp_lock_path("try-locked");
    cleanup(&path);

    // First acquire should succeed
    let output1 = run_cli(&["try", &path]);
    assert!(output1.status.success());

    // Second acquire should fail
    let output2 = run_cli(&["try", &path]);
    let stderr = String::from_utf8_lossy(&output2.stderr);

    assert!(stderr.contains("Lock already held") || stderr.contains("already"));
    assert_eq!(output2.status.code(), Some(1));

    cleanup(&path);
}

#[test]
fn test_cli_try_acquire_stale_lock_recovery() {
    let path = temp_lock_path("try-stale");
    cleanup(&path);

    // Create a stale lock (2 hours old)
    let old_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - 7200;
    let content = format!("pid=99999\ntimestamp={}", old_timestamp);
    fs::write(&path, content).unwrap();

    // Try to acquire with 1 hour stale timeout - should succeed
    let output = run_cli(&["try", &path, "--stale", "3600"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Lock acquired"));
    assert!(output.status.success());

    cleanup(&path);
}

// ============================================================================
// Acquire Command Tests
// ============================================================================

#[test]
fn test_cli_acquire_success() {
    let path = temp_lock_path("acquire-success");
    cleanup(&path);

    let output = run_cli(&["acquire", &path, "--timeout", "1"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Lock acquired"));
    assert!(output.status.success());

    cleanup(&path);
}

#[test]
fn test_cli_acquire_timeout() {
    let path = temp_lock_path("acquire-timeout");
    cleanup(&path);

    // First acquire
    run_cli(&["try", &path]);

    // Second acquire with short timeout should fail
    let output = run_cli(&["acquire", &path, "--timeout", "1"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("Timeout") || stderr.contains("timeout"));
    assert_eq!(output.status.code(), Some(1));

    cleanup(&path);
}

#[test]
fn test_cli_acquire_with_tag() {
    let path = temp_lock_path("acquire-tag");
    cleanup(&path);

    let output = run_cli(&["acquire", &path, "--timeout", "1", "--tag", "acquire-test"]);
    assert!(output.status.success());

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("tag=acquire-test"));

    cleanup(&path);
}

// ============================================================================
// Release Command Tests
// ============================================================================

#[test]
fn test_cli_release_success() {
    let path = temp_lock_path("release-success");
    cleanup(&path);

    // Acquire first
    run_cli(&["try", &path]);
    assert!(std::path::Path::new(&path).exists());

    // Release
    let output = run_cli(&["release", &path]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Lock released"));
    assert!(output.status.success());
    assert!(!std::path::Path::new(&path).exists());

    cleanup(&path);
}

#[test]
fn test_cli_release_not_locked() {
    let path = temp_lock_path("release-not-locked");
    cleanup(&path);

    // Release when not locked - should still succeed (force release)
    let output = run_cli(&["release", &path]);

    // force_release returns Ok even if file doesn't exist
    assert!(output.status.success() || output.status.code() == Some(0));

    cleanup(&path);
}

#[test]
fn test_cli_release_quiet() {
    let path = temp_lock_path("release-quiet");
    cleanup(&path);

    run_cli(&["try", &path]);

    let output = run_cli(&["release", &path, "--quiet"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.is_empty() || stdout.trim().is_empty());
    assert!(output.status.success());

    cleanup(&path);
}

// ============================================================================
// Status Command Tests
// ============================================================================

#[test]
fn test_cli_status_locked() {
    let path = temp_lock_path("status-locked");
    cleanup(&path);

    run_cli(&["try", &path, "--tag", "status-test"]);

    let output = run_cli(&["status", &path]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("LOCKED"));
    assert!(stdout.contains("PID:"));
    assert!(stdout.contains("Timestamp:"));
    assert!(stdout.contains("Tag: status-test"));
    assert_eq!(output.status.code(), Some(1)); // Returns 1 when locked

    cleanup(&path);
}

#[test]
fn test_cli_status_free() {
    let path = temp_lock_path("status-free");
    cleanup(&path);

    let output = run_cli(&["status", &path]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("FREE"));
    assert!(output.status.success()); // Returns 0 when free

    cleanup(&path);
}

#[test]
fn test_cli_status_json_locked() {
    let path = temp_lock_path("status-json-locked");
    cleanup(&path);

    run_cli(&["try", &path, "--tag", "json-test"]);

    let output = run_cli(&["status", &path, "--json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains(r#""locked":true"#));
    assert!(stdout.contains(r#""pid":"#));
    assert!(stdout.contains(r#""timestamp":"#));
    assert!(stdout.contains(r#""tag":"json-test""#));

    cleanup(&path);
}

#[test]
fn test_cli_status_json_free() {
    let path = temp_lock_path("status-json-free");
    cleanup(&path);

    let output = run_cli(&["status", &path, "--json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains(r#""locked":false"#));
    assert!(output.status.success());

    cleanup(&path);
}

#[test]
fn test_cli_status_no_tag() {
    let path = temp_lock_path("status-no-tag");
    cleanup(&path);

    run_cli(&["try", &path]); // No tag

    let output = run_cli(&["status", &path, "--json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains(r#""tag":null"#));

    cleanup(&path);
}

// ============================================================================
// Wait Command Tests
// ============================================================================

#[test]
fn test_cli_wait_already_free() {
    let path = temp_lock_path("wait-free");
    cleanup(&path);

    let output = run_cli(&["wait", &path, "--timeout", "1"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Lock is free"));
    assert!(output.status.success());

    cleanup(&path);
}

#[test]
fn test_cli_wait_timeout() {
    let path = temp_lock_path("wait-timeout");
    cleanup(&path);

    // Create a lock
    run_cli(&["try", &path]);

    // Wait with short timeout
    let output = run_cli(&["wait", &path, "--timeout", "1"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("Timeout") || stderr.contains("timeout"));
    assert_eq!(output.status.code(), Some(1));

    cleanup(&path);
}

#[test]
fn test_cli_wait_released() {
    let path = temp_lock_path("wait-released");
    cleanup(&path);

    // Create a lock
    run_cli(&["try", &path]);

    // Spawn thread to release after a short delay
    let path_clone = path.clone();
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let _ = fs::remove_file(&path_clone);
    });

    // Wait for lock (should succeed after release)
    let output = run_cli(&["wait", &path, "--timeout", "5"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    handle.join().unwrap();

    assert!(stdout.contains("Lock is free"));
    assert!(output.status.success());

    cleanup(&path);
}

#[test]
fn test_cli_wait_quiet() {
    let path = temp_lock_path("wait-quiet");
    cleanup(&path);

    let output = run_cli(&["wait", &path, "--timeout", "1", "--quiet"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.is_empty() || stdout.trim().is_empty());
    assert!(output.status.success());

    cleanup(&path);
}

// ============================================================================
// Edge Cases and Special Characters
// ============================================================================

#[test]
fn test_cli_tag_with_spaces() {
    let path = temp_lock_path("tag-spaces");
    cleanup(&path);

    let output = run_cli(&["try", &path, "--tag", "my tag with spaces"]);
    assert!(output.status.success());

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("tag=my tag with spaces"));

    cleanup(&path);
}

#[test]
fn test_cli_tag_with_special_chars() {
    let path = temp_lock_path("tag-special");
    cleanup(&path);

    let output = run_cli(&["try", &path, "--tag", "test@#$%"]);
    assert!(output.status.success());

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("tag=test@#$%"));

    cleanup(&path);
}

#[test]
fn test_cli_stale_zero_disables() {
    let path = temp_lock_path("stale-zero");
    cleanup(&path);

    // Create a stale lock
    let old_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - 7200;
    let content = format!("pid=99999\ntimestamp={}", old_timestamp);
    fs::write(&path, content).unwrap();

    // With stale=0, stale detection is disabled, so should fail
    let output = run_cli(&["try", &path, "--stale", "0"]);

    assert_eq!(output.status.code(), Some(1)); // Lock held

    cleanup(&path);
}

#[test]
fn test_cli_invalid_lock_path_parent() {
    // Try to create lock in non-existent directory
    let output = run_cli(&["try", "/nonexistent/directory/test.lock"]);

    // Should fail with IO error
    assert!(!output.status.success());
    assert!(output.status.code() == Some(3) || output.status.code() == Some(1));
}

// ============================================================================
// Multiple Options Combination Tests
// ============================================================================

#[test]
fn test_cli_multiple_options() {
    let path = temp_lock_path("multi-opts");
    cleanup(&path);

    let output = run_cli(&[
        "try",
        &path,
        "--tag", "multi-test",
        "--stale", "1800",
        "--quiet"
    ]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.is_empty() || stdout.trim().is_empty()); // quiet
    assert!(output.status.success());

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("tag=multi-test"));

    cleanup(&path);
}

#[test]
fn test_cli_acquire_all_options() {
    let path = temp_lock_path("acquire-all");
    cleanup(&path);

    let output = run_cli(&[
        "acquire",
        &path,
        "--timeout", "5",
        "--tag", "full-test",
        "--stale", "60",
        "-q"
    ]);

    assert!(output.status.success());

    cleanup(&path);
}
