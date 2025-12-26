# Examples

Real-world usage examples for Structured Error Handler.

## Error Creation Patterns

### Example 1: Basic Structured Error

```typescript
import { StructuredError } from '../src/index.js';

// Create with full options
const error = new StructuredError('User not found', {
  code: 'USER_NOT_FOUND',
  category: 'not_found',
  operation: 'getUser',
  component: 'UserService',
  metadata: { userId: '12345' }
});

console.log(error.toString());
// [USER_NOT_FOUND] User not found
//
// Context:
//   → getUser (UserService) {"userId":"12345"}
```

### Example 2: Wrapping Native Errors

```typescript
import { StructuredError } from '../src/index.js';
import { readFile } from 'node:fs/promises';

async function loadConfig(path: string): Promise<object> {
  try {
    const content = await readFile(path, 'utf-8');
    return JSON.parse(content);
  } catch (err) {
    throw StructuredError.wrap(err, 'Failed to load configuration', {
      code: 'CONFIG_LOAD_FAILED',
      category: 'io',
      operation: 'loadConfig',
      component: 'ConfigLoader',
      metadata: { path }
    });
  }
}

// Usage
try {
  await loadConfig('/etc/app/config.json');
} catch (err) {
  console.error(err.toString());
  // [CONFIG_LOAD_FAILED] Failed to load configuration
  //
  // Context:
  //   → loadConfig (ConfigLoader) {"path":"/etc/app/config.json"}
  //
  // Caused by:
  //   ENOENT: no such file or directory, open '/etc/app/config.json'
}
```

### Example 3: Error Context Chaining

```typescript
import { StructuredError } from '../src/index.js';

// Database layer
function queryDatabase(query: string): never {
  throw new Error(`Connection timeout: ${query}`);
}

// Repository layer
function findUserById(userId: string): never {
  try {
    queryDatabase(`SELECT * FROM users WHERE id = '${userId}'`);
  } catch (err) {
    throw StructuredError.wrap(err, 'Database query failed', {
      code: 'DB_ERROR',
      category: 'database',
      operation: 'findUserById',
      component: 'UserRepository',
      metadata: { userId }
    });
  }
  throw new Error('Unreachable');
}

// Service layer
function getUserProfile(userId: string): never {
  try {
    findUserById(userId);
  } catch (err) {
    if (err instanceof StructuredError) {
      throw err.addContext('getUserProfile', {
        component: 'UserService'
      });
    }
    throw err;
  }
  throw new Error('Unreachable');
}

// Controller layer
function handleRequest(userId: string): StructuredError {
  try {
    getUserProfile(userId);
    throw new Error('Unreachable');
  } catch (err) {
    if (err instanceof StructuredError) {
      return err.addContext('handleRequest', {
        component: 'UserController',
        metadata: { endpoint: '/api/users' }
      });
    }
    return StructuredError.from(err, { operation: 'handleRequest' });
  }
}

const error = handleRequest('12345');
console.log(error.toString());
// [DB_ERROR] Database query failed
//
// Context:
//   → handleRequest (UserController) {"endpoint":"/api/users"}
//   → getUserProfile (UserService)
//   → findUserById (UserRepository) {"userId":"12345"}
//
// Caused by:
//   Connection timeout: SELECT * FROM users WHERE id = '12345'
```

## HTTP/API Patterns

### Example 4: Express.js Error Middleware

```typescript
import { StructuredError } from '../src/index.js';
import type { Request, Response, NextFunction } from 'express';

// Error categories to HTTP status codes
const categoryToStatus: Record<string, number> = {
  validation: 400,
  authentication: 401,
  authorization: 403,
  not_found: 404,
  conflict: 409,
  rate_limit: 429,
  database: 500,
  network: 502
};

function errorMiddleware(
  err: Error,
  req: Request,
  res: Response,
  next: NextFunction
): void {
  // Convert to StructuredError
  const structured = err instanceof StructuredError
    ? err
    : StructuredError.from(err, {
        operation: `${req.method} ${req.path}`,
        component: 'ExpressApp',
        metadata: {
          method: req.method,
          path: req.path,
          query: req.query,
          ip: req.ip
        }
      });

  // Determine status code
  let status = 500;
  if (structured.category && categoryToStatus[structured.category]) {
    status = categoryToStatus[structured.category];
  }

  // Log detailed error (internal)
  console.error(JSON.stringify({
    timestamp: new Date().toISOString(),
    requestId: req.headers['x-request-id'],
    error: structured.toJSON()
  }));

  // Return safe response (external)
  const response: Record<string, unknown> = {
    error: status < 500 ? structured.message : 'Internal server error',
    code: structured.code
  };

  // Include details in development
  if (process.env.NODE_ENV === 'development') {
    response.details = structured.toJSON();
  }

  res.status(status).json(response);
}

// Usage
app.use(errorMiddleware);
```

