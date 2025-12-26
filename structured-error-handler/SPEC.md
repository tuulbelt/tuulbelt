# Structured Error Handler Specification

## Overview

A structured error format that preserves context through call stacks and enables serialization for logging, transmission, and debugging.

## Problem

Standard JavaScript/TypeScript errors lose context when they propagate through call stacks:

- **Context loss**: When re-throwing errors, "what operation failed" is lost
- **Serialization issues**: Error objects don't serialize cleanly to JSON
- **No cause chain**: Prior to ES2022, no standard way to chain errors
- **Unstructured data**: Error messages contain unstructured text mixing metadata

This tool provides a structured error format that:

- Preserves operation context at each level of the call stack
- Serializes to clean JSON for logging and transmission
- Maintains cause chains for root cause analysis
- Separates error codes, categories, and metadata from messages

## Design Goals

1. **Zero dependencies** — Uses only Node.js standard library
2. **Type safe** — Full TypeScript support with strict mode
3. **Composable** — Works as both library and CLI
4. **Serializable** — Round-trip JSON serialization/deserialization
5. **Backwards compatible** — Extends standard Error class

## Core Types

### ErrorContext

Represents metadata about an operation that failed:

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

### SerializedError

JSON-serializable representation of an error:

```typescript
interface SerializedError {
  /** Error name/type (e.g., "StructuredError", "TypeError") */
  name: string;
  /** Human-readable error message */
  message: string;
  /** Error code for programmatic handling (optional) */
  code?: string;
  /** Error category for grouping (optional) */
  category?: string;
  /** Stack trace (optional) */
  stack?: string;
  /** Context chain, most recent first */
  context: ErrorContext[];
  /** Serialized cause error (optional) */
  cause?: SerializedError;
}
```

### StructuredErrorOptions

Options for creating a StructuredError:

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
  /** Additional metadata */
  metadata?: Record<string, unknown>;
  /** The underlying cause */
  cause?: Error;
}
```

## Serialization Format

### JSON Output Structure

```json
{
  "name": "StructuredError",
  "message": "Failed to process user request",
  "code": "USER_NOT_FOUND",
  "category": "validation",
  "stack": "StructuredError: Failed to process...\n    at ...",
  "context": [
    {
      "operation": "handleRequest",
      "component": "UserController",
      "metadata": { "endpoint": "/api/users/123" },
      "timestamp": "2025-12-26T12:00:00.000Z"
    },
    {
      "operation": "fetchUser",
      "component": "UserService",
      "metadata": { "userId": "123" },
      "timestamp": "2025-12-26T11:59:59.500Z"
    }
  ],
  "cause": {
    "name": "Error",
    "message": "User not found in database",
    "context": []
  }
}
```

### Context Chain Order

Context is stored **most recent first**:

```
context[0] = handleRequest (most recent, top of call stack)
context[1] = fetchUser (earlier, one level down)
context[2] = queryDatabase (earliest, bottom of call stack)
```

This matches the typical reading order when debugging (start from where the error surfaced).

### Text Output Format

For human-readable output (`toString()`):

```
[USER_NOT_FOUND] Failed to process user request

Context:
  → handleRequest (UserController) {"endpoint":"/api/users/123"}
  → fetchUser (UserService) {"userId":"123"}

Caused by:
  User not found in database
```

## Behavior

### Error Creation

```typescript
const error = new StructuredError('Message', {
  code: 'ERROR_CODE',
  category: 'category',
  operation: 'operationName',
  component: 'ComponentName',
  metadata: { key: 'value' }
});
```

Behavior:
1. Sets `name` to `"StructuredError"`
2. Sets `message` to provided message
3. Captures stack trace via `Error.captureStackTrace`
4. If `operation` provided, creates initial context entry with current timestamp

### Wrapping Errors

```typescript
const wrapped = StructuredError.wrap(originalError, 'New message', options);
```

Behavior:
1. Creates new StructuredError with provided message
2. Sets `cause` to the original error
3. If original was StructuredError:
   - Inherits `code` and `category` (unless overridden)
   - Copies entire context chain
4. If operation provided, adds new context entry

### Adding Context

```typescript
const enriched = error.addContext('operation', { component, metadata });
```

Behavior:
1. Creates NEW StructuredError instance (immutable pattern)
2. Copies all properties from original
3. Adds new context entry at position 0 (most recent)
4. Preserves original stack trace

### Serialization

```typescript
const json = error.toJSON();  // Returns SerializedError
const str = JSON.stringify(error);  // Uses toJSON()
```

Behavior:
1. Serializes all fields to JSON-compatible format
2. Recursively serializes cause chain
3. Only includes optional fields if they have values

### Deserialization

```typescript
const error = StructuredError.fromJSON(json);
```

Behavior:
1. Creates StructuredError from SerializedError
2. Restores context chain
3. Recursively deserializes cause chain
4. Restores stack trace if present

## CLI Interface

```
Usage: structured-error-handler <command> [options] [input]

