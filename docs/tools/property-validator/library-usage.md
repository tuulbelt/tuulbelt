# Library Usage

Import and use Property Validator in your TypeScript or JavaScript projects.

## Installation

```bash
# Add as dependency via git URL
npm install --save git+https://github.com/tuulbelt/property-validator.git
```

Or clone directly:
```bash
git clone https://github.com/tuulbelt/property-validator.git
```

## Import Styles

### Named Exports (v0.9.0+, Recommended)

Import only what you need for optimal bundle size:

```typescript
import {
  string, number, boolean,     // Primitives
  array, tuple, object,        // Collections
  optional, nullable,          // Modifiers
  union, literal, lazy, enum_, // Special types
  validate, check, compileCheck  // Functions
} from '@tuulbelt/property-validator';

// Use directly
const UserSchema = object({
  name: string().min(1),
  age: number().positive()
});
```

### Namespace Import (Fluent API)

```typescript
import { v, validate } from '@tuulbelt/property-validator';

// Use via namespace
const UserSchema = v.object({
  name: v.string(),
  age: v.number()
});
```

### Type-Only Import

```typescript
import type {
  Validator,
  Result,
  ValidationError
} from '@tuulbelt/property-validator/types';
```

### Entry Points (v0.9.2+)

Property Validator provides multiple entry points for different use cases:

| Entry Point | Import From | Use Case |
|-------------|-------------|----------|
| Main | `@tuulbelt/property-validator` | Full API (v namespace + named exports) |
| /types | `@tuulbelt/property-validator/types` | Type definitions only |

**Example: Main entry point (all APIs):**
```typescript
import { v, validate, string, number, object, email, positive } from '@tuulbelt/property-validator';

// Fluent API with v namespace
const UserSchema = v.object({
  name: v.string().email(),
  age: v.number().positive()
});

// Or functional API with named exports
const AgeSchema = number(positive());
```

**Both styles from one import:**
- **Fluent API**: `v.string().email()` — Compact, chainable syntax
- **Functional API**: `string(email())` — Explicit imports, tree-shakeable refinements

### Functional Refinement API (v0.9.1+)

For maximum tree-shaking, use refinement functions instead of chained methods:

```typescript
import {
  string, number,               // Base validators
  email, url, minLength,        // String refinements
  int, positive, range,         // Number refinements
  validate
} from '@tuulbelt/property-validator';

// Functional composition (tree-shakeable)
const EmailSchema = string(email(), minLength(5));
const AgeSchema = number(int(), positive());
const PortSchema = number(int(), range(1, 65535));

validate(EmailSchema, 'test@example.com');  // ✓
validate(AgeSchema, 25);                     // ✓
validate(PortSchema, 8080);                  // ✓
```

**Available Refinement Exports:**

| Category | Refinements |
|----------|-------------|
| String Length | `minLength(n)`, `maxLength(n)`, `length(n)`, `nonempty()` |
| String Format | `email()`, `url()`, `uuid()`, `pattern(regex, message?)` |
| String Content | `startsWith()`, `endsWith()`, `includes()` |
| String Date/Time | `datetime()`, `date()`, `time()` |
| String Network | `ip()`, `ipv4()`, `ipv6()` |
| Number Type | `int()`, `safeInt()`, `finite()` |
| Number Sign | `positive()`, `negative()`, `nonnegative()`, `nonpositive()` |
| Number Range | `min(n)`, `max(n)`, `range(min, max)`, `multipleOf(n)` |
| Array Length | `minItems(n)`, `maxItems(n)`, `itemCount(n)`, `nonemptyArray()` |

**Chainable vs Functional (both from main entry):**

```typescript
import { v, string, email, minLength } from '@tuulbelt/property-validator';

// Chainable (compact, all methods bundled)
const schema1 = v.string().email().min(5);

// Functional (tree-shakeable, explicit imports)
const schema2 = string(email(), minLength(5));

// Both produce equivalent validators
```

---

## Three API Tiers

Property Validator v0.8.5+ offers three validation APIs optimized for different use cases:

### `validate()` - Full Validation

Returns detailed results with error information. Best for forms, APIs, debugging.

```typescript
import { object, string, number, validate } from '@tuulbelt/property-validator';

const UserSchema = object({
  name: string().min(1),
  age: number().positive()
});

const result = validate(UserSchema, unknownData);

if (result.ok) {
  console.log(result.value.name);  // Type-safe access
} else {
  console.error(result.error.message);
  console.error(result.error.path);      // ["age"]
  console.error(result.error.expected);  // "positive number"
}
```

