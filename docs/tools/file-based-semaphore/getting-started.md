# Getting Started

This guide walks you through installing and setting up File-Based Semaphore.

## Prerequisites

- **Rust 1.70+** (for building from source)
- **Unix/Windows** system with filesystem access

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/file-based-semaphore

# Build release binary
cargo build --release

# Binary is at: target/release/file-semaphore
./target/release/file-semaphore --version
```

### As a Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
file-based-semaphore = { git = "https://github.com/tuulbelt/tuulbelt.git", path = "file-based-semaphore" }
```

## Quick Verification

```bash
# Create a test lock
./target/release/file-semaphore try /tmp/test.lock
# Output: Lock acquired: /tmp/test.lock

# Check status
./target/release/file-semaphore status /tmp/test.lock
# Output: Lock status: LOCKED

# Release it
./target/release/file-semaphore release /tmp/test.lock
# Output: Lock released: /tmp/test.lock
```

## Running Tests

```bash
cargo test
# 85 tests should pass

cargo clippy -- -D warnings
# Zero warnings required
```

## Project Structure

```
file-based-semaphore/
├── Cargo.toml          # Package manifest
├── src/
│   ├── lib.rs          # Library implementation
│   └── main.rs         # CLI implementation
├── tests/
│   └── integration.rs  # Integration tests
├── examples/
│   ├── basic.rs        # Basic usage example
│   └── concurrent.rs   # Concurrent access example
├── README.md           # Documentation
├── SPEC.md             # Protocol specification
└── LICENSE             # MIT license
```

## Next Steps

- [CLI Usage](/tools/file-based-semaphore/cli-usage) - Learn the command-line interface
- [Library Usage](/tools/file-based-semaphore/library-usage) - Integrate into Rust projects
- [Examples](/tools/file-based-semaphore/examples) - See real-world patterns
