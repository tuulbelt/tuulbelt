//! Advanced usage patterns for tool-name
//!
//! This example demonstrates:
//! - Performance optimization patterns
//! - Resource limit handling
//! - Batch processing with pre-allocation
//! - Error handling strategies
//!
//! Run this example:
//!   cargo run --example advanced

use tool_name::{process, Config, ProcessError};

/// Example: Pre-allocate collections when size is known
///
/// Performance pattern used in output-diffing-utility:
/// - Vec::with_capacity() avoids repeated reallocation
/// - Reduces memory fragmentation
/// - Improves performance for known-size collections
fn batch_process_optimized(inputs: &[&str], config: &Config) -> Result<Vec<String>, ProcessError> {
    // Pre-allocate: we know we'll have exactly inputs.len() results
    let mut results = Vec::with_capacity(inputs.len());

    for input in inputs {
        results.push(process(input, config)?);
    }

    Ok(results)
}

/// Example: Resource limit pattern
///
/// Pattern used in output-diffing-utility for file size limits:
/// - Define constants for resource limits
/// - Check limits before processing
/// - Provide clear error messages with usage hints
const MAX_INPUT_LENGTH: usize = 1_000_000; // 1 million characters

fn process_with_limit(input: &str, config: &Config) -> Result<String, ProcessError> {
    // Check resource limit before processing
    if input.len() > MAX_INPUT_LENGTH {
        return Err(ProcessError::InvalidInput(
            format!("Input length {} exceeds maximum {} characters",
                    input.len(), MAX_INPUT_LENGTH)
        ));
    }

    process(input, config)
}

/// Example: Streaming/chunked processing for large inputs
///
/// Use when input is too large to process all at once
fn process_chunks(input: &str, chunk_size: usize, config: &Config) -> Result<Vec<String>, ProcessError> {
    let chunks: Vec<&str> = input
        .as_bytes()
        .chunks(chunk_size)
        .filter_map(|chunk| std::str::from_utf8(chunk).ok())
        .collect();

    // Pre-allocate for all chunks
    let mut results = Vec::with_capacity(chunks.len());

    for chunk in chunks {
        results.push(process(chunk, config)?);
    }

    Ok(results)
}

/// Example: Iterator-based processing (no intermediate collections)
///
/// More memory-efficient than collecting all results
fn process_iterator<'a>(
    inputs: &'a [&'a str],
    config: &'a Config,
) -> impl Iterator<Item = Result<String, ProcessError>> + 'a {
    inputs.iter().map(move |input| process(input, config))
}

fn main() {
    println!("Tool Name - Advanced Usage Patterns\n");

    let config = Config::default();

    // Example 1: Batch processing with pre-allocation
    println!("Example 1: Batch processing (optimized)");
    let inputs = vec!["hello", "world", "rust", "performance"];
    match batch_process_optimized(&inputs, &config) {
        Ok(results) => {
            println!("  Processed {} inputs:", results.len());
            for (i, result) in results.iter().enumerate() {
                println!("    [{}] {}", i, result);
            }
        }
        Err(e) => eprintln!("  Error: {}", e),
    }
    println!();

    // Example 2: Resource limit handling
    println!("Example 2: Resource limit");
    let large_input = "x".repeat(2_000_000); // Exceeds MAX_INPUT_LENGTH
    match process_with_limit(&large_input, &config) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Error (expected): {}", e),
    }
    println!();

    // Example 3: Chunked processing
    println!("Example 3: Chunked processing");
    let large_text = "hello world ".repeat(100); // 1200 chars
    match process_chunks(&large_text, 100, &config) {
        Ok(chunks) => println!("  Processed {} chunks", chunks.len()),
        Err(e) => eprintln!("  Error: {}", e),
    }
    println!();

    // Example 4: Iterator-based processing (lazy evaluation)
    println!("Example 4: Iterator processing (lazy)");
    let inputs = vec!["one", "two", "three"];
    let mut success_count = 0;
    let mut error_count = 0;

    for result in process_iterator(&inputs, &config) {
        match result {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }
    }
    println!("  Successes: {}, Errors: {}", success_count, error_count);
    println!();

    // Example 5: Collecting only successful results
    println!("Example 5: Filter successful results");
    let mixed_inputs = vec!["valid", "", "also valid"];
    let results: Vec<String> = process_iterator(&mixed_inputs, &config)
        .filter_map(Result::ok)
        .collect();
    println!("  Valid results: {}", results.len());
    println!();

    println!("Performance Tips:");
    println!("  - Use Vec::with_capacity() when final size is known");
    println!("  - Check resource limits before processing");
    println!("  - Use iterators for lazy evaluation");
    println!("  - Profile with: cargo build --release && time ./target/release/tool-name");
    println!();

    println!("Done!");
}
