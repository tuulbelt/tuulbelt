//! Basic usage example for tool-name
//!
//! Run this example:
//!   cargo run --example basic

use tool_name::{process, process_strict, Config};

fn main() {
    println!("Tool Name - Basic Usage Examples\n");

    // Example 1: Basic usage with default config
    println!("Example 1: Basic usage");
    let config = Config::default();
    let result = process("hello world", &config).unwrap();
    println!("  Input:  \"hello world\"");
    println!("  Output: \"{}\"\n", result);

    // Example 2: With verbose mode
    println!("Example 2: Verbose mode");
    let config = Config { verbose: true };
    let result = process("testing verbose", &config).unwrap();
    println!("  Output: \"{}\"\n", result);

    // Example 3: Empty input (allowed with process)
    println!("Example 3: Empty input with process()");
    let config = Config::default();
    let result = process("", &config).unwrap();
    println!("  Input:  \"\"");
    println!("  Output: \"{}\" (empty)\n", result);

    // Example 4: Empty input with strict mode (returns error)
    println!("Example 4: Empty input with process_strict()");
    let config = Config::default();
    match process_strict("", &config) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 5: Unicode handling
    println!("Example 5: Unicode handling");
    let config = Config::default();
    let inputs = vec!["café", "über", "日本語"];
    for input in inputs {
        let result = process(input, &config).unwrap();
        println!("  \"{}\" -> \"{}\"", input, result);
    }

    println!("\nDone!");
}
