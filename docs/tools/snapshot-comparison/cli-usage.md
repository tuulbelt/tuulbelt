# CLI Usage

Complete command-line reference for Snapshot Comparison.

## Synopsis

```bash
snapcmp <COMMAND> [OPTIONS]
```

## Commands

### create

Create a new snapshot from stdin.

```bash
snapcmp create <name> [OPTIONS]
```

**Arguments:**
- `<name>` - Snapshot name (no path separators)

**Example:**
```bash
echo "Hello" | snapcmp create greeting
# Output: Created snapshot 'greeting' (6 bytes, type: Text)

cat data.json | snapcmp create config -t json
# Output: Created snapshot 'config' (128 bytes, type: Json)
```

### check

Check stdin against a stored snapshot.

```bash
snapcmp check <name> [OPTIONS]
```

**Arguments:**
- `<name>` - Snapshot name to check against

**Behavior:**
- Returns exit code 0 on match
- Returns exit code 1 on mismatch (prints diff)
- With `--update`, updates snapshot instead of failing

**Example:**
```bash
echo "Hello" | snapcmp check greeting
# Output: ✓ Snapshot 'greeting' matches

echo "Changed" | snapcmp check greeting
# Output: ✗ Snapshot 'greeting' does not match
# [Diff output]
# Exit code: 1

echo "Changed" | snapcmp check greeting --update
# Output: ↻ Snapshot 'greeting' updated
# Exit code: 0
```

### update

Update an existing snapshot with new content.

```bash
snapcmp update <name> [OPTIONS]
```

**Arguments:**
- `<name>` - Snapshot name to update

**Example:**
```bash
echo "New content" | snapcmp update greeting
# Output: Updated snapshot 'greeting' (12 bytes)
```

### list

List all snapshots in the store.

```bash
snapcmp list [OPTIONS]
```

**Example:**
```bash
snapcmp list
# Output:
# Snapshots in 'snapshots':
#
#   greeting (6 bytes, Text)
#   config (128 bytes, Json)
#
# Total: 2 snapshot(s)
```

### delete

Delete a snapshot.

```bash
snapcmp delete <name> [OPTIONS]
```

**Arguments:**
- `<name>` - Snapshot name to delete

**Example:**
```bash
snapcmp delete old-test
# Output: Deleted snapshot 'old-test'
```

### clean

Remove orphaned snapshots not in a keep list.

```bash
snapcmp clean --keep <names> [OPTIONS]
```

**Arguments:**
- `--keep <names>` - Comma-separated list of snapshot names to keep

**Example:**
```bash
# See what would be deleted
snapcmp clean --keep test-1,test-2 --dry-run
# Output: Would delete 3 snapshot(s):
#   old-test-1
#   old-test-2
#   deprecated

# Actually delete
snapcmp clean --keep test-1,test-2
# Output: Deleted 3 snapshot(s):
#   old-test-1
#   old-test-2
#   deprecated
```

## Options

### -d, --dir \<PATH\>

Snapshot directory (default: `./snapshots`)

```bash
snapcmp list -d my-snapshots
echo "test" | snapcmp create test -d /tmp/snapshots
```

### -t, --type \<TYPE\>

File type hint (default: auto-detect)

Values: `text`, `json`, `binary`

```bash
cat data.json | snapcmp create config -t json
cat image.png | snapcmp create logo -t binary
```

### -c, --color

Enable colored diff output.

```bash
echo "changed" | snapcmp check test --color
```

### -u, --update

Update mode: update snapshot on mismatch instead of failing.

```bash
echo "new content" | snapcmp check test --update
```

### --context \<N\>

Number of context lines in diff output (default: 3).

```bash
echo "changed" | snapcmp check test --context 5
```

### --keep \<names\>

Comma-separated list of snapshot names to keep (for `clean` command).

```bash
snapcmp clean --keep test-1,test-2,important
```

### --dry-run

For `clean`: show what would be deleted without deleting.

```bash
snapcmp clean --keep active-tests --dry-run
```

### -h, --help

Show help message.

```bash
snapcmp --help
snapcmp create --help
```

### -V, --version

Show version information.

```bash
snapcmp --version
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (match, update, or operation completed) |
| 1 | Error (mismatch, not found, invalid input) |

## Examples

### CI Integration

```bash
#!/bin/bash
set -e

# Check all API snapshots
for endpoint in users products orders; do
    curl -s "https://api.example.com/$endpoint" | snapcmp check "api-$endpoint"
done

echo "All API responses match snapshots"
```

### Development Workflow

```bash
# Run tests, update snapshots if needed
./run-tests.sh | snapcmp check test-output --update

# Review changes
git diff snapshots/
```

### Binary Snapshots

```bash
# Create binary snapshot
./generate-image.sh | snapcmp create logo -t binary

# Check binary output
./generate-image.sh | snapcmp check logo -t binary
```
