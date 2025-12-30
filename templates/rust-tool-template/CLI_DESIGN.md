# CLI Interface Design

> **Note:** This file is OPTIONAL. Delete if your tool has a simple CLI that doesn't need design documentation. Keep if your CLI is complex enough to benefit from upfront planning.

**Status:** Planning
**Last Updated:** {{DATE}}

This document specifies the command-line interface design before implementation begins.

## Purpose

Planning the CLI upfront ensures:
- Consistent argument structure
- Complete option coverage
- Clear help documentation
- Thought-through edge cases

## Command Signature

```bash
{{TOOL_NAME}} [OPTIONS] <ARGS>
```

## Arguments

### Positional Arguments

- `<ARG1>` - Description (required/optional)
- `<ARG2>` - Description (required/optional)

**Examples:**
```bash
{{TOOL_NAME}} input.txt output.txt
{{TOOL_NAME}} --config settings.json file.dat
```

## Options

### Required Options

#### `--option <VALUE>`

Description of what this option does.

**Values:** list|of|possible|values
**Default:** default_value
**Example:**
```bash
{{TOOL_NAME}} --option value
```

### Optional Flags

#### `-f, --flag`

Description of flag behavior.

**Example:**
```bash
{{TOOL_NAME}} --flag input.txt
```

### Output Control

#### `-o, --output <FILE>`

Write output to file instead of stdout.

**Example:**
```bash
{{TOOL_NAME}} --output result.txt input.dat
```

#### `-q, --quiet`

Suppress output, only exit code.

**Example:**
```bash
{{TOOL_NAME}} --quiet input.txt && echo "Success"
```

#### `-v, --verbose`

Enable verbose/debug output.

**Example:**
```bash
{{TOOL_NAME}} --verbose input.txt
```

### Utility Options

#### `-h, --help`

Show help message and exit.

#### `-V, --version`

Show version and exit.

## Exit Codes

| Code | Meaning | Use Case |
|------|---------|----------|
| 0 | Success | Operation completed successfully |
| 1 | Error | General error occurred |
| 2 | Invalid arguments | Bad CLI usage |

**Example:**
```bash
{{TOOL_NAME}} input.txt
echo $?  # 0=success, 1=error, 2=invalid args
```

## Output Formats

### Format 1: Default (Human-Readable)

```
Description of default output format
```

### Format 2: JSON (Machine-Parseable)

```json
{
  "status": "success",
  "result": {}
}
```

## Usage Examples

### Example 1: Basic Usage

```bash
{{TOOL_NAME}} input.txt
```

### Example 2: With Options

```bash
{{TOOL_NAME}} --option value --output result.txt input.txt
```

### Example 3: Quiet Mode for Scripting

```bash
if {{TOOL_NAME}} --quiet input.txt; then
    echo "Success"
else
    echo "Failed"
fi
```

## Help Output

```
{{TOOL_NAME}} {{VERSION}}
One-line description of what this tool does

USAGE:
    {{TOOL_NAME}} [OPTIONS] <ARGS>

ARGS:
    <ARG>    Description

OPTIONS:
    -o, --output <FILE>    Write output to file
    -q, --quiet            Suppress output
    -v, --verbose          Verbose output
    -h, --help             Print help
    -V, --version          Print version

EXIT CODES:
    0    Success
    1    Error occurred
    2    Invalid arguments

EXAMPLES:
    # Basic usage
    {{TOOL_NAME}} input.txt

    # With output file
    {{TOOL_NAME}} --output result.txt input.txt

For more information, see: https://github.com/tuulbelt/{{TOOL_NAME}}
```

## Error Handling

### Invalid Arguments

```
Error: Expected <arg>, got <value>
Use --help for usage information
Exit code: 2
```

### File Not Found

```
Error reading <file>: No such file or directory
Exit code: 1
```

### Processing Error

```
Error: <description of what went wrong>
Exit code: 1
```

## Design Decisions

**Decision 1:** Why X instead of Y
**Rationale:** Explanation

**Decision 2:** Default value for Z
**Rationale:** Explanation

## Future Considerations

- [ ] Feature X that might be added later
- [ ] Option Y if users request it
- [ ] Format Z for specific use case

---

*Delete this template before first release. Keep the actual CLI design documentation.*
