# Protocol Name

[![Tests](https://github.com/tuulbelt/protocol-name/actions/workflows/test.yml/badge.svg)](https://github.com/tuulbelt/protocol-name/actions/workflows/test.yml)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-brightgreen.svg)](Cargo.toml)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

One sentence description of what this protocol does.

## Specification

**This is a specification-first project.** See [SPEC.md](SPEC.md) for the complete protocol specification.

The specification covers:
- Wire format and encoding
- Message types and semantics
- Error handling
- Security considerations
- Test vectors for compliance

## Why This Protocol?

Explain the problem this protocol solves and why existing solutions don't work.

- **Zero Dependencies** — Reference implementation uses only Rust std
- **Specification First** — Clear, complete specification document
- **Test Vectors** — Compliance tests for cross-implementation verification
- **Simple** — Minimal complexity, easy to implement

## Installation

```bash
# Clone from GitHub
git clone https://github.com/tuulbelt/protocol-name.git
cd protocol-name
cargo build --release
```

## Quick Start

### Library Usage

```rust
use protocol_name::{Message, encode, decode};

// Create a request
let request = Message::request(1, b"hello");

// Encode to bytes
let bytes = encode(&request)?;

// Decode from bytes
let message = decode(&bytes)?;
println!("Received: {:?}", message);
```

### CLI Usage

```bash
# Encode a message
./target/release/protocol-name encode --type request --id 1 --payload "hello"
# Output: 545501010000000100000005hello

# Decode a message
./target/release/protocol-name decode 545501010000000100000005hello
# Output:
# Version: 1
# Type: Request
# ID: 1
# Payload (5 bytes): "hello"

# Validate a message
./target/release/protocol-name validate 545501010000000100000005hello
# Output: Valid message
```

## API Overview

### Types

| Type | Description |
|------|-------------|
| `Message` | A protocol message (request, response, or error) |
| `MessageType` | Message type enum (Request, Response, Error) |
| `ProtocolError` | Error types (InvalidMagic, UnknownType, etc.) |

### Functions

| Function | Description |
|----------|-------------|
| `encode(&Message)` | Encode message to bytes |
| `decode(&[u8])` | Decode bytes to message |
| `write_message(writer, &Message)` | Write message to stream |
| `read_message(reader)` | Read message from stream |

### Message Constructors

| Constructor | Description |
|-------------|-------------|
| `Message::request(id, payload)` | Create a request |
| `Message::response(id, status, payload)` | Create a response |
| `Message::error(id, code, message)` | Create an error |

## Implementing in Other Languages

The specification in [SPEC.md](SPEC.md) is language-agnostic. To implement:

1. Read the wire format section
2. Implement encode/decode per specification
3. Run compliance tests from `tests/vectors/`
4. Verify all test vectors pass

## Development

```bash
# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Check for issues
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build release
cargo build --release
```

## Philosophy

This protocol follows [Tuulbelt principles](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md):

- **Specification First** — SPEC.md is the authoritative source
- **Zero External Dependencies** — Uses only Rust standard library
- **Test Vectors** — Compliance tests for any implementation
- **Simple Wire Format** — Easy to implement in any language

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Important:** Changes to the wire format require updating SPEC.md first.

## License

MIT - See [LICENSE](LICENSE)

## Part of Tuulbelt

This protocol is part of the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) ecosystem — a collection of zero-dependency tools, libraries, and protocols.