### Example 5: API Client Error Wrapping

```typescript
import { StructuredError } from '../src/index.js';

interface ApiResponse<T> {
  ok: true;
  data: T;
} | {
  ok: false;
  error: StructuredError;
}

async function apiRequest<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<ApiResponse<T>> {
  const baseUrl = process.env.API_BASE_URL || 'https://api.example.com';
  const url = `${baseUrl}${endpoint}`;

  try {
    const response = await fetch(url, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options.headers
      }
    });

    if (!response.ok) {
      const body = await response.text();
      return {
        ok: false,
        error: new StructuredError(`API request failed: ${response.statusText}`, {
          code: `HTTP_${response.status}`,
          category: response.status >= 500 ? 'network' : 'validation',
          operation: 'apiRequest',
          component: 'ApiClient',
          metadata: {
            url,
            method: options.method || 'GET',
            status: response.status,
            responseBody: body.slice(0, 500)
          }
        })
      };
    }

    const data = await response.json();
    return { ok: true, data: data as T };

  } catch (err) {
    return {
      ok: false,
      error: StructuredError.wrap(err, 'Network request failed', {
        code: 'NETWORK_ERROR',
        category: 'network',
        operation: 'apiRequest',
        component: 'ApiClient',
        metadata: { url, method: options.method || 'GET' }
      })
    };
  }
}

// Usage
const result = await apiRequest<{ user: { id: string } }>('/users/123');
if (!result.ok) {
  console.error(result.error.toString());
  // Handle error
} else {
  console.log(result.data.user);
}
```

## Validation Patterns

### Example 6: Input Validation Errors

```typescript
import { StructuredError } from '../src/index.js';

interface ValidationResult {
  ok: true;
} | {
  ok: false;
  error: StructuredError;
}

interface UserInput {
  email?: string;
  age?: number;
  username?: string;
}

function validateUser(input: UserInput): ValidationResult {
  const errors: Array<{ field: string; message: string }> = [];

  // Email validation
  if (!input.email) {
    errors.push({ field: 'email', message: 'Email is required' });
  } else if (!input.email.includes('@')) {
    errors.push({ field: 'email', message: 'Invalid email format' });
  }

  // Age validation
  if (input.age !== undefined) {
    if (input.age < 0 || input.age > 150) {
      errors.push({ field: 'age', message: 'Age must be between 0 and 150' });
    }
  }

  // Username validation
  if (!input.username) {
    errors.push({ field: 'username', message: 'Username is required' });
  } else if (input.username.length < 3) {
    errors.push({ field: 'username', message: 'Username must be at least 3 characters' });
  }

  if (errors.length > 0) {
    return {
      ok: false,
      error: new StructuredError(
        errors.length === 1
          ? errors[0].message
          : `${errors.length} validation errors`,
        {
          code: 'VALIDATION_FAILED',
          category: 'validation',
          operation: 'validateUser',
          component: 'UserValidator',
          metadata: {
            fieldErrors: errors,
            input: { ...input, email: input.email ? '***' : undefined }
          }
        }
      )
    };
  }

  return { ok: true };
}

// Usage
const result = validateUser({ email: 'invalid', age: -5 });
if (!result.ok) {
  console.log(result.error.toString());
  // [VALIDATION_FAILED] 3 validation errors
  //
  // Context:
  //   → validateUser (UserValidator) {"fieldErrors":[...]}

  // Access field errors programmatically
  const metadata = result.error.context[0]?.metadata as { fieldErrors: Array<{ field: string; message: string }> };
  metadata.fieldErrors.forEach(err => {
    console.log(`  - ${err.field}: ${err.message}`);
  });
}
```

### Example 7: Schema Validation