Commands:
  demo             Show a demo of structured errors
  parse <json>     Parse JSON error and format it
  validate <json>  Validate JSON error format

Options:
  -f, --format <format>  Output format: json, text (default: json)
  -s, --stack            Include stack traces in output
  -h, --help             Show this help message
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (invalid input, parse failure) |

## Edge Cases

### Empty Context

```typescript
const error = new StructuredError('Message');  // No operation
error.context  // => []
```

### Non-Error Cause

```typescript
StructuredError.wrap('string error', 'Wrapped');
// cause becomes: new Error('string error')
```

### Circular Metadata

Metadata must be JSON-serializable. Circular references will cause serialization to fail. This is by design - metadata should be simple, serializable data.

### Very Deep Context Chains

No enforced limit on context chain depth. Applications should manage this based on their needs.

## Performance

- **Creation**: O(1) - constant time error creation
- **addContext**: O(n) - copies context array
- **toJSON**: O(n×m) - n context entries, m cause chain depth
- **fromJSON**: O(n×m) - same as toJSON

## Security Considerations

- **No sensitive data in metadata**: Applications should avoid putting passwords, tokens, or PII in metadata
- **Stack traces**: May reveal file paths; consider stripping in production logs
- **Input validation**: `fromJSON` trusts input structure; validate untrusted JSON before parsing
- **No code execution**: Deserialization creates data structures only, no eval or dynamic execution

## Compatibility

### ES2022+ Cause Support

Uses the standard `Error.cause` property from ES2022. For older runtimes:
- The `cause` property is still set and accessible
- Some environments may not serialize it by default

### Node.js Versions

- Requires Node.js 18+ (for native test runner)
- Uses `Error.captureStackTrace` when available
- Falls back gracefully in environments without it

## Examples

### Example 1: Database Error Chain

```typescript
// Low-level database error
const dbError = new Error('Connection timeout');

// Wrap with database context
const repoError = StructuredError.wrap(dbError, 'Failed to fetch user', {
  code: 'DB_TIMEOUT',
  category: 'database',
  operation: 'findUserById',
  component: 'UserRepository',
  metadata: { userId: '123', timeout: 5000 }
});

// Wrap with service context
const serviceError = repoError.addContext('getUserProfile', {
  component: 'UserService',
  metadata: { includePreferences: true }
});

// Wrap with API context
const apiError = serviceError.addContext('handleGetUser', {
  component: 'UserController',
  metadata: { endpoint: '/api/users/123' }
});

console.log(apiError.toString());
```

Output:
```
[DB_TIMEOUT] Failed to fetch user

Context:
  → handleGetUser (UserController) {"endpoint":"/api/users/123"}
  → getUserProfile (UserService) {"includePreferences":true}
  → findUserById (UserRepository) {"userId":"123","timeout":5000}

Caused by:
  Connection timeout
```

### Example 2: Validation Error

```typescript
const error = new StructuredError('Invalid email format', {
  code: 'VALIDATION_FAILED',
  category: 'validation',
  operation: 'validateUserInput',
  metadata: {
    field: 'email',
    value: 'not-an-email',
    rule: 'email'
  }
});

// Programmatic handling
if (error.hasCode('VALIDATION_FAILED')) {
  // Return 400 Bad Request
}

if (error.hasCategory('validation')) {
  // Log to validation error dashboard
}
```

### Example 3: Logging Integration

```typescript
try {
  await processOrder(orderId);
} catch (err) {
  const structured = StructuredError.from(err, {
    operation: 'processOrder',
    metadata: { orderId }
  });

  // Send to logging service
  logger.error({
    ...structured.toJSON(),
    requestId: req.id,
    userId: req.user.id
  });

  // Return appropriate HTTP response
  if (structured.hasCategory('validation')) {
    res.status(400).json({ error: structured.message });
  } else {
    res.status(500).json({ error: 'Internal server error' });
  }
}
```

## Changelog

### v0.1.0

- Initial release
- StructuredError class with context preservation
- Serialization/deserialization support
- CLI interface with demo, parse, and validate commands
- Helper functions: serializeError, deserializeError, formatError
