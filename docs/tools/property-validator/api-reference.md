# API Reference

Complete API documentation for Property Validator v0.9.0.

## Imports

### Named Exports (v0.9.0+, Tree-Shakeable)

```typescript
// Import only what you need
import {
  string, number, boolean,     // Primitives
  array, tuple, object,        // Collections
  optional, nullable,          // Modifiers
  union, literal, lazy, enum_, // Special types
  validate, check, compile, compileCheck,  // Functions
  ValidationError              // Error class
} from '@tuulbelt/property-validator';
```

### Namespace Import (Fluent API)

```typescript
import { v, validate, check, compileCheck } from '@tuulbelt/property-validator/v';
```

### Type-Only Import

```typescript
import type {
  Validator,
  Result,
  ValidationOptions,
  StringValidator,
  NumberValidator
} from '@tuulbelt/property-validator/types';
```

## Core Functions

### `validate(validator, data, options?)`

Full validation with detailed error messages.

**Parameters:**
- `validator: Validator<T>` - The validator to use
- `data: unknown` - The data to validate
- `options?: ValidationOptions` - Optional configuration

**Returns:** `Result<T>`
```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: ValidationError };
```

**Example:**
```typescript
import { object, string, validate } from '@tuulbelt/property-validator';

const UserSchema = object({ name: string() });
const result = validate(UserSchema, data);

if (result.ok) {
  console.log(result.value.name);  // Type-safe
} else {
  console.error(result.error.format('text'));
}
```

**Best for:** Form validation, API responses, debugging

---

### `check(validator, data)` (v0.8.5+)

Fast boolean-only validation. Skips error path computation entirely.

**Parameters:**
- `validator: Validator<T>` - The validator to use
- `data: unknown` - The data to validate

**Returns:** `boolean`

**Example:**
```typescript
import { check, object, string } from '@tuulbelt/property-validator';

const UserSchema = object({ name: string() });

// Fast pass/fail check
if (check(UserSchema, data)) {
  processUser(data);
}

// Filter arrays efficiently
const validUsers = users.filter(u => check(UserSchema, u));
```

**Performance:** ~3x faster than `validate()` for valid data, ~6x faster for invalid data

**Best for:** Conditionals, filtering, type guards

---

### `compileCheck(validator)` (v0.8.5+)

Pre-compile a validator for maximum-speed boolean validation. Returns a cached function.

**Parameters:**
- `validator: Validator<T>` - The validator to compile

**Returns:** `(data: unknown) => boolean`

**Example:**
```typescript
import { compileCheck, object, number } from '@tuulbelt/property-validator';

const PointSchema = object({ x: number(), y: number() });
const isValidPoint = compileCheck(PointSchema);  // Compile once

// Use in hot loops
for (const point of points) {
  if (isValidPoint(point)) {
    render(point);
  }
}
```

**Performance:** Additional 5-15% faster than `check()` for unions

**Best for:** Hot paths, large datasets, performance-critical loops

---

### `compile(validator)`

Pre-compile a validator for repeated use with full validation.

**Parameters:**
- `validator: Validator<T>` - The validator to compile

**Returns:** `CompiledValidator<T>` with `.validate(data)` method

**Example:**
```typescript
import { compile, object, string } from '@tuulbelt/property-validator';

const UserSchema = object({ name: string() });
const compiledUser = compile(UserSchema);

// Use repeatedly
for (const data of dataList) {
  const result = compiledUser.validate(data);
}
```

---

## Choosing the Right API

| Use Case | API | Why |
|----------|-----|-----|
| Form validation | `validate()` | Need error messages for UX |
| API request validation | `validate()` | Need detailed errors for debugging |
| Type guards / conditionals | `check()` | Simple pass/fail, faster |
| Filtering arrays | `check()` | Boolean predicate needed |
| High-throughput pipelines | `compileCheck()` | Maximum speed, pre-compiled |
| Same schema validated 1000+ times | `compileCheck()` | Compilation overhead amortized |

---

## Primitive Validators

### `string()` / `v.string()`

Creates a string validator with chainable constraints.

**Basic:**
```typescript
const NameSchema = string();
validate(NameSchema, "Alice");  // { ok: true, value: "Alice" }
validate(NameSchema, 123);      // { ok: false, error: ... }
```

**Built-in String Constraints (v0.8.5+):**