**Performance:** ~170 ns per validation

### `check()` - Boolean Validation

Returns only `true`/`false`. Skips error path computation entirely.

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

**Performance:** ~60 ns per validation (~3x faster than `validate()`)

### `compileCheck()` - Pre-compiled Boolean

Pre-compiles validator for maximum speed in hot paths.

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

**Performance:** ~55 ns per validation (additional 5-15% faster than `check()`)

### Choosing the Right API

| Use Case | API | Why |
|----------|-----|-----|
| Form validation | `validate()` | Need error messages for UX |
| API request validation | `validate()` | Need detailed errors for debugging |
| Type guards / conditionals | `check()` | Simple pass/fail, faster |
| Filtering arrays | `check()` | Boolean predicate needed |
| High-throughput pipelines | `compileCheck()` | Maximum speed, pre-compiled |
| Same schema validated 1000+ times | `compileCheck()` | Compilation overhead amortized |

---

## Schema Builders

### Primitives

```typescript
import { string, number, boolean } from '@tuulbelt/property-validator';

// Basic validators
const NameSchema = string();
const AgeSchema = number();
const ActiveSchema = boolean();
```

### Built-in String Validators (v0.8.5+)

```typescript
// Email, URL, UUID validation
const EmailSchema = string().email();
const WebsiteSchema = string().url();
const IdSchema = string().uuid();

// Length constraints
const UsernameSchema = string().min(3).max(20);
const CodeSchema = string().length(6);
const RequiredSchema = string().nonempty();

// Pattern matching
const PhoneSchema = string().pattern(/^\+?[1-9]\d{1,14}$/);

// String content
const UrlSchema = string().startsWith('https://');
const JsonFileSchema = string().endsWith('.json');
const TagSchema = string().includes('@');
```

### Built-in Number Validators (v0.8.5+)

```typescript
// Sign constraints
const PositiveSchema = number().positive();      // > 0
const NegativeSchema = number().negative();      // < 0
const NonNegativeSchema = number().nonnegative(); // >= 0

// Value bounds
const PercentSchema = number().range(0, 100);
const PortSchema = number().int().range(1, 65535);
const PriceSchema = number().positive().finite();

// Integer validation
const CountSchema = number().int();
const SafeIdSchema = number().safeInt();
```

### Objects

```typescript
import { object, string, number, optional } from '@tuulbelt/property-validator';

const UserSchema = object({
  name: string().min(1),
  age: number().positive(),
  email: string().email(),
  bio: optional(string())  // Can be undefined
});

// Nested objects
const CompanySchema = object({
  name: string(),
  address: object({
    street: string(),
    city: string(),
    zip: string().pattern(/^\d{5}$/)
  })
});
```

### Arrays

```typescript
import { array, object, string, number } from '@tuulbelt/property-validator';

// Simple arrays
const NumbersSchema = array(number());
const TagsSchema = array(string()).min(1).max(10);

// Array of objects
const UsersSchema = array(object({
  name: string(),
  age: number()
}));

// Non-empty arrays
const RequiredTagsSchema = array(string()).nonempty();
```

### Tuples

```typescript
import { tuple, string, number, boolean } from '@tuulbelt/property-validator';

// Fixed-length arrays with specific types per position
const PointSchema = tuple([number(), number()]);
const RecordSchema = tuple([string(), number(), boolean()]);
```

### Unions

```typescript
import { union, string, number, literal, object } from '@tuulbelt/property-validator';

// Simple union
const StringOrNumber = union([string(), number()]);

// Discriminated union
const ResultSchema = union([
  object({ type: literal('success'), data: string() }),
  object({ type: literal('error'), message: string() })
]);
```

### Literals and Enums

```typescript
import { literal, enum_ } from '@tuulbelt/property-validator';

// Exact value
const AdminRole = literal('admin');

// Enum (union of literals)
const StatusSchema = enum_(['active', 'inactive', 'pending']);
```

### Recursive Types

```typescript
import { object, string, array, lazy } from '@tuulbelt/property-validator';
import type { Validator } from '@tuulbelt/property-validator/types';

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

## Type Modifiers

### Optional

```typescript
const UserSchema = object({
  name: string(),
  nickname: string().optional()  // Can be undefined
});

// Valid:
validate(UserSchema, { name: "Alice" });
validate(UserSchema, { name: "Alice", nickname: "Ali" });
```

### Nullable

```typescript
const UserSchema = object({
  name: string(),
  deletedAt: string().nullable()  // Can be null
});

