# Getting Started

Get up and running with Property Validator in minutes.

## Installation

### Clone the Repository

```bash
git clone https://github.com/tuulbelt/property-validator.git
cd property-validator
```

### Install Dev Dependencies

```bash
npm install
npm link     # Enable the 'propval' command globally
```

:::tip Zero Runtime Dependencies
Property Validator has **zero runtime dependencies**. The `npm install` step only installs development tools (TypeScript compiler, test runner). The tool itself uses only Node.js built-in modules.

**Bundle size:** 30KB minified, 8KB gzipped
:::

## Quick Example

### Basic Validation (Named Imports)

```typescript
import { object, string, number, validate } from '@tuulbelt/property-validator';

// Define a schema with built-in validators
const UserSchema = object({
  name: string().min(1),
  age: number().positive(),
  email: string().email()
});

// Validate unknown data
const unknownData = {
  name: "Alice",
  age: 30,
  email: "alice@example.com"
};

const result = validate(UserSchema, unknownData);

if (result.ok) {
  // TypeScript knows the exact type
  console.log(`User: ${result.value.name}, Age: ${result.value.age}`);
} else {
  console.error(`Validation failed: ${result.error.message}`);
}
```

### Namespace Import (Fluent API)

```typescript
import { v, validate } from '@tuulbelt/property-validator/v';

const UserSchema = v.object({
  name: v.string(),
  age: v.number(),
  email: v.string()
});
```

## Three Ways to Validate

Property Validator offers three APIs for different use cases:

### `validate()` - Full Details

Use when you need error messages (forms, APIs):

```typescript
import { object, string, validate } from '@tuulbelt/property-validator';

const UserSchema = object({ name: string().min(1) });
const result = validate(UserSchema, data);

if (result.ok) {
  console.log(result.value.name);  // Type-safe
} else {
  console.error(result.error.message);
  console.error(result.error.path);  // ["name"]
}
```

### `check()` - Fast Boolean

Use when you only need pass/fail:

```typescript
import { object, string, check } from '@tuulbelt/property-validator';

const UserSchema = object({ name: string() });

// 3x faster than validate() - skips error construction
if (check(UserSchema, data)) {
  processUser(data);
}

// Great for filtering
const validUsers = users.filter(u => check(UserSchema, u));
```

### `compileCheck()` - Maximum Speed

Use for hot paths and large datasets:

```typescript
import { object, number, compileCheck } from '@tuulbelt/property-validator';

const PointSchema = object({ x: number(), y: number() });
const isValidPoint = compileCheck(PointSchema);  // Compile once

// Use in performance-critical loops
for (const point of points) {
  if (isValidPoint(point)) {
    render(point);
  }
}
```

## Built-in Validators

### String Validators

```typescript
import { string } from '@tuulbelt/property-validator';

// Format validation
const email = string().email();
const website = string().url();
const id = string().uuid();

// Length constraints
const username = string().min(3).max(20);
const code = string().length(6);

// Pattern matching
const phone = string().pattern(/^\+?[1-9]\d{1,14}$/);
```

### Number Validators

```typescript
import { number } from '@tuulbelt/property-validator';

// Sign constraints
const positive = number().positive();
const nonNegative = number().nonnegative();

// Bounds
const percentage = number().range(0, 100);
const port = number().int().range(1, 65535);

// Integer only
const count = number().int();
```

## Optional and Nullable

```typescript
import { object, string, number, optional, nullable } from '@tuulbelt/property-validator';

const UserSchema = object({
  name: string(),
  age: optional(number()),     // Can be undefined
  bio: nullable(string()),     // Can be null
  nickname: string().nullish() // Can be undefined or null
});

// All valid:
validate(UserSchema, { name: "Alice" });
validate(UserSchema, { name: "Alice", age: undefined, bio: null });
validate(UserSchema, { name: "Alice", age: 30, bio: "Developer" });
```

## Default Values

```typescript
import { object, string, number, boolean, validate } from '@tuulbelt/property-validator';

const ConfigSchema = object({
  port: number().default(3000),
  host: string().default('localhost'),
  debug: boolean().default(false)
});

const result = validate(ConfigSchema, {});
console.log(result.value);
// { port: 3000, host: 'localhost', debug: false }
```

## Nested Objects

```typescript
import { object, string, array, validate } from '@tuulbelt/property-validator';

const AddressSchema = object({
  street: string(),
  city: string(),
  zip: string().pattern(/^\d{5}$/)
});

const UserSchema = object({
  name: string().min(1),
  addresses: array(AddressSchema)
});

const result = validate(UserSchema, {
  name: "Charlie",
  addresses: [{
    street: "123 Main St",
    city: "Springfield",
    zip: "12345"
  }]
});
```

## Understanding Results

Property Validator returns a result type instead of throwing exceptions:

### Success Result

```typescript
{
  ok: true,
  value: <validated data>  // Type matches schema
}
```

### Error Result

```typescript
{
  ok: false,
  error: {
    message: string,           // Human-readable error
    path: (string | number)[], // Path to invalid field
    expected: string,          // Expected type
    actual: string,            // Actual type
    format(style): string      // Format as 'text', 'json', or 'color'
  }
}
```

## Error Handling Example

```typescript
import { object, string, number, validate } from '@tuulbelt/property-validator';

const UserSchema = object({
  name: string().min(1),
  age: number().positive()
});

const result = validate(UserSchema, {
  name: "Alice",
  age: "thirty"  // Invalid: string instead of number
});

if (!result.ok) {
  console.log(result.error.message);
  // "Validation failed at age: expected number, got string"

  console.log(result.error.path);
  // ["age"]

  console.log(result.error.format('text'));
  // "Validation failed at age: expected number, got 'thirty'"
}
```

## Next Steps

- [CLI Usage](/tools/property-validator/cli-usage) - Use from command line
- [Library Usage](/tools/property-validator/library-usage) - Comprehensive API guide
- [Examples](/tools/property-validator/examples) - Real-world scenarios
- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
