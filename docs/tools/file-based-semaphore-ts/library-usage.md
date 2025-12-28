# Library Usage

Use File-Based Semaphore as a TypeScript/JavaScript library for programmatic access.

## Import

```typescript
import {
  Semaphore,
  serializeLockInfo,
  parseLockInfo,
  type LockInfo,
  type SemaphoreResult,
  type SemaphoreError,
  type SemaphoreConfig
} from '@tuulbelt/file-based-semaphore-ts';
```

For local development in the monorepo:

```typescript
import { Semaphore } from '../file-based-semaphore-ts/src/index.js';
```

## Creating a Semaphore

```typescript
// Basic usage
const semaphore = new Semaphore('/tmp/my-lock.lock');

// With configuration
const semaphore = new Semaphore('/tmp/my-lock.lock', {
  staleTimeout: 3600000,  // 1 hour (default)
  retryInterval: 100,      // 100ms retry interval (default)
  maxTagLength: 10000      // Maximum tag length (default)
});
```

## Acquiring Locks

### Non-Blocking (tryAcquire)

Returns immediately with success or failure:

```typescript
const result = semaphore.tryAcquire('my-process');

if (result.ok) {
  console.log('Lock acquired!');
  console.log('PID:', result.value.pid);
  console.log('Timestamp:', result.value.timestamp);
  console.log('Tag:', result.value.tag);
} else {
  console.log('Failed:', result.error.type);
  console.log('Holder PID:', result.error.holderPid);
}
```

### Blocking with Timeout (acquire)

Waits until lock is available or timeout:

```typescript
const result = await semaphore.acquire({
  timeout: 5000,  // 5 seconds
  tag: 'my-process'
});

if (result.ok) {
  console.log('Lock acquired after waiting');
} else {
  console.log('Timeout:', result.error.message);
}
```

## Releasing Locks

```typescript
// Release lock you own
const result = semaphore.release();

if (result.ok) {
  console.log('Lock released');
} else {
  console.log('Release failed:', result.error.message);
}

// Force release (use with caution)
const result = semaphore.release(true);
```

## Checking Status

```typescript
const status = semaphore.status();

console.log('Locked:', status.locked);
console.log('Stale:', status.isStale);
console.log('Owned by me:', status.isOwnedByCurrentProcess);

if (status.info) {
  console.log('Holder PID:', status.info.pid);
  console.log('Acquired at:', new Date(status.info.timestamp * 1000));
  console.log('Tag:', status.info.tag);
}
```

## Getting Lock Info

```typescript
const info = semaphore.getLockInfo();

if (info) {
  console.log('Lock exists:', info);
} else {
  console.log('No lock file');
}
```

## Cleaning Stale Locks

```typescript
const cleaned = semaphore.cleanStale();

if (cleaned) {
  console.log('Stale lock removed');
} else {
  console.log('No stale lock to clean');
}
```

## Result Pattern

All operations return a `SemaphoreResult<T>`:

```typescript
type SemaphoreResult<T> =
  | { ok: true; value: T }
  | { ok: false; error: SemaphoreError };
```

This enables explicit error handling without exceptions:

```typescript
const result = semaphore.tryAcquire();

if (result.ok) {
  // result.value is the lock info
  processWithLock(result.value);
} else {
  // result.error contains error details
  handleError(result.error);
}
```

## Error Types

```typescript
interface SemaphoreError {
  type:
    | 'ALREADY_LOCKED'
    | 'NOT_LOCKED'
    | 'PERMISSION_DENIED'
    | 'TIMEOUT'
    | 'IO_ERROR'
    | 'PATH_TRAVERSAL'
    | 'PARSE_ERROR';
  message: string;
  holderPid?: number;  // For ALREADY_LOCKED errors
  holderTag?: string;  // For ALREADY_LOCKED errors
}
```

## Configuration Options

```typescript
interface SemaphoreConfig {
  // Timeout for stale lock detection (ms)
  // Set to null to disable stale detection
  staleTimeout?: number | null;  // Default: 3600000 (1 hour)

  // Retry interval when waiting for lock (ms)
  retryInterval?: number;  // Default: 100

  // Maximum tag length to prevent resource exhaustion
  maxTagLength?: number;  // Default: 10000
}
```

## Lock File Serialization

You can also work with lock files directly:

```typescript
import { serializeLockInfo, parseLockInfo, type LockInfo } from './index.js';

// Create lock info
const info: LockInfo = {
  pid: process.pid,
  timestamp: Math.floor(Date.now() / 1000),
  tag: 'my-process'
};

// Serialize to string
const content = serializeLockInfo(info);
console.log(content);
// pid=12345
// timestamp=1735420800
// tag=my-process

// Parse from string
const result = parseLockInfo(content);
if (result.ok) {
  console.log('Parsed:', result.value);
}
```

## Best Practices

### Always Use try/finally

```typescript
const result = semaphore.tryAcquire('critical-section');
if (result.ok) {
  try {
    await doCriticalWork();
  } finally {
    semaphore.release();
  }
}
```

### Handle All Error Cases

```typescript
const result = semaphore.tryAcquire();

if (!result.ok) {
  switch (result.error.type) {
    case 'ALREADY_LOCKED':
      console.log(`Lock held by PID ${result.error.holderPid}`);
      break;
    case 'IO_ERROR':
      console.error('File system error:', result.error.message);
      break;
    case 'PATH_TRAVERSAL':
      console.error('Security error:', result.error.message);
      break;
    default:
      console.error('Unknown error:', result.error);
  }
}
```

### Use Descriptive Tags

```typescript
const result = semaphore.tryAcquire(`build-${process.cwd()}`);
```

### Clean Stale Locks Periodically

```typescript
// Before attempting to acquire
semaphore.cleanStale();
const result = semaphore.tryAcquire();
```

## TypeScript Types

```typescript
import type {
  Semaphore,
  LockInfo,
  SemaphoreResult,
  SemaphoreError,
  SemaphoreConfig,
  SemaphoreStatus
} from '@tuulbelt/file-based-semaphore-ts';
```
