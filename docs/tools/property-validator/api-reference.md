# API Reference

Complete API documentation for Property Validator.

## Core Functions

### `validate(data, schema)`

Validates data against a schema and returns a result type.

**Parameters:**
- `data: unknown` - The data to validate
- `schema: Schema` - The schema to validate against

**Returns:** `ValidationResult<T>`
```typescript
type ValidationResult<T> =
  | { ok: true; value: T }
  | { ok: false; error: ValidationError };
```

**Example:**
```typescript
const result = validate(data, UserSchema);
if (result.ok) {
  console.log(result.value);  // Type-safe validated data
} else {
  console.error(result.error.message);
}
```

## Schema Builders

### `v.string()`

Creates a string validator.

**Returns:** `StringSchema`

**Example:**
```typescript
const NameSchema = v.string();
validate("Alice", NameSchema);  // { ok: true, value: "Alice" }
validate(123, NameSchema);      // { ok: false, error: ... }
```

---

### `v.number()`

Creates a number validator.

**Returns:** `NumberSchema`

**Example:**
```typescript
const AgeSchema = v.number();
validate(30, AgeSchema);      // { ok: true, value: 30 }
validate("30", AgeSchema);    // { ok: false, error: ... }
```

---

### `v.boolean()`

Creates a boolean validator.

**Returns:** `BooleanSchema`

**Example:**
```typescript
const FlagSchema = v.boolean();
validate(true, FlagSchema);   // { ok: true, value: true }
validate(1, FlagSchema);      // { ok: false, error: ... }
```

---

### `v.object(properties)`

Creates an object validator with specific properties.

**Parameters:**
- `properties: Record<string, Schema>` - Object property schemas

**Returns:** `ObjectSchema<T>`

**Example:**
```typescript
const UserSchema = v.object({
  name: v.string(),
  age: v.number(),
  active: v.boolean()
});

type User = {
  name: string;
  age: number;
  active: boolean;
};

validate({ name: "Alice", age: 30, active: true }, UserSchema);
// { ok: true, value: User }
```

---

### `v.array(schema)`

Creates an array validator for arrays of a specific type.

**Parameters:**
- `schema: Schema` - Schema for array elements

**Returns:** `ArraySchema<T>`

**Example:**
```typescript
const NumberListSchema = v.array(v.number());
validate([1, 2, 3], NumberListSchema);
// { ok: true, value: number[] }

const UserListSchema = v.array(v.object({
  name: v.string(),
  age: v.number()
}));
validate([
  { name: "Alice", age: 30 },
  { name: "Bob", age: 25 }
], UserListSchema);
// { ok: true, value: User[] }
```

---

### `v.optional(schema)`

Makes a field optional (can be `undefined`).

**Parameters:**
- `schema: Schema` - Base schema

**Returns:** `OptionalSchema<T>`

**Example:**
```typescript
const UserSchema = v.object({
  name: v.string(),
  age: v.optional(v.number())  // age can be undefined
});

validate({ name: "Alice" }, UserSchema);
// { ok: true, value: { name: "Alice", age?: number } }

validate({ name: "Alice", age: undefined }, UserSchema);
// { ok: true, value: { name: "Alice", age?: number } }

validate({ name: "Alice", age: 30 }, UserSchema);
// { ok: true, value: { name: "Alice", age: 30 } }
```

---

### `v.nullable(schema)`

Allows `null` values in addition to the base type.

**Parameters:**
- `schema: Schema` - Base schema

**Returns:** `NullableSchema<T>`

**Example:**
```typescript
const UserSchema = v.object({
  name: v.string(),
  email: v.nullable(v.string())  // email can be null
});

validate({ name: "Alice", email: null }, UserSchema);
// { ok: true, value: { name: "Alice", email: string | null } }

validate({ name: "Alice", email: "alice@example.com" }, UserSchema);
// { ok: true, value: { name: "Alice", email: "alice@example.com" } }
```

## Types

### `ValidationResult<T>`

The result of a validation operation.

```typescript
type ValidationResult<T> =
  | SuccessResult<T>
  | ErrorResult;

type SuccessResult<T> = {
  ok: true;
  value: T;
};

type ErrorResult = {
  ok: false;
  error: ValidationError;
};
```

---

### `ValidationError`

Error details when validation fails.

```typescript
type ValidationError = {
  message: string;    // Human-readable error message
  path: string[];     // Path to invalid field (e.g., ["user", "age"])
  expected: string;   // Expected type (e.g., "number")
  actual: string;     // Actual type (e.g., "string")
};
```

**Example:**
```typescript
const result = validate({ name: 123 }, v.object({ name: v.string() }));
if (!result.ok) {
  console.log(result.error);
  // {
  //   message: "Validation failed at name: expected string, got number",
  //   path: ["name"],
  //   expected: "string",
  //   actual: "number"
  // }
}
```

---

### `Schema`

Base type for all schema builders.

```typescript
type Schema =
  | StringSchema
  | NumberSchema
  | BooleanSchema
  | ObjectSchema<any>
  | ArraySchema<any>
  | OptionalSchema<any>
  | NullableSchema<any>;
```

---

## Type Inference

Property Validator automatically infers TypeScript types from schemas.

### `Infer<T>`

Utility type to extract the TypeScript type from a schema.

```typescript
type Infer<T extends Schema> = /* inferred type */;

// Example:
const UserSchema = v.object({
  name: v.string(),
  age: v.number()
});

type User = Infer<typeof UserSchema>;
// Equivalent to:
// type User = {
//   name: string;
//   age: number;
// };
```

**Usage:**
```typescript
const result = validate(data, UserSchema);
if (result.ok) {
  const user: Infer<typeof UserSchema> = result.value;
  // TypeScript knows user.name is string and user.age is number
}
```

## CLI Reference

### Command Syntax

```bash
propval --schema <file> --data <file> [options]
```

### Options

| Option | Description | Required | Default |
|--------|-------------|----------|---------|
| `--schema <file>` | Path to JSON schema file | Yes | - |
| `--data <file>` | Path to JSON data file | Yes | - |
| `--verbose` | Show detailed error information | No | `false` |
| `--json` | Output results as JSON | No | `false` |
| `--help` | Show help message | No | - |

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Validation passed |
| `1` | Validation failed |
| `2` | Invalid arguments or file not found |

## Error Handling Best Practices

### Pattern 1: Early Return

```typescript
function processUser(data: unknown) {
  const result = validate(data, UserSchema);
  if (!result.ok) {
    return { error: result.error.message };
  }

  // Continue with validated data
  const user = result.value;
  // ...
}
```

### Pattern 2: Throw on Failure

```typescript
function requireValidUser(data: unknown): User {
  const result = validate(data, UserSchema);
  if (!result.ok) {
    throw new Error(`Invalid user data: ${result.error.message}`);
  }

  return result.value;
}
```

### Pattern 3: Custom Error Formatting

```typescript
function formatValidationError(error: ValidationError) {
  return {
    field: error.path.join('.'),
    message: `Invalid ${error.path.join('.')}: expected ${error.expected}, got ${error.actual}`,
    code: 'VALIDATION_ERROR'
  };
}

const result = validate(data, UserSchema);
if (!result.ok) {
  const formatted = formatValidationError(result.error);
  // Use formatted error in API response
}
```

## Next Steps

- [Examples](/tools/property-validator/examples) - Real-world scenarios
- [Library Usage](/tools/property-validator/library-usage) - Import and use in code
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface
- [Getting Started](/tools/property-validator/getting-started) - Quick start guide