```typescript
import { StructuredError } from '../src/index.js';

type Schema = Record<string, { type: string; required?: boolean; min?: number; max?: number }>;

function validateSchema(data: unknown, schema: Schema): StructuredError | null {
  if (typeof data !== 'object' || data === null) {
    return new StructuredError('Data must be an object', {
      code: 'INVALID_DATA_TYPE',
      category: 'validation',
      operation: 'validateSchema'
    });
  }

  const record = data as Record<string, unknown>;
  const errors: string[] = [];

  for (const [field, rules] of Object.entries(schema)) {
    const value = record[field];

    if (rules.required && value === undefined) {
      errors.push(`${field} is required`);
      continue;
    }

    if (value !== undefined) {
      const actualType = typeof value;
      if (actualType !== rules.type) {
        errors.push(`${field} must be ${rules.type}, got ${actualType}`);
      }

      if (rules.type === 'number' && typeof value === 'number') {
        if (rules.min !== undefined && value < rules.min) {
          errors.push(`${field} must be >= ${rules.min}`);
        }
        if (rules.max !== undefined && value > rules.max) {
          errors.push(`${field} must be <= ${rules.max}`);
        }
      }
    }
  }

  if (errors.length > 0) {
    return new StructuredError(errors.join('; '), {
      code: 'SCHEMA_VALIDATION_FAILED',
      category: 'validation',
      operation: 'validateSchema',
      metadata: { errors, schemaFields: Object.keys(schema) }
    });
  }

  return null;
}

// Usage
const schema: Schema = {
  name: { type: 'string', required: true },
  age: { type: 'number', min: 0, max: 150 },
  email: { type: 'string', required: true }
};

const error = validateSchema({ name: 123, age: -5 }, schema);
if (error) {
  console.log(error.toString());
}
```

## Logging Integration

### Example 8: Structured Logging

```typescript
import { StructuredError, type SerializedError } from '../src/index.js';

interface LogEntry {
  timestamp: string;
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  context?: Record<string, unknown>;
  error?: SerializedError;
}

class StructuredLogger {
  private logs: LogEntry[] = [];

  log(level: LogEntry['level'], message: string, context?: Record<string, unknown>): void {
    this.logs.push({
      timestamp: new Date().toISOString(),
      level,
      message,
      context
    });
  }

  error(error: StructuredError, context?: Record<string, unknown>): void {
    this.logs.push({
      timestamp: new Date().toISOString(),
      level: 'error',
      message: error.message,
      context: {
        ...context,
        errorCode: error.code,
        errorCategory: error.category,
        rootCause: error.getRootCause().message,
        contextChain: error.context.map(c => c.operation)
      },
      error: error.toJSON()
    });
  }

  toJSON(): LogEntry[] {
    return this.logs;
  }
}

// Usage
const logger = new StructuredLogger();

try {
  throw StructuredError.wrap(
    new Error('Connection refused'),
    'Database unavailable',
    { code: 'DB_UNAVAILABLE', category: 'database', operation: 'connect' }
  );
} catch (err) {
  if (err instanceof StructuredError) {
    logger.error(err, { requestId: 'req-123', userId: 'user-456' });
  }
}

console.log(JSON.stringify(logger.toJSON(), null, 2));
```

### Example 9: Error Aggregation

```typescript
import { StructuredError } from '../src/index.js';

class ErrorAggregator {
  private errors: Map<string, { count: number; lastSeen: string; example: StructuredError }> = new Map();

  add(error: StructuredError): void {
    const key = `${error.code || 'UNKNOWN'}:${error.category || 'unknown'}`;
    const existing = this.errors.get(key);

    if (existing) {
      existing.count++;
      existing.lastSeen = new Date().toISOString();
    } else {
      this.errors.set(key, {
        count: 1,
        lastSeen: new Date().toISOString(),
        example: error
      });
    }
  }

  getSummary(): Array<{ key: string; count: number; lastSeen: string; message: string }> {
    return Array.from(this.errors.entries()).map(([key, data]) => ({
      key,
      count: data.count,
      lastSeen: data.lastSeen,
      message: data.example.message
    }));
  }

  getMostFrequent(n: number = 10): Array<{ code: string; count: number }> {
    return Array.from(this.errors.entries())
      .sort((a, b) => b[1].count - a[1].count)
      .slice(0, n)
      .map(([key, data]) => ({ code: key, count: data.count }));
  }
}

// Usage
const aggregator = new ErrorAggregator();

// Simulate error collection
for (let i = 0; i < 100; i++) {
  const code = Math.random() > 0.7 ? 'TIMEOUT' : 'CONNECTION_REFUSED';
  aggregator.add(new StructuredError('Network error', {
    code,
    category: 'network'
  }));
}

console.log('Most frequent errors:', aggregator.getMostFrequent(5));
```

## Testing Patterns

### Example 10: Error Assertion Helper

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { StructuredError } from '../src/index.js';

