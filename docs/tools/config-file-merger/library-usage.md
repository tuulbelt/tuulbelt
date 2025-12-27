# Library Usage

Config File Merger can be used as a TypeScript/JavaScript library for programmatic configuration management.

## Import

```typescript
import {
  mergeConfig,
  getValue,
  parseJsonFile,
  parseCliArgs,
  parseEnv,
  parseValue,
} from './src/index.js';
```

## Core Function: `mergeConfig`

The main function for merging configuration from multiple sources.

### Basic Usage

```typescript
import { mergeConfig } from './src/index.js';

const result = mergeConfig({
  defaults: { port: 8080, host: 'localhost' },
  file: { port: 3000 },
  cli: { debug: true },
});

if (result.ok) {
  console.log(result.config);
  // { port: 3000, host: 'localhost', debug: true }
}
```

### With Environment Variables

```typescript
const result = mergeConfig({
  defaults: { port: 8080, host: 'localhost' },
  env: process.env,
  envPrefix: 'APP_',
  lowercaseEnv: true,
  stripPrefix: true,
});

if (result.ok) {
  console.log(result.config);
}
```

### With Source Tracking

```typescript
const result = mergeConfig({
  defaults: { port: 8080 },
  file: { host: 'api.example.com' },
  cli: { port: 443 },
  trackSources: true,
});

if (result.ok) {
  console.log(result.config);
  // {
  //   port: { value: 443, source: 'cli' },
  //   host: { value: 'api.example.com', source: 'file' }
  // }
}
```

### MergeOptions Interface

```typescript
interface MergeOptions {
  /** Environment variables to merge (pass process.env or subset) */
  env?: Record<string, string | undefined>;

  /** Prefix to filter environment variables (e.g., 'APP_') */
  envPrefix?: string;

  /** Whether to strip the prefix from env var names (default: true) */
  stripPrefix?: boolean;

  /** Whether to convert env var names to lowercase (default: true) */
  lowercaseEnv?: boolean;

  /** Config file contents (parsed JSON object) */
  file?: Record<string, unknown>;

  /** CLI arguments as key=value pairs */
  cli?: Record<string, unknown>;

  /** Default values */
  defaults?: Record<string, unknown>;

  /** Whether to track sources in output (default: false) */
  trackSources?: boolean;
}
```

## Getting Typed Values: `getValue`

Safely extract typed values from merged configuration.

### Basic Usage

```typescript
import { mergeConfig, getValue } from './src/index.js';

const result = mergeConfig({
  defaults: { port: 8080, host: 'localhost', debug: false },
});

if (result.ok) {
  const port = getValue<number>(result.config, 'port');
  const host = getValue<string>(result.config, 'host');
  const debug = getValue<boolean>(result.config, 'debug');

  console.log(`Starting ${host}:${port} (debug: ${debug})`);
}
```

### With Default Values

```typescript
const timeout = getValue<number>(config, 'timeout', 30000);
const retries = getValue<number>(config, 'retries', 3);
```

### With Tracked Config

`getValue` works with both simple and tracked configurations:

```typescript
const result = mergeConfig({
  defaults: { port: 8080 },
  trackSources: true,
});

if (result.ok) {
  // Works whether trackSources is true or false
  const port = getValue<number>(result.config, 'port');
  console.log(port); // 8080
}
```

## Parsing Functions

### `parseJsonFile`

Parse a JSON configuration file.

```typescript
import { parseJsonFile } from './src/index.js';

const result = parseJsonFile('config.json');

if (result.ok) {
  console.log('Config:', result.data);
} else {
  console.error('Error:', result.error);
}
```

**Return Types:**

```typescript
type ParseResult =
  | { ok: true; data: Record<string, unknown> }
  | { ok: false; error: string };
```

**Error Cases:**

```typescript
// File not found
{ ok: false, error: 'File not found: missing.json' }

// Invalid JSON
{ ok: false, error: 'Failed to parse config file: Unexpected token...' }

// Non-object JSON
{ ok: false, error: 'Config file must contain a JSON object' }
```

### `parseEnv`

Parse environment variables with optional filtering.

```typescript
import { parseEnv } from './src/index.js';

// Parse all env vars
const all = parseEnv(process.env);

// Parse with prefix, strip prefix, lowercase
const filtered = parseEnv(process.env, 'APP_', true, true);
// APP_DATABASE_URL -> { database_url: { value: '...', source: 'env' } }

// Keep prefix, preserve case
const preserved = parseEnv(process.env, 'APP_', false, false);
// APP_DATABASE_URL -> { APP_DATABASE_URL: { value: '...', source: 'env' } }
```

