# CLI Usage

File-Based Semaphore provides a command-line interface for process coordination in shell scripts and automation.

## Commands

### `try` - Non-Blocking Acquire

Try to acquire a lock without waiting. Returns immediately.

```bash
sema try /tmp/my.lock
```

**Exit Codes:**
- `0` - Lock acquired
- `1` - Lock already held
- `3` - Error

### `acquire` - Blocking Acquire

Acquire a lock, waiting until available or timeout.

```bash
sema acquire /tmp/my.lock --timeout 30
```

**Options:**
- `--timeout <SECONDS>` - Maximum wait time (default: wait forever)

### `release` - Force Release

Remove a lock file (force release).

```bash
sema release /tmp/my.lock
```

### `status` - Check Status

Check if a lock is held and who holds it.

```bash
sema status /tmp/my.lock
```

**Output:**
```
Lock status: LOCKED
  Path: /tmp/my.lock
  PID: 12345
  Timestamp: 1703520000
  Tag: my-operation
  Process running: yes
```

### `wait` - Wait for Release

Wait for a lock to be released without acquiring it.

```bash
sema wait /tmp/my.lock --timeout 60
```

## Global Options

| Option | Description |
|--------|-------------|
| `--timeout <SECONDS>` | Timeout for acquire/wait |
| `--stale <SECONDS>` | Stale lock timeout (default: 3600) |
| `--tag <STRING>` | Tag to include in lock info |
| `--json` | Output status in JSON format |
| `--quiet, -q` | Suppress non-essential output |
| `--help, -h` | Show help |
| `--version, -V` | Show version |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Lock already held or timeout |
| 2 | Invalid arguments |
| 3 | IO or system error |

## Examples

### Basic Lock/Unlock

```bash
#!/bin/bash
if sema try /tmp/deploy.lock; then
    echo "Deploying..."
    ./deploy.sh
    sema release /tmp/deploy.lock
else
    echo "Another deployment in progress"
    exit 1
fi
```

### With Timeout

```bash
#!/bin/bash
if sema acquire /tmp/critical.lock --timeout 30; then
    echo "Got lock, running critical section"
    ./critical-operation.sh
    sema release /tmp/critical.lock
else
    echo "Timeout waiting for lock"
    exit 1
fi
```

### JSON Output for Scripting

```bash
STATUS=$(file-semaphore status /tmp/my.lock --json)
if echo "$STATUS" | grep -q '"locked":true'; then
    PID=$(echo "$STATUS" | jq -r '.pid')
    echo "Lock held by PID: $PID"
fi
```

### Tagged Locks

```bash
sema try /tmp/job.lock --tag "nightly-backup-$(date +%Y%m%d)"
```

### Quiet Mode for Scripts

```bash
# Only show errors
sema try /tmp/silent.lock -q && ./my-script.sh
```

## Shell Integration Tips

### Trap Handler for Cleanup

```bash
#!/bin/bash
cleanup() {
    sema release /tmp/my.lock -q
}
trap cleanup EXIT

sema acquire /tmp/my.lock --timeout 60
# Your code here...
# Lock auto-released on script exit
```

### Check Before Long Operation

```bash
# Check if lock is available before starting
if sema status /tmp/heavy-job.lock -q; then
    echo "Job already running"
    exit 1
fi

sema try /tmp/heavy-job.lock
./heavy-operation.sh
sema release /tmp/heavy-job.lock
```
