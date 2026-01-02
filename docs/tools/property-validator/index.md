# Property Validator

Runtime type validation with TypeScript inference for JavaScript and TypeScript projects.

## Overview

Property Validator provides schema-based runtime type validation with full TypeScript type inference. Validate data from any source - API responses, user input, config files, function arguments - with clear error messages and graceful failure.

**Status:** <img src="/icons/tool.svg" class="inline-icon" alt=""> In Development (v0.1.0)

**Language:** TypeScript

**Repository:** [tuulbelt/property-validator](https://github.com/tuulbelt/property-validator)

## Features

### <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Schema-Based Validation

Define schemas once, get both runtime validation and TypeScript types automatically. No code duplication between runtime and compile-time types.

### <img src="/icons/shield.svg" class="inline-icon" alt=""> Graceful Error Handling

Returns result types (`{ ok: true, value }` or `{ ok: false, error }`) instead of throwing exceptions. Your code stays resilient.

### <img src="/icons/search.svg" class="inline-icon" alt=""> Clear Error Messages

Validation errors include exact paths to invalid fields, expected vs actual types, and helpful context for debugging.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Node.js built-ins. No `npm install` required in production.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/property-validator.git
cd property-validator

# Install dev dependencies (for TypeScript)
npm install

# Use in your project
import { v, validate } from './src/index.ts';

const UserSchema = v.object({
  name: v.string(),
  age: v.number(),
  email: v.string()
});

const result = validate(unknownData, UserSchema);
if (result.ok) {
  console.log(result.value); // TypeScript knows the exact type
} else {
  console.error(result.error.message);
}
```

## Use Cases

- **API Response Validation:** Verify external API data matches expected schemas
- **User Input Validation:** Validate form data, CLI arguments, or configuration files
- **Function Argument Validation:** Add runtime checks to critical functions
- **Data Transformation:** Safely transform and validate data pipelines
- **Component Props:** Validate props at runtime in any framework (React, Vue, etc.)

## Demo

See the tool in action:

![Property Validator Demo](/property-validator/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/S9zWPiJiKwMNTd8EfoUcZa1xz)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/property-validator" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

## Why Property Validator?

**Runtime Safety:** TypeScript only checks types at compile-time. Property Validator ensures data from external sources (APIs, files, user input) matches your schemas at runtime.

**Type Inference:** Write schemas once, get TypeScript types automatically. No manual type definitions needed.

**Framework Agnostic:** Works anywhere JavaScript runs - not tied to React, Vue, or any specific framework.

**Graceful Failure:** Returns result types instead of throwing. Your app stays resilient to invalid data.

## Next Steps

- [Getting Started](/tools/property-validator/getting-started) - Installation and basic usage
- [CLI Usage](/tools/property-validator/cli-usage) - Command-line interface
- [Library Usage](/tools/property-validator/library-usage) - Import and use in your code
- [Examples](/tools/property-validator/examples) - Real-world validation scenarios
- [API Reference](/tools/property-validator/api-reference) - Complete API documentation
