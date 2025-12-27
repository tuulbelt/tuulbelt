# CLI Usage

Complete reference for the command-line interface.

## Commands

### `init` - Initialize Progress Tracker

Create a new progress tracker.

```bash
prog init --total <number> --message <string> [options]
```

**Required:**
- `--total <number>` - Total number of items to process
- `--message <string>` - Initial status message

**Optional:**
- `--id <string>` - Unique identifier (default: "default")
- `--path <string>` - Custom storage directory

**Example:**
```bash
prog init --total 100 --message "Processing files" --id myproject
```

---

### `increment` - Update Progress

Increment progress by a specified amount.

```bash
prog increment [options]
```

**Optional:**
- `--amount <number>` - Amount to increment (default: 1)
- `--message <string>` - Updated status message
- `--id <string>` - Tracker ID
- `--path <string>` - Custom storage directory

**Example:**
```bash
prog increment --amount 10 --message "Processing batch 2" --id myproject
```

---

### `set` - Set Absolute Progress

Set progress to an absolute value.

```bash
prog set --current <number> [options]
```

**Required:**
- `--current <number>` - New current progress value

**Optional:**
- `--message <string>` - Updated status message
- `--id <string>` - Tracker ID
- `--path <string>` - Custom storage directory

**Example:**
```bash
prog set --current 75 --message "Almost done" --id myproject
```

---

### `get` - Query Current Progress

Retrieve current progress state as JSON.

```bash
prog get [options]
```

**Optional:**
- `--id <string>` - Tracker ID
- `--path <string>` - Custom storage directory

**Example:**
```bash
prog get --id myproject
```

**Output:**
```json
{
  "current": 75,
  "total": 100,
  "percentage": 75,
  "message": "Almost done",
  "complete": false,
  "startTime": 1234567890,
  "lastUpdate": 1234567895
}
```

---

### `finish` - Mark as Complete

Mark progress as 100% complete.

```bash
prog finish [options]
```

**Optional:**
- `--message <string>` - Final completion message
- `--id <string>` - Tracker ID
- `--path <string>` - Custom storage directory

**Example:**
```bash
prog finish --message "All files processed successfully" --id myproject
```

---

### `clear` - Remove Progress File

Delete the progress tracker file.

```bash
prog clear [options]
```

**Optional:**
- `--id <string>` - Tracker ID
- `--path <string>` - Custom storage directory

**Example:**
```bash
prog clear --id myproject
```

---

## Common Options

These options are available for most commands:

| Option | Description | Default |
|--------|-------------|---------|
| `--id <string>` | Unique tracker identifier | `"default"` |
| `--path <string>` | Custom storage directory | System temp dir |
| `--message <string>` | Status message | (varies by command) |

## Shell Script Integration

### Basic Example

```bash
#!/bin/bash
TASK="backup-$(date +%Y%m%d)"
FILES=$(ls data/*.csv | wc -l)

# Initialize
prog init --total $FILES --message "Starting backup" --id "$TASK"

# Process files
for file in data/*.csv; do
  cp "$file" backup/
  prog increment --id "$TASK"
done

# Finish
prog finish --message "Backup complete" --id "$TASK"
```

### Parallel Processing

```bash
#!/bin/bash
TOTAL=100
prog init --total $TOTAL --message "Parallel processing" --id parallel

# Run 4 workers in parallel
for i in {1..4}; do
  (
    for j in $(seq 1 25); do
      # Do work...
      sleep 0.1
      prog increment --id parallel
    done
  ) &
done

wait
prog finish --message "All workers complete" --id parallel
```

### Error Handling

```bash
#!/bin/bash
set -e  # Exit on error

TASK_ID="important-job"

# Trap errors and update progress
trap 'prog set --current 0 --message "Failed: $BASH_COMMAND" --id "$TASK_ID"' ERR

prog init --total 10 --message "Starting job" --id "$TASK_ID"

# Do work...
prog increment --id "$TASK_ID"

prog finish --message "Success!" --id "$TASK_ID"
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Error (invalid arguments, file not found, etc.) |

## Environment Variables

None. All configuration is via command-line flags.

## Tips

### Use Unique IDs

When tracking multiple operations, use descriptive IDs:

```bash
prog init --total 100 --message "Database backup" --id db-backup-$(date +%Y%m%d)
prog init --total 50 --message "File cleanup" --id cleanup-$(date +%Y%m%d)
```

### Monitor from Separate Process

Progress can be queried from a separate monitoring script:

```bash
# Worker script
prog increment --id longrunning

# Monitor script (separate terminal)
watch -n 1 'prog get --id longrunning | jq ".percentage"'
```

### Clean Up Old Progress Files

```bash
# Remove all progress files older than 1 day
find /tmp -name "progress-*.json" -mtime +1 -delete
```
