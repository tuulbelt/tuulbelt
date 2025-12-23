---
name: typescript-patterns
description: "Enforce TypeScript best practices including strict typing, proper error handling, async/await patterns, and type-safe utilities. Use when writing TypeScript code for Tuulbelt tools."
---

# TypeScript Patterns and Best Practices for Tuulbelt

## Core Principles

1. **Strict Mode Always** - No compromises on type safety
2. **Explicit Types** - Return types, parameter types, no inference for public APIs
3. **Zero Dependencies** - Use Node.js built-ins only
4. **Error Handling** - Result types, not thrown exceptions for control flow
5. **ESM Modules** - Modern import/export syntax

## Type Safety

### Strict TypeScript Configuration

Required `tsconfig.json` settings:

```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "noImplicitReturns": true,
    "noUncheckedIndexedAccess": true,
    "noFallthroughCasesInSwitch": true,
    "forceConsistentCasingInFileNames": true,
    "skipLibCheck": false,
    "exactOptionalPropertyTypes": true
  }
}
```

### Explicit Return Types

**Always annotate function return types:**

```typescript
// GOOD: Explicit return type
export function processData(input: string): Result<Data> {
  // Implementation
}

// BAD: Inferred return type
export function processData(input: string) {
  // Return type not explicit
}
```

### No `any` Type

**Use `unknown` and type guards instead:**

```typescript
// GOOD: Use unknown and narrow
function parse(input: unknown): Data {
  if (typeof input !== 'string') {
    throw new Error('Expected string');
  }
  return JSON.parse(input) as Data;
}

// BAD: Using any
function parse(input: any): Data {
  return JSON.parse(input);
}
```

### Discriminated Unions for Type Safety

**Pattern:**

```typescript
// Result type for success/failure
export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

// Usage with type narrowing
function processData(input: string): Result<Data> {
  if (!input) {
    return { ok: false, error: new Error('Empty input') };
  }

  const data = transform(input);
  return { ok: true, value: data };
}

// Consumer uses type narrowing
const result = processData(input);
if (result.ok) {
  console.log(result.value);  // TypeScript knows this is Data
} else {
  console.error(result.error); // TypeScript knows this is Error
}
```

## Error Handling

### Result Pattern (Preferred)

**For expected errors (validation, parsing, etc.):**

```typescript
export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

export function parseConfig(path: string): Result<Config> {
  try {
    const content = readFileSync(path, 'utf-8');
    const config = JSON.parse(content);
    return { ok: true, value: config as Config };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}
```

### Exception Pattern (For Unexpected Errors)

**For programming errors and unexpected failures:**

```typescript
export function assertNonNull<T>(value: T | null | undefined, message: string): T {
  if (value == null) {
    throw new Error(message);
  }
  return value;
}

// Use for invariants
const data = assertNonNull(maybeData, 'Data must be initialized');
```

### Error Types

**Define custom error types for clarity:**

```typescript
export class ValidationError extends Error {
  constructor(
    message: string,
    public readonly field: string,
    public readonly value: unknown
  ) {
    super(message);
    this.name = 'ValidationError';
  }
}

export class ParseError extends Error {
  constructor(
    message: string,
    public readonly line: number,
    public readonly column: number
  ) {
    super(message);
    this.name = 'ParseError';
  }
}
```

## Async/Await Patterns

### Always Use async/await

**Don't use raw Promises or callbacks:**

```typescript
// GOOD: async/await
export async function fetchData(url: string): Promise<Result<Data>> {
  try {
    const response = await fetch(url);
    const json = await response.json();
    return { ok: true, value: json as Data };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}

// BAD: Promise chains
export function fetchData(url: string): Promise<Result<Data>> {
  return fetch(url)
    .then(res => res.json())
    .then(json => ({ ok: true, value: json as Data }))
    .catch(error => ({ ok: false, error: error as Error }));
}

// BAD: Callbacks
export function fetchData(url: string, callback: (err: Error | null, data?: Data) => void): void {
  // Never use callbacks
}
```

### Parallel Async Operations

**Use Promise.all for concurrency:**

```typescript
// GOOD: Parallel execution
const results = await Promise.all([
  fetchData(url1),
  fetchData(url2),
  fetchData(url3),
]);

// BAD: Sequential when parallelizable
const result1 = await fetchData(url1);
const result2 = await fetchData(url2);
const result3 = await fetchData(url3);
```

### Timeout Handling

```typescript
function timeout<T>(promise: Promise<T>, ms: number): Promise<T> {
  return Promise.race([
    promise,
    new Promise<T>((_, reject) =>
      setTimeout(() => reject(new Error(`Timeout after ${ms}ms`)), ms)
    ),
  ]);
}

// Usage
const result = await timeout(fetchData(url), 5000);
```

## File I/O (Zero Dependencies)

### Use Node.js Built-in fs Module

**Prefer fs promises API:**

