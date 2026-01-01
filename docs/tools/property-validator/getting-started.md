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
Property Validator has **zero runtime dependencies**. The `npm install` step only installs development tools (TypeScript compiler, test runner). The tool itself uses only Node.js built-in modules. The `npm link` command creates a global symlink so you can use the `propval` command from anywhere.
:::

## Quick Example

### Basic Validation

```typescript
import { v, validate } from 'property-validator';

// Define a schema
const UserSchema = v.object({
  name: v.string(),
  age: v.number(),
  email: v.string()
});

// Validate unknown data
const unknownData = {
  name: "Alice",
  age: 30,
  email: "alice@example.com"
};

const result = validate(unknownData, UserSchema);

if (result.ok) {
  // TypeScript knows result.value is { name: string, age: number, email: string }
  console.log(`User: ${result.value.name}, Age: ${result.value.age}`);
} else {
  console.error(`Validation failed: ${result.error.message}`);
}
```

### Optional Fields

```typescript
const UserSchema = v.object({
  name: v.string(),
  age: v.optional(v.number()),     // Optional field
  email: v.nullable(v.string())    // Can be null
});

// Valid:
validate({ name: "Bob" }, UserSchema);                    // age missing (OK)
validate({ name: "Bob", age: 25, email: null }, UserSchema);  // email null (OK)
```

### Nested Objects

```typescript
const AddressSchema = v.object({
  street: v.string(),
  city: v.string(),
  zip: v.string()
});

const UserSchema = v.object({
  name: v.string(),
  address: AddressSchema    // Nested schema
});

const result = validate({
  name: "Charlie",
  address: {
    street: "123 Main St",
    city: "Springfield",
    zip: "12345"
  }
}, UserSchema);
```

### Array Validation

```typescript
const UserListSchema = v.array(v.object({
  name: v.string(),
  age: v.number()
}));

const result = validate([
  { name: "Alice", age: 30 },
  { name: "Bob", age: 25 }
], UserListSchema);
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
    message: string,  // Human-readable error message
    path: string[],   // Path to invalid field
    expected: string, // Expected type
    actual: string    // Actual type
  }
}
```

## Validation Error Example

```typescript
const result = validate({
  name: "Alice",
  age: "thirty"  // Invalid: string instead of number
}, UserSchema);

if (!result.ok) {
  console.log(result.error.message);
  // "Validation failed at age: expected number, got string"

  console.log(result.error.path);
  // ["age"]
}
```

## Next Steps

- [CLI Usage](/tools/property-validator/cli-usage) - Use from command line
- [Library Usage](/tools/property-validator/library-usage) - Import in your code
- [Examples](/tools/property-validator/examples) - Real-world scenarios
- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
