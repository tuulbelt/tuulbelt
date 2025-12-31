# Snapshot Comparison

Snapshot testing utility for regression detection with integrated diff output.

## Overview

Snapshot Comparison (`snapcmp`) catches regressions by comparing current output against stored "golden" snapshots. It integrates with output-diffing-utility for rich, semantic diffs of text, JSON, and binary data.

::: tip <img src="/icons/package.svg" class="inline-icon" alt=""> First Tool Using Library Composition
This is the first Tuulbelt tool to use another Tuulbelt tool as a **library dependency** (not CLI). It imports `output-diffing-utility` via Cargo path dependency, demonstrating [PRINCIPLES.md Exception 2](/guide/principles#zero-external-dependencies): Tuulbelt tools can compose while maintaining zero external dependencies.
:::

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** Rust

**Repository:** [tuulbelt/snapshot-comparison](https://github.com/tuulbelt/snapshot-comparison)

## Features

### <img src="/icons/target.svg" class="inline-icon" alt=""> Snapshot Testing

Store expected output and automatically detect regressions. Create, check, and update snapshots with simple commands.

### <img src="/icons/file-diff.svg" class="inline-icon" alt=""> Integrated Semantic Diffs

Uses output-diffing-utility for intelligent comparisons:
- **Text**: Unified diff with context lines
- **JSON**: Semantic comparison showing field changes
- **Binary**: Hex dump with byte-level differences

### <img src="/icons/terminal.svg" class="inline-icon" alt=""> CLI-First Design

Works with any language via stdin/stdout. Pipe output from any program for snapshot testing.

### <img src="/icons/tool.svg" class="inline-icon" alt=""> CLI & Library

Use as a command-line tool for shell scripts or integrate as a Rust library.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Rust standard library plus output-diffing-utility (another zero-dep Tuulbelt tool).

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/snapshot-comparison.git
cd snapshot-comparison

# Build
cargo build --release

# Install globally
cargo install --path .

# Create a snapshot
my-program | snapcmp create my-test

# Check against snapshot
my-program | snapcmp check my-test
```

## Demo

See the tool in action:

![Snapshot Comparison Demo](/snapshot-comparison/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/bMuH0jaX9RHOkhtXzJOxh8K8y)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/snapshot-comparison?file=examples/basic.rs" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

## Use Cases

### API Response Testing

```bash
curl -s https://api.example.com/users | snapcmp create users-api
curl -s https://api.example.com/users | snapcmp check users-api
```

### Build Verification

```bash
./build.sh | snapcmp create build-output
./build.sh | snapcmp check build-output
```

### Configuration Validation

```bash
cat config.json | snapcmp create -t json valid-config
cat config.json | snapcmp check valid-config
```

## Why Snapshot Comparison?

Unlike Jest snapshots or insta:

1. **Zero dependencies** - Pure Rust, no runtime dependencies
2. **Integrated diffs** - Rich semantic diffs included
3. **Format-aware** - Understands text, JSON, and binary
4. **CLI-first** - Works with any language

## Documentation

- [Getting Started](/tools/snapshot-comparison/getting-started) - Installation and setup
- [CLI Usage](/tools/snapshot-comparison/cli-usage) - Command-line reference
- [Library Usage](/tools/snapshot-comparison/library-usage) - Rust API guide
- [Examples](/tools/snapshot-comparison/examples) - Real-world usage patterns
- [API Reference](/tools/snapshot-comparison/api-reference) - Complete API documentation

## Related Tools

- [Output Diffing Utility](/tools/output-diffing-utility/) - Semantic diff engine (used internally)
- [Test Flakiness Detector](/tools/test-flakiness-detector/) - Detect unreliable tests