// Valid:
validate(UserSchema, { name: "Alice", deletedAt: null });
```

### Default Values

```typescript
const ConfigSchema = object({
  port: number().default(3000),
  host: string().default('localhost'),
  debug: boolean().default(false)
});

const result = validate(ConfigSchema, {});
// result.value = { port: 3000, host: 'localhost', debug: false }

// Lazy defaults (called each time)
const TimestampSchema = object({
  createdAt: number().default(() => Date.now())
});
```

---

## Refinements and Transforms

### Custom Validation

```typescript
const PasswordSchema = string()
  .min(8)
  .refine(s => /[A-Z]/.test(s), 'Must contain uppercase')
  .refine(s => /[0-9]/.test(s), 'Must contain number')
  .refine(s => /[!@#$%]/.test(s), 'Must contain special character');

const EvenNumberSchema = number()
  .int()
  .refine(n => n % 2 === 0, 'Must be even');
```

### Value Transformation

```typescript
// Transform string to number
const ParsedIntSchema = string().transform(s => parseInt(s, 10));
validate(ParsedIntSchema, "42");  // { ok: true, value: 42 }

// Normalize strings
const NormalizedSchema = string()
  .transform(s => s.trim())
  .transform(s => s.toLowerCase());
validate(NormalizedSchema, "  HELLO  ");  // { ok: true, value: "hello" }
```

---

## Error Handling

### Error Properties

```typescript
const result = validate(UserSchema, invalidData);

if (!result.ok) {
  const { error } = result;

  console.log(error.message);   // Human-readable message
  console.log(error.path);      // ["users", 2, "email"]
  console.log(error.expected);  // "valid email"
  console.log(error.actual);    // "not-an-email"
}
```

### Error Formatting

```typescript
if (!result.ok) {
  // Different output formats
  console.log(result.error.format('text'));
  // "Validation failed at users[2].email: expected valid email, got 'not-an-email'"

  console.log(result.error.format('json'));
  // {"path":"users[2].email","expected":"valid email","actual":"not-an-email"}

  console.log(result.error.format('color'));
  // ANSI-colored output for terminals
}
```

---

## Common Patterns

### API Response Validation

```typescript
import { object, string, number, array, validate } from '@tuulbelt/property-validator';

const ApiResponseSchema = object({
  data: array(object({
    id: number(),
    name: string(),
    email: string().email()
  })),
  meta: object({
    total: number(),
    page: number()
  })
});

async function fetchUsers() {
  const response = await fetch('/api/users');
  const data = await response.json();

  const result = validate(ApiResponseSchema, data);
  if (!result.ok) {
    throw new Error(`Invalid API response: ${result.error.format('text')}`);
  }

  return result.value;  // Fully typed!
}
```

### Form Validation

```typescript
import { object, string, number, validate } from '@tuulbelt/property-validator';

const FormSchema = object({
  email: string().email(),
  password: string().min(8),
  age: number().int().positive().max(150)
});

function validateForm(formData: unknown) {
  const result = validate(FormSchema, formData);

  if (!result.ok) {
    return {
      valid: false,
      field: result.error.path[0],
      message: result.error.message
    };
  }

  return { valid: true, data: result.value };
}
```

### High-Throughput Filtering

```typescript
import { compileCheck, object, number } from '@tuulbelt/property-validator';

const PointSchema = object({
  x: number().finite(),
  y: number().finite()
});

// Compile once at startup
const isValidPoint = compileCheck(PointSchema);

// Use in performance-critical code
function processPoints(points: unknown[]) {
  return points
    .filter(isValidPoint)
    .map(p => ({ x: p.x * 2, y: p.y * 2 }));
}
```

### Configuration Validation

```typescript
import { object, string, number, boolean, validate } from '@tuulbelt/property-validator';
import { readFileSync } from 'node:fs';

const ConfigSchema = object({
  server: object({
    port: number().int().range(1, 65535).default(3000),
    host: string().default('localhost')
  }),
  database: object({
    url: string().url(),
    poolSize: number().int().positive().default(10)
  }),
  features: object({
    debug: boolean().default(false),
    metrics: boolean().default(true)
  })
});

function loadConfig(path: string) {
  const content = readFileSync(path, 'utf-8');
  const result = validate(ConfigSchema, JSON.parse(content));

  if (!result.ok) {
    throw new Error(`Invalid config at ${result.error.formatPathString()}: ${result.error.message}`);
  }

  return result.value;
}
```

---

## Next Steps

- [Examples](/tools/property-validator/examples) - More real-world scenarios
- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface
- [Getting Started](/tools/property-validator/getting-started) - Quick start guide
