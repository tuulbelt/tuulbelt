# Library Usage

Integrate Structured Error Handler into your TypeScript/JavaScript projects as a library.

## Adding as Dependency

Since this tool is part of the Tuulbelt monorepo, you can either:

### Option 1: Local Path Import (Monorepo)

```typescript
// Direct import from sibling directory
import { StructuredError, serializeError, formatError } from '../structured-error-handler/src/index.js';
```

### Option 2: Copy Source (Zero Dependencies)

Since this tool has zero runtime dependencies, you can copy `src/index.ts` directly into your project:

```bash
cp structured-error-handler/src/index.ts your-project/src/structured-error.ts
```

### Option 3: npm Link (Development)

```bash
cd structured-error-handler
npm link

cd ../your-project
npm link structured-error-handler
```

## Basic Usage

### Creating Structured Errors

```typescript
import { StructuredError } from './structured-error-handler/src/index.js';

// Create with all options
const error = new StructuredError('File not found', {
  code: 'ENOENT',
  category: 'io',
  operation: 'readConfig',
  component: 'ConfigLoader',
  metadata: { path: '/etc/config.json' }
});

console.log(error.message);   // 'File not found'
console.log(error.code);      // 'ENOENT'
console.log(error.category);  // 'io'
console.log(error.context);   // [{ operation: 'readConfig', ... }]
```

### Minimal Error (Just Message)

```typescript
// Minimal: just a message
const simple = new StructuredError('Something went wrong');
console.log(simple.context);  // [] (empty context)
```

### Wrapping Existing Errors

```typescript
try {
  await fetch('https://api.example.com/data');
} catch (err) {
  throw StructuredError.wrap(err, 'API request failed', {
    code: 'API_ERROR',
    category: 'network',
    operation: 'fetchUserData',
    component: 'ApiClient',
    metadata: { endpoint: '/data' }
  });
}
```

### Converting Any Error

```typescript
// Convert any error to StructuredError
function handleError(err: unknown): StructuredError {
  return StructuredError.from(err, {
    operation: 'handleError',
    component: 'ErrorHandler'
  });
}

// Works with strings, Error objects, StructuredErrors
handleError('Something broke');          // Creates new StructuredError
handleError(new Error('Original'));       // Wraps with context
handleError(existingStructuredError);     // Returns as-is with added context
```

## Context Chaining

### Adding Context at Each Layer

```typescript
// Database layer
function queryDatabase(query: string): never {
  throw StructuredError.wrap(
    new Error('Connection timeout'),
    'Database query failed',
    {
      code: 'DB_TIMEOUT',
      category: 'database',
      operation: 'executeQuery',
      component: 'DatabaseClient',
      metadata: { query, timeout: 5000 }
    }
  );
}

// Service layer
function findUser(userId: string): never {
  try {
    queryDatabase(`SELECT * FROM users WHERE id = '${userId}'`);
  } catch (err) {
    if (err instanceof StructuredError) {
      throw err.addContext('findUser', {
        component: 'UserService',
        metadata: { userId }
      });
    }
    throw err;
  }
  throw new Error('Unreachable');
}

// Controller layer
function handleGetUser(req: Request): StructuredError {
  try {
    findUser(req.params.id);
    throw new Error('Unreachable');
  } catch (err) {
    if (err instanceof StructuredError) {
      return err.addContext('handleGetUser', {
        component: 'UserController',
        metadata: { endpoint: req.path, method: 'GET' }
      });
    }
    return StructuredError.from(err, {
      operation: 'handleGetUser',
      component: 'UserController'
    });
  }
}
```

### Context Chain Order

Context is stored **most recent first** (top of stack):

```typescript
const error = handleGetUser(mockRequest);

// context[0] is the most recent (top of call stack)
console.log(error.context[0].operation); // 'handleGetUser'
console.log(error.context[1].operation); // 'findUser'
console.log(error.context[2].operation); // 'executeQuery'
```

