# Getting Started

Get up and running with Structured Error Handler in minutes.

## Prerequisites

- Node.js 18 or later
- npm or yarn

## Installation

Clone the Tuulbelt repository:

```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/structured-error-handler
```

Install dev dependencies:

```bash
npm install
```

That's it! No runtime dependencies are required.

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
npx tsx src/index.ts demo

# Demo in text format
npx tsx src/index.ts demo --format text

# Parse a JSON error
npx tsx src/index.ts parse '{"message":"test","context":[]}'

# Validate error format
npx tsx src/index.ts validate '{"message":"hello"}'

# Show help
npx tsx src/index.ts --help
```

## Next Steps

- [CLI Usage](/tools/structured-error-handler/cli-usage) - Full CLI reference
- [Library Usage](/tools/structured-error-handler/library-usage) - API patterns
- [Examples](/tools/structured-error-handler/examples) - Real-world scenarios
- [API Reference](/tools/structured-error-handler/api-reference) - Complete API docs
