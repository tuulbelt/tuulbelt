# Tool Name

One sentence description of what this tool does.

## Problem

Describe the problem this tool solves. Why does it exist? What gap does it fill?

## Features

- Zero runtime dependencies
- Works with Node.js 18+
- TypeScript support with strict mode
- Composable via CLI or library API

## Installation

Clone the repository:

```bash
git clone https://github.com/tuulbelt/tool-name.git
cd tool-name
npm install  # Install dev dependencies only
```

No runtime dependencies — this tool uses only Node.js standard library.

## Usage

### As a Library

```typescript
import { process } from './src/index.js';

const result = process('hello world', { verbose: false });

if (result.success) {
  console.log(result.data); // 'HELLO WORLD'
} else {
  console.error(result.error);
}
```

### As a CLI

```bash
# Basic usage
npx tsx src/index.ts "hello world"

# With verbose output
npx tsx src/index.ts --verbose "hello world"

# Show help
npx tsx src/index.ts --help
```

## API

### `process(input: string, config?: Config): Result`

Process the input string and return a result.

**Parameters:**
- `input` — The string to process
- `config` — Optional configuration object
  - `verbose` — Enable verbose output (default: `false`)

**Returns:**
- `Result` object with:
  - `success` — `true` if processing succeeded
  - `data` — The processed string
  - `error` — Error message if `success` is `false`

## Examples

See the `examples/` directory for runnable examples:

```bash
npx tsx examples/basic.ts
```

## Testing

```bash
npm test              # Run all tests
npm test -- --watch   # Watch mode
```

## Error Handling

Exit codes:
- `0` — Success
- `1` — Error (invalid input, processing failure)

Errors are returned in the `error` field of the result object, not thrown.

## Specification

See [SPEC.md](SPEC.md) for detailed specification.

## License

MIT — see [LICENSE](LICENSE)
