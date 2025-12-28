# CLI Progress Reporting

Concurrent-safe progress tracking with file-based atomic writes.

## Overview

CLI Progress Reporting provides a simple, reliable way to track progress across multiple concurrent processes. It uses file-based atomic writes to ensure data consistency, making it perfect for parallel workflows, shell scripts, and CI/CD pipelines.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** TypeScript

**Repository:** [tuulbelt/tuulbelt/cli-progress-reporting](https://github.com/tuulbelt/tuulbelt/tree/main/cli-progress-reporting)

## Features

### <img src="/icons/lock.svg" class="inline-icon" alt=""> Concurrent-Safe Operations
File-based atomic writes prevent race conditions when multiple processes update progress simultaneously.

### <img src="/icons/hash.svg" class="inline-icon" alt=""> Multiple Independent Trackers
Track multiple operations in parallel using unique IDs. Each tracker maintains its own isolated state.

### <img src="/icons/tool.svg" class="inline-icon" alt=""> CLI and Library API
Use from command line for shell scripts or import as a library for TypeScript/JavaScript applications.

### <img src="/icons/database.svg" class="inline-icon" alt=""> State Persistence
Progress state persists across process boundaries. Pick up where you left off after crashes or restarts.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies
Uses only Node.js built-ins. No `npm install` required in production.

### <img src="/icons/beaker.svg" class="inline-icon" alt=""> Dogfooding Validated
Validated using Test Flakiness Detector with 100% pass rate and zero flaky tests. All tests are deterministic and reliable.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/cli-progress-reporting

# Install dev dependencies (for TypeScript)
npm install

# Initialize a progress tracker
prog init --total 100 --message "Processing files"

# Update progress
prog increment --amount 10

# Check current status
prog get
```

## Use Cases

- **Parallel Scripts:** Track progress across concurrent shell scripts
- **CI/CD Pipelines:** Report build/deployment progress
- **Data Processing:** Monitor long-running batch jobs
- **Multi-Stage Workflows:** Track progress through multi-step processes
- **Progress Bars:** Build custom progress visualization tools

## Why File-Based?

File-based state management provides several advantages:

1. **Simple:** No external services or databases required
2. **Portable:** Works everywhere - local, CI, containers
3. **Debuggable:** State is plain JSON - inspect with `cat`
4. **Recoverable:** State persists across process crashes
5. **Concurrent-Safe:** Atomic rename operations prevent corruption

## Demo

See the tool in action:

![CLI Progress Demo](/cli-progress-reporting/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/DMUfsw7Ln0xox7v1x3CogM5Fm)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/cli-progress-reporting" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

Run the tool directly in your browser with zero setup. Initialize trackers, update progress, and explore concurrent-safe operations.

> Demos are automatically generated via GitHub Actions when demo scripts are updated.

## Next Steps

- [Getting Started Guide](/tools/cli-progress-reporting/getting-started) - Installation and setup
- [CLI Usage](/tools/cli-progress-reporting/cli-usage) - Command-line interface
- [Library Usage](/tools/cli-progress-reporting/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/cli-progress-reporting/examples) - Real-world usage patterns
- [API Reference](/tools/cli-progress-reporting/api-reference) - Complete API documentation
- [Dogfooding Validation](/tools/cli-progress-reporting/dogfooding) - How we validate reliability

## License

MIT License - see repository for details.
