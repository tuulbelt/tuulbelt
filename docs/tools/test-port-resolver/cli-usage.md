# CLI Usage

The `portres` CLI provides commands for allocating and managing test ports.

## Commands

### `get` - Allocate a Port

```bash
portres get [options]
```

Options:
- `--tag <name>` - Identifier for grouping ports
- `--min <port>` - Minimum port number (default: 10000)
- `--max <port>` - Maximum port number (default: 65535)
- `--registry <path>` - Custom registry file path

Examples:
```bash
# Basic allocation
portres get
# Output: 54321

# With tag for grouping
portres get --tag "api-server"

# Custom port range
portres get --min 8000 --max 9000

# Custom registry
portres get --registry /tmp/my-ports.json
```

### `release` - Release a Port

```bash
portres release --port <number>
```

Options:
- `--port <number>` - Port number to release (required)
- `--registry <path>` - Custom registry file path

Example:
```bash
portres release --port 54321
```

### `release-all` - Release All Ports

```bash
portres release-all [options]
```

Options:
- `--tag <name>` - Only release ports with this tag
- `--registry <path>` - Custom registry file path

Examples:
```bash
# Release all ports
portres release-all

# Release only ports with specific tag
portres release-all --tag "api-server"
```

### `list` - List Allocated Ports

```bash
portres list [options]
```

Options:
- `--registry <path>` - Custom registry file path

Example output:
```
Port  Tag         PID    Allocated
54321 api-server  12345  2025-12-29T10:30:00Z
54322 db-test     12346  2025-12-29T10:30:05Z
```

### `clean` - Remove Stale Entries

```bash
portres clean [options]
```

Removes entries from crashed processes (PID no longer exists).

Options:
- `--registry <path>` - Custom registry file path

Example:
```bash
portres clean
# Output: Cleaned 3 stale entries
```

### `status` - Show Registry Status

```bash
portres status [options]
```

Options:
- `--registry <path>` - Custom registry file path

Example output:
```json
{
  "totalEntries": 5,
  "activeEntries": 3,
  "staleEntries": 2,
  "registryPath": "/tmp/tuulbelt-port-registry.json"
}
```

### `clear` - Clear Entire Registry

```bash
portres clear [options]
```

**Warning:** This removes all entries, not just your process's entries.

Options:
- `--registry <path>` - Custom registry file path

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (port not found, allocation failed, etc.) |

## Shell Integration

### Capture Port in Variable

```bash
PORT=$(portres get --tag "my-test")
echo "Using port: $PORT"
```

### Cleanup on Exit

```bash
#!/bin/bash
PORT=$(portres get --tag "cleanup-demo")
trap "portres release --port $PORT" EXIT

# Your test code here
npm test
```

### Multiple Ports

```bash
# Allocate multiple ports
API_PORT=$(portres get --tag "api")
DB_PORT=$(portres get --tag "db")
CACHE_PORT=$(portres get --tag "cache")

# Use in your application
export API_PORT DB_PORT CACHE_PORT
npm test

# Cleanup
portres release-all --tag "api"
portres release-all --tag "db"
portres release-all --tag "cache"
```

## Troubleshooting

### Port Still in Use

If a port is reported as allocated but the process crashed:

```bash
portres clean
```

### Registry Corruption

If the registry file is corrupted:

```bash
portres clear
```

### Check Process Ownership

```bash
portres list
# Note the PID column - verify these processes exist
ps aux | grep <PID>
```