function assertStructuredError(
  fn: () => void,
  expected: { code?: string; category?: string; messageContains?: string }
): StructuredError {
  let caught: unknown;
  try {
    fn();
    assert.fail('Expected function to throw');
  } catch (err) {
    caught = err;
  }

  assert(caught instanceof StructuredError, 'Expected StructuredError');

  if (expected.code) {
    assert.strictEqual(caught.code, expected.code, `Expected code ${expected.code}`);
  }

  if (expected.category) {
    assert.strictEqual(caught.category, expected.category, `Expected category ${expected.category}`);
  }

  if (expected.messageContains) {
    assert(
      caught.message.includes(expected.messageContains),
      `Expected message to contain "${expected.messageContains}"`
    );
  }

  return caught;
}

// Usage in tests
test('validateInput throws on empty input', () => {
  const error = assertStructuredError(
    () => validateInput(''),
    {
      code: 'VALIDATION_FAILED',
      category: 'validation',
      messageContains: 'empty'
    }
  );

  // Additional assertions on the caught error
  assert(error.context.length > 0);
});

function validateInput(input: string): void {
  if (!input) {
    throw new StructuredError('Input cannot be empty', {
      code: 'VALIDATION_FAILED',
      category: 'validation',
      operation: 'validateInput'
    });
  }
}
```

### Example 11: Error Snapshot Testing

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { writeFileSync, readFileSync, existsSync } from 'node:fs';
import { StructuredError } from '../src/index.js';

function normalizeErrorForSnapshot(error: StructuredError): object {
  const json = error.toJSON();
  // Remove non-deterministic fields
  delete json.stack;
  if (json.context) {
    json.context.forEach((ctx: Record<string, unknown>) => delete ctx.timestamp);
  }
  return json;
}

function assertErrorMatchesSnapshot(error: StructuredError, snapshotName: string): void {
  const snapshotPath = `test/snapshots/${snapshotName}.json`;
  const normalized = normalizeErrorForSnapshot(error);

  if (process.env.UPDATE_SNAPSHOTS || !existsSync(snapshotPath)) {
    writeFileSync(snapshotPath, JSON.stringify(normalized, null, 2));
    console.log(`Updated snapshot: ${snapshotPath}`);
    return;
  }

  const expected = JSON.parse(readFileSync(snapshotPath, 'utf-8'));
  assert.deepStrictEqual(normalized, expected, `Snapshot mismatch: ${snapshotName}`);
}

// Usage
test('database error snapshot', () => {
  const error = StructuredError.wrap(
    new Error('Connection refused'),
    'Database unavailable',
    {
      code: 'DB_UNAVAILABLE',
      category: 'database',
      operation: 'connect',
      metadata: { host: 'localhost', port: 5432 }
    }
  );

  assertErrorMatchesSnapshot(error, 'database-error');
});
```

## CLI Integration

### Example 12: CLI Error Handling

```typescript
import { StructuredError, formatError } from '../src/index.js';

interface CLIResult {
  exitCode: number;
  output: string;
}

function runCLI(args: string[]): CLIResult {
  try {
    // Parse arguments
    if (args.length === 0) {
      throw new StructuredError('No command provided', {
        code: 'NO_COMMAND',
        category: 'validation',
        operation: 'parseArgs',
        component: 'CLI'
      });
    }

    const command = args[0];
    if (!['run', 'build', 'test'].includes(command)) {
      throw new StructuredError(`Unknown command: ${command}`, {
        code: 'UNKNOWN_COMMAND',
        category: 'validation',
        operation: 'parseArgs',
        component: 'CLI',
        metadata: { command, validCommands: ['run', 'build', 'test'] }
      });
    }

    // Execute command
    return { exitCode: 0, output: `Executed: ${command}` };

  } catch (err) {
    const structured = err instanceof StructuredError
      ? err
      : StructuredError.from(err, { operation: 'runCLI' });

    // Determine exit code
    let exitCode = 1;
    if (structured.hasCategory('validation')) exitCode = 2;
    if (structured.hasCode('UNKNOWN_COMMAND')) exitCode = 127;

    // Format output
    const output = process.env.DEBUG
      ? formatError(structured, { includeStack: true })
      : `Error: ${structured.message}`;

    return { exitCode, output };
  }
}

// Usage
const result = runCLI(['invalid-command']);
console.error(result.output);
process.exitCode = result.exitCode;
```

### Example 13: JSON Output Mode

