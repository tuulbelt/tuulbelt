# Tool Name

One sentence description of what this tool does.

## Problem

Describe the problem this tool solves. Why does it exist? What gap does it fill?

## Features

- Zero runtime dependencies (uses only Rust standard library)
- Cross-platform support (Linux, macOS, Windows)
- Both library and CLI interfaces
- Comprehensive error handling

## Installation

### From Source

```bash
git clone https://github.com/tuulbelt/tool-name.git
cd tool-name
cargo build --release
```

The binary will be at `target/release/tool-name`.

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
tuulbelt-tool-name = { git = "https://github.com/tuulbelt/tool-name.git" }
```

## Usage

### As a CLI

```bash
# Basic usage
./tool-name "hello world"
# Output: HELLO WORLD

# With verbose output
./tool-name --verbose "hello world"

# Show help
./tool-name --help

# Show version
./tool-name --version
```

### As a Library

```rust
use tool_name::{process, Config};

fn main() {
    let config = Config::default();

    match process("hello world", &config) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## API

### `process(input: &str, config: &Config) -> Result<String, ProcessError>`

Process the input string and return the result.

**Arguments:**
- `input` - The string to process
- `config` - Configuration options
  - `verbose` - Enable verbose output (default: `false`)

**Returns:**
- `Ok(String)` - The processed result
- `Err(ProcessError)` - Error if processing fails

### `process_strict(input: &str, config: &Config) -> Result<String, ProcessError>`

Same as `process`, but returns an error if input is empty.

## Examples

Run the example:

```bash
cargo run --example basic
```

## Testing

```bash
cargo test              # Run all tests
cargo test -- --nocapture  # Show output
```

## Error Handling

Exit codes:
- `0` - Success
- `1` - Error (invalid input, processing failure)

All errors implement `std::error::Error` and can be displayed with `{}`.

## License

MIT — see [LICENSE](LICENSE)
