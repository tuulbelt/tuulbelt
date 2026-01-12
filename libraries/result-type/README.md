# @tuulbelt/result-type

Rust-style `Result<T, E>` for TypeScript with zero dependencies.

[![Tests](https://github.com/tuulbelt/result-type/actions/workflows/test.yml/badge.svg)](https://github.com/tuulbelt/result-type/actions/workflows/test.yml)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.3+-blue.svg)](https://www.typescriptlang.org/)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-brightgreen.svg)](package.json)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Why Result Types?

Exceptions are invisible in function signatures and can propagate unexpectedly. Result types make errors **explicit** and **type-safe**:

```typescript
// With exceptions - caller might not know this can fail
function divide(a: number, b: number): number {
  if (b === 0) throw new Error('Division by zero');
  return a / b;
}

// With Result - error handling is explicit in the type
function divide(a: number, b: number): Result<number, string> {
  if (b === 0) return err('Division by zero');
  return ok(a / b);
}
```

## Installation

```bash
# Clone from Tuulbelt (no npm registry needed)
git clone https://github.com/tuulbelt/result-type.git

# Or as a git dependency
npm install git+https://github.com/tuulbelt/result-type.git
```

## Quick Start

```typescript
import { ok, err, match, andThen, type Result } from '@tuulbelt/result-type';

// Create Results
const success = ok(42);       // { ok: true, value: 42 }
const failure = err('oops');  // { ok: false, error: 'oops' }

// Pattern matching
const message = match(success, {
  ok: (value) => `Got: ${value}`,
  err: (error) => `Error: ${error}`
});
// "Got: 42"

// Chaining operations
function parseNumber(s: string): Result<number, string> {
  const n = parseInt(s, 10);
  return isNaN(n) ? err('Not a number') : ok(n);
}

function double(n: number): Result<number, string> {
  return ok(n * 2);
}

const result = andThen(parseNumber('21'), double);
// { ok: true, value: 42 }
```

## API Overview

### Constructors

| Function | Description |
|----------|-------------|
| `ok(value)` | Creates a successful Result |
| `err(error)` | Creates a failed Result |

### Type Guards

| Function | Description |
|----------|-------------|
| `isOk(result)` | Type guard for Ok variant |
| `isErr(result)` | Type guard for Err variant |

### Transformations

| Function | Description |
|----------|-------------|
| `map(result, fn)` | Transform the success value |
| `mapErr(result, fn)` | Transform the error value |
| `andThen(result, fn)` | Chain Result-returning functions |
| `or(result, alt)` | Return first Ok, or alternative |
| `orElse(result, fn)` | Recover from error with function |

### Pattern Matching

| Function | Description |
|----------|-------------|
| `match(result, handlers)` | Call appropriate handler for Ok/Err |

### Unwrapping

| Function | Description |
|----------|-------------|
| `unwrap(result)` | Get value or throw error |
| `unwrapOr(result, default)` | Get value or return default |
| `unwrapOrElse(result, fn)` | Get value or compute from error |
| `expect(result, msg)` | Get value or throw custom message |
| `unwrapErr(result)` | Get error or throw value |

### Collections

| Function | Description |
|----------|-------------|
| `all(results)` | Collect array of Results into Result of array |
| `any(results)` | Return first Ok, or all errors |
| `partition(results)` | Split into [okValues, errValues] |

### Try/Catch

| Function | Description |
|----------|-------------|
| `tryCatch(fn)` | Wrap throwing function |
| `tryCatchAsync(fn)` | Wrap async throwing function |
| `fromPromise(promise)` | Convert Promise to Result |

### Optional Interop

| Function | Description |
|----------|-------------|
| `toOptional(result)` | Convert to value | undefined |
| `fromNullable(value, error)` | Convert nullable to Result |

### Side Effects

| Function | Description |
|----------|-------------|
| `tap(result, fn)` | Call function for Ok (for logging) |
| `tapErr(result, fn)` | Call function for Err (for logging) |

## Examples

### Parsing and Validation Chain

```typescript
import { andThen, tryCatch, map, err, ok, type Result } from '@tuulbelt/result-type';

interface User {
  name: string;
  age: number;
}

function parseJson(json: string): Result<unknown, Error> {
  return tryCatch(() => JSON.parse(json));
}

function validateUser(data: unknown): Result<User, Error> {
  if (typeof data !== 'object' || data === null) {
    return err(new Error('Expected object'));
  }
  const obj = data as Record<string, unknown>;
  if (typeof obj.name !== 'string' || typeof obj.age !== 'number') {
    return err(new Error('Invalid user data'));
  }
  return ok({ name: obj.name, age: obj.age });
}

const result = andThen(parseJson('{"name":"Alice","age":30}'), validateUser);
// { ok: true, value: { name: 'Alice', age: 30 } }
```

### Configuration with Fallbacks

```typescript
import { orElse, ok, err, type Result } from '@tuulbelt/result-type';

interface Config {
  port: number;
  host: string;
}

function readEnvConfig(): Result<Config, string> {
  const port = process.env.PORT;
  if (!port) return err('PORT not set');
  return ok({ port: parseInt(port, 10), host: process.env.HOST || 'localhost' });
}

function defaultConfig(): Result<Config, never> {
  return ok({ port: 8080, host: '0.0.0.0' });
}

// Use env config or fall back to defaults
const config = orElse(readEnvConfig(), defaultConfig);
```

### Async Operations

```typescript
import { fromPromise, andThen, map, type Result } from '@tuulbelt/result-type';

async function fetchUser(id: number): Promise<Result<User, Error>> {
  const result = await fromPromise(fetch(`/api/users/${id}`));
  if (!result.ok) return result;

  return fromPromise(result.value.json());
}

const user = await fetchUser(123);
```

### Collecting Multiple Results

```typescript
import { all, partition, ok, err } from '@tuulbelt/result-type';

// All must succeed
const allResults = all([ok(1), ok(2), ok(3)]);
// { ok: true, value: [1, 2, 3] }

// Separate successes and failures
const [successes, failures] = partition([ok(1), err('a'), ok(2)]);
// successes = [1, 2], failures = ['a']
```

## Types

```typescript
// Core Result type
type Result<T, E = Error> =
  | { readonly ok: true; readonly value: T }
  | { readonly ok: false; readonly error: E };

// Variant types
type Ok<T> = { readonly ok: true; readonly value: T };
type Err<E> = { readonly ok: false; readonly error: E };

// Type utilities
type ResultValue<R> = R extends Result<infer T, unknown> ? T : never;
type ResultError<R> = R extends Result<unknown, infer E> ? E : never;
```

## Comparison with Other Libraries

| Feature | @tuulbelt/result-type | neverthrow | ts-results | fp-ts |
|---------|----------------------|------------|------------|-------|
| Zero deps | Yes | Yes | Yes | No |
| Bundle size | ~2KB | ~4KB | ~3KB | ~50KB+ |
| Method chaining | Functions | Methods | Methods | Pipe |
| Async support | Yes | Yes | Yes | Yes |
| Type inference | Full | Full | Full | Complex |

## Philosophy

This library follows Tuulbelt principles:

- **Zero Dependencies** — Only uses TypeScript and Node.js standard library
- **Type-Safe** — Full TypeScript support with strict type inference
- **Functional** — Composable functions, no class methods
- **Minimal** — Only essential operations, no bloat
- **Documented** — Clear API documentation with examples

## License

MIT

## Part of Tuulbelt

This library is part of the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) ecosystem of zero-dependency tools and libraries.
