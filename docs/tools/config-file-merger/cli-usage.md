# CLI Usage

Config File Merger provides a powerful CLI for merging configuration from multiple sources.

## Basic Usage

```bash
cfgmerge [OPTIONS]
```

Both `cfgmerge` (short) and `config-file-merger` (long) work identically.

## Options Reference

| Option | Short | Description |
|--------|-------|-------------|
| `--env` | `-e` | Include environment variables |
| `--prefix PREFIX` | `-p` | Filter env vars by prefix (e.g., `APP_`) |
| `--no-strip-prefix` | | Keep prefix in output keys |
| `--no-lowercase` | | Keep original env var case |
| `--file FILE` | `-f` | Load config from JSON file |
| `--defaults FILE` | `-d` | Load default values from JSON file |
| `--args ARGS` | `-a` | CLI arguments as `key=value,key2=value2` |
| `--track-sources` | `-t` | Show source of each value in output |
| `--help` | `-h` | Show help message |
| `--version` | `-V` | Show version |

## Precedence Order

Values are merged with this precedence (highest first):

1. **CLI arguments** (`--args`)
2. **Environment variables** (`--env`)
3. **Config file** (`--file`)
4. **Default values** (`--defaults`)

## Configuration Sources

### Config File (`--file`, `-f`)

Load configuration from a JSON file:

```bash
# config.json
# {"host": "localhost", "port": 8080, "debug": false}

cfgmerge --file config.json
```

Output:
```json
{
  "host": "localhost",
  "port": 8080,
  "debug": false
}
```

### Default Values (`--defaults`, `-d`)

Set fallback values that are overridden by other sources:

```bash
# defaults.json
# {"timeout": 30000, "retries": 3}

cfgmerge --defaults defaults.json --file config.json
```

### Environment Variables (`--env`, `-e`)

Include environment variables in the merge:

```bash
export APP_HOST=api.example.com
export APP_PORT=443

cfgmerge --env
```

Output includes all environment variables (filtered by `--prefix` if specified).

### Environment Prefix (`--prefix`, `-p`)

Filter environment variables by prefix:

```bash
export APP_HOST=api.example.com
export APP_PORT=443
export OTHER_VAR=ignored

cfgmerge --env --prefix APP_
```

Output:
```json
{
  "host": "api.example.com",
  "port": "443"
}
```

By default, the prefix is stripped and keys are lowercased.

### Preserving Prefix and Case

```bash
cfgmerge --env --prefix APP_ --no-strip-prefix --no-lowercase
```

Output:
```json
{
  "APP_HOST": "api.example.com",
  "APP_PORT": "443"
}
```

### CLI Arguments (`--args`, `-a`)

Override any value with CLI arguments:

```bash
cfgmerge --file config.json --args "port=3000,debug=true"
```

Format: `key=value,key2=value2`

**Type Coercion:**

| Input | Result | Type |
|-------|--------|------|
| `true` | `true` | boolean |
| `false` | `false` | boolean |
| `null` | `null` | null |
| `42` | `42` | number |
| `3.14` | `3.14` | number |
| `hello` | `"hello"` | string |
| `"42"` | `"42"` | string (quoted) |

### Source Tracking (`--track-sources`, `-t`)

Show where each value came from:

```bash
cfgmerge --file config.json --args "port=3000" --track-sources
```

Output:
```json
{
  "host": { "value": "localhost", "source": "file" },
  "port": { "value": 3000, "source": "cli" },
  "debug": { "value": false, "source": "file" }
}
```

Possible sources: `cli`, `env`, `file`, `default`

## Complete Examples

### Basic Merge

```bash
cfgmerge --file config.json
```

### Environment Override

```bash
export APP_DATABASE_URL=postgres://prod.db.com/app
cfgmerge --file config.json --env --prefix APP_
```

### Full Stack

```bash
cfgmerge \
  --defaults defaults.json \
  --file config.json \
  --env --prefix APP_ \
  --args "debug=true,log_level=verbose" \
  --track-sources
```

### Docker/Container Usage

```bash
# In container, use env vars
cfgmerge --defaults defaults.json --env --prefix APP_

# Locally, use config file
cfgmerge --defaults defaults.json --file local.json
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success - configuration merged and output |
| `1` | Error - file not found, invalid JSON, etc. |

## Error Messages

```bash
# File not found
cfgmerge --file missing.json
# Error loading config: File not found: missing.json

# Invalid JSON
cfgmerge --file invalid.json
# Error loading config: Failed to parse config file: Unexpected token...

# Non-object JSON
cfgmerge --file array.json
# Error loading config: Config file must contain a JSON object
```

## Piping and Scripting

```bash
# Pipe to jq for processing
cfgmerge --file config.json | jq '.port'

# Save to file
cfgmerge --file config.json > merged.json

# Use in shell scripts
PORT=$(cfgmerge --file config.json | jq -r '.port')
echo "Starting on port $PORT"
```

## Tips and Best Practices

1. **Use defaults for fallbacks** - Put shared defaults in a separate file
2. **Environment for deployment** - Use env vars in containers/cloud
3. **CLI for overrides** - Use `--args` for one-off testing
4. **Track sources for debugging** - Use `--track-sources` to understand config origin
5. **Consistent prefixes** - Use `APP_` or similar to namespace env vars
