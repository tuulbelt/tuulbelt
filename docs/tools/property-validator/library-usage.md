# Library Usage

Import and use Property Validator in your TypeScript or JavaScript projects.

## Installation

```bash
# Add as dependency via git URL
npm install --save git+https://github.com/tuulbelt/property-validator.git
```

Or use directly from source:
```bash
git clone https://github.com/tuulbelt/property-validator.git
```

## Basic Import

```typescript
import { v, validate } from 'property-validator';
```

## Core API

### `validate(data, schema)`

Validates data against a schema and returns a result type.

```typescript
const result = validate(unknownData, UserSchema);

if (result.ok) {
  // Type-safe access to validated data
  console.log(result.value);
} else {
  // Access error details
  console.error(result.error.message);
}
```

### Schema Builders

#### `v.string()`

Validates string values.

```typescript
const NameSchema = v.string();
const result = validate("Alice", NameSchema);
// result.ok === true
```

#### `v.number()`

Validates number values.

```typescript
const AgeSchema = v.number();
const result = validate(30, AgeSchema);
// result.ok === true
```

#### `v.boolean()`

Validates boolean values.

```typescript
const FlagSchema = v.boolean();
const result = validate(true, FlagSchema);
// result.ok === true
```

#### `v.object(props)`

Validates objects with specific properties.

```typescript
const UserSchema = v.object({
  name: v.string(),
  age: v.number(),
  active: v.boolean()
});

const result = validate({
  name: "Alice",
  age: 30,
  active: true
}, UserSchema);
```

#### `v.array(schema)`

Validates arrays of a specific type.

```typescript
const NumberListSchema = v.array(v.number());
const result = validate([1, 2, 3], NumberListSchema);

const UserListSchema = v.array(v.object({
  name: v.string(),
  age: v.number()
}));
```

#### `v.optional(schema)`

Makes a field optional (can be undefined).

```typescript
const UserSchema = v.object({
  name: v.string(),
  age: v.optional(v.number())  // age can be undefined
});

// Valid:
validate({ name: "Alice" }, UserSchema);
validate({ name: "Alice", age: 30 }, UserSchema);
```

#### `v.nullable(schema)`

Allows null values.

```typescript
const UserSchema = v.object({
  name: v.string(),
  email: v.nullable(v.string())  // email can be null
});

// Valid:
validate({ name: "Alice", email: null }, UserSchema);
validate({ name: "Alice", email: "alice@example.com" }, UserSchema);
```

## Type Inference

Property Validator automatically infers TypeScript types from schemas:

```typescript
const UserSchema = v.object({
  name: v.string(),
  age: v.number(),
  email: v.optional(v.string())
});

type User = ReturnType<typeof UserSchema.parse>;
// Equivalent to:
// type User = {
//   name: string;
//   age: number;
//   email?: string;
// }

const result = validate(data, UserSchema);
if (result.ok) {
  // TypeScript knows result.value is User type
  const user: User = result.value;
}
```

## Error Handling

```typescript
const result = validate(invalidData, UserSchema);

if (!result.ok) {
  console.log(result.error.message);
  // "Validation failed at age: expected number, got string"

  console.log(result.error.path);
  // ["age"]

  console.log(result.error.expected);
  // "number"

  console.log(result.error.actual);
  // "string"
}
```

## Common Patterns

### Validate API Response

```typescript
async function fetchUser(id: number) {
  const response = await fetch(`/api/users/${id}`);
  const data = await response.json();

  const result = validate(data, UserSchema);
  if (!result.ok) {
    throw new Error(`Invalid API response: ${result.error.message}`);
  }

  return result.value;  // Type-safe user object
}
```

### Validate Configuration

```typescript
import { readFileSync } from 'fs';

const ConfigSchema = v.object({
  port: v.number(),
  host: v.string(),
  debug: v.optional(v.boolean())
});

function loadConfig(path: string) {
  const content = readFileSync(path, 'utf-8');
  const data = JSON.parse(content);

  const result = validate(data, ConfigSchema);
  if (!result.ok) {
    throw new Error(`Invalid config: ${result.error.message}`);
  }

  return result.value;
}
```

### Validate Function Arguments

```typescript
function createUser(data: unknown) {
  const result = validate(data, UserSchema);
  if (!result.ok) {
    return { error: result.error.message };
  }

  // data is now type-safe
  const user = result.value;
  // ... create user in database
  return { user };
}
```

## Next Steps

- [Examples](/tools/property-validator/examples) - Real-world scenarios
- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface
