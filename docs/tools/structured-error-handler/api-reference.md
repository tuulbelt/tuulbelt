# API Reference

Complete TypeScript API documentation for Structured Error Handler.

## Class: StructuredError

Main error class that extends the standard Error with context preservation.

```typescript
class StructuredError extends Error {
  readonly code?: string;
  readonly category?: string;
  readonly context: ErrorContext[];
  readonly cause?: Error;
}
```

### Constructor

```typescript
new StructuredError(message: string, options?: StructuredErrorOptions)
```

**Parameters:**
- `message` - Human-readable error message
- `options` - Optional configuration (see StructuredErrorOptions)

**Example:**
```typescript
const error = new StructuredError('File not found', {
  code: 'ENOENT',
  category: 'io',
  operation: 'readFile',
  component: 'FileSystem',
  metadata: { path: '/etc/config.json' }
});
```

---

### Static Methods

#### `StructuredError.wrap`

Wrap an existing error with additional context.

```typescript
static wrap(
  error: unknown,
  message: string,
  options?: StructuredErrorOptions
): StructuredError
```

**Parameters:**
- `error` - Original error (Error, string, or unknown)
- `message` - New error message
- `options` - Optional configuration

**Returns:** New StructuredError with cause set to original

**Behavior:**
- If `error` is a StructuredError, inherits `code` and `category` (unless overridden)
- If `error` is a string, converts to Error first
- Copies context chain from original StructuredError

**Example:**
```typescript
try {
  await fetch(url);
} catch (err) {
  throw StructuredError.wrap(err, 'API request failed', {
    code: 'API_ERROR',
    operation: 'fetchData'
  });
}
```

---

#### `StructuredError.from`

Convert any value to a StructuredError.

```typescript
static from(
  error: unknown,
  options?: Omit<StructuredErrorOptions, 'cause'>
): StructuredError
```

**Parameters:**
- `error` - Any value (Error, StructuredError, string, unknown)
- `options` - Optional context to add

**Returns:** StructuredError

**Behavior:**
- If already a StructuredError, adds context and returns clone
- If Error, wraps with the same message
- If string, creates new StructuredError with that message
- If other, creates with `String(error)` as message

**Example:**
```typescript
function handleError(err: unknown): StructuredError {
  return StructuredError.from(err, {
    operation: 'handleError',
    component: 'ErrorHandler'
  });
}
```

---

#### `StructuredError.fromJSON`

Deserialize a StructuredError from JSON.

```typescript
static fromJSON(json: SerializedError): StructuredError
```

**Parameters:**
- `json` - Serialized error object

**Returns:** Reconstituted StructuredError

**Example:**
```typescript
const json = JSON.parse(errorString);
const error = StructuredError.fromJSON(json);
console.log(error instanceof StructuredError); // true
```

---

### Instance Methods

#### `addContext`

Add context information to the error (returns new instance).

```typescript
addContext(
  operation: string,
  options?: {
    component?: string;
    metadata?: Record<string, unknown>;
  }
): StructuredError
```

**Parameters:**
- `operation` - Name of the operation being performed
- `options.component` - Optional component or module name
- `options.metadata` - Optional additional metadata

**Returns:** New StructuredError with context prepended (immutable pattern)

**Example:**
```typescript
const enriched = error.addContext('processRequest', {
  component: 'RequestHandler',
  metadata: { requestId: 'abc123' }
});
```

---

#### `toJSON`

Serialize error to JSON-compatible object.

```typescript
toJSON(): SerializedError
```

**Returns:** SerializedError object

**Note:** Automatically called by `JSON.stringify()`

**Example:**
```typescript
const serialized = error.toJSON();
const jsonString = JSON.stringify(error); // Uses toJSON()
```

---

#### `toString`

Format error as human-readable string.

```typescript
toString(): string
```

**Returns:** Formatted string with code, message, context, and cause chain

**Example Output:**
```
[DB_ERROR] Database connection failed

Context:
  → handleRequest (Controller) {"endpoint":"/api"}
  → queryUser (Repository) {"userId":"123"}

Caused by:
  Connection refused
```

---

#### `hasCode`

Check if error or any cause has the specified code.

```typescript
hasCode(code: string): boolean
```

**Parameters:**
- `code` - Error code to search for

**Returns:** `true` if code found in error or any cause

**Example:**
```typescript
if (error.hasCode('ENOENT')) {
  console.log('File not found');
}
```

---

#### `hasCategory`

Check if error or any cause has the specified category.

```typescript
hasCategory(category: string): boolean
```

**Parameters:**
- `category` - Category to search for

**Returns:** `true` if category found in error or any cause

**Example:**
```typescript
if (error.hasCategory('validation')) {
  return res.status(400).json({ error: error.message });
}
```

---

#### `getRootCause`

Get the deepest error in the cause chain.

```typescript
getRootCause(): Error
```

**Returns:** The original/root error

**Example:**
```typescript
const root = error.getRootCause();
console.log('Original error:', root.message);
```

---

#### `getCauseChain`

Get all errors in the cause chain as an array.

```typescript
getCauseChain(): Error[]
```

**Returns:** Array of errors, starting with current error

**Example:**
```typescript
const chain = error.getCauseChain();
console.log('Chain depth:', chain.length);
chain.forEach((err, i) => {
  console.log(`[${i}] ${err.message}`);
});
```

---

## Types

### `ErrorContext`

Context information attached to an error.

```typescript
interface ErrorContext {
  /** The operation that was being performed */
  operation: string;

  /** Optional component or module name */
  component?: string;

  /** Additional metadata (must be JSON-serializable) */
  metadata?: Record<string, unknown>;

  /** ISO 8601 timestamp when context was added */
  timestamp: string;
}
```

