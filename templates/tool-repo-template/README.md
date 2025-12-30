# Tool Name

[![Tests](https://github.com/tuulbelt/{{tool-name}}/actions/workflows/test.yml/badge.svg)](https://github.com/tuulbelt/{{tool-name}}/actions/workflows/test.yml)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Node](https://img.shields.io/badge/node-%3E%3D18.0.0-brightgreen)
![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success)
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

Tuulbelt tools validate and enhance each other via dogfooding. This tool includes
dogfood scripts that run in CI to catch regressions.

**Dogfood Scripts (run in CI):**

```bash
# Validate test reliability (runs tests N times)
./scripts/dogfood-flaky.sh 10

# Validate output determinism (compares two test runs)
./scripts/dogfood-diff.sh
```

**Local Development Only:**

Dogfood scripts are for local development verification when this tool is in the meta repo context (tools/ submodule). They test cross-tool composition by importing other Tuulbelt tools from sibling directories.

When installed standalone (via `git clone`), dogfood scripts detect the standalone context and exit gracefully. Tests are validated by this tool's own CI workflow (`test.yml`). See [DOGFOODING_STRATEGY.md](./DOGFOODING_STRATEGY.md) for the full composition strategy.

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
- Document meta repo enhancements in README
- Use CLI for cross-language integration

See [QUALITY_CHECKLIST.md](../docs/QUALITY_CHECKLIST.md) for dogfooding patterns.

### Library Composition (PRINCIPLES.md Exception 2)

<!--
INCLUDE THIS SECTION if your tool uses another Tuulbelt tool as a library dependency.
DELETE THIS SECTION if your tool has no Tuulbelt library dependencies.
-->

If this tool uses another Tuulbelt tool as a library dependency (not CLI), document it here:

**Example:** A TypeScript tool using another TypeScript Tuulbelt tool:

```json
// package.json - git URL dependency for standalone repo
{
  "dependencies": {
    "@tuulbelt/other-tool": "git+https://github.com/tuulbelt/other-tool.git"
  }
}
```

```typescript
// src/index.ts - Import from npm package
import { someFunction } from '@tuulbelt/other-tool';

// Or use dynamic import for optional integration
const otherTool = await import('@tuulbelt/other-tool').catch(() => null);
```

**Why use library composition:**
- Reuse proven algorithms (parsing, validation, formatting)
- Reduce code duplication across tools
- Maintain zero *external* dependencies (Tuulbelt tools have zero deps themselves)

**When to use library composition:**
- Another Tuulbelt tool solves a subproblem you need
- The functionality is core to your tool (not optional)
- CLI composition would add unnecessary overhead

**Documentation requirements:**
- Document the integration in README (this section)
- Update DOGFOODING_STRATEGY.md to explain the value
- Add to root README.md Dogfooding section
- Ensure tool still works standalone (graceful fallback)

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
  <a href="https://stackblitz.com/github/tuulbelt/{{tool-name}}" style="display: inline-block; vertical-align: middle;">
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
