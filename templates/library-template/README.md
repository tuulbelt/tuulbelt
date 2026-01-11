# @tuulbelt/library-name

[![Tests](https://github.com/tuulbelt/library-name/actions/workflows/test.yml/badge.svg)](https://github.com/tuulbelt/library-name/actions/workflows/test.yml)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-brightgreen.svg)](package.json)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.3+-blue.svg)](https://www.typescriptlang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

One sentence description of what this library does.

## Why This Library?

Explain the problem this library solves and why existing solutions don't work.

- **Zero Dependencies** — No external runtime dependencies
- **Type-Safe** — Full TypeScript support with comprehensive types
- **Tree-Shakeable** — ESM exports for optimal bundle size
- **Well-Tested** — Comprehensive test coverage

## Installation

```bash
# Clone from GitHub (zero deps, no npm publish yet)
git clone https://github.com/tuulbelt/library-name.git
cd library-name
npm install
npm run build
```

Or as a git dependency:

```json
{
  "dependencies": {
    "@tuulbelt/library-name": "git+https://github.com/tuulbelt/library-name.git"
  }
}
```

## Quick Start

```typescript
import { process, validate } from '@tuulbelt/library-name';

// Validate input
const input: unknown = getUserInput();
if (validate(input)) {
  // Process with type safety
  const result = process(input);

  if (result.ok) {
    console.log(result.value);
  } else {
    console.error(result.error.message);
  }
}
```

## API Overview

### Core Functions

| Function | Description |
|----------|-------------|
| `process(input, options?)` | Main processing function |
| `validate(input)` | Type guard for input validation |
| `ok(value)` | Create success Result |
| `err(error)` | Create failure Result |

### Types

| Type | Description |
|------|-------------|
| `Result<T, E>` | Success/failure discriminated union |
| `LibraryOptions` | Configuration options |

See [API.md](API.md) for complete documentation.

## Examples

### Basic Usage

```typescript
import { process } from '@tuulbelt/library-name';

const result = process('hello world');
if (result.ok) {
  console.log(result.value); // "HELLO WORLD"
}
```

### Error Handling

```typescript
import { process } from '@tuulbelt/library-name';

const result = process('');
if (!result.ok) {
  console.error('Error:', result.error.message);
  // "Error: Input is required"
}
```

### With Options

```typescript
import { process } from '@tuulbelt/library-name';

const result = process('debug me', { debug: true });
// Logs: [DEBUG] Processing input: debug me
```

See [examples/](examples/) for more usage patterns.

## Philosophy

This library follows [Tuulbelt principles](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md):

- **Zero External Dependencies** — Uses only Node.js built-ins
- **Focused Domain** — Solves one problem well
- **Type-Safe API** — Comprehensive TypeScript types
- **Result Pattern** — Explicit error handling, no exceptions

## Development

```bash
# Install dependencies
npm install

# Run tests
npm test

# Build
npm run build

# Watch mode
npm run test:watch
```

## Testing

```bash
# Run all tests
npm test

# Run with flakiness detection (dogfooding)
npm run dogfood
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT - See [LICENSE](LICENSE)

## Part of Tuulbelt

This library is part of the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) ecosystem — a collection of zero-dependency tools and libraries.
