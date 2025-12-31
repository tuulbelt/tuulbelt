# Getting Started

This guide will help you install and set up Config File Merger for your project.

## Prerequisites

- **Node.js 18 or later** - Config File Merger uses modern ES module syntax and Node.js built-ins
- **TypeScript 5.3+** (optional) - For library usage with full type safety

## Installation

### Clone the Repository

```bash
git clone https://github.com/tuulbelt/config-file-merger.git
cd config-file-merger
```

### Install Dependencies

```bash
npm install
```

This installs TypeScript and development dependencies only. There are **zero runtime dependencies**.

### Enable Global CLI

```bash
npm link
```

This enables the `cfgmerge` command globally (or use the longer `config-file-merger`).

### Verify Installation

```bash
cfgmerge --version
# cfgmerge 0.1.0

cfgmerge --help
```

## Quick Configuration Example

### 1. Create a Config File

```bash
echo '{"host": "localhost", "port": 8080, "debug": false}' > config.json
```

### 2. Merge with CLI Overrides

```bash
cfgmerge --file config.json --args "port=3000,debug=true"
```

Output:
```json
{
  "host": "localhost",
  "port": 3000,
  "debug": true
}
```

### 3. Add Environment Variables

```bash
export APP_HOST=production.example.com
cfgmerge --file config.json --env --prefix APP_ --args "port=443"
```

Output:
```json
{
  "host": "production.example.com",
  "port": 443,
  "debug": false
}
```

### 4. Track Sources

```bash
cfgmerge --file config.json --env --prefix APP_ --args "port=443" --track-sources
```

Output:
```json
{
  "host": { "value": "production.example.com", "source": "env" },
  "port": { "value": 443, "source": "cli" },
  "debug": { "value": false, "source": "file" }
}
```

## Library Usage

For programmatic usage in TypeScript/JavaScript:

```typescript
import { mergeConfig, getValue } from './src/index.js';

const result = mergeConfig({
  defaults: { port: 8080, host: 'localhost' },
  file: { port: 3000 },
  env: process.env,
  envPrefix: 'APP_',
  cli: { debug: true },
  trackSources: false,
});

if (result.ok) {
  const port = getValue<number>(result.config, 'port', 8080);
  console.log(`Starting on port ${port}`);
}
```

## Project Structure

```
config-file-merger/
├── src/
│   └── index.ts      # Main implementation
├── test/
│   └── index.test.ts # 135 comprehensive tests
├── examples/
│   ├── basic.ts      # Basic usage examples
│   └── advanced.ts   # Advanced patterns
├── package.json      # Zero runtime dependencies
├── tsconfig.json     # TypeScript configuration
└── README.md         # Documentation
```

## Next Steps

- [CLI Usage](/tools/config-file-merger/cli-usage) - Complete CLI reference
- [Library Usage](/tools/config-file-merger/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/config-file-merger/examples) - Real-world patterns
- [API Reference](/tools/config-file-merger/api-reference) - Full API documentation

## Troubleshooting

### Command Not Found

If `cfgmerge` isn't found after `npm link`:

```bash
# Check npm prefix
npm prefix -g

# Ensure it's in your PATH
export PATH="$(npm prefix -g)/bin:$PATH"
```

### TypeScript Compilation Errors

Ensure you have TypeScript 5.3+ and `@types/node`:

```bash
npm install
npx tsc --noEmit
```

### File Not Found Errors

Config File Merger requires absolute or valid relative paths:

```bash
# Good
cfgmerge --file ./config.json
cfgmerge --file /home/user/config.json

# Bad (missing file)
cfgmerge --file missing.json
# Error loading config: File not found: missing.json
```
