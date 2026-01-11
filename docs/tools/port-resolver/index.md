# Port Resolver

Concurrent port allocation for any application - avoid port conflicts in tests, servers, microservices, and development.

## Overview

Port Resolver (`portres`) provides a centralized, file-based registry for port allocation. When running tests in parallel or multiple services in development, it ensures each process gets a unique port without conflicts.

::: tip <img src="/icons/package.svg" class="inline-icon" alt=""> Uses semats Library
This tool uses [file-based-semaphore-ts](/tools/file-based-semaphore-ts/) as a **library dependency** for atomic registry access. See [PRINCIPLES.md Exception 2](/guide/principles#zero-external-dependencies) for details on Tuulbelt tool composition.
:::

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.3.0) · 194 tests passing

**Language:** TypeScript

**Repository:** [tuulbelt/port-resolver](https://github.com/tuulbelt/port-resolver)

## Features

### <img src="/icons/target.svg" class="inline-icon" alt=""> File-Based Registry

Centralized port registry with atomic operations. Multiple processes coordinate without conflicts.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Real Port Verification

TCP bind test confirms ports are actually available, not just recorded as free.

### <img src="/icons/clock.svg" class="inline-icon" alt=""> Automatic Stale Cleanup

Detects and removes stale entries from crashed processes based on PID checks.

### <img src="/icons/lock.svg" class="inline-icon" alt=""> Semaphore-Protected

Uses [file-based-semaphore-ts](/tools/file-based-semaphore-ts/) for atomic registry access.

### <img src="/icons/tool.svg" class="inline-icon" alt=""> CLI & Library

Use as a command-line tool for shell scripts or integrate as a TypeScript/JavaScript library.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Node.js built-ins plus Tuulbelt tools (zero external dependencies maintained).

### <img src="/icons/layers.svg" class="inline-icon" alt=""> Batch Allocation (v0.2.0)

Allocate multiple ports atomically with all-or-nothing semantics and per-port tag tracking.

### <img src="/icons/package.svg" class="inline-icon" alt=""> Port Lifecycle Management (v0.2.0)

Track allocations by tag with `PortManager` for simplified port tracking and cleanup.

### <img src="/icons/sliders.svg" class="inline-icon" alt=""> Port Range Allocation (v0.2.0)

Reserve contiguous port ranges for microservices clusters or get ports within specific bounds for firewall compliance.

### <img src="/icons/package.svg" class="inline-icon" alt=""> Tree-Shakable & Modular (v0.3.0)

8 entry points for optimal tree-shaking. Import only what you need and save 40-80% bundle size.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/port-resolver.git
cd port-resolver

# Install dev dependencies
npm install

# CLI usage
portres get --tag "api-server"
# Output: 54321

# Release when done
portres release --port 54321
```

```typescript
// Library usage (v0.2.0 - PortManager)
import { PortManager } from '@tuulbelt/port-resolver';

const manager = new PortManager();
await manager.allocate('frontend');
await manager.allocate('backend');
await manager.allocate('database');

// Access by tag
const frontend = manager.get('frontend');
console.log(`Frontend port: ${frontend?.port}`);

// Release all at once
await manager.releaseAll();
```

## Demo

See the tool in action:

![Port Resolver Demo](/port-resolver/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/oSg2LQQuexnWqmNSdZpuCL6pX)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/port-resolver" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

## Use Cases

- **Parallel Test Workers:** Running tests with `--workers=4` or similar
- **Integration Tests:** Tests that spawn servers or databases
- **E2E Tests:** Multiple services running simultaneously
- **Development:** Multiple microservices in local development
- **CI/CD Pipelines:** Avoiding port conflicts in shared runners

## Why Port Resolver?

The common problem with hardcoded ports:

```
Error: listen EADDRINUSE: address already in use :::3000
```

This happens because:
- Tests hardcode port numbers (`:3000`, `:8080`)
- Multiple test workers race for the same ports
- Flaky "works locally, fails in CI" behavior

Port Resolver solves this by providing a centralized registry with atomic operations.

## API Overview

### Module-Level (v0.2.0)

| Function | Description |
|----------|-------------|
| `getPort(options?)` | Convenience wrapper for single port allocation |
| `getPorts(count, options?)` | Batch allocation with individual tags or shared tag |

### PortManager Class (v0.2.0)

| Method | Description |
|--------|-------------|
| `allocate(tag?)` | Allocate port and track by tag (prevents duplicate tags) |
| `allocateMultiple(count, tag?)` | Allocate multiple ports atomically |
| `release(tagOrPort)` | Release by tag or port number (idempotent) |
| `releaseAll()` | Release all managed ports |
| `getAllocations()` | Get all tracked allocations |
| `get(tag)` | Get specific allocation by tag |

### PortResolver Class

| Method | Description |
|--------|-------------|
| `get(config?)` | Allocate a single port |
| `getMultiple(count, config?)` | Allocate multiple ports |
| `reserveRange(options)` | Reserve contiguous port range (v0.2.0) |
| `getPortInRange(options)` | Get any port within specific bounds (v0.2.0) |
| `release(port)` | Release an allocated port |
| `releaseAll(tag?)` | Release all ports (optionally by tag) |
| `list()` | List all allocated ports |
| `clean()` | Remove stale entries |
| `status()` | Get registry statistics |

## Documentation

- [Getting Started](/tools/port-resolver/getting-started) - Installation and setup
- [CLI Usage](/tools/port-resolver/cli-usage) - Command-line interface
- [Library Usage](/tools/port-resolver/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/port-resolver/examples) - Real-world patterns
- [API Reference](/tools/port-resolver/api-reference) - Full API documentation

## Related Tools

- [File-Based Semaphore (TS)](/tools/file-based-semaphore-ts/) - Atomic locking (used internally)
- [Test Flakiness Detector](/tools/test-flakiness-detector/) - Detect unreliable tests

## License

MIT License - see repository for details.
