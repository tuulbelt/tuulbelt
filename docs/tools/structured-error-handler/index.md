# Structured Error Handler

Structured error format with context preservation and serialization for debugging, logging, and error transmission.

## Overview

Structured Error Handler provides a rich error format that preserves context as errors propagate through call stacks. Unlike standard JavaScript errors that lose information when wrapped, this tool maintains the full history of operations, components, and metadata that led to a failure.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** TypeScript

**Repository:** [tuulbelt/tuulbelt/structured-error-handler](https://github.com/tuulbelt/tuulbelt/tree/main/structured-error-handler)

## Features

### <img src="/icons/layers.svg" class="inline-icon" alt=""> Context Chain Preservation

Errors maintain a chain of context entries as they propagate up the call stack. Each layer can add its operation name, component, and metadata.

### <img src="/icons/code.svg" class="inline-icon" alt=""> Full JSON Serialization

Round-trip serialization to/from JSON for logging, transmission across network boundaries, and persistent storage.

### <img src="/icons/search.svg" class="inline-icon" alt=""> Error Codes & Categories

Assign error codes (like `ENOENT`, `VALIDATION_FAILED`) and categories (like `io`, `validation`, `database`) for programmatic error handling.

### <img src="/icons/git-branch.svg" class="inline-icon" alt=""> Cause Chain Support

Maintain the full cause chain from root error to top-level error. Navigate with `getRootCause()` and `getCauseChain()`.

### <img src="/icons/file-text.svg" class="inline-icon" alt=""> Human-Readable Output

Format errors for console output with context chain visualization and cause information.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Node.js standard library. No external packages required at runtime.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/structured-error-handler

# Install dev dependencies
npm install

# Run examples
npx tsx examples/basic.ts
npx tsx examples/advanced.ts

# Run tests
npm test
```

```typescript
import { StructuredError, serializeError } from './src/index.js';

// Create a structured error
const error = new StructuredError('Failed to process request', {
  code: 'VALIDATION_FAILED',
  category: 'validation',
  operation: 'validateInput',
  component: 'RequestHandler',
  metadata: { field: 'email' }
});

// Check error type programmatically
if (error.hasCode('VALIDATION_FAILED')) {
  // Return 400 Bad Request
}

// Serialize for logging
console.log(JSON.stringify(error.toJSON(), null, 2));
```

## Use Cases

### <img src="/icons/terminal.svg" class="inline-icon" alt=""> API Error Handling

Wrap errors at each layer of your API with context, then serialize for logging and return appropriate HTTP responses.

### <img src="/icons/database.svg" class="inline-icon" alt=""> Database Error Chains

When a database query fails, wrap with context at repository, service, and controller layers while preserving the root cause.

### <img src="/icons/file-text.svg" class="inline-icon" alt=""> Structured Logging

Serialize errors to JSON for structured logging systems. Includes full context chain and cause information.

### <img src="/icons/check.svg" class="inline-icon" alt=""> Error Routing

Use `hasCode()` and `hasCategory()` to route errors to appropriate handlers or return correct HTTP status codes.

## Dogfooding

This tool demonstrates composability with other Tuulbelt tools:

### Test Validation

**[Test Flakiness Detector](/tools/test-flakiness-detector/)** - Validate test reliability:
```bash
./scripts/dogfood-flaky.sh 10
# Validates all tests are deterministic
```

### Output Consistency

**[Output Diffing Utility](/tools/output-diffing-utility/)** - Verify serialization:
```bash
./scripts/dogfood-diff.sh
# Proves serialization produces identical output
```

See [`DOGFOODING_STRATEGY.md`](https://github.com/tuulbelt/tuulbelt/blob/main/structured-error-handler/DOGFOODING_STRATEGY.md) for implementation details.

## Demo

![Structured Error Handler Demo](/structured-error-handler/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/7hm7wlStFTTF6YXvQFezAuyij)**

### Try it Locally

```bash
# Clone and setup
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/structured-error-handler
npm install

# Run CLI demo
serr demo

# Human-readable format
serr demo --format text

# Parse a JSON error
serr parse '{"message":"test","context":[]}'
```

<div>
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/structured-error-handler" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

> Demo recordings are automatically generated via GitHub Actions.

## Context Chain Explained

Context is stored **most recent first**:

```
context[0] = handleRequest (most recent, top of call stack)
context[1] = fetchUser (earlier, one level down)
context[2] = queryDatabase (earliest, bottom of call stack)
```

This matches the natural reading order when debugging: start from where the error surfaced.

## Output Formats

### JSON Format

```json
{
  "name": "StructuredError",
  "message": "Failed to process request",
  "code": "VALIDATION_FAILED",
  "category": "validation",
  "context": [
    {
      "operation": "handleRequest",
      "component": "UserController",
      "metadata": { "endpoint": "/api/users" },
      "timestamp": "2025-12-26T12:00:00.000Z"
    }
  ],
  "cause": {
    "name": "Error",
    "message": "User not found",
    "context": []
  }
}
```

### Text Format

```
[VALIDATION_FAILED] Failed to process request

Context:
  → handleRequest (UserController) {"endpoint":"/api/users"}

Caused by:
  User not found
```

## Why Structured Error Handler?

**vs. Standard Error:**
- Preserves context chain through re-throws
- Full JSON serialization support
- Error codes and categories for routing

**vs. Error Cause (ES2022):**
- Structured context at each level, not just message
- Metadata attached to each context entry
- Helper methods for cause chain navigation

**vs. Custom Error Classes:**
- Standardized format across your codebase
- Immutable enrichment pattern
- CLI tools for parsing and validation

## Next Steps

- [Getting Started](/tools/structured-error-handler/getting-started) - Installation and first error
- [CLI Usage](/tools/structured-error-handler/cli-usage) - CLI reference
- [Library Usage](/tools/structured-error-handler/library-usage) - API examples
- [Examples](/tools/structured-error-handler/examples) - Real-world use cases
- [API Reference](/tools/structured-error-handler/api-reference) - Full API docs

## Related Tools

- [Test Flakiness Detector](/tools/test-flakiness-detector/) - Validates this tool's test suite
- [Output Diffing Utility](/tools/output-diffing-utility/) - Compare serialized errors

## License

MIT License - See [LICENSE](https://github.com/tuulbelt/tuulbelt/blob/main/structured-error-handler/LICENSE)
