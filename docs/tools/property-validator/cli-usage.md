# CLI Usage

Use Property Validator from the command line to validate JSON files against schemas.

## Installation

```bash
cd property-validator
npm link  # Enable 'propval' command globally
```

## Basic Usage

### Validate JSON File

```bash
propval --schema schema.json --data data.json
```

Example `schema.json`:
```json
{
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "age": { "type": "number" }
  }
}
```

Example `data.json`:
```json
{
  "name": "Alice",
  "age": 30
}
```

Output:
```
✓ Validation passed
```

### Validation Failure

```bash
propval --schema schema.json --data invalid.json
```

Output:
```
✗ Validation failed at age: expected number, got string
```

## CLI Options

- `--schema <file>` — Path to JSON schema file (required)
- `--data <file>` — Path to JSON data file to validate (required)
- `--verbose` — Show detailed error information
- `--json` — Output results as JSON
- `--help` — Show help message

## Output Modes

### Standard Output (Default)

```bash
propval --schema schema.json --data data.json
```

Output:
```
✓ Validation passed
```

### JSON Output

```bash
propval --schema schema.json --data data.json --json
```

Output:
```json
{
  "ok": true,
  "value": {
    "name": "Alice",
    "age": 30
  }
}
```

### Verbose Mode

```bash
propval --schema schema.json --data invalid.json --verbose
```

Output:
```
✗ Validation failed

Error Details:
  Path: age
  Expected: number
  Actual: string
  Value: "thirty"

Full error: Validation failed at age: expected number, got string
```

## Exit Codes

- `0` — Validation passed
- `1` — Validation failed
- `2` — Invalid arguments or file not found

## Examples

### Validate API Response

```bash
# Fetch API response
curl https://api.example.com/user/1 > response.json

# Validate against schema
propval --schema user-schema.json --data response.json
```

### Validate Configuration File

```bash
propval --schema config-schema.json --data config.json
```

### Batch Validation

```bash
# Validate multiple files
for file in data/*.json; do
  propval --schema schema.json --data "$file" || echo "Failed: $file"
done
```

## Next Steps

- [Library Usage](/tools/property-validator/library-usage) - Use in your code
- [Examples](/tools/property-validator/examples) - Real-world scenarios
- [API Reference](/tools/property-validator/api-reference) - Complete API
