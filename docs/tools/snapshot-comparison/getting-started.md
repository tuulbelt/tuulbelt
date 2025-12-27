# Getting Started

Get up and running with Snapshot Comparison in minutes.

## Prerequisites

- Rust 1.70+ (for building from source)
- Git

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/snapshot-comparison

# Build release binary
cargo build --release

# The binary is at target/release/snapcmp
./target/release/snapcmp --help
```

### Global Installation

Install globally for easy access:

```bash
cargo install --path .

# Now use 'snapcmp' anywhere
snapcmp --help
```

### CLI Names

Both short and long names work:
- **Short (recommended):** `snapcmp`
- **Long:** `snapshot-comparison`

## Quick Start

### 1. Create Your First Snapshot

```bash
# Create a snapshot from command output
echo "Hello, World!" | snapcmp create hello

# Output: Created snapshot 'hello' (14 bytes, type: Text)
```

### 2. Check Against Snapshot

```bash
# Check matching content
echo "Hello, World!" | snapcmp check hello
# Output: ✓ Snapshot 'hello' matches
```

### 3. See a Mismatch

```bash
# Check different content
echo "Hello, Universe!" | snapcmp check hello
# Output: ✗ Snapshot 'hello' does not match
# [Shows diff output]
```

### 4. Update the Snapshot

```bash
# Update with new expected output
echo "Hello, Universe!" | snapcmp update hello
# Output: Updated snapshot 'hello' (17 bytes)
```

## Snapshot Directory

By default, snapshots are stored in `./snapshots/`. Change this with the `-d` flag:

```bash
# Use custom directory
echo "test" | snapcmp create test -d my-snapshots

# Check in custom directory
echo "test" | snapcmp check test -d my-snapshots
```

## File Type Detection

Snapshot Comparison auto-detects file types:

- **Text**: Default for most content
- **JSON**: Content starting with `{` or `[`
- **Binary**: Content with null bytes

Override with `--type`:

```bash
# Force JSON type
cat data.json | snapcmp create config -t json

# Force binary type
cat image.png | snapcmp create logo -t binary
```

## Next Steps

- [CLI Usage](/tools/snapshot-comparison/cli-usage) - Full command reference
- [Library Usage](/tools/snapshot-comparison/library-usage) - Rust API guide
- [Examples](/tools/snapshot-comparison/examples) - Real-world patterns