```typescript
import { StructuredError } from '../src/index.js';

interface CLIOptions {
  format: 'text' | 'json';
  verbose: boolean;
}

function handleCLIError(error: StructuredError, options: CLIOptions): string {
  if (options.format === 'json') {
    const json = error.toJSON();
    if (!options.verbose) {
      // Minimal JSON output
      return JSON.stringify({
        error: json.message,
        code: json.code,
        category: json.category
      });
    }
    return JSON.stringify(json, null, 2);
  }

  // Text format
  if (options.verbose) {
    return error.toString();
  }

  // Minimal text
  const prefix = error.code ? `[${error.code}] ` : '';
  return `Error: ${prefix}${error.message}`;
}

// Usage
const error = new StructuredError('File not found', {
  code: 'ENOENT',
  category: 'io',
  operation: 'readFile',
  metadata: { path: '/missing.txt' }
});

console.log(handleCLIError(error, { format: 'json', verbose: false }));
// {"error":"File not found","code":"ENOENT","category":"io"}

console.log(handleCLIError(error, { format: 'text', verbose: true }));
// [ENOENT] File not found
//
// Context:
//   → readFile () {"path":"/missing.txt"}
```

## Advanced Patterns

### Example 14: Retry with Error History

```typescript
import { StructuredError } from '../src/index.js';

interface RetryOptions {
  maxRetries: number;
  delayMs: number;
  backoff: number;
}

async function withRetry<T>(
  fn: () => Promise<T>,
  options: RetryOptions
): Promise<T> {
  const errors: Error[] = [];
  let delay = options.delayMs;

  for (let attempt = 1; attempt <= options.maxRetries; attempt++) {
    try {
      return await fn();
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      errors.push(error);

      if (attempt < options.maxRetries) {
        await new Promise(resolve => setTimeout(resolve, delay));
        delay *= options.backoff;
      }
    }
  }

  // All retries failed
  throw new StructuredError(
    `Operation failed after ${options.maxRetries} attempts`,
    {
      code: 'RETRY_EXHAUSTED',
      category: 'retry',
      operation: 'withRetry',
      metadata: {
        maxRetries: options.maxRetries,
        attempts: errors.map((e, i) => ({
          attempt: i + 1,
          error: e.message
        }))
      },
      cause: errors[errors.length - 1]
    }
  );
}

// Usage
const result = await withRetry(
  () => fetch('https://api.example.com/unstable'),
  { maxRetries: 3, delayMs: 1000, backoff: 2 }
);
```

### Example 15: Error Boundary Pattern

```typescript
import { StructuredError } from '../src/index.js';

type ErrorBoundaryHandler = (error: StructuredError) => void;

class ErrorBoundary {
  private handlers: Map<string, ErrorBoundaryHandler> = new Map();
  private defaultHandler: ErrorBoundaryHandler;

  constructor(defaultHandler: ErrorBoundaryHandler) {
    this.defaultHandler = defaultHandler;
  }

  register(codeOrCategory: string, handler: ErrorBoundaryHandler): void {
    this.handlers.set(codeOrCategory, handler);
  }

  handle(error: unknown): void {
    const structured = error instanceof StructuredError
      ? error
      : StructuredError.from(error, { operation: 'unknown' });

    // Try code-specific handler
    if (structured.code && this.handlers.has(structured.code)) {
      this.handlers.get(structured.code)!(structured);
      return;
    }

    // Try category-specific handler
    if (structured.category && this.handlers.has(structured.category)) {
      this.handlers.get(structured.category)!(structured);
      return;
    }

    // Fall back to default
    this.defaultHandler(structured);
  }

  wrap<T>(fn: () => T): T | undefined {
    try {
      return fn();
    } catch (err) {
      this.handle(err);
      return undefined;
    }
  }
}

// Usage
const boundary = new ErrorBoundary((error) => {
  console.error('Unhandled error:', error.message);
});

boundary.register('validation', (error) => {
  console.warn('Validation error:', error.message);
});

boundary.register('RATE_LIMITED', (error) => {
  console.info('Rate limited, will retry...');
});

// Automatically routes errors to appropriate handlers
boundary.wrap(() => {
  throw new StructuredError('Invalid input', {
    code: 'VALIDATION_FAILED',
    category: 'validation'
  });
});
// Output: Validation error: Invalid input
```

## Running Examples

Save any example to a file and run:

```bash
# From structured-error-handler directory
npx tsx examples/your-example.ts

# Or run the included examples
npx tsx examples/basic.ts
npx tsx examples/advanced.ts
```

## See Also

- [Getting Started](/tools/structured-error-handler/getting-started) - Quick start guide
- [Library Usage](/tools/structured-error-handler/library-usage) - Integration patterns
- [API Reference](/tools/structured-error-handler/api-reference) - Full API documentation
