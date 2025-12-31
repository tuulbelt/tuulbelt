# Library Usage

Use CLI Progress Reporting programmatically in TypeScript/JavaScript applications.

## Installation

```bash
# Clone the repository
git clone https://github.com/tuulbelt/cli-progress-reporting.git
cd cli-progress-reporting
npm install  # Dev dependencies only
```

## Import

```typescript
import {
  init,
  increment,
  set,
  get,
  finish,
  clear,
  formatProgress
} from './src/index.js';
```

## Basic Usage

```typescript
const config = { id: 'my-task' };

// Initialize
const initResult = init(100, 'Processing items', config);
if (!initResult.ok) {
  console.error('Failed to initialize:', initResult.error);
  process.exit(1);
}

console.log(formatProgress(initResult.value));
// [0%] 0/100 - Processing items (0s)

// Update progress
for (let i = 0; i < 100; i++) {
  const result = increment(1, `Processing item ${i + 1}`, config);
  if (result.ok) {
    console.log(formatProgress(result.value));
  }
}

// Finish
const finalResult = finish('All items processed!', config);
if (finalResult.ok) {
  console.log(formatProgress(finalResult.value));
}
```

## API Reference

### `init()`

Initialize a new progress tracker.

```typescript
function init(
  total: number,
  message: string,
  config?: ProgressConfig
): Result<ProgressState>
```

**Parameters:**
- `total` - Total number of items (must be > 0)
- `message` - Initial status message
- `config` - Optional configuration

**Returns:** `Result<ProgressState>` - Success or error

**Example:**
```typescript
const result = init(100, 'Starting task', { id: 'task1' });
if (result.ok) {
  console.log('Initialized:', result.value);
} else {
  console.error('Error:', result.error);
}
```

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

**Parameters:**
- `amount` - Amount to increment (default: 1)
- `message` - Updated status message (optional)
- `config` - Optional configuration

**Returns:** `Result<ProgressState>`

**Example:**
```typescript
const result = increment(10, 'Batch complete', { id: 'task1' });
```

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

**Parameters:**
- `current` - New current value
- `message` - Updated status message (optional)
- `config` - Optional configuration

**Returns:** `Result<ProgressState>`

**Example:**
```typescript
const result = set(75, 'Almost done', { id: 'task1' });
```

---

### `get()`

Retrieve current progress state.

```typescript
function get(config?: ProgressConfig): Result<ProgressState>
```

**Parameters:**
- `config` - Optional configuration

**Returns:** `Result<ProgressState>`

**Example:**
```typescript
const result = get({ id: 'task1' });
if (result.ok) {
  console.log(`Progress: ${result.value.percentage}%`);
}
```

---

### `finish()`

Mark progress as complete (100%).

```typescript
function finish(
  message?: string,
  config?: ProgressConfig
): Result<ProgressState>
```

**Parameters:**
- `message` - Final completion message (optional)
- `config` - Optional configuration

**Returns:** `Result<ProgressState>`

**Example:**
```typescript
const result = finish('Task complete!', { id: 'task1' });
```

---

### `clear()`

Remove the progress tracker file.

```typescript
function clear(config?: ProgressConfig): Result<void>
```

**Parameters:**
- `config` - Optional configuration

**Returns:** `Result<void>`

**Example:**
```typescript
const result = clear({ id: 'task1' });
if (result.ok) {
  console.log('Progress cleared');
}
```

---

### `formatProgress()`

Format progress state as a human-readable string.

```typescript
function formatProgress(state: ProgressState): string
```

**Parameters:**
- `state` - Progress state object

**Returns:** Formatted string

**Example:**
```typescript
const state = get({ id: 'task1' });
if (state.ok) {
  console.log(formatProgress(state.value));
  // [45%] 45/100 - Processing items (12s)
}
```

## Types

### `ProgressConfig`

```typescript
interface ProgressConfig {
  id?: string;        // Unique identifier (default: "default")
  filePath?: string;  // Custom storage path (default: temp dir)
}
```

### `ProgressState`

```typescript
interface ProgressState {
  current: number;      // Current progress value
  total: number;        // Total items
  percentage: number;   // Calculated percentage (0-100)
  message: string;      // Current status message
  complete: boolean;    // Whether tracking is complete
  startTime: number;    // Unix timestamp (ms) when initialized
  lastUpdate: number;   // Unix timestamp (ms) of last update
}
```

### `Result<T>`

```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: string };
```

## Error Handling

All functions return a `Result` type. Always check `ok` before accessing `value`:

```typescript
const result = get({ id: 'task1' });

if (result.ok) {
  // Success - access result.value
  console.log(result.value.percentage);
} else {
  // Error - access result.error
  console.error('Error:', result.error);
}
```

## Async/Await Pattern

```typescript
async function processItems(items: string[]) {
  const config = { id: 'async-task' };

  init(items.length, 'Processing items', config);

  for (const item of items) {
    await processItem(item);
    increment(1, undefined, config);
  }

  finish('All items processed', config);
}
```

## Multiple Trackers

Track multiple operations simultaneously:

```typescript
// Track two separate tasks
init(100, 'Task 1', { id: 'task1' });
init(50, 'Task 2', { id: 'task2' });

// Update independently
increment(10, undefined, { id: 'task1' });
increment(5, undefined, { id: 'task2' });

// Query independently
const task1 = get({ id: 'task1' });
const task2 = get({ id: 'task2' });
```

## Custom Storage Path

```typescript
const config = {
  id: 'my-task',
  filePath: './progress-data'
};

init(100, 'Processing', config);
increment(10, undefined, config);
```

For complete API documentation, see [API Reference](/tools/cli-progress-reporting/api-reference).
