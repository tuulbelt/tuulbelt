# Protocol Specification

The lock file format used by File-Based Semaphore (TypeScript).

## Overview

This tool uses the same lock file format as the [Rust File-Based Semaphore](/tools/file-based-semaphore/), enabling cross-language process coordination.

## Lock File Format

Lock files are plain text with key-value pairs:

```
pid=<process_id>
timestamp=<unix_timestamp>
tag=<optional_description>
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pid` | integer | Yes | Process ID of the lock holder |
| `timestamp` | integer | Yes | Unix timestamp (seconds) when acquired |
| `tag` | string | No | Optional description/identifier |

### Example Lock File

```
pid=12345
timestamp=1735420800
tag=npm build
```

## Line Endings

Both Unix (`\n`) and Windows (`\r\n`) line endings are accepted when parsing. Lock files are always written with Unix line endings.

## Character Encoding

- Lock files use UTF-8 encoding
- Tags may contain Unicode characters
- Control characters in tags are replaced with spaces

## Atomic Operations

### Lock Acquisition

1. Create temp file: `<path>.<pid>.<random>.tmp`
2. Write lock info to temp file
3. Attempt atomic rename to final path
4. On success: lock acquired
5. On failure (file exists): lock held by another process

### Lock Release

1. Verify lock file exists
2. Read lock file content
3. Verify PID matches (unless force)
4. Delete lock file

## Stale Lock Detection

A lock is considered stale when:

1. The holding PID no longer exists (process check)
2. **AND** the timestamp is older than `staleTimeout`

The second condition prevents false positives when:
- PIDs are reused quickly after process death
- System clocks are skewed

Default stale timeout: 1 hour (3600000ms)

## Security Considerations

### Path Traversal Prevention

Paths containing `..` are rejected to prevent directory traversal attacks.

### Symlink Resolution

Symlinks are resolved to their target paths, including dangling symlinks.

### Tag Sanitization

All control characters (0x00-0x1F, 0x7F) are replaced with spaces to prevent:
- Newline injection (fake key-value pairs)
- Terminal escape sequence injection
- Null byte attacks

### Temp File Security

- Temp files use cryptographic random suffixes
- Created with restrictive permissions (0600)
- Orphaned temp files are cleaned by `cleanStale()`

## Compatibility

### Rust `sema` Tool

This format is fully compatible with the Rust `sema` tool:

```bash
# TypeScript creates lock
semats try-acquire /tmp/shared.lock --tag "ts-process"

# Rust reads it
sema status /tmp/shared.lock

# Output:
# locked: true
# pid: 12345
# timestamp: 1735420800
# tag: ts-process
```

### Version Compatibility

The format is designed to be forward-compatible:
- Unknown keys are ignored when parsing
- Future versions may add optional fields

## Parsing Rules

1. Split content by newlines (`\n` or `\r\n`)
2. For each line, split by first `=`
3. Trim whitespace from key and value
4. Skip empty lines and lines without `=`
5. `pid` and `timestamp` must be valid integers
6. `tag` is optional; missing means no tag

### Error Cases

Parsing fails if:
- `pid` is missing or non-numeric
- `timestamp` is missing or non-numeric
- File is empty or contains only whitespace
- File contains binary garbage

## Implementation Notes

### Atomic Rename

The temp file + rename pattern provides atomicity on most filesystems:
- POSIX: `rename()` is atomic within same filesystem
- Windows: `MoveFileEx` with `MOVEFILE_REPLACE_EXISTING`

### Known Limitations

- **TOCTOU Race:** Time-of-check to time-of-use gap between checking lock existence and creating it. Mitigated but not eliminated by atomic rename.
- **Network Filesystems:** NFS and other network filesystems may not guarantee rename atomicity.
- **File Permissions:** Depend on filesystem support.
