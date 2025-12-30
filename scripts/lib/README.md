# Demo Recording Framework

Shared library for recording tool demonstrations with asciinema.

## Overview

The `demo-framework.sh` library provides a standardized way to create demo recordings for Tuulbelt tools. It handles:

- Dependency installation (npm/cargo)
- asciinema recording
- Upload to asciinema.org
- GIF conversion with agg
- Setup/cleanup hooks for test data

## Quick Start

Create a new demo script by sourcing the framework and defining required variables:

```bash
#!/bin/bash
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="my-tool"
SHORT_NAME="mytool"
LANGUAGE="typescript"  # or "rust"

demo_commands() {
  echo "# My Tool Demo"
  sleep 1

  echo "$ mytool --help"
  mytool --help
  sleep 2
}

run_demo
```

## Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `TOOL_NAME` | Full tool name (directory name) | `cli-progress-reporting` |
| `SHORT_NAME` | CLI short name | `prog` |
| `LANGUAGE` | `typescript` or `rust` | `typescript` |

## Optional Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `GIF_COLS` | Terminal columns | 80 |
| `GIF_ROWS` | Terminal rows | 24 |
| `GIF_SPEED` | Playback speed | 1.0 |
| `GIF_FONT_SIZE` | Font size | 16 |
| `GIF_THEME` | agg theme | monokai |

## Optional Hooks

### `demo_setup()`

Runs before recording. Use for creating test files or temp directories:

```bash
demo_setup() {
  mkdir -p "$TOOL_DIR/demo-files"
  echo '{"key": "value"}' > "$TOOL_DIR/demo-files/config.json"
}
```

### `demo_cleanup()`

Runs after recording. Use for removing temp files:

```bash
demo_cleanup() {
  rm -rf "$TOOL_DIR/demo-files"
}
```

## Available Variables in demo_commands()

| Variable | Description |
|----------|-------------|
| `$TOOL_DIR` | Absolute path to tool directory |
| `$SHORT_NAME` | CLI short name |
| `$BIN` | Path to built binary (Rust only) |

## Rust Tools

For Rust tools, use `$BIN` instead of the short name:

```bash
demo_commands() {
  echo "$ sema --help"
  $BIN --help
}
```

The framework automatically builds with `cargo build --release` and sets `$BIN` to the release binary path.

## Output Files

The framework creates:

| File | Description |
|------|-------------|
| `demo.cast` | asciinema recording |
| `demo-url.txt` | asciinema.org upload URL |
| `docs/demo.gif` | Animated GIF (if agg available) |

## CI Integration

The `create-demos.yml` workflow:
1. Detects changed scripts
2. Installs dependencies
3. Runs recording scripts
4. Uploads to asciinema.org
5. Generates GIFs
6. Updates README and VitePress docs

Trigger manually via workflow_dispatch if needed.

## Creating Demos for New Tools

1. Copy the template: `templates/{language}-tool-template/scripts/record-demo.sh`
2. Rename to: `scripts/record-{tool-name}-demo.sh`
3. Update `TOOL_NAME`, `SHORT_NAME`, `LANGUAGE`
4. Implement `demo_commands()` with your tool's demo flow
5. Add `demo_setup()` / `demo_cleanup()` if needed
6. Test locally: `bash scripts/record-{tool-name}-demo.sh`
7. Commit to trigger CI recording
