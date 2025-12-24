# Tool Name

[![Tests](https://github.com/tuulbelt/tuulbelt/actions/workflows/test-all-tools.yml/badge.svg)](https://github.com/tuulbelt/tuulbelt/actions/workflows/test-all-tools.yml)
[![Tool Tests](https://github.com/tuulbelt/tuulbelt/workflows/Test%20All%20Tools/badge.svg?branch=main)](https://github.com/tuulbelt/tuulbelt/actions)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Node](https://img.shields.io/badge/node-%3E%3D18.0.0-brightgreen)
![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success)
![Tests](https://img.shields.io/badge/tests-passing-success)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

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

### Dogfooding (Optional)

If this tool can use or validate other Tuulbelt tools, consider adding:

**Using Other Tools:**
- Use dynamic imports with graceful fallback (see `@docs/QUALITY_CHECKLIST.md` for pattern)
- Tool must work standalone (when cloned independently)
- Document monorepo enhancements in README

**Validating Other Tools:**
- Add `test:dogfood` script if using Test Flakiness Detector
- Document validation in README and VitePress docs

Example: Test Flakiness Detector uses CLI Progress Reporting for progress tracking, and validates other tool test suites.

## Error Handling

Exit codes:
- `0` — Success
- `1` — Error (invalid input, processing failure)

Errors are returned in the `error` field of the result object, not thrown.

## Future Enhancements

Potential improvements for future versions:

- Feature idea 1
- Feature idea 2
- Feature idea 3

## Specification

See [SPEC.md](SPEC.md) for detailed specification.

## Demo

![Demo](docs/demo.gif)

**[▶ View interactive recording on asciinema.org](#)**

<div>
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;"><strong>Try it online:</strong></span>
  <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/tool-name" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

> Demos are automatically generated and embedded via GitHub Actions when demo scripts are updated.

## License

MIT — see [LICENSE](LICENSE)

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

## Related Tools

Part of the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) collection:
- [Test Flakiness Detector](../test-flakiness-detector/) — Detect unreliable tests
- [CLI Progress Reporting](../cli-progress-reporting/) — Concurrent-safe progress updates
- More tools coming soon...
