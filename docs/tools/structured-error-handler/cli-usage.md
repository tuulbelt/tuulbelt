# CLI Usage

Complete command-line interface reference for Structured Error Handler.

## Basic Usage

```bash
npx tsx src/index.ts <command> [options] [input]
```

## Commands

### demo

Show a demonstration of structured errors with context chaining:

```bash
# JSON output (default)
npx tsx src/index.ts demo

# Human-readable text output
npx tsx src/index.ts demo --format text

# Include stack traces
npx tsx src/index.ts demo --format text --stack
```

**Output (JSON):**
```json
{
  "name": "StructuredError",
  "message": "Failed to connect to database",
  "code": "DB_CONNECTION_FAILED",
  "category": "database",
  "context": [
    {
      "operation": "handleGetUser",
      "component": "UserController",
      "metadata": { "endpoint": "/api/users/12345" },
      "timestamp": "2025-12-26T12:00:00.000Z"
    },
    {
      "operation": "fetchUserProfile",
      "component": "UserService",
      "metadata": { "userId": "12345" },
      "timestamp": "2025-12-26T11:59:59.500Z"
    }
  ],
  "cause": {
    "name": "Error",
    "message": "Connection refused",
    "context": []
  }
}
```

**Output (Text):**
```
[DB_CONNECTION_FAILED] Failed to connect to database

Context:
  → handleGetUser (UserController) {"endpoint":"/api/users/12345"}
  → fetchUserProfile (UserService) {"userId":"12345"}
  → connectToDatabase (DatabaseClient) {"host":"localhost","port":5432}

Caused by:
  Connection refused
```

### parse

Parse a JSON error and display it:

```bash
# Parse and show as JSON
npx tsx src/index.ts parse '{"message":"Error","code":"TEST","context":[]}'

# Parse and show as text
npx tsx src/index.ts parse '{"message":"Error","code":"TEST","context":[]}' --format text
```

**Example:**
```bash
npx tsx src/index.ts parse '{"message":"File not found","code":"ENOENT","category":"io","context":[{"operation":"readConfig","metadata":{"path":"/etc/config"},"timestamp":"2025-12-26T12:00:00Z"}]}'
```

### validate

Validate that a JSON string matches the error format:

```bash
# Valid error
npx tsx src/index.ts validate '{"message":"Valid error"}'
# Output: Valid error format

# Invalid error (missing message)
npx tsx src/index.ts validate '{"code":"NO_MESSAGE"}'
# Output: Invalid error format: missing required "message" field
# Exit code: 1
```

## Options

### --format, -f

Output format: `json` (default) or `text`

```bash
# JSON output
npx tsx src/index.ts demo --format json

# Human-readable text
npx tsx src/index.ts demo --format text
npx tsx src/index.ts demo -f text
```

### --stack, -s

Include stack traces in output (text format only):

```bash
npx tsx src/index.ts demo --format text --stack
```

**Output with stack:**
```
[DB_CONNECTION_FAILED] Failed to connect to database

Context:
  → handleGetUser (UserController) {"endpoint":"/api/users/12345"}

Caused by:
  Connection refused

Stack trace:
StructuredError: Failed to connect to database
    at handleGetUser (/path/to/file.ts:10:15)
    at main (/path/to/file.ts:25:3)
```

### --help, -h

Show help message:

```bash
npx tsx src/index.ts --help
```

**Output:**
```
Structured Error Handler - Error format with context preservation

Usage:
  structured-error-handler <command> [options] [input]

Commands:
  parse <json>     Parse a JSON error and format it
  validate <json>  Validate JSON error format
  demo             Show a demo of structured errors

Options:
  -f, --format <format>  Output format: json, text (default: json)
  -s, --stack           Include stack traces in output
  -h, --help            Show this help message
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (invalid input, parse failure, unknown command) |

## Piping and Scripting

### Parse from stdin

```bash
echo '{"message":"Piped error","context":[]}' | xargs -0 npx tsx src/index.ts parse
```

### Use in shell scripts

```bash
#!/bin/bash
# Check if error is valid
if npx tsx src/index.ts validate "$ERROR_JSON" 2>/dev/null; then
  echo "Valid error format"
else
  echo "Invalid error format"
  exit 1
fi
```

### Extract error code

```bash
ERROR_JSON='{"message":"Test","code":"MY_CODE","context":[]}'
CODE=$(npx tsx src/index.ts parse "$ERROR_JSON" 2>/dev/null | jq -r '.code')
echo "Error code: $CODE"
```

## Examples

### Generate Demo for Logging

```bash
# Capture demo output for analysis
npx tsx src/index.ts demo > error-sample.json

# Pretty-print with jq
npx tsx src/index.ts demo | jq '.'
```

### Validate Multiple Errors

```bash
#!/bin/bash
ERRORS=(
  '{"message":"Error 1","context":[]}'
  '{"message":"Error 2","code":"E2","context":[]}'
  '{"invalid":"no message"}'
)

for error in "${ERRORS[@]}"; do
  if npx tsx src/index.ts validate "$error" 2>/dev/null; then
    echo "✓ Valid: ${error:0:30}..."
  else
    echo "✗ Invalid: ${error:0:30}..."
  fi
done
```

### Format for Human Review

```bash
# Parse stored error and display nicely
ERROR_LOG=$(cat /var/log/app/error.json)
npx tsx src/index.ts parse "$ERROR_LOG" --format text
```

## Next Steps

- [Library Usage](/tools/structured-error-handler/library-usage) - API integration
- [Examples](/tools/structured-error-handler/examples) - Real-world scenarios
- [API Reference](/tools/structured-error-handler/api-reference) - Full API docs