## Serialization

### To JSON

```typescript
import { StructuredError, serializeError, type SerializedError } from './index.js';

const error = new StructuredError('Operation failed', {
  code: 'OP_FAILED',
  operation: 'myOperation'
});

// Method 1: Instance method
const json: SerializedError = error.toJSON();

// Method 2: Helper function (works with any error)
const serialized: SerializedError = serializeError(error);

// Both work with JSON.stringify
const jsonString = JSON.stringify(error);
console.log(jsonString);
// {"name":"StructuredError","message":"Operation failed","code":"OP_FAILED",...}
```

### From JSON

```typescript
import { StructuredError, deserializeError } from './index.js';

// Parse JSON string
const jsonString = '{"name":"StructuredError","message":"Restored","code":"TEST",...}';
const parsed = JSON.parse(jsonString);

// Method 1: Static method (creates StructuredError)
const restored = StructuredError.fromJSON(parsed);
console.log(restored instanceof StructuredError); // true

// Method 2: Helper function (creates StructuredError or Error)
const deserialized = deserializeError(parsed);
console.log(deserialized.message); // 'Restored'
```

### Round-Trip Serialization

```typescript
const original = new StructuredError('Test error', {
  code: 'TEST',
  category: 'testing',
  operation: 'roundTrip',
  metadata: { key: 'value' }
});

// Serialize
const json = JSON.stringify(original);

// Deserialize
const restored = StructuredError.fromJSON(JSON.parse(json));

// Verify
console.log(restored.message === original.message);   // true
console.log(restored.code === original.code);         // true
console.log(restored.category === original.category); // true
```

## Error Handling Patterns

### Programmatic Error Routing

```typescript
function handleError(error: StructuredError): Response {
  // Route by error code
  if (error.hasCode('VALIDATION_FAILED')) {
    return { status: 400, body: { error: error.message } };
  }

  if (error.hasCode('NOT_FOUND')) {
    return { status: 404, body: { error: error.message } };
  }

  // Route by category
  if (error.hasCategory('authorization')) {
    return { status: 403, body: { error: 'Forbidden' } };
  }

  if (error.hasCategory('validation')) {
    return { status: 400, body: { error: error.message, code: error.code } };
  }

  // Default: internal error
  return { status: 500, body: { error: 'Internal server error' } };
}
```

### Checking Codes in Cause Chain

```typescript
const wrappedError = StructuredError.wrap(
  new StructuredError('Original', { code: 'ORIGINAL_CODE' }),
  'Wrapper message',
  { code: 'WRAPPER_CODE' }
);

// Checks current error AND entire cause chain
console.log(wrappedError.hasCode('WRAPPER_CODE'));   // true (current)
console.log(wrappedError.hasCode('ORIGINAL_CODE')); // true (in cause)
console.log(wrappedError.hasCode('OTHER'));         // false
```

### Root Cause Analysis

```typescript
const topLevelError = StructuredError.wrap(
  StructuredError.wrap(
    new Error('Connection refused'),
    'Database connection failed',
    { code: 'DB_ERROR' }
  ),
  'User lookup failed',
  { code: 'USER_ERROR' }
);

// Get root cause (original error)
const rootCause = topLevelError.getRootCause();
console.log(rootCause.message); // 'Connection refused'

// Get full cause chain
const chain = topLevelError.getCauseChain();
console.log(chain.length);        // 3
console.log(chain[0].message);    // 'User lookup failed'
console.log(chain[1].message);    // 'Database connection failed'
console.log(chain[2].message);    // 'Connection refused'
```

## Formatting

### Human-Readable Output

```typescript
import { StructuredError, formatError } from './index.js';

const error = new StructuredError('Database error', {
  code: 'DB_ERROR',
  operation: 'query',
  component: 'Database'
});

// Method 1: toString()
console.log(error.toString());
// [DB_ERROR] Database error
//
// Context:
//   → query (Database)

// Method 2: formatError helper
console.log(formatError(error));
// Same output

// With stack traces
console.log(formatError(error, { includeStack: true }));
// Includes full stack trace at the end
```

