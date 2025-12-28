# CLI Usage

File-Based Semaphore provides a comprehensive CLI for process coordination.

## CLI Names

Both short and long forms work:
- **Short (recommended):** `semats`
- **Long:** `file-semaphore-ts`

## Commands

### try-acquire

Try to acquire a lock (non-blocking). Returns immediately with success or failure.

```bash
semats try-acquire <path> [options]
```

**Arguments:**
- `path` - Path to the lock file

**Options:**
- `-t, --tag <tag>` - Optional description for the lock
- `-j, --json` - Output in JSON format
- `-v, --verbose` - Verbose output

**Examples:**

```bash
# Basic acquisition
semats try-acquire /tmp/build.lock

# With descriptive tag
semats try-acquire /tmp/build.lock --tag "npm build"

# JSON output for scripting
semats try-acquire /tmp/build.lock --json
```

**Output:**

```
status: acquired
pid: 12345
timestamp: 1735420800
tag: npm build
```

**Exit Codes:**
- `0` - Lock acquired successfully
- `1` - Lock already held by another process

### acquire

Acquire a lock with blocking and timeout (waits until lock is available).

```bash
semats acquire <path> [options]
```

**Arguments:**
- `path` - Path to the lock file

**Options:**
- `--timeout <ms>` - Maximum time to wait in milliseconds
- `-t, --tag <tag>` - Optional description for the lock
- `-j, --json` - Output in JSON format
- `-v, --verbose` - Verbose output

**Examples:**

```bash
# Wait up to 5 seconds for lock
semats acquire /tmp/build.lock --timeout 5000

# With tag
semats acquire /tmp/build.lock --timeout 30000 --tag "npm build"
```

**Exit Codes:**
- `0` - Lock acquired successfully
- `1` - Timeout waiting for lock

### release

Release a held lock.

```bash
semats release <path> [options]
```

**Arguments:**
- `path` - Path to the lock file

**Options:**
- `-f, --force` - Force release even if held by another process
- `-j, --json` - Output in JSON format
- `-v, --verbose` - Verbose output

**Examples:**

```bash
# Release lock you own
semats release /tmp/build.lock

# Force release (use with caution)
semats release /tmp/build.lock --force
```

**Exit Codes:**
- `0` - Lock released successfully
- `1` - Lock not held or permission denied

### status

Check the status of a lock.

```bash
semats status <path> [options]
```

**Arguments:**
- `path` - Path to the lock file

**Options:**
- `-j, --json` - Output in JSON format
- `-v, --verbose` - Verbose output

**Examples:**

```bash
# Human-readable status
semats status /tmp/build.lock

# JSON for scripting
semats status /tmp/build.lock --json
```

**Output:**

```
locked: true
pid: 12345
timestamp: 1735420800
tag: npm build
isStale: false
isOwnedByCurrentProcess: true
```

**JSON Output:**

```json
{
  "locked": true,
  "info": {
    "pid": 12345,
    "timestamp": 1735420800,
    "tag": "npm build"
  },
  "isStale": false,
  "isOwnedByCurrentProcess": true
}
```

### clean

Clean stale locks and orphaned temp files.

```bash
semats clean <path> [options]
```

**Arguments:**
- `path` - Path to the lock file

**Options:**
- `-j, --json` - Output in JSON format
- `-v, --verbose` - Verbose output

**Examples:**

```bash
# Clean stale locks
semats clean /tmp/build.lock

# JSON output
semats clean /tmp/build.lock --json
```

**Output:**

```
cleaned: true
```

A lock is considered stale when:
1. The holding process (PID) no longer exists, AND
2. The timestamp is older than the stale timeout (default: 1 hour)

### help

Show help information.

```bash
semats --help
semats -h
```

## Global Options

These options work with all commands:

| Option | Short | Description |
|--------|-------|-------------|
| `--json` | `-j` | Output in JSON format |
| `--verbose` | `-v` | Verbose output |
| `--help` | `-h` | Show help message |

## Shell Scripting

Example shell script using semats:

```bash
#!/bin/bash
set -e

LOCKFILE="/tmp/deploy.lock"

# Acquire lock with timeout
if semats acquire "$LOCKFILE" --timeout 60000 --tag "deploy $(date)"; then
    trap 'semats release "$LOCKFILE"' EXIT

    # Do deployment
    npm run build
    npm run deploy

    echo "Deployment complete"
else
    echo "Failed to acquire lock - another deployment in progress"
    exit 1
fi
```

## JSON Mode

Use `--json` for machine-readable output:

```bash
# Capture JSON output
STATUS=$(semats status /tmp/build.lock --json)

# Parse with jq
IS_LOCKED=$(echo "$STATUS" | jq -r '.locked')
HOLDER_PID=$(echo "$STATUS" | jq -r '.info.pid')

if [ "$IS_LOCKED" = "true" ]; then
    echo "Lock held by PID $HOLDER_PID"
fi
```

## Error Handling

All errors are returned as JSON when `--json` is used:

```json
{
  "status": "failed",
  "type": "ALREADY_LOCKED",
  "message": "Lock is already held by another process",
  "holderPid": 12345
}
```

Error types:
- `ALREADY_LOCKED` - Lock held by another process
- `NOT_LOCKED` - Attempting to release unlocked lock
- `PERMISSION_DENIED` - Not the lock owner (use `--force`)
- `TIMEOUT` - Timeout waiting for lock
- `IO_ERROR` - File system error
- `PATH_TRAVERSAL` - Dangerous path pattern detected
