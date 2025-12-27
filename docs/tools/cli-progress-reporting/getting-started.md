# Getting Started

Get up and running with CLI Progress Reporting in minutes.

## Installation

### Clone the Repository

```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/cli-progress-reporting
```

### Install Dev Dependencies

```bash
npm install
```

:::tip Zero Runtime Dependencies
CLI Progress Reporting has **zero runtime dependencies**. The `npm install` step only installs development tools (TypeScript compiler, test runner). The tool itself uses only Node.js built-in modules.
:::

## Quick Example

### Initialize Progress

```bash
prog init --total 100 --message "Processing files"
```

Output:
```
[0%] 0/100 - Processing files (0s)
```

### Update Progress

```bash
prog increment --amount 10
```

Output:
```
[10%] 10/100 - Processing files (1s)
```

### Check Current Status

```bash
prog get
```

Output (JSON):
```json
{
  "current": 10,
  "total": 100,
  "percentage": 10,
  "message": "Processing files",
  "complete": false,
  "startTime": 1234567890,
  "lastUpdate": 1234567891
}
```

### Mark as Complete

```bash
prog finish --message "All done!"
```

Output:
```
[100%] 100/100 - All done! (5s)
```

## Basic Concepts

### Progress State

Each progress tracker maintains these properties:

- **current** — Current progress value
- **total** — Total items to process
- **percentage** — Calculated percentage (0-100)
- **message** — Current status message
- **complete** — Whether tracking is complete
- **startTime** — Unix timestamp when initialized
- **lastUpdate** — Unix timestamp of last update

### Progress IDs

Use unique IDs to track multiple operations simultaneously:

```bash
# Track two separate jobs
prog init --total 100 --message "Job 1" --id job1
prog init --total 50 --message "Job 2" --id job2

# Update them independently
prog increment --amount 10 --id job1
prog increment --amount 5 --id job2
```

If no ID is specified, a default ID is used.

### File-Based Storage

Progress state is stored as JSON files in:
- **Default:** System temp directory (e.g., `/tmp/`)
- **Custom:** Set via `--path` flag or `filePath` config

Files are named: `progress-{id}.json`

### Atomic Writes

Updates use a write-then-rename pattern:
1. Write new state to temporary file
2. Atomically rename to target file

This ensures concurrent processes never see partial updates.

## Next Steps

- [CLI Usage](/tools/cli-progress-reporting/cli-usage) - All CLI commands and options
- [Library Usage](/tools/cli-progress-reporting/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/cli-progress-reporting/examples) - Real-world usage patterns

## Troubleshooting

### Permission Denied

If you get permission errors, the temp directory may not be writable. Use a custom path:

```bash
prog init --total 100 --message "Test" --path ./progress
```

### Progress Not Updating

Make sure you're using the same ID for all commands:

```bash
# Correct - same ID
prog init --total 100 --id myproject
prog increment --id myproject

# Wrong - different IDs
prog init --total 100 --id project1
prog increment --id project2  # Won't find project1
```

### JSON Parse Errors

If the progress file becomes corrupted, clear it and start fresh:

```bash
prog clear --id myproject
prog init --total 100 --message "Restarting" --id myproject
```
