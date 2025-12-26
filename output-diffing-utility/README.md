# Output Diffing Utility

Semantic diff tool for JSON, text, and binary files with zero dependencies.

## Problem

When comparing files in tests, builds, or data pipelines, you need to see **what changed** in a structured, semantic way. Standard `diff` works for text but doesn't understand JSON structure or binary formats. This tool provides:
- **Text diffs** with LCS algorithm (line-by-line comparison)
- **JSON diffs** with structural understanding (field paths, not lines)
- **Binary diffs** with byte-level comparison (hex output)

All with zero external dependencies.

## Features

- **Zero runtime dependencies** (uses only Rust standard library)
- **Multiple diff types**: Text (LCS), JSON (structural), Binary (byte-by-byte)
- **Multiple output formats**: Unified, JSON, side-by-side, compact
- **Cross-platform support** (Linux, macOS, Windows)
- **Both library and CLI interfaces**
- **Proper exit codes** (0=identical, 1=differ, 2=error)
- **Composable** with other Tuulbelt tools

## Installation

### From Source

```bash
git clone https://github.com/tuulbelt/output-diffing-utility.git
cd output-diffing-utility
cargo build --release
```

The binary will be at `target/release/output-diff`.

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
output-diffing-utility = { git = "https://github.com/tuulbelt/output-diffing-utility.git" }
```

## Usage

### As a CLI

```bash
# Basic text diff
output-diff file1.txt file2.txt

# JSON structural diff
output-diff data1.json data2.json

# Binary diff
output-diff image1.png image2.png

# Verbose output
output-diff --verbose file1.txt file2.txt
```

**Output:**
```diff
--- file1.txt
+++ file2.txt
  line 1
- line 2
+ line 2 modified
  line 3
+ line 4
```

**Exit codes:**
- `0` - Files are identical
- `1` - Files differ
- `2` - Error occurred

### As a Library

```rust
use output_diffing_utility::{diff_text, DiffConfig};

let old = "line 1\nline 2";
let new = "line 1\nline 2 modified";

let config = DiffConfig::default();
let result = diff_text(old, new, &config);

if result.has_changes() {
    println!("Files differ!");
    println!("Additions: {}", result.additions());
    println!("Deletions: {}", result.deletions());
}
```

## Algorithm: Longest Common Subsequence (LCS)

The text diff uses dynamic programming to find the optimal diff:
- **Time complexity:** O(m×n) where m, n are line counts
- **Space complexity:** O(m×n)
- **Output:** Minimal set of changes (additions/deletions)

See `DIFF_ALGORITHMS_RESEARCH.md` for detailed algorithm analysis.

## Composability with Tuulbelt Tools

This tool integrates with other Tuulbelt tools (when in monorepo):

- **cli-progress-reporting** - Shows progress for large files
- **cross-platform-path-normalizer** - Accepts paths in any format
- **file-based-semaphore** - Protects diff cache from concurrent access
- **test-flakiness-detector** - Validates test suite reliability

See `DOGFOODING_STRATEGY.md` for integration patterns.

## Development

```bash
# Run tests
cargo test

# Run with zero warnings
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build optimized release
cargo build --release
```

## Examples

See `examples/` directory for real-world usage patterns.

## License

MIT

## Part of Tuulbelt

This is tool #5 of 33 in the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) collection of focused, zero-dependency utilities.