### Custom Formatting

```typescript
function customFormat(error: StructuredError): string {
  const lines: string[] = [];

  // Header
  const codePrefix = error.code ? `[${error.code}] ` : '';
  lines.push(`ERROR: ${codePrefix}${error.message}`);

  // Category
  if (error.category) {
    lines.push(`Category: ${error.category}`);
  }

  // Context (reversed to show oldest first)
  if (error.context.length > 0) {
    lines.push('\nCall Stack:');
    [...error.context].reverse().forEach((ctx, i) => {
      const indent = '  '.repeat(i + 1);
      lines.push(`${indent}└─ ${ctx.operation} (${ctx.component || 'unknown'})`);
    });
  }

  // Root cause
  const root = error.getRootCause();
  if (root !== error) {
    lines.push(`\nRoot Cause: ${root.message}`);
  }

  return lines.join('\n');
}
```

## Logging Integration

### Structured Logging

```typescript
import { StructuredError, type SerializedError } from './index.js';

interface LogEntry {
  level: 'error' | 'warn' | 'info';
  message: string;
  error?: SerializedError;
  context: Record<string, unknown>;
}

function logError(error: StructuredError, requestContext: Record<string, unknown>): void {
  const entry: LogEntry = {
    level: 'error',
    message: error.message,
    error: error.toJSON(),
    context: {
      ...requestContext,
      errorCode: error.code,
      errorCategory: error.category,
      rootCause: error.getRootCause().message,
      contextDepth: error.context.length
    }
  };

  // Send to logging service
  console.log(JSON.stringify(entry));
}

// Usage
const error = new StructuredError('Request failed', { code: 'REQ_FAILED' });
logError(error, { requestId: 'abc123', userId: 'user456' });
```

### Integration with Winston/Pino

```typescript
import { StructuredError } from './index.js';

// Winston example
const winston = require('winston');
const logger = winston.createLogger({ /* config */ });

function logStructuredError(error: StructuredError, meta: Record<string, unknown> = {}): void {
  logger.error(error.message, {
    ...meta,
    error: error.toJSON(),
    code: error.code,
    category: error.category
  });
}

// Pino example
const pino = require('pino');
const pinoLogger = pino();

function logWithPino(error: StructuredError, meta: Record<string, unknown> = {}): void {
  pinoLogger.error({
    ...meta,
    err: error.toJSON(),
    code: error.code,
    category: error.category
  }, error.message);
}
```

## Express.js Integration

### Error Middleware

```typescript
import { StructuredError } from './index.js';
import type { Request, Response, NextFunction } from 'express';

function errorMiddleware(err: Error, req: Request, res: Response, next: NextFunction): void {
  // Convert to StructuredError
  const structured = err instanceof StructuredError
    ? err
    : StructuredError.from(err, {
        operation: `${req.method} ${req.path}`,
        component: 'ExpressApp',
        metadata: { path: req.path, method: req.method }
      });

  // Determine status code
  let status = 500;
  if (structured.hasCategory('validation')) status = 400;
  else if (structured.hasCategory('not_found')) status = 404;
  else if (structured.hasCategory('authorization')) status = 403;

  // Log
  console.error(JSON.stringify(structured.toJSON()));

  // Respond
  res.status(status).json({
    error: structured.message,
    code: structured.code,
    // Don't expose internal details in production
    ...(process.env.NODE_ENV !== 'production' && { details: structured.toJSON() })
  });
}

// Usage
app.use(errorMiddleware);
```

### Async Route Wrapper

