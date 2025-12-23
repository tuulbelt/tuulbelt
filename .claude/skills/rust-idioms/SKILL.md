---
name: rust-idioms
description: "Apply Rust idioms and best practices including ownership patterns, error handling with Result types, safe concurrency, and zero-cost abstractions. Use when writing Rust code for Tuulbelt tools."
---

# Rust Idioms and Best Practices for Tuulbelt

## Core Principles

1. **Ownership and Borrowing** - Leverage Rust's memory safety guarantees
2. **Result Types** - Always use Result for fallible operations
3. **Zero Cost Abstractions** - High-level code with no runtime overhead
4. **Zero Dependencies** - Use std library only
5. **Explicit Error Handling** - No unwrap() in production code

## Ownership and Borrowing

### Prefer Ownership

**Take ownership when the function needs to own the data:**

```rust
// GOOD: Take ownership when consuming
pub fn process_data(data: String) -> Result<Output, Error> {
    // Function owns and can mutate/consume data
    let processed = data.to_uppercase();
    Ok(Output { value: processed })
}

// GOOD: Borrow when reading
pub fn validate_data(data: &str) -> Result<(), ValidationError> {
    // Function just reads, doesn't need ownership
    if data.is_empty() {
        return Err(ValidationError::Empty);
    }
    Ok(())
}
```

### Avoid Unnecessary Clones

**Use references instead of cloning:**

```rust
// GOOD: Borrow instead of clone
pub fn format_output(data: &Data) -> String {
    format!("{}: {}", data.name, data.value)
}

// BAD: Unnecessary clone
pub fn format_output(data: Data) -> String {
    format!("{}: {}", data.name, data.value)
}
// Caller would need to do: format_output(data.clone())
```

### Lifetime Annotations

**Be explicit with lifetimes when needed:**

```rust
// GOOD: Explicit lifetime for returned references
pub fn first_line<'a>(text: &'a str) -> &'a str {
    text.lines().next().unwrap_or("")
}

// GOOD: Multiple lifetimes when relationships differ
pub fn choose<'a, 'b>(flag: bool, a: &'a str, b: &'b str) -> &'a str
where 'b: 'a
{
    if flag { a } else { b }
}
```

## Error Handling

### Result Type (Always)

**All fallible operations must return Result:**

```rust
use std::error::Error;
use std::fmt;

// Define custom error types
#[derive(Debug)]
pub enum ProcessError {
    InvalidInput(String),
    IoError(std::io::Error),
    ParseError(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ProcessError::IoError(e) => write!(f, "IO error: {}", e),
            ProcessError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for ProcessError {}

impl From<std::io::Error> for ProcessError {
    fn from(error: std::io::Error) -> Self {
        ProcessError::IoError(error)
    }
}

// Use in functions
pub fn process_file(path: &str) -> Result<Data, ProcessError> {
    let content = std::fs::read_to_string(path)?; // Auto-converts io::Error

    if content.is_empty() {
        return Err(ProcessError::InvalidInput("empty file".to_string()));
    }

    let data = parse_content(&content)?;
    Ok(data)
}
```

### Never Use unwrap() or expect() in Production

```rust
// GOOD: Handle errors explicitly
pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

// BAD: Using unwrap (only OK in tests)
pub fn parse_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path).unwrap(); // Panics on error!
    serde_json::from_str(&content).unwrap()
}

// ACCEPTABLE: Only in tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let config = parse_config("test.json").unwrap(); // OK in tests
        assert_eq!(config.name, "test");
    }
}
```

### Question Mark Operator

**Use `?` for error propagation:**

```rust
pub fn complex_operation(path: &str) -> Result<Output, Box<dyn Error>> {
    let data = read_file(path)?;
    let parsed = parse_data(&data)?;
    let validated = validate(parsed)?;
    let result = transform(validated)?;
    Ok(result)
}
```

## Type Safety

### Newtype Pattern

**Use newtypes for type safety:**

```rust
// GOOD: Newtype prevents mixing up values
pub struct UserId(u64);
pub struct ProductId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        UserId(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

// Won't compile: can't pass ProductId where UserId expected
fn get_user(id: UserId) -> Result<User, Error> {
    // Implementation
}
```

### Builder Pattern

**For complex configuration:**

```rust
pub struct Config {
    host: String,
    port: u16,
    timeout: u64,
    retry: bool,
}

pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
    retry: bool,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            host: None,
            port: None,
            timeout: None,
            retry: false,
        }
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.ok_or("host is required")?,
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(30),
            retry: self.retry,
        })
    }
}

// Usage
let config = ConfigBuilder::new()
    .host("localhost".to_string())
    .port(3000)
    .build()?;
```

## Iterators and Functional Patterns

### Use Iterators (Not Loops)

**Prefer iterator chains for transformations:**

```rust
// GOOD: Functional style with iterators
pub fn process_items(items: &[Item]) -> Vec<Output> {
    items
        .iter()
        .filter(|item| item.is_valid())
        .map(|item| transform(item))
        .collect()
}

// BAD: Imperative loop
pub fn process_items(items: &[Item]) -> Vec<Output> {
    let mut results = Vec::new();
    for item in items {
        if item.is_valid() {
            results.push(transform(item));
        }
    }
    results
}
```

### Common Iterator Patterns

```rust
// Map and filter
let results: Vec<_> = data
    .iter()
    .filter(|x| x.score > 50)
    .map(|x| x.name.clone())
    .collect();

// Find first match
let found = data
    .iter()
    .find(|x| x.id == target_id);

// Any/all predicates
let has_errors = results.iter().any(|r| r.is_err());
let all_valid = inputs.iter().all(|i| i.validate());

// Fold for aggregation
let sum = numbers.iter().fold(0, |acc, x| acc + x);
```

