# Snapshot Comparison Specification

Version: 1.0.0

## Overview

This document specifies the snapshot file format used by snapshot-comparison for storing and comparing test outputs.

## Snapshot File Format

### File Extension

Snapshot files use the `.snap` extension.

### File Structure

A snapshot file consists of two sections separated by a `---` delimiter:

```
HEADER
---
CONTENT
```

### Header Section

The header contains metadata about the snapshot as comment lines. Each line starts with `# ` followed by a key-value pair.

#### Required Header Fields

| Field | Format | Description |
|-------|--------|-------------|
| Snapshot | String | Name of the snapshot (no path separators) |
| Created | Integer | Unix timestamp when first created |
| Updated | Integer | Unix timestamp of last update |
| Hash | Hex String | FNV-1a hash of content (16 hex chars) |
| Size | Integer | Content size in bytes |
| Type | Enum | File type: `text`, `json`, or `binary` |

#### Header Grammar

```
header     = header_line+
header_line = "# " key ": " value "\n"
key        = "Snapshot" | "Created" | "Updated" | "Hash" | "Size" | "Type"
value      = string | integer | hash | file_type
hash       = [0-9a-f]{16}
file_type  = "text" | "json" | "binary"
```

### Content Section

Everything after the `---` delimiter is the raw snapshot content. The content is stored as-is, with no encoding or escaping.

### Example

```
# Snapshot: api-response
# Created: 1703721600
# Updated: 1703721700
# Hash: a1b2c3d4e5f67890
# Size: 42
# Type: json
---
{"status": "ok", "message": "Hello, World!"}
```

## Hash Algorithm

### FNV-1a (64-bit)

Content hashes use FNV-1a for fast comparison:

```rust
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x00000100000001B3;

fn fnv1a_hash(data: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in data {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
```

The hash is formatted as a 16-character lowercase hexadecimal string.

### Collision Handling

FNV-1a is used for fast detection of potential changes, not for cryptographic security. When hashes match:
1. First check: Hash comparison (fast)
2. Second check: Byte-by-byte comparison (definitive)

## File Type Detection

When file type is not explicitly specified, auto-detection is used:

1. **JSON**: Content starts with `{` or `[` after trimming whitespace
2. **Binary**: Content contains null bytes (`\0`)
3. **Text**: All other content

## Name Validation

Snapshot names must satisfy these constraints:

- No path separators: `/`, `\`
- No parent directory references: `..`
- No null bytes: `\0`
- Non-empty string

Valid: `api-response`, `test_123`, `my.snapshot`
Invalid: `../escape`, `sub/path`, `bad\0name`

## Comparison Semantics

### Match Detection

A snapshot matches if:
1. The content hash matches, AND
2. Byte-by-byte content comparison confirms equality

### Mismatch Handling

On mismatch, a diff is generated based on file type:

| Type | Diff Format |
|------|-------------|
| text | Unified diff with context lines |
| json | Semantic JSON diff (added/removed/changed) |
| binary | Hex dump with byte-level differences |

### Update Mode

When `update_mode` is true and content differs:
1. The new content replaces the old snapshot
2. `Updated` timestamp is set to current time
3. `Hash` and `Size` are recalculated
4. Original `Created` timestamp is preserved

## Directory Structure

```
snapshots/
├── api-response.snap
├── user-list.snap
├── binary-data.snap
└── config.snap
```

All snapshots in a store share a common directory. Subdirectories are not supported.

## Version Compatibility

### Forward Compatibility

Readers should ignore unknown header fields to allow future extensions.

### Backward Compatibility

This specification version (1.0.0) is the initial format. Future versions will maintain backward compatibility for reading existing snapshots.

## Error Conditions

| Error | Cause |
|-------|-------|
| NotFound | Snapshot file does not exist |
| InvalidName | Name contains forbidden characters |
| CorruptedSnapshot | Missing header, missing separator, or invalid metadata |
| InvalidUtf8 | Text/JSON content is not valid UTF-8 |
| IoError | File system error during read/write |

## Conformance

A conforming implementation must:

1. Read and write the specified file format
2. Validate snapshot names before use
3. Use FNV-1a for hash calculation
4. Support all three file types (text, json, binary)
5. Preserve content exactly (no normalization)