```typescript
import { StructuredError } from './index.js';
import type { Request, Response, NextFunction } from 'express';

type AsyncHandler = (req: Request, res: Response, next: NextFunction) => Promise<void>;

function asyncRoute(handler: AsyncHandler): AsyncHandler {
  return async (req, res, next) => {
    try {
      await handler(req, res, next);
    } catch (err) {
      const structured = StructuredError.from(err, {
        operation: handler.name || 'asyncRoute',
        component: 'Router',
        metadata: { path: req.path }
      });
      next(structured);
    }
  };
}

// Usage
app.get('/users/:id', asyncRoute(async (req, res) => {
  const user = await findUser(req.params.id);
  res.json(user);
}));
```

## Testing

### Assert Error Properties

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { StructuredError } from './index.js';

test('myFunction throws StructuredError on invalid input', () => {
  assert.throws(
    () => myFunction(''),
    (err: StructuredError) => {
      assert(err instanceof StructuredError);
      assert.strictEqual(err.code, 'INVALID_INPUT');
      assert.strictEqual(err.category, 'validation');
      assert(err.hasCode('INVALID_INPUT'));
      return true;
    }
  );
});
```

### Test Error Context

```typescript
test('error includes full context chain', async () => {
  try {
    await apiCall('/nonexistent');
    assert.fail('Should have thrown');
  } catch (err) {
    assert(err instanceof StructuredError);
    assert(err.context.length >= 2);
    assert.strictEqual(err.context[0].operation, 'apiCall');
    assert.strictEqual(err.context[1].operation, 'httpClient');
  }
});
```

### Snapshot Testing Errors

```typescript
import { StructuredError } from './index.js';
import { writeFileSync, readFileSync, existsSync } from 'node:fs';

function assertErrorSnapshot(error: StructuredError, snapshotName: string): void {
  const snapshot = error.toJSON();
  // Remove non-deterministic fields
  delete snapshot.stack;
  snapshot.context.forEach(ctx => delete ctx.timestamp);

  const snapshotPath = `test/snapshots/${snapshotName}.json`;

  if (process.env.UPDATE_SNAPSHOTS || !existsSync(snapshotPath)) {
    writeFileSync(snapshotPath, JSON.stringify(snapshot, null, 2));
    return;
  }

  const expected = JSON.parse(readFileSync(snapshotPath, 'utf-8'));
  assert.deepStrictEqual(snapshot, expected);
}
```

## Type Safety

### Full Type Definitions

```typescript
import {
  StructuredError,
  type ErrorContext,
  type SerializedError,
  type StructuredErrorOptions
} from './index.js';

// ErrorContext type
const context: ErrorContext = {
  operation: 'myOperation',
  component: 'MyComponent',
  metadata: { key: 'value' },
  timestamp: new Date().toISOString()
};

// StructuredErrorOptions type
const options: StructuredErrorOptions = {
  code: 'MY_CODE',
  category: 'my_category',
  operation: 'myOperation',
  component: 'MyComponent',
  metadata: { key: 'value' },
  cause: new Error('Original')
};

// SerializedError type
const serialized: SerializedError = {
  name: 'StructuredError',
  message: 'Error message',
  code: 'CODE',
  category: 'category',
  stack: '...',
  context: [],
  cause: undefined
};
```

### Generic Error Handler

```typescript
type ErrorHandler<T> = (error: StructuredError) => T;

function withErrorHandling<T, R>(
  fn: () => T,
  handler: ErrorHandler<R>
): T | R {
  try {
    return fn();
  } catch (err) {
    const structured = StructuredError.from(err, {
      operation: fn.name || 'anonymous'
    });
    return handler(structured);
  }
}

// Usage
const result = withErrorHandling(
  () => riskyOperation(),
  (error) => ({ success: false, error: error.message })
);
```

## See Also

- [CLI Usage](/tools/structured-error-handler/cli-usage) - Command-line interface
- [API Reference](/tools/structured-error-handler/api-reference) - Full API docs
- [Examples](/tools/structured-error-handler/examples) - More examples
