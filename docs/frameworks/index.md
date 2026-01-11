# Frameworks

Opinionated structures that guide how applications are built.

## What Are Frameworks?

Frameworks provide **structure and conventions** — they define how your application should be organized and provide patterns for common tasks. Unlike tools or libraries, frameworks exhibit inversion of control: the framework calls your code.

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Minimal (prefer Tuulbelt composition) |
| Tuulbelt Dependencies | Allowed (encouraged) |
| Scope | Broad, opinionated |
| Interface | API + Conventions |
| Documentation | ARCHITECTURE.md required |

## Governance

Frameworks require **medium governance** due to scope creep risk:
- Clear scope boundary defined upfront
- Architecture review before acceptance
- Incremental adoption path required
- No "all-or-nothing" commitment

## Available Frameworks

*No frameworks implemented yet. Coming soon!*

## Planned Frameworks

| Framework | Description | Language |
|-----------|-------------|----------|
| web-framework | Minimal HTTP framework | TypeScript |
| test-framework | Test runner with fixtures | TypeScript |
| cli-framework | CLI application framework | TypeScript |
| plugin-framework | Extensibility patterns | TypeScript |

## Creating a Framework

```bash
/new-framework <name> <typescript|rust>
```

See [Contributing Guide](/guide/contributing) for details.
