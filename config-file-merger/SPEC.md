# Config File Merger Specification

## Overview

Config File Merger (`cfgmerge`) merges configuration from multiple sources (environment variables, JSON files, CLI arguments, and defaults) with explicit precedence rules and optional source tracking.

## Problem

Applications need to combine configuration from multiple sources:
- Environment variables for deployment-specific settings
- Config files for application defaults
- CLI arguments for runtime overrides
- Hardcoded defaults as fallbacks

Existing solutions either require heavy dependencies (dotenv, convict, cosmiconfig), don't handle all sources, or have unclear precedence rules.

## Design Goals

1. **Zero dependencies** — Uses only Node.js standard library
2. **Clear precedence** — Explicit, documented merge order
3. **Source tracking** — Know where each value came from
4. **Type coercion** — Automatic parsing of primitives
5. **Composable** — Works as both library and CLI

## Precedence Rules

Values are merged in this order (highest precedence first):

```
CLI arguments > Environment variables > Config file > Defaults
```

When the same key exists in multiple sources, the highest precedence source wins.

### Example

```
Defaults:     { port: 8080, host: "localhost", debug: false }
Config file:  { port: 3000, host: "0.0.0.0" }
Environment:  { PORT: "4000" }
CLI args:     { debug: true }

Result:       { port: 4000, host: "0.0.0.0", debug: true }
              (port from env, host from file, debug from CLI)
```

## Interface

### CLI

```bash
cfgmerge [options]

Options:
  -e, --env              Include environment variables
  -p, --prefix PREFIX    Filter env vars by prefix (e.g., APP_)
  --no-strip-prefix      Keep prefix in output keys
  --no-lowercase         Keep original env var case
  -f, --file FILE        Load config from JSON file
  -d, --defaults FILE    Load default values from JSON file
  -a, --args ARGS        CLI arguments as key=value,key2=value2
  -t, --track-sources    Show source of each value in output
  -h, --help             Show help message
  -V, --version          Show version
```

### Library API

```typescript
import { mergeConfig, getValue, parseJsonFile } from 'config-file-merger';

interface MergeOptions {
  env?: Record<string, string | undefined>;
  envPrefix?: string;
  stripPrefix?: boolean;      // default: true
  lowercaseEnv?: boolean;     // default: true
  file?: Record<string, unknown>;
  cli?: Record<string, unknown>;
  defaults?: Record<string, unknown>;
  trackSources?: boolean;     // default: false
}

interface MergeResult {
  ok: true;
  config: Record<string, unknown>;
} | {
  ok: false;
  error: string;
}

function mergeConfig(options: MergeOptions): MergeResult;
function getValue<T>(config: Record<string, unknown>, key: string, defaultValue?: T): T | undefined;
function parseJsonFile(path: string): { ok: true; data: Record<string, unknown> } | { ok: false; error: string };
```

## Type Coercion

CLI arguments and environment variables are parsed as follows:

| Input | Output | Type |
|-------|--------|------|
| `"true"` | `true` | boolean |
| `"false"` | `false` | boolean |
| `"null"` | `null` | null |
| `"42"` | `42` | number |
| `"3.14"` | `3.14` | number |
| `"-5"` | `-5` | number |
| `"hello"` | `"hello"` | string |
| `'"42"'` | `"42"` | string (quoted) |
| `"'text'"` | `"text"` | string (quoted) |

Quoted strings preserve their string type even if they look like numbers or booleans.

## Output Format

### Without Source Tracking

```json
{
  "port": 3000,
  "host": "localhost",
  "debug": true
}
```

### With Source Tracking (`--track-sources`)

```json
{
  "port": { "value": 3000, "source": "cli" },
  "host": { "value": "localhost", "source": "file" },
  "debug": { "value": true, "source": "env" }
}
```

Source values: `"cli"`, `"env"`, `"file"`, `"default"`

## Environment Variable Handling

### Prefix Filtering

With `--prefix APP_`:
- `APP_PORT=3000` → `{ "port": 3000 }`
- `OTHER_VAR=x` → ignored

### Case Normalization

Default behavior (`--lowercase`):
- `APP_DATABASE_URL` → `database_url`

With `--no-lowercase`:
- `APP_DATABASE_URL` → `DATABASE_URL`

### Prefix Stripping

Default behavior (`--strip-prefix`):
- `APP_PORT` with prefix `APP_` → `port`

With `--no-strip-prefix`:
- `APP_PORT` with prefix `APP_` → `app_port`

## Error Handling

### Exit Codes

- `0` — Success
- `1` — Error (file not found, invalid JSON, etc.)

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| File not found | `File not found: <path>` |
| Invalid JSON | `Invalid JSON in file: <path>` |
| JSON is not an object | `Config file must contain a JSON object` |

Errors are returned via Result pattern, not thrown:

```typescript
const result = parseJsonFile('config.json');
if (!result.ok) {
  console.error(result.error);
  process.exit(1);
}
```

## Determinism

The tool is fully deterministic:
- Same inputs always produce same outputs
- Key order is preserved from source objects
- No randomness or timing-dependent behavior

This is validated by the `dogfood-diff.sh` script.

## Limitations

1. **JSON only** — Config files must be valid JSON (no YAML, TOML, etc.)
2. **Flat merging** — Nested objects are not deep-merged, they are replaced
3. **No schema validation** — Values are not validated against a schema
4. **CLI array limitation** — Arrays in `--args` are split by commas

## Future Considerations

These are explicitly out of scope for v0.1.0 but may be considered later:

- YAML/TOML config file support
- Deep merge for nested objects
- Schema validation
- Environment variable interpolation in config files

---

**Version:** 0.1.0
**Last Updated:** 2025-12-27
