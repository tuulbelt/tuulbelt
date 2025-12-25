# Protocol Specification

The lock file format and protocol for interoperability between implementations.

## Overview

File-Based Semaphore uses a simple lock file protocol. The presence of the lock file indicates the lock is held; its absence means it's free.

## Lock File Format

### Content Structure

```
pid=<process_id>
timestamp=<unix_epoch_seconds>
tag=<optional_description>
```

### Example

```
pid=12345
timestamp=1703520000
tag=deploy-v1.2.3
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pid` | Integer | Yes | Process ID of lock holder |
| `timestamp` | Integer | Yes | Unix timestamp when acquired |
| `tag` | String | No | Human-readable description |

### Format Rules

1. One field per line
2. Lines terminated by LF (`\n`) or CRLF (`\r\n`)
3. Format: `key=value` (no spaces around `=`)
4. Unknown fields are ignored (forward compatibility)
5. Empty lines are ignored
6. File encoding: UTF-8

## Protocol Operations

### Acquiring a Lock

```
1. Attempt to create lock file with O_CREAT | O_EXCL
2. If creation succeeds:
   a. Write lock info (pid, timestamp, optional tag)
   b. Sync file to disk (fsync)
   c. Lock is acquired
3. If creation fails (file exists):
   a. Read existing lock file
   b. Check if lock is stale
   c. If stale: remove and retry from step 1
   d. If not stale: acquisition failed
```

### Releasing a Lock

```
1. Remove (unlink) the lock file
2. Lock is released
```

### Stale Detection

A lock is considered stale when:

```
current_time - lock_timestamp > stale_timeout
```

**Default stale timeout:** 3600 seconds (1 hour)

### Blocking Acquisition

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

**Default retry interval:** 100 milliseconds

## Platform Implementation

### Unix/Linux/macOS

```c
// Acquire
int fd = open(path, O_WRONLY | O_CREAT | O_EXCL, 0644);
if (fd >= 0) {
    write(fd, lock_info, strlen(lock_info));
    fsync(fd);
    close(fd);
    return SUCCESS;
}
if (errno == EEXIST) {
    return ALREADY_LOCKED;
}
return ERROR;

// Release
unlink(path);
```

### Windows

```c
// Acquire
HANDLE h = CreateFileW(path, GENERIC_WRITE, 0, NULL,
                       CREATE_NEW, FILE_ATTRIBUTE_NORMAL, NULL);
if (h != INVALID_HANDLE_VALUE) {
    WriteFile(h, lock_info, strlen(lock_info), &written, NULL);
    FlushFileBuffers(h);
    CloseHandle(h);
    return SUCCESS;
}
if (GetLastError() == ERROR_FILE_EXISTS) {
    return ALREADY_LOCKED;
}
return ERROR;

// Release
DeleteFileW(path);
```

## Interoperability

Any implementation following this specification can interoperate. This enables:

- Rust library with Python script
- Shell script with Node.js service
- Cross-platform builds coordinated by lock files

### Example: Shell Script Interop

```bash
# Shell script creating lock
echo "pid=$$" > /tmp/my.lock
echo "timestamp=$(date +%s)" >> /tmp/my.lock
echo "tag=shell-script" >> /tmp/my.lock

# Rust program can read this lock file
```

### Example: Parsing in Python

```python
def parse_lock_file(path):
    with open(path) as f:
        info = {}
        for line in f:
            if '=' in line:
                key, value = line.strip().split('=', 1)
                info[key] = value
    return info

# { 'pid': '12345', 'timestamp': '1703520000', 'tag': 'shell-script' }
```

## Security Considerations

### File Permissions

- Create with restrictive permissions: `0644` (Unix) or inherited ACLs (Windows)
- Lock file directory should not be world-writable without sticky bit

### Lock File Location

Recommended:
- `/tmp/` for temporary locks
- `/var/lock/` for system services
- `/run/` for runtime locks

Avoid:
- Network filesystems (unless NFSv4+ with proper locking)
- World-writable directories without sticky bit

### Denial of Service Mitigation

- Use stale detection to recover from crashed processes
- Consider separate lock directories per application
- Monitor for stuck locks

## Version History

| Version | Changes |
|---------|---------|
| 1.0 | Initial specification |

## License

This specification is released under CC0 (public domain).
