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

**CLI names** — both short and long forms work:
- Short (recommended): `short-name`
- Long: `tool-name`

**Recommended setup** — install globally for easy access:

```bash
npm link  # Enable the 'short-name' command globally
short-name --help
```

For local development without global install:

```bash
npx tsx src/index.ts --help
```

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

Using short name (recommended after `npm link`):

```bash
# Basic usage
short-name "hello world"

# With verbose output
short-name --verbose "hello world"

# Show help
short-name --help
```

Using long name:

```bash
tool-name "hello world"
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

## Integration with Tuulbelt Meta Repository

When adding this tool to the Tuulbelt meta repository, ensure CI/CD integration:

### Demo Recording Workflow

**Required:** Add path filter to `.github/workflows/create-demos.yml`:

```yaml
paths:
  - 'tool-name/**'  # Add this line for your tool
```

This enables smart detection to only record demos when your tool changes, not on every commit.

**Required:** Create demo recording script at `scripts/record-tool-name-demo.sh`:

```bash
#!/bin/bash
set -e

asciinema rec "demo.cast" --overwrite --title "Tool Name - Tuulbelt" --command "bash -c '
  # Your demo commands here
'"
```

See existing tools for examples (test-flakiness-detector, cli-progress-reporting).

### Dogfooding

Tuulbelt tools validate and enhance each other via dogfooding.

**Validate This Tool's Tests:**

Run the flakiness detection test to validate test reliability:

```bash
npx tsx test/flakiness-detection.test.ts
```

Or add to package.json:

```json
{
  "scripts": {
    "test:dogfood": "npx tsx test/flakiness-detection.test.ts"
  }
}
```

**Using Other TypeScript Tools (Dynamic Import):**

```typescript
import { existsSync } from 'node:fs';
import { join } from 'node:path';

async function loadOptionalTool(): Promise<any | null> {
  try {
    const toolPath = join(process.cwd(), '..', 'tool-name', 'src', 'index.ts');
    if (!existsSync(toolPath)) return null;
    return await import(`file://${toolPath}`);
  } catch {
    return null; // Graceful fallback when standalone
  }
}
```

**Using Rust CLI Tools:**

```typescript
import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

// Check if Rust tool is available (pre-built)
const rustToolPath = join(process.cwd(), '..', 'rust-tool', 'target', 'release', 'rust-tool');
if (existsSync(rustToolPath)) {
  const result = execSync(`${rustToolPath} --arg value`, { encoding: 'utf-8' });
  // Use result...
}
```

**Key Principles:**
- Tools must work standalone (graceful fallback when dependencies unavailable)
- Document monorepo enhancements in README
- Use CLI for cross-language integration

See [QUALITY_CHECKLIST.md](../docs/QUALITY_CHECKLIST.md) for dogfooding patterns.

## Error Handling

Exit codes:
- `0` — Success
- `1` — Error (invalid input, processing failure)

Errors are returned in the `error` field of the result object, not thrown.

## Specification (SPEC.md)

**When to create SPEC.md:**

If your tool defines a **format, protocol, or algorithm**, create a `SPEC.md` file:

- **File formats** (e.g., JSON schema, custom binary format)
- **Protocols** (e.g., WebSocket message structure, API contract)
- **Algorithms** (e.g., parsing algorithm, transformation rules)
- **Data structures** (e.g., cache format, index structure)

**What to include:**
- Formal description of the format/protocol/algorithm
- Examples with sample inputs/outputs
- Edge cases and validation rules
- Version compatibility notes

**Examples in Tuulbelt:**
- `output-diffing-utility/SPEC.md` - Diff format and LCS algorithm
- `file-based-semaphore/SPEC.md` - Lock file structure and semantics

**When NOT to create SPEC.md:**
- Simple CLI tools with no custom format
- Wrappers around existing formats
- Tools that just process data without defining structure

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
