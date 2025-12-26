//! CLI integration tests for output-diff

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn binary_path() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("Failed to get current executable")
        .parent()
        .expect("Failed to get parent")
        .parent()
        .expect("Failed to get grandparent")
        .to_path_buf();

    // In debug builds, binary is at target/debug/output-diff
    path.push("output-diff");

    if !path.exists() {
        panic!("Binary not found at {:?}", path);
    }

    path
}

#[test]
fn test_cli_help() {
    let output = Command::new(binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("output-diff"));
    assert!(stdout.contains("Semantic diff tool"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("output-diff"));
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_cli_no_arguments() {
    let output = Command::new(binary_path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Expected exactly 2 file arguments"));
}

#[test]
fn test_cli_too_many_arguments() {
    let output = Command::new(binary_path())
        .arg("file1")
        .arg("file2")
        .arg("file3")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Expected exactly 2 file arguments, got 3"));
}

#[test]
fn test_cli_unknown_option() {
    let output = Command::new(binary_path())
        .arg("--unknown")
        .arg("file1")
        .arg("file2")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown option"));
}

#[test]
fn test_cli_text_diff_identical() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_text1.txt");
    let file2 = tmp_dir.join("test_cli_text2.txt");

    fs::write(&file1, "line 1\nline 2\nline 3").expect("Failed to write file1");
    fs::write(&file2, "line 1\nline 2\nline 3").expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    // Identical files should exit with code 0
    assert!(output.status.success());

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_text_diff_changes() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_text_diff1.txt");
    let file2 = tmp_dir.join("test_cli_text_diff2.txt");

    fs::write(&file1, "line 1\nline 2\nline 3").expect("Failed to write file1");
    fs::write(&file2, "line 1\nline 2 modified\nline 3").expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    // Different files should exit with code 1
    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("---"));
    assert!(stdout.contains("+++"));
    assert!(stdout.contains("- line 2")); // Space after -
    assert!(stdout.contains("+ line 2 modified")); // Space after +

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_text_diff_verbose() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_verbose1.txt");
    let file2 = tmp_dir.join("test_cli_verbose2.txt");

    fs::write(&file1, "hello").expect("Failed to write file1");
    fs::write(&file2, "hello").expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg("--verbose")
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("[DEBUG]"));

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_binary_diff_identical() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_binary1.bin");
    let file2 = tmp_dir.join("test_cli_binary2.bin");

    fs::write(&file1, [0x00, 0x01, 0x02, 0xFF]).expect("Failed to write file1");
    fs::write(&file2, [0x00, 0x01, 0x02, 0xFF]).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_binary_diff_changes() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_binary_diff1.bin");
    let file2 = tmp_dir.join("test_cli_binary_diff2.bin");

    fs::write(&file1, [0x00, 0x01, 0x02]).expect("Failed to write file1");
    fs::write(&file2, [0x00, 0xFF, 0x02]).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Binary files differ"));
    assert!(stdout.contains("0x00000001")); // Offset of change
    assert!(stdout.contains("01")); // Old byte
    assert!(stdout.contains("ff")); // New byte

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_json_diff_identical() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_json1.json");
    let file2 = tmp_dir.join("test_cli_json2.json");

    let json = r#"{"name": "Alice", "age": 30}"#;
    fs::write(&file1, json).expect("Failed to write file1");
    fs::write(&file2, json).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_json_diff_changes() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_json_diff1.json");
    let file2 = tmp_dir.join("test_cli_json_diff2.json");

    fs::write(&file1, r#"{"name": "Alice", "age": 30}"#).expect("Failed to write file1");
    fs::write(&file2, r#"{"name": "Bob", "age": 30}"#).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("JSON diff"));
    assert!(stdout.contains("Modified at 'name'"));
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Bob"));

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_json_diff_nested() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_json_nested1.json");
    let file2 = tmp_dir.join("test_cli_json_nested2.json");

    fs::write(&file1, r#"{"user": {"name": "Alice", "age": 30}}"#).expect("Failed to write file1");
    fs::write(&file2, r#"{"user": {"name": "Bob", "age": 30}}"#).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("user.name")); // Correct path

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_json_diff_array() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_json_array1.json");
    let file2 = tmp_dir.join("test_cli_json_array2.json");

    fs::write(&file1, r#"{"items": [1, 2, 3]}"#).expect("Failed to write file1");
    fs::write(&file2, r#"{"items": [1, 5, 3]}"#).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("items[1]")); // Correct array path

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_json_diff_added_removed() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_json_add_rm1.json");
    let file2 = tmp_dir.join("test_cli_json_add_rm2.json");

    fs::write(&file1, r#"{"name": "Alice", "age": 30}"#).expect("Failed to write file1");
    fs::write(&file2, r#"{"name": "Alice", "city": "NYC"}"#).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Removed at 'age'"));
    assert!(stdout.contains("Added at 'city'"));

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_json_invalid_parse_error() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_cli_json_invalid1.json");
    let file2 = tmp_dir.join("test_cli_json_invalid2.json");

    fs::write(&file1, r#"{"invalid"#).expect("Failed to write file1");
    fs::write(&file2, r#"{"valid": true}"#).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    // Parse error should exit with code 2
    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error parsing JSON"));

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}

#[test]
fn test_cli_file_not_found() {
    let output = Command::new(binary_path())
        .arg("/tmp/nonexistent_file_12345.txt")
        .arg("/tmp/another_nonexistent_file_12345.txt")
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error reading"));
}

#[test]
fn test_cli_auto_detection_text_to_json() {
    let tmp_dir = std::env::temp_dir();
    let file1 = tmp_dir.join("test_detection1.txt");
    let file2 = tmp_dir.join("test_detection2.json");

    fs::write(&file1, r#"{"name": "Alice"}"#).expect("Failed to write file1");
    fs::write(&file2, r#"{"name": "Bob"}"#).expect("Failed to write file2");

    let output = Command::new(binary_path())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute command");

    // Should prefer JSON over Text
    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8_lossy(&output.stdout);
    // If treated as JSON, should show structural diff
    assert!(stdout.contains("JSON diff") || stdout.contains("Modified at 'name'"));

    fs::remove_file(&file1).ok();
    fs::remove_file(&file2).ok();
}
