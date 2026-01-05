# Property Validator

Runtime type validation with TypeScript inference and high performance.

## Overview

Property Validator provides schema-based runtime type validation with full TypeScript type inference and competitive performance. Beats Zod in all benchmark categories, achieves Valibot-tier performance with JIT optimization.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.9.2)

**Language:** TypeScript

**Tests:** 680 tests passing

**Bundle Size:** 30KB minified, 8KB gzipped

**Repository:** [tuulbelt/property-validator](https://github.com/tuulbelt/property-validator)

## Features

### <img src="/icons/package.svg" class="inline-icon" alt=""> Tree-Shakeable Imports (v0.9.0)

Import only what you need with named exports:

```typescript
// Named exports - tree-shakeable
import { string, number, object, validate } from '@tuulbelt/property-validator';

// Namespace import - still works
import { v, validate } from '@tuulbelt/property-validator';
```

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Three API Tiers (v0.8.5)

Choose your speed vs. detail trade-off:

| API | Speed | Returns | Use Case |
|-----|-------|---------|----------|
| `validate()` | ~170 ns | Result with errors | Forms, APIs, debugging |
| `check()` | ~60 ns | Boolean only | Filtering, conditionals |
| `compileCheck()` | ~55 ns | Pre-compiled boolean | Hot paths, pipelines |

### <img src="/icons/shield.svg" class="inline-icon" alt=""> Built-in Validators (v0.8.5)

Common validation patterns included:

```typescript
// String validators
v.string().email()    // RFC 5322 email
v.string().url()      // HTTP/HTTPS URL
v.string().uuid()     // UUID v1-v5
v.string().min(3).max(100)

// Number validators
v.number().int()      // Integer only
v.number().positive() // > 0
v.number().range(0, 100)
```

### <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Schema-Based Validation

Define schemas once, get both runtime validation and TypeScript types automatically.

### <img src="/icons/search.svg" class="inline-icon" alt=""> Clear Error Messages

Validation errors include exact paths to invalid fields, expected vs actual types, and helpful context.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Node.js built-ins. No `npm install` required in production.

## Performance

Property Validator v0.8.0+ uses JIT (Just-In-Time) compilation for validation-heavy workloads.

### vs Zod (6/6 wins)

Property Validator beats Zod in all benchmark categories - typically 3-25x faster.

### vs Valibot (6/7 wins)

| Category | Property Validator | Valibot | Winner |
|----------|-------------------|---------|--------|
| Primitives | 66 ns | 68 ns | **propval 1.02x** ✅ |
| Simple objects | 65 ns | 201 ns | **propval 3.09x** ✅ |
| Complex nested | 174 ns | 933 ns | **propval 5.36x** ✅ |
| Number arrays [100] | 112 ns | 671 ns | **propval 5.97x** ✅ |
| String arrays [100] | 157 ns | 665 ns | **propval 4.23x** ✅ |
| Unions | 88 ns | 83 ns | valibot 1.05x |

**Score: 6 wins, 1 near-tie** (improved from v0.7.5's 2 wins, 3 losses)

### API Performance Comparison

| Scenario | validate() | check() | compileCheck() |
|----------|------------|---------|----------------|
| Simple Object | 170 ns | 58 ns | 55 ns |
| Complex Nested | 190 ns | 60 ns | 58 ns |
| Union (3 types) | 90 ns | 66 ns | 56 ns |
| Invalid Data | 357 ns | 55 ns | 55 ns |

**Key insight:** `check()` is 6x faster for invalid data (skips error path entirely).

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/property-validator.git
cd property-validator
npm install
```

### Basic Usage

```typescript
import { object, string, number, validate } from '@tuulbelt/property-validator';

const UserSchema = object({
  name: string().min(1),
  age: number().positive(),
  email: string().email()
});

const result = validate(UserSchema, unknownData);
if (result.ok) {
  console.log(result.value); // TypeScript knows the exact type
} else {
  console.error(result.error.message);
}
```

### Fast Boolean Check

```typescript
import { check, object, string } from '@tuulbelt/property-validator';

const UserSchema = object({ name: string() });

// Fast pass/fail - no error details
if (check(UserSchema, data)) {
  processUser(data);
}

// Filter arrays efficiently
const validUsers = users.filter(u => check(UserSchema, u));
```

### Pre-compiled for Hot Paths

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

## Use Cases

- **API Response Validation:** Verify external API data matches expected schemas
- **User Input Validation:** Validate form data, CLI arguments, or configuration files
- **High-Throughput Pipelines:** Use `check()` or `compileCheck()` for data filtering
- **Component Props:** Validate props at runtime in any framework (React, Vue, etc.)

## Demo

See the tool in action:

![Property Validator Demo](/property-validator/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/PMBxtiHAdpPXdHXE6Go88tK3M)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/property-validator" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

## What's New

### v0.9.2 - Multiple Entry Points
- `/v` entry point for fluent API: `import { v } from '@tuulbelt/property-validator/v'`
- `/lite` entry point for functional API: `import { string, email } from '@tuulbelt/property-validator/lite'`
- Bundle size benchmark script for measuring tree-shaking impact
- Documentation for choosing the right entry point

### v0.9.1 - Functional Refinement API
- Tree-shakeable refinement functions: `email()`, `int()`, `positive()`, etc.
- Functional composition: `string(email(), minLength(5))`
- 32 refinement exports for maximum tree-shaking
- 44 new tests, 680 total tests passing

### v0.9.0 - Modularization
- Tree-shakeable named exports for all validators
- Separate types module: `@tuulbelt/property-validator/types`
- `sideEffects: false` for bundler optimization

### v0.8.5 - Three API Tiers
- `check()` API for boolean-only validation (~3x faster)
- `compileCheck()` API for pre-compiled hot paths
- Built-in string validators (email, url, uuid, pattern, etc.)
- Built-in number validators (int, positive, range, etc.)

### v0.8.0 - JIT Bypass Pattern
- 5.36x faster than Valibot on complex nested objects
- 5.97x faster on arrays
- Recursive JIT bypass for nested validators

## Next Steps

- [Getting Started](/tools/property-validator/getting-started) - Installation and basic usage
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface
- [Library Usage](/tools/property-validator/library-usage) - Import and use in your code
- [Examples](/tools/property-validator/examples) - Real-world validation scenarios
- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
