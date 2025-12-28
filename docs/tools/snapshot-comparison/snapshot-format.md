# Snapshot Format Specification

This page documents the `.snap` file format used by Snapshot Comparison.

## File Structure

A snapshot file consists of two sections separated by a `---` delimiter:

```
HEADER
---
CONTENT
```

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

## Header Fields

| Field | Type | Description |
|-------|------|-------------|
| `Snapshot` | String | Name of the snapshot (no path separators) |
| `Created` | Integer | Unix timestamp when first created |
| `Updated` | Integer | Unix timestamp of last update |
| `Hash` | Hex (16 chars) | FNV-1a hash of content |
| `Size` | Integer | Content size in bytes |
| `Type` | Enum | `text`, `json`, or `binary` |

## Hash Algorithm

Content hashes use **FNV-1a (64-bit)** for fast comparison:

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

### Why FNV-1a?

- **Fast**: Single-pass, no complex operations
- **Good distribution**: Minimizes collisions for typical content
- **Non-cryptographic**: Not for security, just change detection

::: warning Hash Collisions
FNV-1a is used for fast detection, not cryptographic security. When hashes match, byte-by-byte comparison confirms equality.
:::

## File Type Detection

When file type is not explicitly specified:

| Detection Rule | File Type |
|---------------|-----------|
| Starts with `{` or `[` (after trim) | `json` |
| Contains null bytes (`\0`) | `binary` |
| All other content | `text` |

## Name Validation

Snapshot names must satisfy these constraints:

| Constraint | Reason |
|------------|--------|
| No `/` or `\` | Prevent path traversal |
| No `..` | Prevent directory escape |
| No null bytes | Prevent injection |
| Non-empty | Valid identifier required |

**Valid names:** `api-response`, `test_123`, `my.snapshot`

**Invalid names:** `../escape`, `sub/path`, `bad\0name`

## Directory Structure

```
snapshots/
├── api-response.snap
├── user-list.snap
├── binary-data.snap
└── config.snap
```

All snapshots share a flat directory. Subdirectories are not supported.

## Comparison Semantics

### Match Detection

A snapshot matches when:
1. Content hash matches (fast path)
2. Byte-by-byte comparison confirms (definitive)

### Mismatch Handling

When content differs, a diff is generated based on file type:

| Type | Diff Format | Powered By |
|------|-------------|------------|
| `text` | Unified diff with context | [odiff](/tools/output-diffing-utility/) |
| `json` | Semantic JSON diff | [odiff](/tools/output-diffing-utility/) |
| `binary` | Hex dump with differences | [odiff](/tools/output-diffing-utility/) |

### Update Mode

When `update_mode` is enabled and content differs:
1. New content replaces old snapshot
2. `Updated` timestamp refreshed
3. `Hash` and `Size` recalculated
4. Original `Created` preserved

## Version Compatibility

### Forward Compatibility

Readers should ignore unknown header fields to allow future extensions.

### Backward Compatibility

Version 1.0.0 is the initial format. Future versions will maintain backward compatibility.

## Error Conditions

| Error | Cause |
|-------|-------|
| `NotFound` | Snapshot file does not exist |
| `InvalidName` | Name contains forbidden characters |
| `CorruptedSnapshot` | Missing header, separator, or invalid metadata |
| `InvalidUtf8` | Text/JSON content is not valid UTF-8 |
| `IoError` | File system error during read/write |

## Conformance

A conforming implementation must:

1. Read and write the specified file format
2. Validate snapshot names before use
3. Use FNV-1a for hash calculation
4. Support all three file types
5. Preserve content exactly (no normalization)

## See Also

- [CLI Usage](/tools/snapshot-comparison/cli-usage) - Command-line interface
- [Library Usage](/tools/snapshot-comparison/library-usage) - Rust API
- [Output Diffing Utility](/tools/output-diffing-utility/) - Powers the diff output