---

### `SerializedError`

JSON-serializable representation of an error.

```typescript
interface SerializedError {
  /** Error name/type (e.g., "StructuredError", "TypeError") */
  name: string;

  /** Human-readable error message */
  message: string;

  /** Error code for programmatic handling */
  code?: string;

  /** Error category for grouping */
  category?: string;

  /** Stack trace */
  stack?: string;

  /** Context chain, most recent first */
  context: ErrorContext[];

  /** Serialized cause error */
  cause?: SerializedError;
}
```

---

### `StructuredErrorOptions`

Options for creating a StructuredError.

```typescript
interface StructuredErrorOptions {
  /** Error code for programmatic handling */
  code?: string;

  /** Error category for grouping */
  category?: string;

  /** The operation being performed */
  operation?: string;

  /** Component or module name */
  component?: string;

  /** Additional metadata (must be JSON-serializable) */
  metadata?: Record<string, unknown>;

  /** The underlying cause */
  cause?: Error;
}
```

---

## Helper Functions

### `serializeError`

Serialize any error to JSON format.

```typescript
function serializeError(error: Error): SerializedError
```

**Parameters:**
- `error` - Any Error (including StructuredError)

**Returns:** SerializedError

**Behavior:**
- Works with StructuredError (uses toJSON)
- Works with plain Error (extracts name, message, stack)
- Recursively serializes cause chain

**Example:**
```typescript
const plainError = new TypeError('Invalid argument');
const serialized = serializeError(plainError);
console.log(serialized.name); // 'TypeError'
```

---

### `deserializeError`

Deserialize a SerializedError back to Error.

```typescript
function deserializeError(serialized: SerializedError): Error
```

**Parameters:**
- `serialized` - SerializedError object

**Returns:** StructuredError if name is "StructuredError", otherwise Error

**Example:**
```typescript
const json = { name: 'Error', message: 'Test', context: [] };
const error = deserializeError(json);
console.log(error.message); // 'Test'
```

---

### `formatError`

Format any error for human-readable display.

```typescript
function formatError(
  error: Error,
  options?: { includeStack?: boolean }
): string
```

**Parameters:**
- `error` - Any Error (StructuredError uses its toString)
- `options.includeStack` - Include stack trace (default: false)

**Returns:** Formatted string

**Example:**
```typescript
console.log(formatError(error)); // Basic format
console.log(formatError(error, { includeStack: true })); // With stack
```

---

## Complete Example

```typescript
import {
  StructuredError,
  serializeError,
  deserializeError,
  formatError,
  type ErrorContext,
  type SerializedError,
  type StructuredErrorOptions
} from './index.js';

// Create structured error
const error = new StructuredError('Operation failed', {
  code: 'OP_FAILED',
  category: 'business',
  operation: 'processOrder',
  component: 'OrderService',
  metadata: { orderId: '12345' }
});

// Add context
const enriched = error.addContext('handleRequest', {
  component: 'OrderController',
  metadata: { requestId: 'req-abc' }
});

// Check properties
if (enriched.hasCode('OP_FAILED')) {
  console.log('Order processing failed');
}

// Serialize for logging
const serialized: SerializedError = enriched.toJSON();
console.log(JSON.stringify(serialized));

// Deserialize
const restored = StructuredError.fromJSON(serialized);
console.log(restored.toString());

// Format for display
console.log(formatError(restored, { includeStack: true }));

// Root cause analysis
const chain = restored.getCauseChain();
const root = restored.getRootCause();
console.log(`Chain depth: ${chain.length}, Root: ${root.message}`);
```

---

## Context Chain Order

Context is stored **most recent first** (stack order):

```
context[0] = handleRequest (most recent, top of stack)
context[1] = processOrder (earlier)
context[2] = queryDatabase (original operation)
```

This matches natural debugging order: start from where the error surfaced and trace back.

---

## Serialization Format

JSON output structure:

```json
{
  "name": "StructuredError",
  "message": "Database query failed",
  "code": "DB_ERROR",
  "category": "database",
  "stack": "StructuredError: Database query failed\n    at ...",
  "context": [
    {
      "operation": "handleRequest",
      "component": "Controller",
      "metadata": { "endpoint": "/api/users" },
      "timestamp": "2025-12-26T12:00:00.000Z"
    },
    {
      "operation": "findUser",
      "component": "Repository",
      "metadata": { "userId": "123" },
      "timestamp": "2025-12-26T11:59:59.500Z"
    }
  ],
  "cause": {
    "name": "Error",
    "message": "Connection refused",
    "stack": "...",
    "context": []
  }
}
```

---

## Error Properties

| Property | Type | Description |
|----------|------|-------------|
| `name` | string | Always "StructuredError" |
| `message` | string | Human-readable error message |
| `code` | string? | Programmatic error code |
| `category` | string? | Error category for grouping |
| `context` | ErrorContext[] | Context chain (most recent first) |
| `cause` | Error? | Underlying error |
| `stack` | string? | Stack trace |

---

## Exit Codes (CLI)

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (invalid input, parse failure, unknown command) |

---

## See Also

- [Library Usage](/tools/structured-error-handler/library-usage) - Integration patterns
- [Examples](/tools/structured-error-handler/examples) - Code examples
- [CLI Usage](/tools/structured-error-handler/cli-usage) - Command-line interface
- [SPEC.md](https://github.com/tuulbelt/tuulbelt/blob/main/structured-error-handler/SPEC.md) - Technical specification