```typescript
import { readFile, writeFile } from 'fs/promises';

// GOOD: Promises API
export async function loadConfig(path: string): Promise<Result<Config>> {
  try {
    const content = await readFile(path, 'utf-8');
    const config = JSON.parse(content);
    return { ok: true, value: config as Config };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}

// Acceptable: Sync for simple CLIs
import { readFileSync } from 'fs';

export function loadConfigSync(path: string): Result<Config> {
  try {
    const content = readFileSync(path, 'utf-8');
    const config = JSON.parse(content);
    return { ok: true, value: config as Config };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}
```

### Path Handling

**Always use path module:**

```typescript
import { join, resolve, dirname } from 'path';

// GOOD: Use path module
const configPath = join(baseDir, 'config', 'app.json');
const absolutePath = resolve(configPath);

// BAD: String concatenation
const configPath = baseDir + '/config/app.json';
```

## Testing with Node.js Native Test Runner

### Test Structure

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';

test('should process valid input', () => {
  const result = processData('valid input');
  assert.strictEqual(result.ok, true);
  if (result.ok) {
    assert.deepStrictEqual(result.value, expectedData);
  }
});

test('should handle empty input', () => {
  const result = processData('');
  assert.strictEqual(result.ok, false);
  if (!result.ok) {
    assert.match(result.error.message, /empty/i);
  }
});
```

### Async Tests

```typescript
test('should fetch data successfully', async () => {
  const result = await fetchData('https://example.com/data');
  assert.strictEqual(result.ok, true);
});
```

### Test Organization

```
src/
├── index.ts
├── parser.ts
└── validator.ts

test/
├── index.test.ts
├── parser.test.ts
└── validator.test.ts
```

## CLI Patterns

### Argument Parsing (Zero Dependencies)

```typescript
// Simple argument parsing without dependencies
function parseArgs(args: string[]): { flags: Set<string>; params: Map<string, string>; positional: string[] } {
  const flags = new Set<string>();
  const params = new Map<string, string>();
  const positional: string[] = [];

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg.startsWith('--')) {
      const name = arg.slice(2);
      if (args[i + 1] && !args[i + 1].startsWith('-')) {
        params.set(name, args[i + 1]);
        i++;
      } else {
        flags.add(name);
      }
    } else if (arg.startsWith('-')) {
      flags.add(arg.slice(1));
    } else {
      positional.push(arg);
    }
  }

  return { flags, params, positional };
}

// Usage
const { flags, params, positional } = parseArgs(process.argv.slice(2));

if (flags.has('help')) {
  console.log('Usage: ...');
  process.exit(0);
}
```

### stdin/stdout Handling

```typescript
import { stdin, stdout } from 'process';

// Read from stdin
async function readStdin(): Promise<string> {
  const chunks: Buffer[] = [];

  for await (const chunk of stdin) {
    chunks.push(chunk as Buffer);
  }

  return Buffer.concat(chunks).toString('utf-8');
}

// Write to stdout
stdout.write('Output message\n');

// stderr for errors
import { stderr } from 'process';
stderr.write('Error message\n');
```

### Exit Codes

```typescript
// Success
process.exit(0);

// General error
process.exit(1);

// Invalid arguments
process.exit(2);
```

## Common Patterns

### Lazy Initialization

```typescript
let cachedConfig: Config | null = null;

export function getConfig(): Config {
  if (cachedConfig === null) {
    cachedConfig = loadConfigSync('config.json');
  }
  return cachedConfig;
}
```

### Type Guards

```typescript
function isString(value: unknown): value is string {
  return typeof value === 'string';
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
```

### Exhaustive Checks

```typescript
type Status = 'pending' | 'running' | 'completed' | 'failed';

function handleStatus(status: Status): string {
  switch (status) {
    case 'pending':
      return 'Waiting to start';
    case 'running':
      return 'In progress';
    case 'completed':
      return 'Done';
    case 'failed':
      return 'Error';
    default:
      // TypeScript will error if we add a new Status without handling it
      const _exhaustive: never = status;
      throw new Error(`Unhandled status: ${_exhaustive}`);
  }
}
```

## Anti-Patterns to Avoid

### Don't Use

- ❌ `any` type (use `unknown`)
- ❌ Non-null assertion `!` (use type guards)
- ❌ `@ts-ignore` comments (fix the type error)
- ❌ External dependencies for core functionality
- ❌ Default exports (use named exports)
- ❌ `var` keyword (use `const` or `let`)
- ❌ `require()` (use ESM `import`)

### Prefer

- ✅ Explicit types over inference for public APIs
- ✅ `const` over `let` when possible
- ✅ Named exports over default exports
- ✅ Result types over exceptions for control flow
- ✅ Async/await over Promise chains
- ✅ Node.js built-ins over external packages

## Remember

This skill applies to **all TypeScript code in Tuulbelt tools**. Enforce these patterns consistently for maintainability, type safety, and zero-dependency compliance.
