# Tool Name

[![Tests](https://github.com/tuulbelt/{{TOOL_NAME}}/actions/workflows/test.yml/badge.svg)](https://github.com/tuulbelt/{{TOOL_NAME}}/actions/workflows/test.yml)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

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

The binary supports **both short and long command names**:
- Short (recommended): `target/release/short-name`
- Long: `target/release/tool-name`

**Recommended setup** — install globally for easy access:

```bash
cargo install --path .
# Now use `short-name` anywhere
short-name --help
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
tuulbelt-tool-name = { git = "https://github.com/tuulbelt/tool-name.git" }
```

## Usage

### As a CLI

Using short name (recommended after `cargo install --path .`):

```bash
# Basic usage
short-name "hello world"
# Output: HELLO WORLD

# With verbose output
short-name --verbose "hello world"

# Show help
short-name --help

# Show version
short-name --version
```

Using long name:

```bash
tool-name "hello world"
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

Tuulbelt tools validate each other via CLI-based dogfooding. This tool includes
dogfood scripts that run in CI to catch regressions.

**Dogfood Scripts (run in CI):**

```bash
# Validate test reliability (runs tests N times)
./scripts/dogfood-flaky.sh 10

# Validate output determinism (compares two test runs)
./scripts/dogfood-diff.sh
```

**Local Development Only:**

Dogfood scripts are for local development verification in the monorepo context.
Tests are validated by `test-all-tools.yml` in CI. See [DOGFOODING_STRATEGY.md](./DOGFOODING_STRATEGY.md)
for the full composition strategy.

**How It Works:**
- TypeScript tools (test-flakiness-detector) validate Rust tools via CLI
- Rust tools (odiff) can validate TypeScript tool outputs
- Scripts run `cargo test` multiple times and check for inconsistent results
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

### Library Composition (PRINCIPLES.md Exception 2)

<!--
INCLUDE THIS SECTION if your tool uses another Tuulbelt tool as a library dependency.
DELETE THIS SECTION if your tool has no Tuulbelt library dependencies.
-->

If this tool uses another Tuulbelt tool as a library dependency (not CLI), document it here:

**Example:** Snapshot Comparison uses Output Diffing Utility for semantic diffs:

```toml
# Cargo.toml
[dependencies]
output-diffing-utility = { git = "https://github.com/tuulbelt/output-diffing-utility" }
```

```rust
// src/lib.rs
use output_diffing_utility::{diff_text, diff_json, diff_binary};
```

**Why use library composition:**
- Reuse proven algorithms (diff, hashing, encoding)
- Reduce code duplication across tools
- Maintain zero *external* dependencies (Tuulbelt tools have zero deps themselves)

**When to use library composition:**
- Another Tuulbelt tool solves a subproblem you need
- The functionality is core to your tool (not optional)
- CLI composition would add unnecessary overhead

**Documentation requirements:**
- Add `[dependencies]` entry in Cargo.toml with git URL dependency
- Document the integration in README (this section)
- Update DOGFOODING_STRATEGY.md to explain the value
- Add to root README.md Dogfooding section

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