## Pattern Matching

### Exhaustive Matching

**Always handle all cases:**

```rust
pub enum Status {
    Pending,
    Running,
    Completed,
    Failed(String),
}

pub fn handle_status(status: Status) -> String {
    match status {
        Status::Pending => "Waiting to start".to_string(),
        Status::Running => "In progress".to_string(),
        Status::Completed => "Done".to_string(),
        Status::Failed(reason) => format!("Error: {}", reason),
        // Compiler ensures all variants handled
    }
}
```

### Pattern Guards

```rust
match value {
    x if x < 0 => "negative",
    x if x == 0 => "zero",
    x if x < 100 => "small positive",
    _ => "large positive",
}
```

## Concurrency (If Needed)

### Thread Safety with Arc and Mutex

**For shared state across threads:**

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_process(data: Vec<Item>) -> Vec<Output> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for item in data {
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let output = process_item(item);
            results_clone.lock().unwrap().push(output);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}
```

### Async/Await (With tokio in dev-dependencies)

**For I/O-bound operations:**

```rust
// Cargo.toml: [dev-dependencies] tokio = { version = "1", features = ["full"] }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = fetch_data("https://api.example.com").await?;
    println!("Result: {:?}", result);
    Ok(())
}

async fn fetch_data(url: &str) -> Result<Data, Box<dyn Error>> {
    // Async implementation
    Ok(Data { /* ... */ })
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let result = process_data("valid");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, "expected");
    }

    #[test]
    fn test_invalid_input() {
        let result = process_data("");
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_panic_case() {
        assert_eq!(1, 2);
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use my_tool::{Config, process};

#[test]
fn test_end_to_end() {
    let config = Config::new("test");
    let result = process(config).unwrap();
    assert_eq!(result.status, "success");
}
```

### Property-Based Testing (Optional)

```rust
// [dev-dependencies] quickcheck = "1"

#[cfg(test)]
mod tests {
    use quickcheck::{quickcheck, TestResult};

    #[test]
    fn prop_reversible() {
        fn test(input: Vec<u8>) -> TestResult {
            let encoded = encode(&input);
            let decoded = decode(&encoded).unwrap();
            TestResult::from_bool(input == decoded)
        }
        quickcheck(test as fn(Vec<u8>) -> TestResult);
    }
}
```

## CLI Patterns

### Argument Parsing (Zero Dependencies)

```rust
use std::env;

pub struct Args {
    pub verbose: bool,
    pub output: Option<String>,
    pub input_files: Vec<String>,
}

pub fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();
    let mut verbose = false;
    let mut output = None;
    let mut input_files = Vec::new();

    let mut i = 1; // Skip program name
    while i < args.len() {
        match args[i].as_str() {
            "-v" | "--verbose" => verbose = true,
            "-o" | "--output" => {
                i += 1;
                output = Some(args.get(i).ok_or("Missing output file")?.clone());
            }
            arg if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg));
            }
            _ => input_files.push(args[i].clone()),
        }
        i += 1;
    }

    Ok(Args { verbose, output, input_files })
}
```

### stdin/stdout

```rust
use std::io::{self, Read, Write};

// Read from stdin
pub fn read_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

// Write to stdout
pub fn write_output(data: &str) -> Result<(), io::Error> {
    io::stdout().write_all(data.as_bytes())?;
    Ok(())
}

// Write to stderr
pub fn write_error(msg: &str) -> Result<(), io::Error> {
    io::stderr().write_all(msg.as_bytes())?;
    Ok(())
}
```

### Exit Codes

```rust
use std::process;

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Main logic
    Ok(())
}
```

## Performance Patterns

### Avoid Allocations in Hot Paths

```rust
// GOOD: Reuse buffer
pub fn process_lines(input: &str) -> Vec<String> {
    let mut results = Vec::with_capacity(100); // Pre-allocate
    for line in input.lines() {
        if !line.is_empty() {
            results.push(line.to_uppercase());
        }
    }
    results
}

// GOOD: Use references to avoid clones
pub fn find_max<'a>(items: &'a [Item]) -> Option<&'a Item> {
    items.iter().max_by_key(|item| item.score)
}
```

### String Handling

```rust
// Efficient string building
let result = format!("{}-{}-{}", part1, part2, part3);

// For loops, use String with capacity
let mut output = String::with_capacity(1000);
for item in items {
    output.push_str(&item.to_string());
    output.push('\n');
}
```

## Anti-Patterns to Avoid

### Don't Use

- ❌ `unwrap()` or `expect()` in production code
- ❌ Ignoring Result with `let _ = ...`
- ❌ Panicking for recoverable errors
- ❌ Mutable global state
- ❌ Circular dependencies between modules
- ❌ `unsafe` code without justification
- ❌ External crates for core functionality (zero deps!)

### Prefer

- ✅ Result types for all fallible operations
- ✅ Ownership over cloning when possible
- ✅ Iterators over manual loops
- ✅ Pattern matching over if/else chains
- ✅ Explicit error types over String errors
- ✅ References over owned values when appropriate
- ✅ `const` for compile-time constants

## Documentation

**Use doc comments for public APIs:**

```rust
/// Processes the input data and returns the result.
///
/// # Arguments
///
/// * `input` - The input string to process
///
/// # Returns
///
/// Returns `Ok(Data)` on success, or `Err(ProcessError)` if:
/// - The input is empty
/// - The input contains invalid characters
///
/// # Examples
///
/// ```
/// let result = process_data("hello");
/// assert!(result.is_ok());
/// ```
pub fn process_data(input: &str) -> Result<Data, ProcessError> {
    // Implementation
}
```

## Remember

This skill applies to **all Rust code in Tuulbelt tools**. Enforce these patterns for safety, performance, and maintainability while adhering to the zero-dependency principle.