**Signature:**

```typescript
function parseEnv(
  env: Record<string, string | undefined>,
  prefix?: string,
  stripPrefix = true,
  lowercase = true
): Record<string, ConfigValue<string>>;
```

### `parseCliArgs`

Parse CLI argument strings.

```typescript
import { parseCliArgs } from './src/index.js';

const args = parseCliArgs('port=3000,debug=true,name="My App"');

console.log(args);
// {
//   port: { value: 3000, source: 'cli' },
//   debug: { value: true, source: 'cli' },
//   name: { value: 'My App', source: 'cli' }
// }
```

**Type Coercion:**

Values are automatically converted:
- `"true"` → `true` (boolean)
- `"false"` → `false` (boolean)
- `"null"` → `null`
- `"42"` → `42` (number)
- `"3.14"` → `3.14` (number)
- `'"42"'` → `"42"` (string, quoted)
- `"hello"` → `"hello"` (string)

### `parseValue`

Parse a single string value into its appropriate type.

```typescript
import { parseValue } from './src/index.js';

parseValue('true');    // true (boolean)
parseValue('false');   // false (boolean)
parseValue('null');    // null
parseValue('42');      // 42 (number)
parseValue('3.14');    // 3.14 (number)
parseValue('"42"');    // "42" (string)
parseValue('hello');   // "hello" (string)
```

## Complete Application Example

```typescript
import { mergeConfig, getValue, parseJsonFile } from './src/index.js';

// Application configuration loader
async function loadConfig() {
  // Load default configuration
  const defaultsResult = parseJsonFile('defaults.json');
  if (!defaultsResult.ok) {
    console.error('Failed to load defaults:', defaultsResult.error);
    process.exit(1);
  }

  // Load environment-specific config (optional)
  const env = process.env.NODE_ENV || 'development';
  const envConfigPath = `config.${env}.json`;
  let fileConfig: Record<string, unknown> = {};

  const fileResult = parseJsonFile(envConfigPath);
  if (fileResult.ok) {
    fileConfig = fileResult.data;
  } else {
    console.warn(`No ${envConfigPath} found, using defaults`);
  }

  // Parse CLI args (from process.argv)
  const cliArgsStr = process.argv
    .slice(2)
    .filter(arg => arg.includes('='))
    .join(',');

  const cliConfig: Record<string, unknown> = {};
  if (cliArgsStr) {
    const parsed = parseCliArgs(cliArgsStr);
    for (const [key, entry] of Object.entries(parsed)) {
      cliConfig[key] = entry.value;
    }
  }

  // Merge all sources
  const result = mergeConfig({
    defaults: defaultsResult.data,
    file: fileConfig,
    env: process.env,
    envPrefix: 'APP_',
    cli: cliConfig,
    trackSources: process.env.DEBUG === 'true',
  });

  if (!result.ok) {
    console.error('Configuration error');
    process.exit(1);
  }

  return result.config;
}

// Usage
const config = await loadConfig();
const port = getValue<number>(config, 'port', 3000);
const host = getValue<string>(config, 'host', 'localhost');

console.log(`Starting server on ${host}:${port}`);
```

## Types Reference

```typescript
// Configuration source
type ConfigSource = 'cli' | 'env' | 'file' | 'default';

// Tracked value with source
interface ConfigValue<T = unknown> {
  value: T;
  source: ConfigSource;
}

// Configuration types
type TrackedConfig = Record<string, ConfigValue>;
type SimpleConfig = Record<string, unknown>;

// Result types
interface MergeResult {
  ok: true;
  config: TrackedConfig | SimpleConfig;
}

interface MergeError {
  ok: false;
  error: string;
}

type Result = MergeResult | MergeError;
```

## Error Handling

Always check the `ok` property before accessing configuration:

```typescript
const result = mergeConfig({ file: loadedConfig });

if (!result.ok) {
  console.error('Configuration failed');
  process.exit(1);
}

// TypeScript knows result.config is defined here
const port = getValue<number>(result.config, 'port');
```

## Best Practices

1. **Use Result pattern** - Always check `result.ok` before proceeding
2. **Type your getValue calls** - Use `getValue<number>()` for type safety
3. **Provide defaults** - Use the third argument to `getValue` for fallbacks
4. **Track sources in development** - Use `trackSources: true` for debugging
5. **Namespace env vars** - Use consistent prefixes like `APP_`
6. **Validate required fields** - Check for required config after merge
