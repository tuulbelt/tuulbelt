# Library Usage

The `port-resolver` library provides a TypeScript API for port allocation.

## Basic Usage

```typescript
import { PortResolver } from '@tuulbelt/port-resolver';

const resolver = new PortResolver();

// Allocate a port
const result = await resolver.get({ tag: 'my-server' });

if (result.ok) {
  const { port, entry } = result.value;
  console.log(`Allocated port: ${port}`);
  console.log(`PID: ${entry.pid}`);
  console.log(`Tag: ${entry.tag}`);
}
```

## Result Pattern

All methods return a `Result<T, E>` type:

```typescript
type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };
```

This pattern ensures explicit error handling:

```typescript
const result = await resolver.get();

if (!result.ok) {
  console.error('Failed to allocate port:', result.error.message);
  return;
}

// TypeScript knows result.value exists here
const port = result.value.port;
```

## Constructor Options

```typescript
const resolver = new PortResolver({
  registryPath: '/tmp/my-ports.json',  // Custom registry location
  minPort: 8000,                        // Minimum port (default: 10000)
  maxPort: 9000,                        // Maximum port (default: 65535)
});
```

## Methods

### `get(config?)` - Allocate Single Port

```typescript
interface PortConfig {
  minPort?: number;
  maxPort?: number;
  tag?: string;
}

const result = await resolver.get({
  tag: 'api-server',
  minPort: 8000,
  maxPort: 9000,
});

if (result.ok) {
  console.log(result.value.port);      // e.g., 8042
  console.log(result.value.entry.pid); // Process ID
  console.log(result.value.entry.tag); // 'api-server'
}
```

### `getMultiple(count, config?)` - Allocate Multiple Ports

```typescript
const result = await resolver.getMultiple(3, { tag: 'microservices' });

if (result.ok) {
  for (const allocation of result.value) {
    console.log(`Port ${allocation.port} for ${allocation.entry.tag}`);
  }
}
```

### `release(port)` - Release Single Port

```typescript
const released = await resolver.release(8042);

if (released.ok) {
  console.log('Port released');
} else {
  console.error('Release failed:', released.error.message);
}
```

### `releaseAll(tag?)` - Release Multiple Ports

```typescript
// Release all ports for current process
await resolver.releaseAll();

// Release only ports with specific tag
await resolver.releaseAll('api-server');
```

### `list()` - List All Allocations

```typescript
const result = await resolver.list();

if (result.ok) {
  for (const entry of result.value) {
    console.log(`Port ${entry.port}: ${entry.tag} (PID ${entry.pid})`);
  }
}
```

### `clean()` - Remove Stale Entries

```typescript
const result = await resolver.clean();

if (result.ok) {
  console.log(`Cleaned ${result.value} stale entries`);
}
```

### `status()` - Get Registry Statistics

```typescript
const result = await resolver.status();

if (result.ok) {
  console.log(`Total entries: ${result.value.totalEntries}`);
  console.log(`Active: ${result.value.activeEntries}`);
  console.log(`Stale: ${result.value.staleEntries}`);
}
```

### `clear()` - Clear Entire Registry

```typescript
await resolver.clear();
```

## Test Framework Integration

### Node.js Test Runner

```typescript
import { test, before, after } from 'node:test';
import { PortResolver } from '@tuulbelt/port-resolver';

const resolver = new PortResolver();
let testPort: number;

before(async () => {
  const result = await resolver.get({ tag: 'node-test' });
  if (!result.ok) throw result.error;
  testPort = result.value.port;
});

after(async () => {
  await resolver.releaseAll('node-test');
});

test('server starts on allocated port', async () => {
  // Use testPort for your server
});
```

### Jest

```typescript
import { PortResolver } from '@tuulbelt/port-resolver';

const resolver = new PortResolver();
let testPort: number;

beforeAll(async () => {
  const result = await resolver.get({ tag: 'jest-test' });
  if (!result.ok) throw result.error;
  testPort = result.value.port;
});

afterAll(async () => {
  await resolver.releaseAll('jest-test');
});

describe('API Server', () => {
  test('responds on allocated port', async () => {
    // Use testPort
  });
});
```

### Vitest

```typescript
import { beforeAll, afterAll, describe, test } from 'vitest';
import { PortResolver } from '@tuulbelt/port-resolver';

const resolver = new PortResolver();
let testPort: number;

beforeAll(async () => {
  const result = await resolver.get({ tag: 'vitest' });
  if (!result.ok) throw result.error;
  testPort = result.value.port;
});

afterAll(async () => {
  await resolver.releaseAll('vitest');
});
```

## Utility Functions

### `isPortAvailable(port)`

Check if a specific port is available:

```typescript
import { isPortAvailable } from '@tuulbelt/port-resolver';

const available = await isPortAvailable(3000);
console.log(available); // true or false
```

### `findAvailablePort(min, max)`

Find any available port in a range:

```typescript
import { findAvailablePort } from '@tuulbelt/port-resolver';

const result = await findAvailablePort(8000, 9000);
if (result.ok) {
  console.log(`Found port: ${result.value}`);
}
```

## Library Composition

`portres` uses [file-based-semaphore-ts](/tools/file-based-semaphore-ts/) (`semats`) as a library dependency for atomic registry operations. This follows [PRINCIPLES.md Exception 2](/guide/principles#zero-external-dependencies).

```typescript
// Semaphore is used internally - no configuration needed
const resolver = new PortResolver();

// All registry operations are atomic via semats
const result = await resolver.get({ tag: 'my-server' });
```

The semaphore ensures concurrent port allocations from multiple processes never corrupt the registry file.

## Error Handling Patterns

### Retry on Failure

```typescript
async function getPortWithRetry(
  resolver: PortResolver,
  maxAttempts = 3
): Promise<number> {
  for (let i = 0; i < maxAttempts; i++) {
    const result = await resolver.get({ tag: 'retry-demo' });
    if (result.ok) return result.value.port;

    // Clean stale entries and retry
    await resolver.clean();
  }
  throw new Error('Failed to allocate port after retries');
}
```

### Graceful Cleanup

```typescript
const resolver = new PortResolver();
const allocatedPorts: number[] = [];

try {
  const result = await resolver.get({ tag: 'graceful' });
  if (result.ok) {
    allocatedPorts.push(result.value.port);
    // Do work...
  }
} finally {
  // Always cleanup
  for (const port of allocatedPorts) {
    await resolver.release(port);
  }
}
```

## Type Definitions

```typescript
interface PortEntry {
  port: number;
  pid: number;
  tag?: string;
  allocatedAt: string; // ISO timestamp
}

interface PortAllocation {
  port: number;
  entry: PortEntry;
}

interface RegistryStatus {
  totalEntries: number;
  activeEntries: number;
  staleEntries: number;
  registryPath: string;
}
```
