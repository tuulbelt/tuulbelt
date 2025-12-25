# File-Based Semaphore Protocol Specification

**Version:** 1.0
**Status:** Stable

This document specifies the lock file format and protocol for interoperability between different implementations.

## Overview

A file-based semaphore uses a lock file to coordinate access between processes. The presence of the lock file indicates the lock is held; its absence indicates the lock is free.

## Lock File Format

### File Content

Lock files contain newline-separated key-value pairs:

```
pid=<process_id>
timestamp=<unix_epoch_seconds>
tag=<optional_description>
```

**Required Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `pid` | Integer | Process ID of the lock holder |
| `timestamp` | Integer | Unix timestamp (seconds since epoch) when lock was acquired |

**Optional Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `tag` | String | Human-readable description or identifier |

### Example

```
pid=12345
timestamp=1703520000
tag=deploy-v1.2.3
```

### Format Rules

1. Each field is on its own line
2. Lines are terminated by `\n` (LF) or `\r\n` (CRLF)
3. Field format: `key=value`
4. No spaces around `=`
5. Unknown fields MUST be ignored (forward compatibility)
6. Empty lines are ignored
7. File encoding: UTF-8

## Protocol

### Acquiring a Lock

1. Attempt to create the lock file with `O_CREAT | O_EXCL` (atomic exclusive create)
2. If creation succeeds:
   - Write lock info (pid, timestamp, optional tag)
   - Sync file to disk (`fsync`)
   - Lock is acquired
3. If creation fails with "file exists":
   - Read existing lock file
   - Check if lock is stale (see Stale Detection)
   - If stale, remove and retry
   - If not stale, lock acquisition failed

### Releasing a Lock

1. Remove (unlink) the lock file
2. Lock is released

### Stale Detection

A lock is considered stale when:

```
current_time - lock_timestamp > stale_timeout
```

**Recommended stale timeout:** 3600 seconds (1 hour)

When a stale lock is detected:
1. Remove the stale lock file
2. Proceed with normal acquisition

**Note:** There is an inherent race condition between stale detection and removal. Implementations should handle `ENOENT` gracefully during removal.

### Blocking Acquisition

For blocking acquisition with timeout:

```
start_time = now()
while true:
    result = try_acquire()
    if result == success:
        return success
    if now() - start_time >= timeout:
        return timeout_error
    sleep(retry_interval)
```

**Recommended retry interval:** 100 milliseconds

## Platform Considerations

### Linux/macOS

- Use `open()` with `O_CREAT | O_EXCL`
- Use `unlink()` for removal
- Check `/proc/<pid>` to verify process existence (Linux only)

### Windows

- Use `CreateFileW` with `CREATE_NEW`
- Use `DeleteFileW` for removal
- Use `OpenProcess` to verify process existence

### Network Filesystems

- NFSv4+ provides proper atomic create semantics
- NFSv3 may have issues with `O_EXCL`; consider using `.lock` directory instead
- SMB/CIFS generally works but test in your environment

## Security Considerations

### File Permissions

Lock files should be created with restrictive permissions:
- Unix: `0644` or `0600`
- Windows: Inherit parent directory ACLs

### Lock File Location

Recommended locations:
- `/tmp/` for temporary locks
- `/var/lock/` for system services
- Application-specific directory for application locks

Avoid:
- World-writable directories without sticky bit
- Network locations for high-frequency locking

### Denial of Service

An attacker with write access to the lock file location could:
- Create lock files that never release
- Delete lock files to break coordination

Mitigate by:
- Using appropriate file permissions
- Using a dedicated directory for locks
- Implementing stale detection

## Interoperability

This specification enables interoperability between:

- Different programming languages
- Different operating systems
- Shell scripts and compiled programs

Any implementation that follows this specification can interoperate with any other compliant implementation.

## Reference Implementation

The reference implementation is available at:
https://github.com/tuulbelt/file-based-semaphore

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-25 | Initial specification |

## License

This specification is released under CC0 (public domain).
