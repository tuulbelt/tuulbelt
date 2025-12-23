//! # Tool Name
//!
//! One sentence description of what this tool does.
//!
//! ## Features
//!
//! - Zero runtime dependencies
//! - Cross-platform support
//! - Both library and CLI interfaces
//!
//! ## Example
//!
//! ```rust
//! use tool_name::{process, Config};
//!
//! let config = Config::default();
//! let result = process("hello world", &config);
//!
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(), "HELLO WORLD");
//! ```

use std::error::Error;
use std::fmt;

/// Configuration options for processing
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Enable verbose output
    pub verbose: bool,
}

/// Error type for processing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    /// Input was empty when non-empty input was required
    EmptyInput,
    /// Input contained invalid characters
    InvalidInput(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::EmptyInput => write!(f, "Input cannot be empty"),
            ProcessError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl Error for ProcessError {}

/// Process the input string and return the result
///
/// # Arguments
///
/// * `input` - The input string to process
/// * `config` - Configuration options
///
/// # Returns
///
/// Returns `Ok(String)` with the processed result, or `Err(ProcessError)` on failure.
///
/// # Example
///
/// ```rust
/// use tool_name::{process, Config};
///
/// let config = Config { verbose: false };
/// let result = process("hello", &config).unwrap();
/// assert_eq!(result, "HELLO");
/// ```
pub fn process(input: &str, config: &Config) -> Result<String, ProcessError> {
    if config.verbose {
        eprintln!("[DEBUG] Processing input: {}", input);
    }

    // Example implementation: convert to uppercase
    let processed = input.to_uppercase();

    Ok(processed)
}

/// Process input with validation that input is non-empty
///
/// # Arguments
///
/// * `input` - The input string to process (must be non-empty)
/// * `config` - Configuration options
///
/// # Returns
///
/// Returns `Ok(String)` with the processed result, or `Err(ProcessError)` if input is empty.
pub fn process_strict(input: &str, config: &Config) -> Result<String, ProcessError> {
    if input.is_empty() {
        return Err(ProcessError::EmptyInput);
    }

    process(input, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_basic() {
        let config = Config::default();
        let result = process("hello", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn test_process_empty_string() {
        let config = Config::default();
        let result = process("", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_process_mixed_case() {
        let config = Config::default();
        let result = process("HeLLo WoRLd", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO WORLD");
    }

    #[test]
    fn test_process_special_characters() {
        let config = Config::default();
        let result = process("hello! @#$% 123", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO! @#$% 123");
    }

    #[test]
    fn test_process_unicode() {
        let config = Config::default();
        let result = process("café résumé", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "CAFÉ RÉSUMÉ");
    }

    #[test]
    fn test_process_strict_empty() {
        let config = Config::default();
        let result = process_strict("", &config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ProcessError::EmptyInput);
    }

    #[test]
    fn test_process_strict_valid() {
        let config = Config::default();
        let result = process_strict("hello", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn test_config_verbose() {
        let config = Config { verbose: true };
        let result = process("test", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_display() {
        let err = ProcessError::EmptyInput;
        assert_eq!(format!("{}", err), "Input cannot be empty");

        let err = ProcessError::InvalidInput("bad data".to_string());
        assert_eq!(format!("{}", err), "Invalid input: bad data");
    }
}
