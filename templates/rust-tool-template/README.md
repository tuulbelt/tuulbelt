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

## Integration with Tuulbelt Meta Repository

When adding this tool to the Tuulbelt meta repository, ensure CI/CD integration:

### Demo Recording Workflow

**Required:** Add path filter to `.github/workflows/create-demos.yml`:

```yaml
paths:
  - 'tool-name/**'  # Add this line for your tool
```

This enables smart detection to only record demos when your tool changes, not on every commit.

**Required:** Create demo recording script at `scripts/record-tool-name-demo.sh`:

```bash
#!/bin/bash
set -e

asciinema rec "demo.cast" --overwrite --title "Tool Name - Tuulbelt" --command "bash -c '
  # Your demo commands here
'"
```

See existing tools for examples (file-based-semaphore, output-diffing-utility).

### Dogfooding

Tuulbelt tools validate each other via CLI-based dogfooding.

**Validate This Tool's Tests:**

Run the dogfood script to use Test Flakiness Detector against this tool:

```bash
./scripts/dogfood.sh        # Default: 10 runs
./scripts/dogfood.sh 20     # Custom: 20 runs
```

This validates that all tests are deterministic and non-flaky.

**How It Works:**
- TypeScript tools (test-flakiness-detector) validate Rust tools via CLI
- The script runs `cargo test` multiple times and checks for inconsistent results
- Works only in monorepo context (exits gracefully when standalone)

**Using Rust CLI in TypeScript Tools (Reverse Dogfooding):**

If a TypeScript tool needs to use this Rust tool's CLI:

```typescript
import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

// Check if Rust tool is available
const rustToolPath = join(process.cwd(), '..', 'tool-name', 'target', 'release', 'tool-name');
if (existsSync(rustToolPath)) {
  const result = execSync(`${rustToolPath} --arg value`, { encoding: 'utf-8' });
  // Use result...
}
```

See [QUALITY_CHECKLIST.md](../docs/QUALITY_CHECKLIST.md) for dogfooding patterns.

## Error Handling

Exit codes:
- `0` - Success
- `1` - Error (invalid input, processing failure)

All errors implement `std::error::Error` and can be displayed with `{}`.

## Specification (SPEC.md)

**When to create SPEC.md:**

If your tool defines a **format, protocol, or algorithm**, create a `SPEC.md` file:

- **File formats** (e.g., binary wire format, serialization scheme)
- **Protocols** (e.g., RPC message structure, handshake sequence)
- **Algorithms** (e.g., diff algorithm, compression scheme)
- **Data structures** (e.g., index format, manifest structure)

**What to include:**
- Formal description of the format/protocol/algorithm
- Examples with hex dumps or diagrams
- Edge cases and constraints
- Version compatibility notes

**Examples in Tuulbelt:**
- `output-diffing-utility/SPEC.md` - Diff format and LCS algorithm
- `file-based-semaphore/SPEC.md` - Lock file structure and semantics

**When NOT to create SPEC.md:**
- Simple CLI tools with no custom format
- Wrappers around existing formats
- Tools that just process data without defining structure

## License

MIT — see [LICENSE](LICENSE)
