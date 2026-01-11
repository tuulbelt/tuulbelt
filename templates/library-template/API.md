# API Reference

Complete API documentation for `@tuulbelt/library-name`.

## Table of Contents

- [Types](#types)
- [Functions](#functions)
- [Utilities](#utilities)

---

## Types

### `Result<T, E>`

A discriminated union representing success or failure.

```typescript
type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };
```

**Usage:**

```typescript
const result = process('input');
if (result.ok) {
  console.log(result.value); // T
} else {
  console.error(result.error); // E
}
```

### `LibraryOptions`

Configuration options for the library.

```typescript
interface LibraryOptions {
  debug?: boolean; // Enable debug mode (default: false)
}
```

---

## Functions

### `process(input, options?)`

Main entry point for the library.

**Signature:**

```typescript
function process(
  input: string,
  options?: LibraryOptions
): Result<string>
```

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `input` | `string` | Yes | The input to process |
| `options` | `LibraryOptions` | No | Configuration options |

**Returns:** `Result<string>` - Success with processed output or failure with error.

**Example:**

```typescript
import { process } from '@tuulbelt/library-name';

const result = process('hello world');
if (result.ok) {
  console.log(result.value); // "HELLO WORLD"
}
```

**Errors:**

| Error Message | Cause |
|---------------|-------|
| `"Input is required"` | Empty string provided |

---

### `validate(input)`

Validates input before processing.

**Signature:**

```typescript
function validate(input: unknown): input is string
```

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `input` | `unknown` | Yes | Value to validate |

**Returns:** `boolean` - Type guard narrowing to `string`.

**Example:**

```typescript
import { validate, process } from '@tuulbelt/library-name';

const input: unknown = getUserInput();
if (validate(input)) {
  const result = process(input); // input is string
}
```

---

## Utilities

### `ok(value)`

Creates a successful Result.

**Signature:**

```typescript
function ok<T>(value: T): Result<T, never>
```

**Example:**

```typescript
import { ok } from '@tuulbelt/library-name';

const result = ok('success');
// { ok: true, value: 'success' }
```

---

### `err(error)`

Creates a failed Result.

**Signature:**

```typescript
function err<E>(error: E): Result<never, E>
```

**Example:**

```typescript
import { err } from '@tuulbelt/library-name';

const result = err(new Error('Something went wrong'));
// { ok: false, error: Error('Something went wrong') }
```

---

## Type Utilities

Additional type utilities exported from `src/types.ts`:

### `Processor<T, U, E>`

A function type that processes input and returns a Result.

```typescript
type Processor<T, U, E = Error> = (input: T) => Result<U, E>;
```

### `Validator<T>`

A function type that validates input.

```typescript
type Validator<T> = (input: unknown) => input is T;
```

### `ResultValue<R>`

Extracts the success type from a Result.

```typescript
type ResultValue<R> = R extends Result<infer T, unknown> ? T : never;

// Example:
type MyResult = Result<string, Error>;
type Value = ResultValue<MyResult>; // string
```

### `ResultError<R>`

Extracts the error type from a Result.

```typescript
type ResultError<R> = R extends Result<unknown, infer E> ? E : never;

// Example:
type MyResult = Result<string, Error>;
type Err = ResultError<MyResult>; // Error
```
