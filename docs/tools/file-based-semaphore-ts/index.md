# File-Based Semaphore (TypeScript) / `semats`

Cross-platform file-based semaphore for process synchronization in Node.js.

## Overview

File-Based Semaphore (TypeScript) provides a simple, reliable way to coordinate access to shared resources across multiple processes. It uses atomic file operations (temp file + rename) to prevent race conditions, making it perfect for build systems, CI/CD pipelines, and parallel workflows.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** TypeScript

**Repository:** [tuulbelt/file-based-semaphore-ts](https://github.com/tuulbelt/file-based-semaphore-ts)

::: tip Compatible with Rust Implementation
This tool uses the same lock file format as the [Rust File-Based Semaphore](/tools/file-based-semaphore/), enabling cross-language process coordination.
:::

## Features

### <img src="/icons/lock.svg" class="inline-icon" alt=""> Atomic File Operations
Uses temp file + rename pattern for atomic lock creation, preventing race conditions.

### <img src="/icons/shield.svg" class="inline-icon" alt=""> Stale Lock Detection
Automatically detects and cleans orphaned locks from crashed processes using PID validation and timestamps.

### <img src="/icons/tool.svg" class="inline-icon" alt=""> CLI and Library API
Use from command line for shell scripts or import as a library for TypeScript/JavaScript applications.

### <img src="/icons/globe.svg" class="inline-icon" alt=""> Cross-Platform
Works on Linux, macOS, and Windows with consistent behavior.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies
Uses only Node.js built-ins. No `npm install` required in production.

### <img src="/icons/beaker.svg" class="inline-icon" alt=""> Comprehensive Security
Path traversal prevention, symlink resolution, tag sanitization, and cryptographic randomness for temp files.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/file-based-semaphore-ts.git
cd file-based-semaphore-ts

# Install dev dependencies (for TypeScript)
npm install

# Try to acquire a lock
semats try-acquire /tmp/my-lock.lock --tag "build process"

# Check lock status
semats status /tmp/my-lock.lock

# Release the lock
semats release /tmp/my-lock.lock
```

## Use Cases

- **Build Systems:** Prevent concurrent builds from conflicting
- **CI/CD Pipelines:** Coordinate parallel test execution
- **Port Allocation:** Lock ports before binding (used by port-conflict-resolver)
- **File Processing:** Serialize access to shared files
- **Database Migrations:** Ensure only one migration runs at a time

## Why File-Based?

File-based locking provides several advantages:

1. **Simple:** No external services or databases required
2. **Portable:** Works everywhere - local, CI, containers
3. **Debuggable:** Lock files are plain text - inspect with `cat`
4. **Cross-Language:** Compatible with Rust `sema` tool
5. **Recoverable:** Stale locks are automatically detected

## Demo

See the tool in action:

![File-Based Semaphore Demo](/file-based-semaphore-ts/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/0x3TB8Gl6UCcb9U5BcmHGHqei)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/file-based-semaphore-ts" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

Run the tool directly in your browser with zero setup.

> Demos are automatically generated via GitHub Actions when demo scripts are updated.

## Next Steps

- [Getting Started Guide](/tools/file-based-semaphore-ts/getting-started) - Installation and setup
- [CLI Usage](/tools/file-based-semaphore-ts/cli-usage) - Command-line interface
- [Library Usage](/tools/file-based-semaphore-ts/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/file-based-semaphore-ts/examples) - Real-world usage patterns
- [Protocol Spec](/tools/file-based-semaphore-ts/protocol-spec) - Lock file format specification
- [API Reference](/tools/file-based-semaphore-ts/api-reference) - Complete API documentation

## Related Tools

- [File-Based Semaphore (Rust)](/tools/file-based-semaphore/) - Rust implementation (same lock file format)
- [Port Resolver](/tools/port-resolver/) - Uses semats for atomic port allocation

## License

MIT License - see repository for details.
