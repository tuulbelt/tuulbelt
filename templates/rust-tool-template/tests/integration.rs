//! Integration tests for tool-name

use tool_name::{process, process_strict, Config, ProcessError};

#[test]
fn test_basic_processing() {
    let config = Config::default();
    let result = process("hello world", &config);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HELLO WORLD");
}

#[test]
fn test_empty_input() {
    let config = Config::default();
    let result = process("", &config);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_strict_mode_rejects_empty() {
    let config = Config::default();
    let result = process_strict("", &config);

    assert!(result.is_err());
    match result {
        Err(ProcessError::EmptyInput) => (),
        _ => panic!("Expected EmptyInput error"),
    }
}

#[test]
fn test_strict_mode_accepts_valid() {
    let config = Config::default();
    let result = process_strict("test", &config);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "TEST");
}

#[test]
fn test_unicode_handling() {
    let config = Config::default();

    // Test various Unicode inputs
    let cases = vec![
        ("café", "CAFÉ"),
        ("日本語", "日本語"), // Japanese doesn't change case
        ("ΩMEGA", "ΩMEGA"),
        ("über", "ÜBER"),
    ];

    for (input, expected) in cases {
        let result = process(input, &config).unwrap();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_whitespace_handling() {
    let config = Config::default();

    let cases = vec![
        ("  hello  ", "  HELLO  "),
        ("\thello\t", "\tHELLO\t"),
        ("hello\nworld", "HELLO\nWORLD"),
    ];

    for (input, expected) in cases {
        let result = process(input, &config).unwrap();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_config_clone() {
    let config = Config { verbose: true };
    let cloned = config.clone();

    assert_eq!(config.verbose, cloned.verbose);
}

#[test]
fn test_error_traits() {
    let err = ProcessError::EmptyInput;

    // Test Display
    let display = format!("{}", err);
    assert!(!display.is_empty());

    // Test Debug
    let debug = format!("{:?}", err);
    assert!(!debug.is_empty());

    // Test Clone
    let cloned = err.clone();
    assert_eq!(err, cloned);
}
