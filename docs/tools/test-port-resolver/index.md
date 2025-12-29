# Test Port Resolver / `portres`

Concurrent test port allocation - avoid port conflicts in parallel tests.

## Problem

When running tests in parallel (e.g., with `--workers=4`), multiple test files may try to use the same ports, causing:

```
Error: listen EADDRINUSE: address already in use :::3000
```

This happens because:
- Tests hardcode port numbers (`:3000`, `:8080`)
- Multiple test workers race for the same ports
- Flaky "works locally, fails in CI" behavior

## Solution

`portres` provides a file-based registry for port allocation:

```typescript
import { PortResolver } from '@tuulbelt/test-port-resolver';

const resolver = new PortResolver();
const result = await resolver.get({ tag: 'my-server' });
if (result.ok) {
  const port = result.value.port; // Guaranteed unique
  // Start your server on this port
}
```

## Features

- **Concurrent-safe** - File-based registry with atomic operations
- **Port verification** - Confirms ports are actually available via TCP bind test
- **Automatic cleanup** - Stale entries from crashed processes are detected
- **Semaphore integration** - Uses file-based-semaphore-ts when available
- **Zero dependencies** - Pure Node.js implementation

## Quick Start

```bash
cd test-port-resolver && npm install

# CLI usage
portres get --tag "api-server"
# Output: 54321

# Release when done
portres release --port 54321
```

## Demo

See the tool in action:

![Test Port Resolver Demo](/test-port-resolver/demo.gif)

**[View interactive recording on asciinema.org](#)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/test-port-resolver" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

## When to Use

- Running multiple test workers in parallel
- Integration tests that spawn servers
- E2E tests with multiple services
- Any scenario where tests compete for ports

## API Overview

| Method | Description |
|--------|-------------|
| `get(config?)` | Allocate a single port |
| `getMultiple(count, config?)` | Allocate multiple ports |
| `release(port)` | Release an allocated port |
| `releaseAll(tag?)` | Release all ports (optionally by tag) |
| `list()` | List all allocated ports |
| `clean()` | Remove stale entries |
| `status()` | Get registry statistics |

## Learn More

- [Getting Started](./getting-started) - Installation and setup
- [CLI Usage](./cli-usage) - Command-line interface
- [Library Usage](./library-usage) - TypeScript/JavaScript API
- [Examples](./examples) - Real-world patterns
- [API Reference](./api-reference) - Full API documentation
