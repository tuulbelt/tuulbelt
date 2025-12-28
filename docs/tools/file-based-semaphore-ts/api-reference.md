# API Reference

Complete API documentation for File-Based Semaphore (TypeScript).

## Classes

### Semaphore

Main class for file-based process synchronization.

```typescript
class Semaphore {
  constructor(lockPath: string, config?: SemaphoreConfig);

  tryAcquire(tag?: string): SemaphoreResult<LockInfo>;
  acquire(options?: AcquireOptions): Promise<SemaphoreResult<LockInfo>>;
  release(force?: boolean): SemaphoreResult<void>;
  status(): SemaphoreStatus;
  getLockInfo(): LockInfo | null;
  cleanStale(): boolean;
}
```

#### Constructor

```typescript
new Semaphore(lockPath: string, config?: SemaphoreConfig)
```

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `lockPath` | `string` | Absolute path to the lock file |
| `config` | `SemaphoreConfig` | Optional configuration |

**Throws:**
- If path contains `..` (path traversal)
- If parent directory doesn't exist

#### tryAcquire

```typescript
tryAcquire(tag?: string): SemaphoreResult<LockInfo>
```

Attempt to acquire the lock immediately (non-blocking).

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `tag` | `string` | Optional description for the lock |

**Returns:** `SemaphoreResult<LockInfo>`

**Example:**

```typescript
const result = semaphore.tryAcquire('my-process');
if (result.ok) {
  console.log('Acquired:', result.value);
}
```

#### acquire

```typescript
acquire(options?: AcquireOptions): Promise<SemaphoreResult<LockInfo>>
```

Wait for the lock to become available (blocking with timeout).

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `options.timeout` | `number` | Maximum wait time in milliseconds |
| `options.tag` | `string` | Optional description for the lock |

**Returns:** `Promise<SemaphoreResult<LockInfo>>`

**Example:**

```typescript
const result = await semaphore.acquire({ timeout: 5000, tag: 'my-process' });
```

#### release

```typescript
release(force?: boolean): SemaphoreResult<void>
```

Release the lock.

**Parameters:**

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `force` | `boolean` | `false` | Release even if not owner |

**Returns:** `SemaphoreResult<void>`

**Example:**

```typescript
const result = semaphore.release();
if (!result.ok) {
  console.error('Failed:', result.error.message);
}
```

#### status

```typescript
status(): SemaphoreStatus
```

Get the current lock status.

**Returns:** `SemaphoreStatus`

**Example:**

```typescript
const status = semaphore.status();
console.log('Locked:', status.locked);
console.log('Owner PID:', status.info?.pid);
```

#### getLockInfo

```typescript
getLockInfo(): LockInfo | null
```

Read lock file and parse its contents.

**Returns:** `LockInfo | null` - Lock info or null if no lock

**Example:**

```typescript
const info = semaphore.getLockInfo();
if (info) {
  console.log('Lock held by PID:', info.pid);
}
```

#### cleanStale

```typescript
cleanStale(): boolean
```

Clean stale locks and orphaned temp files.

**Returns:** `boolean` - True if anything was cleaned

**Example:**

```typescript
if (semaphore.cleanStale()) {
  console.log('Cleaned stale lock');
}
```

## Types

### LockInfo

Information about a held lock.

```typescript
interface LockInfo {
  pid: number;       // Process ID holding the lock
  timestamp: number; // Unix timestamp when acquired (seconds)
  tag?: string;      // Optional description
}
```

### SemaphoreConfig

Configuration options for Semaphore.

```typescript
interface SemaphoreConfig {
  staleTimeout?: number | null;  // Stale detection timeout (ms)
  retryInterval?: number;        // Retry interval for acquire (ms)
  maxTagLength?: number;         // Maximum tag length
}
```

**Defaults:**

| Option | Default | Description |
|--------|---------|-------------|
| `staleTimeout` | `3600000` | 1 hour |
| `retryInterval` | `100` | 100 milliseconds |
| `maxTagLength` | `10000` | 10,000 characters |

### SemaphoreStatus

Current lock status information.

```typescript
interface SemaphoreStatus {
  locked: boolean;                  // Whether lock file exists
  info: LockInfo | null;           // Lock info if locked
  isStale: boolean;                // Whether lock is stale
  isOwnedByCurrentProcess: boolean; // Whether current process owns lock
}
```

### SemaphoreResult

Result type for all operations.

```typescript
type SemaphoreResult<T> =
  | { ok: true; value: T }
  | { ok: false; error: SemaphoreError };
```

### SemaphoreError

Error information.

```typescript
interface SemaphoreError {
  type: SemaphoreErrorType;
  message: string;
  holderPid?: number;   // For ALREADY_LOCKED
  holderTag?: string;   // For ALREADY_LOCKED
}

type SemaphoreErrorType =
  | 'ALREADY_LOCKED'
  | 'NOT_LOCKED'
  | 'PERMISSION_DENIED'
  | 'TIMEOUT'
  | 'IO_ERROR'
  | 'PATH_TRAVERSAL'
  | 'PARSE_ERROR';
```

### AcquireOptions

Options for blocking acquire.

```typescript
interface AcquireOptions {
  timeout?: number;  // Max wait time in milliseconds
  tag?: string;      // Optional lock description
}
```

## Utility Functions

### serializeLockInfo

```typescript
function serializeLockInfo(info: LockInfo): string
```

Serialize lock info to file format.

**Example:**

```typescript
const content = serializeLockInfo({
  pid: 12345,
  timestamp: 1735420800,
  tag: 'my-process'
});
// Result:
// pid=12345
// timestamp=1735420800
// tag=my-process
```

### parseLockInfo

```typescript
function parseLockInfo(content: string): SemaphoreResult<LockInfo>
```

Parse lock file content.

**Example:**

```typescript
const result = parseLockInfo('pid=12345\ntimestamp=1735420800\n');
if (result.ok) {
  console.log('PID:', result.value.pid);
}
```

## Error Handling

All operations return `SemaphoreResult<T>` for explicit error handling:

```typescript
const result = semaphore.tryAcquire();

if (result.ok) {
  // Success path
  console.log('Lock acquired:', result.value);
} else {
  // Error path
  switch (result.error.type) {
    case 'ALREADY_LOCKED':
      console.log(`Held by PID ${result.error.holderPid}`);
      break;
    case 'PATH_TRAVERSAL':
      console.log('Security error:', result.error.message);
      break;
    case 'IO_ERROR':
      console.log('File system error:', result.error.message);
      break;
    default:
      console.log('Error:', result.error.message);
  }
}
```

## CLI Functions

### runCli

```typescript
function runCli(args: string[]): number
```

Execute CLI with given arguments.

**Returns:** Exit code (0 for success, 1 for error)

**Example:**

```typescript
const exitCode = runCli(['try-acquire', '/tmp/my.lock', '--tag', 'test']);
```

### parseArgs

```typescript
function parseArgs(args: string[]): ParsedArgs
```

Parse CLI arguments.

**Example:**

```typescript
const parsed = parseArgs(['try-acquire', '/tmp/my.lock', '--json']);
console.log(parsed.command);  // 'try-acquire'
console.log(parsed.path);     // '/tmp/my.lock'
console.log(parsed.json);     // true
```
