# Getting Started

Get up and running with Structured Error Handler in minutes.

## Prerequisites

- Node.js 18 or later
- npm or yarn

## Installation

Clone the Tuulbelt repository:

```bash
git clone https://github.com/tuulbelt/structured-error-handler.git
cd structured-error-handler
```

Install dev dependencies and link CLI:

```bash
npm install
npm link     # Enable the 'serr' command globally
```

That's it! No runtime dependencies are required. The `npm link` command creates a global symlink so you can use the `serr` command from anywhere.

## Verify Installation

Run the test suite:

```bash
npm test
```

Run the examples:

```bash
npx tsx examples/basic.ts
npx tsx examples/advanced.ts
```

## Your First Structured Error

Create a new file `test-error.ts`:

```typescript
import { StructuredError } from './src/index.js';

// Create a simple structured error
const error = new StructuredError('Something went wrong', {
  code: 'SIMPLE_ERROR',
  category: 'test',
  operation: 'myOperation',
  metadata: { key: 'value' }
});

// Display in human-readable format
console.log(error.toString());

// Display as JSON
console.log(JSON.stringify(error.toJSON(), null, 2));
```

Run it:

```bash
npx tsx test-error.ts
```

## Wrapping Existing Errors

A common pattern is wrapping errors as they propagate up:

```typescript
import { StructuredError } from './src/index.js';

function lowLevelOperation() {
  throw new Error('Connection refused');
}

function serviceOperation() {
  try {
    lowLevelOperation();
  } catch (err) {
    throw StructuredError.wrap(err, 'Service operation failed', {
      code: 'SERVICE_ERROR',
      category: 'service',
      operation: 'serviceOperation'
    });
  }
}

function controllerOperation() {
  try {
    serviceOperation();
  } catch (err) {
    if (err instanceof StructuredError) {
      const enriched = err.addContext('controllerOperation', {
        component: 'Controller',
        metadata: { endpoint: '/api/test' }
      });
      console.log(enriched.toString());
    }
  }
}

controllerOperation();
```

## CLI Quick Start

The tool includes a CLI for parsing and validating errors:

```bash
# Show a demo with error chain
serr demo

# Demo in text format
serr demo --format text

# Parse a JSON error
serr parse '{"message":"test","context":[]}'

# Validate error format
serr validate '{"message":"hello"}'

# Show help
serr --help
```

## Next Steps

- [CLI Usage](/tools/structured-error-handler/cli-usage) - Full CLI reference
- [Library Usage](/tools/structured-error-handler/library-usage) - API patterns
- [Examples](/tools/structured-error-handler/examples) - Real-world scenarios
- [API Reference](/tools/structured-error-handler/api-reference) - Complete API docs
