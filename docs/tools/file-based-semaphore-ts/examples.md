# Examples

Real-world usage patterns for File-Based Semaphore (TypeScript).

## Build System Lock

Prevent concurrent builds from conflicting:

```bash
#!/bin/bash
set -e

LOCKFILE="/tmp/project-build.lock"

# Acquire lock with timeout
if semats acquire "$LOCKFILE" --timeout 60000 --tag "build $(date)"; then
    trap 'semats release "$LOCKFILE"' EXIT

    npm run build
    echo "Build complete"
else
    echo "Build already in progress"
    exit 1
fi
```

## CI/CD Pipeline Coordination

Coordinate parallel test runners:

```typescript
import { Semaphore } from '../file-based-semaphore-ts/src/index.js';

async function runTestSuite(suite: string) {
  const lockPath = `/tmp/test-${suite}.lock`;
  const semaphore = new Semaphore(lockPath);

  // Wait up to 5 minutes for lock
  const result = await semaphore.acquire({
    timeout: 300000,
    tag: `test-runner-${process.pid}`
  });

  if (!result.ok) {
    throw new Error(`Failed to acquire lock: ${result.error.message}`);
  }

  try {
    console.log(`Running test suite: ${suite}`);
    await executeTests(suite);
  } finally {
    semaphore.release();
  }
}
```

## Database Migration Lock

Ensure only one migration runs at a time:

```typescript
import { Semaphore } from '../file-based-semaphore-ts/src/index.js';

class MigrationRunner {
  private semaphore: Semaphore;

  constructor(lockPath = '/tmp/db-migration.lock') {
    this.semaphore = new Semaphore(lockPath, {
      staleTimeout: 7200000 // 2 hours for long migrations
    });
  }

  async runMigration(migrationName: string): Promise<void> {
    const result = this.semaphore.tryAcquire(`migration-${migrationName}`);

    if (!result.ok) {
      const status = this.semaphore.status();
      throw new Error(
        `Migration already running: ${status.info?.tag || 'unknown'}`
      );
    }

    try {
      console.log(`Running migration: ${migrationName}`);
      await this.executeMigration(migrationName);
    } finally {
      this.semaphore.release();
    }
  }

  private async executeMigration(name: string): Promise<void> {
    // Migration logic here
  }
}
```

## Port Allocation

Lock a port before binding (for test runners):

```typescript
import { Semaphore } from '../file-based-semaphore-ts/src/index.js';
import { createServer } from 'node:net';

async function acquirePort(basePort: number): Promise<number> {
  for (let port = basePort; port < basePort + 100; port++) {
    const lockPath = `/tmp/port-${port}.lock`;
    const semaphore = new Semaphore(lockPath);

    const result = semaphore.tryAcquire(`test-server-${process.pid}`);
    if (result.ok) {
      // Verify port is actually available
      const available = await isPortAvailable(port);
      if (available) {
        return port;
      }
      // Port taken, release lock and try next
      semaphore.release();
    }
  }

  throw new Error('No available ports');
}

function isPortAvailable(port: number): Promise<boolean> {
  return new Promise((resolve) => {
    const server = createServer();
    server.once('error', () => resolve(false));
    server.once('listening', () => {
      server.close(() => resolve(true));
    });
    server.listen(port);
  });
}
```

## Parallel File Processing

Serialize access to shared output file:

```typescript
import { Semaphore } from '../file-based-semaphore-ts/src/index.js';
import { appendFileSync } from 'node:fs';

async function appendToSharedLog(message: string): Promise<void> {
  const semaphore = new Semaphore('/tmp/shared-log.lock');

  const result = await semaphore.acquire({
    timeout: 5000,
    tag: `worker-${process.pid}`
  });

  if (!result.ok) {
    console.error('Failed to acquire lock for logging');
    return;
  }

  try {
    appendFileSync('/tmp/shared-log.txt', `${new Date().toISOString()} ${message}\n`);
  } finally {
    semaphore.release();
  }
}
```

## Multi-Process Worker Queue

Coordinate workers processing a shared queue:

```typescript
import { Semaphore } from '../file-based-semaphore-ts/src/index.js';

class WorkerQueue {
  private semaphore: Semaphore;

  constructor(queueName: string) {
    this.semaphore = new Semaphore(`/tmp/queue-${queueName}.lock`);
  }

  async processNext(): Promise<void> {
    // Short timeout - other workers can pick up work
    const result = await this.semaphore.acquire({
      timeout: 100,
      tag: `worker-${process.pid}`
    });

    if (!result.ok) {
      // Another worker is processing
      return;
    }

    try {
      const item = await this.dequeueItem();
      if (item) {
        await this.processItem(item);
      }
    } finally {
      this.semaphore.release();
    }
  }

  private async dequeueItem(): Promise<unknown> {
    // Dequeue logic
    return null;
  }

  private async processItem(item: unknown): Promise<void> {
    // Process logic
  }
}
```

## Stale Lock Recovery

Clean up after crashed processes:

```typescript
import { Semaphore } from '../file-based-semaphore-ts/src/index.js';

async function robustAcquire(lockPath: string): Promise<Semaphore> {
  const semaphore = new Semaphore(lockPath);

  // Try to clean stale locks first
  semaphore.cleanStale();

  // Now try to acquire
  const result = semaphore.tryAcquire('my-process');

  if (result.ok) {
    return semaphore;
  }

  // Check if lock is stale
  const status = semaphore.status();
  if (status.isStale) {
    // Force release stale lock
    semaphore.release(true);

    // Try again
    const retry = semaphore.tryAcquire('my-process');
    if (retry.ok) {
      return semaphore;
    }
  }

  throw new Error(`Lock held by PID ${result.error.holderPid}`);
}
```

## Debugging Lock Issues

Inspect lock state:

```bash
# Check lock status
semats status /tmp/my-lock.lock --json | jq

# View raw lock file
cat /tmp/my-lock.lock

# Clean orphaned temp files
semats clean /tmp/my-lock.lock --verbose
```

## Cross-Language Coordination

Works with Rust `sema` tool:

```bash
# TypeScript acquires
semats try-acquire /tmp/shared.lock --tag "ts-process"

# Rust checks status
sema status /tmp/shared.lock

# Rust releases (with force)
sema release /tmp/shared.lock --force
```
