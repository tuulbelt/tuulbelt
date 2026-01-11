# Frameworks

Opinionated structures that guide how applications are built.

## What Belongs Here

Frameworks provide **structure and conventions**:
- Inversion of control (framework calls your code)
- Clear conventions over configuration
- Broader scope than tools or libraries
- Escape hatches for customization

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Minimal (prefer Tuulbelt composition) |
| Tuulbelt Dependencies | Allowed (encouraged) |
| Interface | API + Conventions |
| Scope | Broad, opinionated |
| Documentation | ARCHITECTURE.md required |

## Governance

Frameworks require **medium governance** due to scope creep risk:
- Clear scope boundary defined upfront
- Architecture review before acceptance
- Incremental adoption path required
- No "all-or-nothing" commitment

## Examples

- `web-framework` — Minimal HTTP framework
- `test-framework` — Test runner with fixtures
- `cli-framework` — CLI application framework
- `plugin-framework` — Extensibility patterns

## Creating a New Framework

```bash
/new-framework <name> <typescript|rust>
```

## Structure

```
framework-name/
├── src/
│   ├── core/              # Core framework
│   ├── plugins/           # Plugin system (if any)
│   └── index.ts
├── test/
├── examples/
│   ├── basic/             # Basic example app
│   └── advanced/          # Advanced example app
├── package.json
├── README.md
├── ARCHITECTURE.md        # Required
└── MIGRATION.md           # Version migration guide
```
