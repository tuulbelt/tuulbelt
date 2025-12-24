# API Reference

Complete API documentation for CLI Progress Reporting.

## Functions

For detailed usage examples, see [Library Usage](/tools/cli-progress-reporting/library-usage).

### Core Functions

- [`init()`](#init) - Initialize progress tracker
- [`increment()`](#increment) - Increment progress
- [`set()`](#set) - Set absolute progress value
- [`get()`](#get) - Retrieve current state
- [`finish()`](#finish) - Mark as complete
- [`clear()`](#clear) - Remove progress file
- [`formatProgress()`](#formatprogress) - Format state as string

### Types

- [`ProgressConfig`](#progressconfig) - Configuration options
- [`ProgressState`](#progressstate) - Progress state object
- [`Result<T>`](#resultt) - Result type for error handling

---

## Functions

### `init()`

Initialize a new progress tracker.

```typescript
function init(
  total: number,
  message: string,
  config?: ProgressConfig
): Result<ProgressState>
```

See [Library Usage - init()](/tools/cli-progress-reporting/library-usage#init) for examples.

---

### `increment()`

Increment progress by a specified amount.

```typescript
function increment(
  amount?: number,
  message?: string,
  config?: ProgressConfig
): Result<ProgressState>
```

See [Library Usage - increment()](/tools/cli-progress-reporting/library-usage#increment) for examples.

---

### `set()`

Set progress to an absolute value.

```typescript
function set(
  current: number,
  message?: string,
  config?: ProgressConfig
): Result<ProgressState>
```

See [Library Usage - set()](/tools/cli-progress-reporting/library-usage#set) for examples.

---

### `get()`

Retrieve current progress state.

```typescript
function get(config?: ProgressConfig): Result<ProgressState>
```

See [Library Usage - get()](/tools/cli-progress-reporting/library-usage#get) for examples.

---

### `finish()`

Mark progress as complete (100%).

```typescript
function finish(
  message?: string,
  config?: ProgressConfig
): Result<ProgressState>
```

See [Library Usage - finish()](/tools/cli-progress-reporting/library-usage#finish) for examples.

---

### `clear()`

Remove the progress tracker file.

```typescript
function clear(config?: ProgressConfig): Result<void>
```

See [Library Usage - clear()](/tools/cli-progress-reporting/library-usage#clear) for examples.

---

### `formatProgress()`

Format progress state as a human-readable string.

```typescript
function formatProgress(state: ProgressState): string
```

See [Library Usage - formatProgress()](/tools/cli-progress-reporting/library-usage#formatprogress) for examples.

---

## Types

### `ProgressConfig`

Configuration options for progress tracking.

```typescript
interface ProgressConfig {
  /** Unique identifier for this progress tracker (default: "default") */
  id?: string;

  /** Custom storage directory path (default: system temp directory) */
  filePath?: string;
}
```

**Properties:**
- `id` - Unique identifier to track multiple operations independently
- `filePath` - Directory where progress JSON files are stored

---

### `ProgressState`

Current state of a progress tracker.

```typescript
interface ProgressState {
  /** Current progress value */
  current: number;

  /** Total number of items to process */
  total: number;

  /** Calculated percentage (0-100) */
  percentage: number;

  /** Current status message */
  message: string;

  /** Whether tracking is complete */
  complete: boolean;

  /** Unix timestamp (ms) when tracker was initialized */
  startTime: number;

  /** Unix timestamp (ms) of last update */
  lastUpdate: number;
}
```

---

### `Result<T>`

Type-safe result type for error handling.

```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: string };
```

**Success:**
- `ok: true`
- `value: T` - The success value

**Error:**
- `ok: false`
- `error: string` - Error message

**Usage:**
```typescript
const result = get({ id: 'task1' });

if (result.ok) {
  console.log(result.value.percentage);
} else {
  console.error('Error:', result.error);
}
```

---

## Constants

### Default ID

```typescript
const DEFAULT_ID = 'default';
```

Used when no `id` is specified in config.

### File Naming

Progress files are named: `progress-{id}.json`

### Storage Location

**Default:** System temp directory
- Linux/macOS: `/tmp/`
- Windows: `%TEMP%`

**Custom:** Specified via `config.filePath`

---

## Error Messages

Common error messages returned in `Result.error`:

| Error | Cause |
|-------|-------|
| `"Total must be a valid number greater than 0"` | Invalid total in `init()` |
| `"Progress not initialized"` | Calling functions before `init()` |
| `"Failed to read progress file"` | File not found or corrupted |
| `"Invalid progress state"` | JSON parse error |
| `"Amount must be greater than 0"` | Invalid increment amount |
| `"Current value must be >= 0"` | Invalid set value |

---

## Implementation Details

### Atomic Writes

All writes use a write-then-rename pattern:
1. Write new state to temporary file
2. Atomically rename to target file

This ensures concurrent processes never see partial updates.

### Concurrency Safety

Multiple processes can safely update the same tracker:
- Atomic file operations prevent corruption
- Read-modify-write cycle is protected
- Last write wins (no conflict resolution needed)

### State Persistence

Progress state is stored as JSON files:
```json
{
  "current": 45,
  "total": 100,
  "percentage": 45,
  "message": "Processing items",
  "complete": false,
  "startTime": 1234567890,
  "lastUpdate": 1234567895
}
```

---

For usage examples, see:
- [Getting Started](/tools/cli-progress-reporting/getting-started)
- [Library Usage](/tools/cli-progress-reporting/library-usage)
- [Examples](/tools/cli-progress-reporting/examples)
