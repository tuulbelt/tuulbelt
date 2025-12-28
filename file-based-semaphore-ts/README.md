# File-Based Semaphore (TypeScript) / `semats`

[![Tests](https://github.com/tuulbelt/tuulbelt/actions/workflows/test-all-tools.yml/badge.svg)](https://github.com/tuulbelt/tuulbelt/actions/workflows/test-all-tools.yml)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Node](https://img.shields.io/badge/node-%3E%3D18.0.0-brightgreen)
![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success)
![Tests](https://img.shields.io/badge/tests-160%20passing-success)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Cross-platform file-based semaphore for process synchronization in Node.js.

## Problem

When multiple processes need to coordinate access to a shared resource (files, ports, build artifacts), you need a cross-platform locking mechanism that:
- Works without external dependencies (no Redis, no database)
- Survives process crashes (stale lock detection)
- Works across different programming languages (compatible file format)

## Features

- **Zero runtime dependencies** - Uses only Node.js built-ins
- **Cross-platform** - Works on Linux, macOS, and Windows
- **CLI and library** - Use from command line or import as a module
- **Atomic operations** - Uses temp file + rename for atomic lock creation
- **Stale lock detection** - Automatically detects and cleans orphaned locks
- **Compatible format** - Uses same lock file format as Rust `sema` tool
- **160 tests** - Comprehensive test coverage including security tests

## Installation

```bash
cd file-based-semaphore-ts
npm install  # Install dev dependencies only
```

No runtime dependencies - this tool uses only Node.js standard library.

**CLI names** - both short and long forms work:
- Short (recommended): `semats`
- Long: `file-semaphore-ts`

**Recommended setup** - install globally for easy access:

```bash
npm link  # Enable the 'semats' command globally
semats --help
```

For local development without global install:

```bash
npx tsx src/index.ts --help
```

## CLI Usage

### Commands

```bash
# Try to acquire a lock (non-blocking)
semats try-acquire /tmp/my-lock.lock

# Acquire with optional tag
semats try-acquire /tmp/my-lock.lock --tag "build process"

# Acquire with blocking/timeout
semats acquire /tmp/my-lock.lock --timeout 5000

# Check lock status
semats status /tmp/my-lock.lock

# Release a lock
semats release /tmp/my-lock.lock

# Force release (even if held by another process)
semats release /tmp/my-lock.lock --force

# Clean stale locks and orphaned temp files
semats clean /tmp/my-lock.lock
```

### Options

```
-t, --tag <tag>       Tag/description for the lock
--timeout <ms>        Timeout in milliseconds (for acquire)
-f, --force           Force operation (for release)
-j, --json            Output in JSON format
-v, --verbose         Verbose output
-h, --help            Show help message
```

### Examples

```bash
# Acquire lock for a build process
semats acquire /tmp/build.lock --tag "npm build" --timeout 30000
npm run build
semats release /tmp/build.lock

# Check if a lock is active
semats status /tmp/my-lock.lock --json
# Output: {"locked":true,"info":{"pid":12345,"timestamp":1234567890,"tag":"my-process"},...}

# Clean up stale locks
semats clean /tmp/my-lock.lock
```

## Library Usage

```typescript
import { Semaphore } from '@tuulbelt/file-based-semaphore-ts';

// Create a semaphore instance
const semaphore = new Semaphore('/tmp/my-lock.lock');

// Try to acquire (non-blocking)
const result = semaphore.tryAcquire('my-process');
if (result.ok) {
  console.log('Lock acquired');
  try {
    // Do protected work here
  } finally {
    semaphore.release();
  }
} else {
  console.log('Lock already held by:', result.error.holderPid);
}

// Acquire with timeout (blocking)
async function withLock() {
  const result = await semaphore.acquire({ timeout: 5000 });
  if (result.ok) {
    try {
      // Do protected work
    } finally {
      semaphore.release();
    }
  } else {
    console.log('Timeout waiting for lock');
  }
}

// Check status
const status = semaphore.status();
console.log('Locked:', status.locked);
console.log('Stale:', status.isStale);
console.log('Owned by me:', status.isOwnedByCurrentProcess);

// Clean stale locks and orphaned temp files
const cleaned = semaphore.cleanStale();
console.log('Cleaned:', cleaned);
```

### Configuration

```typescript
const semaphore = new Semaphore('/tmp/my-lock.lock', {
  // Timeout for stale lock detection (ms)
  // Set to null to disable stale detection
  staleTimeout: 3600000, // 1 hour (default)

  // Retry interval when waiting for lock (ms)
  retryInterval: 100, // default

  // Maximum tag length to prevent resource exhaustion
  maxTagLength: 10000, // default
});
```

### API Reference

#### `Semaphore`

```typescript
class Semaphore {
  constructor(lockPath: string, config?: SemaphoreConfig);

  tryAcquire(tag?: string): SemaphoreResult<LockInfo>;
  acquire(options?: { timeout?: number; tag?: string }): Promise<SemaphoreResult<LockInfo>>;
  release(force?: boolean): SemaphoreResult<void>;
  status(): { locked: boolean; info: LockInfo | null; isStale: boolean; isOwnedByCurrentProcess: boolean };
  getLockInfo(): LockInfo | null;
  cleanStale(): boolean;
}
```

#### `LockInfo`

```typescript
interface LockInfo {
  pid: number;      // Process ID holding the lock
  timestamp: number; // Unix timestamp when acquired
  tag?: string;     // Optional description
}
```

#### `SemaphoreResult<T>`

```typescript
type SemaphoreResult<T> =
  | { ok: true; value: T }
  | { ok: false; error: SemaphoreError };
```

## Lock File Format

Compatible with the Rust `sema` tool:

```
pid=12345
timestamp=1234567890
tag=my-process
```

## Security

- **Path traversal prevention** - Rejects `..` and null bytes in paths
- **Symlink following** - Resolves symlinks to actual target path
- **Tag sanitization** - Removes all control characters to prevent injection
- **PID verification** - Only the owning process can release without `--force`
- **Atomic file creation** - Uses temp file + rename for race condition mitigation
- **Cryptographic randomness** - Temp files use random names to prevent DoS
- **Restrictive permissions** - Lock files created with mode `0600`
- **Orphan cleanup** - Cleans up abandoned temp files

### Known Limitations

- TOCTOU race condition between checking lock existence and creating it (mitigated but not eliminated by atomic rename)
- Platform-specific behavior for process liveness checks (signal 0 on Unix)
- File permissions depend on filesystem support

## Testing

```bash
npm test              # Run all 160 tests
npm run test:unit     # Core functionality (52 tests)
npm run test:security # Security tests (26 tests)
npm run test:integration # CLI tests (31 tests)
npm run test:edge     # Edge cases (36 tests)
npm run test:stress   # Stress tests (15 tests)
```

## Related Tools

- **[file-based-semaphore](../file-based-semaphore/)** - Rust implementation (same lock file format)
- **[port-conflict-resolver](../port-conflict-resolver/)** - Uses this semaphore for port allocation (coming soon)

Part of the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) collection.

## License

MIT
