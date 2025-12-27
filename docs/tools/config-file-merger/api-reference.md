# API Reference

Complete API documentation for Config File Merger.

## Functions

### `mergeConfig(options: MergeOptions): Result`

Merge configuration from multiple sources with defined precedence.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `options` | `MergeOptions` | Configuration options |

**Returns:** `Result` - Either `MergeResult` or `MergeError`

**Example:**

```typescript
const result = mergeConfig({
  defaults: { port: 8080 },
  file: { host: 'localhost' },
  env: process.env,
  envPrefix: 'APP_',
  cli: { debug: true },
  trackSources: false,
});
```

---

### `getValue<T>(config, key, defaultValue?): T | undefined`

Get a typed value from merged configuration.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `config` | `TrackedConfig \| SimpleConfig` | Merged configuration object |
| `key` | `string` | Configuration key to retrieve |
| `defaultValue` | `T` | Optional fallback value |

**Returns:** `T | undefined` - The configuration value or default

**Example:**

```typescript
const port = getValue<number>(config, 'port', 8080);
const host = getValue<string>(config, 'host', 'localhost');
const debug = getValue<boolean>(config, 'debug', false);
```

---

### `parseJsonFile(filePath: string): ParseResult`

Parse a JSON configuration file.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `filePath` | `string` | Path to JSON file |

**Returns:** `ParseResult` - Either success with data or error

**Example:**

```typescript
const result = parseJsonFile('config.json');
if (result.ok) {
  console.log(result.data);
} else {
  console.error(result.error);
}
```

---

### `parseEnv(env, prefix?, stripPrefix?, lowercase?): Record<string, ConfigValue<string>>`

Parse environment variables with optional filtering.

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `env` | `Record<string, string \| undefined>` | - | Environment variables object |
| `prefix` | `string` | `undefined` | Filter by prefix |
| `stripPrefix` | `boolean` | `true` | Remove prefix from keys |
| `lowercase` | `boolean` | `true` | Convert keys to lowercase |

**Returns:** `Record<string, ConfigValue<string>>` - Parsed config entries

**Example:**

```typescript
// APP_HOST=localhost, APP_PORT=3000
const parsed = parseEnv(process.env, 'APP_');
// { host: { value: 'localhost', source: 'env' },
//   port: { value: '3000', source: 'env' } }
```

---

### `parseCliArgs(args: string): Record<string, ConfigValue>`

Parse CLI arguments from a comma-separated string.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `args` | `string` | Arguments as `key=value,key2=value2` |

**Returns:** `Record<string, ConfigValue>` - Parsed config entries

**Example:**

```typescript
const parsed = parseCliArgs('port=3000,debug=true,name="My App"');
// { port: { value: 3000, source: 'cli' },
//   debug: { value: true, source: 'cli' },
//   name: { value: 'My App', source: 'cli' } }
```

---

### `parseValue(raw: string): unknown`

Parse a string value into its appropriate type.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `raw` | `string` | Raw string value |

**Returns:** `unknown` - Parsed value (boolean, number, null, or string)

**Coercion Rules:**

| Input | Output | Type |
|-------|--------|------|
| `"true"` | `true` | boolean |
| `"false"` | `false` | boolean |
| `"null"` | `null` | null |
| `"42"` | `42` | number |
| `"-3.14"` | `-3.14` | number |
| `'"42"'` | `"42"` | string |
| `"hello"` | `"hello"` | string |

**Example:**

```typescript
parseValue('true');    // true
parseValue('42');      // 42
parseValue('"42"');    // "42"
parseValue('hello');   // "hello"
```

---

## Types

### `ConfigSource`

```typescript
type ConfigSource = 'cli' | 'env' | 'file' | 'default';
```

Identifies where a configuration value originated.

---

### `ConfigValue<T>`

```typescript
interface ConfigValue<T = unknown> {
  value: T;
  source: ConfigSource;
}
```

A configuration value with its source for provenance tracking.

---

### `TrackedConfig`

```typescript
type TrackedConfig = Record<string, ConfigValue>;
```

Configuration with source tracking enabled.

---

### `SimpleConfig`

```typescript
type SimpleConfig = Record<string, unknown>;
```

Configuration without source tracking (values only).

---

### `MergeOptions`

```typescript
interface MergeOptions {
  env?: Record<string, string | undefined>;
  envPrefix?: string;
  stripPrefix?: boolean;
  lowercaseEnv?: boolean;
  file?: Record<string, unknown>;
  cli?: Record<string, unknown>;
  defaults?: Record<string, unknown>;
  trackSources?: boolean;
}
```

Options for the `mergeConfig` function.

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `env` | `Record<string, string \| undefined>` | - | Environment variables |
| `envPrefix` | `string` | - | Filter env vars by prefix |
| `stripPrefix` | `boolean` | `true` | Strip prefix from env keys |
| `lowercaseEnv` | `boolean` | `true` | Lowercase env keys |
| `file` | `Record<string, unknown>` | - | Config file contents |
| `cli` | `Record<string, unknown>` | - | CLI argument values |
| `defaults` | `Record<string, unknown>` | - | Default values |
| `trackSources` | `boolean` | `false` | Include source in output |

---

### `MergeResult`

```typescript
interface MergeResult {
  ok: true;
  config: TrackedConfig | SimpleConfig;
}
```

Successful merge result.

---

### `MergeError`

```typescript
interface MergeError {
  ok: false;
  error: string;
}
```

Failed merge result.

---

### `Result`

```typescript
type Result = MergeResult | MergeError;
```

Union type for merge results.

---

### `ParseResult`

```typescript
type ParseResult =
  | { ok: true; data: Record<string, unknown> }
  | { ok: false; error: string };
```

Result type for file parsing operations.

---

## CLI Options

| Option | Short | Description |
|--------|-------|-------------|
| `--env` | `-e` | Include environment variables |
| `--prefix PREFIX` | `-p` | Filter env vars by prefix |
| `--no-strip-prefix` | | Keep prefix in output keys |
| `--no-lowercase` | | Keep original env var case |
| `--file FILE` | `-f` | Load config from JSON file |
| `--defaults FILE` | `-d` | Load default values from JSON file |
| `--args ARGS` | `-a` | CLI arguments as key=value,key2=value2 |
| `--track-sources` | `-t` | Show source of each value |
| `--help` | `-h` | Show help message |
| `--version` | `-V` | Show version |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Error (file not found, invalid JSON) |

---

## Precedence

Configuration sources are merged in this order (highest precedence first):

1. CLI arguments (`cli` / `--args`)
2. Environment variables (`env` / `--env`)
3. Config file (`file` / `--file`)
4. Default values (`defaults` / `--defaults`)