| Method | Description | Example |
|--------|-------------|---------|
| `.email()` | RFC 5322 email | `string().email()` |
| `.url()` | HTTP/HTTPS URL | `string().url()` |
| `.uuid()` | UUID v1-v5 | `string().uuid()` |
| `.pattern(regex, msg?)` | Custom regex | `string().pattern(/^\d+$/)` |
| `.min(n)` | Minimum length | `string().min(1)` |
| `.max(n)` | Maximum length | `string().max(100)` |
| `.length(n)` | Exact length | `string().length(10)` |
| `.nonempty()` | Non-empty | `string().nonempty()` |
| `.startsWith(s)` | Starts with prefix | `string().startsWith('http')` |
| `.endsWith(s)` | Ends with suffix | `string().endsWith('.json')` |
| `.includes(s)` | Contains substring | `string().includes('@')` |

**Example:**
```typescript
const EmailSchema = string().email();
validate(EmailSchema, "alice@example.com");  // ✓
validate(EmailSchema, "not-an-email");       // ✗

const UsernameSchema = string().min(3).max(20).pattern(/^[a-z0-9_]+$/);
validate(UsernameSchema, "john_doe");  // ✓
```

---

### `number()` / `v.number()`

Creates a number validator with chainable constraints.

**Basic:**
```typescript
const AgeSchema = number();
validate(AgeSchema, 30);    // { ok: true, value: 30 }
validate(AgeSchema, "30");  // { ok: false, error: ... }
```

**Built-in Number Constraints (v0.8.5+):**

| Method | Description | Example |
|--------|-------------|---------|
| `.int()` | Integer only | `number().int()` |
| `.positive()` | > 0 | `number().positive()` |
| `.negative()` | < 0 | `number().negative()` |
| `.nonnegative()` | >= 0 | `number().nonnegative()` |
| `.nonpositive()` | <= 0 | `number().nonpositive()` |
| `.min(n)` | Minimum value | `number().min(0)` |
| `.max(n)` | Maximum value | `number().max(100)` |
| `.range(min, max)` | Inclusive range | `number().range(0, 100)` |
| `.finite()` | Not Infinity/NaN | `number().finite()` |
| `.safeInt()` | Safe integer range | `number().safeInt()` |

**Example:**
```typescript
const AgeSchema = number().int().positive().max(150);
validate(AgeSchema, 25);    // ✓
validate(AgeSchema, -5);    // ✗ "Number must be positive"
validate(AgeSchema, 25.5);  // ✗ "Number must be an integer"

const PercentageSchema = number().range(0, 100);
validate(PercentageSchema, 50);   // ✓
validate(PercentageSchema, 150);  // ✗
```

---

### `boolean()` / `v.boolean()`

Creates a boolean validator.

```typescript
const FlagSchema = boolean();
validate(FlagSchema, true);  // { ok: true, value: true }
validate(FlagSchema, 1);     // { ok: false, error: ... }
```

---

## Collection Validators

### `object(shape)` / `v.object(shape)`

Creates an object validator with specific properties.

**Parameters:**
- `shape: Record<string, Validator>` - Object property validators

```typescript
const UserSchema = object({
  name: string().min(1),
  age: number().positive(),
  email: string().email()
});

validate(UserSchema, {
  name: "Alice",
  age: 30,
  email: "alice@example.com"
});  // ✓
```

---

### `array(itemValidator)` / `v.array(itemValidator)`

Creates an array validator with optional length constraints.

**Chainable methods:**
- `.min(n)` - Minimum length
- `.max(n)` - Maximum length
- `.length(n)` - Exact length
- `.nonempty()` - At least 1 element

```typescript
const NumbersSchema = array(number());
validate(NumbersSchema, [1, 2, 3]);  // ✓

const TagsSchema = array(string()).min(1).max(5);
validate(TagsSchema, ["typescript", "validation"]);  // ✓

const NonEmptySchema = array(string()).nonempty();
validate(NonEmptySchema, []);  // ✗
```

---

### `tuple(validators)` / `v.tuple(validators)`

Creates a tuple validator for fixed-length arrays.

```typescript
const PointSchema = tuple([number(), number()]);
validate(PointSchema, [10, 20]);  // ✓
validate(PointSchema, [10]);      // ✗ (wrong length)

const MixedSchema = tuple([string(), number(), boolean()]);
validate(MixedSchema, ["Alice", 30, true]);  // ✓
```

---

## Type Modifiers

### `optional(validator)` / `.optional()`

Makes a field optional (allows `undefined`).

```typescript
// Function style
const Schema = object({ age: optional(number()) });

// Method style (preferred)
const Schema = object({ age: number().optional() });

validate(Schema, { age: undefined });  // ✓
validate(Schema, { age: 30 });         // ✓
validate(Schema, {});                  // ✓
```

---

### `nullable(validator)` / `.nullable()`

Allows `null` values.

```typescript
const Schema = object({ email: string().nullable() });

validate(Schema, { email: null });              // ✓
validate(Schema, { email: "alice@example.com" });  // ✓
```

---

### `.nullish()`

Allows both `undefined` and `null`.

```typescript
const Schema = object({ value: string().nullish() });

validate(Schema, { value: undefined });  // ✓
validate(Schema, { value: null });       // ✓
validate(Schema, { value: "hello" });    // ✓
```

---

### `.default(value)`

Provides a default value when input is `undefined`.

```typescript
const ConfigSchema = object({
  port: number().default(3000),
  host: string().default('localhost'),
  debug: boolean().default(false)
});

validate(ConfigSchema, {});
// { ok: true, value: { port: 3000, host: 'localhost', debug: false } }

// Lazy default (function)
const Schema = object({
  timestamp: number().default(() => Date.now())
});
```

---

## Union & Literal Types

### `union(validators)` / `v.union(validators)`

Creates a union type (value must match one of the validators).

```typescript
const StringOrNumber = union([string(), number()]);
validate(StringOrNumber, "hello");  // ✓
validate(StringOrNumber, 42);       // ✓
validate(StringOrNumber, true);     // ✗

// Discriminated unions
const ResultSchema = union([
  object({ type: literal('success'), data: string() }),
  object({ type: literal('error'), message: string() })
]);
```

---

### `literal(value)` / `v.literal(value)`

Exact value matching using `===`.

```typescript
const AdminRole = literal('admin');
validate(AdminRole, 'admin');  // ✓
validate(AdminRole, 'user');   // ✗

// Use in discriminated unions
const SuccessResponse = object({
  status: literal('success'),
  data: string()
});
```

---

### `enum_(values)` / `v.enum(values)`

Union of string literals (syntactic sugar).

```typescript
const StatusSchema = enum_(['active', 'inactive', 'pending']);
// Note: Use enum_ with named import (enum is reserved keyword)

validate(StatusSchema, 'active');    // ✓
validate(StatusSchema, 'archived');  // ✗
```

---

### `lazy(fn)` / `v.lazy(fn)`

Deferred validator for recursive types.

```typescript
type TreeNode = {
  value: string;
  children: TreeNode[];
};

const TreeNodeSchema: Validator<TreeNode> = object({
  value: string(),
  children: array(lazy(() => TreeNodeSchema))
});
```

---

## Refinements & Transforms

### `.refine(predicate, message)`

Custom validation logic.

```typescript
const PositiveNumber = number().refine(
  n => n > 0,
  'Must be positive'
);

const Password = string()
  .min(8)
  .refine(s => /[A-Z]/.test(s), 'Must contain uppercase')
  .refine(s => /[0-9]/.test(s), 'Must contain number');
```

---

### `.transform(fn)`

Transform validated value (changes type).

```typescript
const ParsedInt = string().transform(s => parseInt(s, 10));
validate(ParsedInt, "42");  // { ok: true, value: 42 } (number)

const Normalized = string()
  .transform(s => s.trim())
  .transform(s => s.toLowerCase());
validate(Normalized, "  HELLO  ");  // { ok: true, value: "hello" }
```

---

## Types

### `Result<T>`

```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: ValidationError };
```

### `ValidationError`

```typescript
class ValidationError extends Error {
  message: string;
  path: (string | number)[];  // e.g., ["users", 2, "email"]
  expected: string;
  actual: string;

  format(style: 'json' | 'text' | 'color'): string;
  formatPathString(): string;  // "users[2].email"
}
```

### `Validator<T>`

```typescript
interface Validator<T> {
  validate(data: unknown): boolean;
  optional(): Validator<T | undefined>;
  nullable(): Validator<T | null>;
  nullish(): Validator<T | undefined | null>;
  default(value: T | (() => T)): Validator<T>;
  refine(check: (v: T) => boolean, message: string): Validator<T>;
  transform<U>(fn: (v: T) => U): Validator<U>;
}
```

### `ValidationOptions`

```typescript
interface ValidationOptions {
  maxDepth?: number;       // Default: 100
  maxProperties?: number;  // Default: 1000
  maxItems?: number;       // Default: 10000
}
```

---

## CLI Reference

### Command Syntax

```bash
propval [options] [data-file]
```

### Options

| Option | Description |
|--------|-------------|
| `--schema <file>` | Path to JSON schema file |
| `--check, -c` | Boolean-only output (exit code only) |
| `--api` | Display available validators |
| `--verbose` | Show detailed error information |
| `--json` | Output results as JSON |
| `--version, -V` | Show version |
| `--help` | Show help message |

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Validation passed |
| `1` | Validation failed |
| `2` | Invalid arguments or file not found |

---

## Next Steps

- [Examples](/tools/property-validator/examples) - Real-world scenarios
- [Library Usage](/tools/property-validator/library-usage) - Import and use in code
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface
- [Getting Started](/tools/property-validator/getting-started) - Quick start guide
