# Meta

Tools and frameworks for building other tools, frameworks, and systems.

## What Is Meta?

Meta projects are **generative** — they produce code, scaffolding, or artifacts. These are higher-abstraction tools that enable creation of other Tuulbelt projects.

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (dogfood principles) |
| Tuulbelt Dependencies | Allowed |
| Scope | Generative |
| Interface | CLI + Library API |
| Output | Generated code/artifacts |

## Governance

Meta projects require **medium governance**:
- Clear generation patterns
- Customizable output
- Version-aware (handles evolution)
- Well-tested generators

## Available Meta Projects

*No meta projects implemented yet. Coming soon!*

## Planned Meta Projects

| Project | Description | Language |
|---------|-------------|----------|
| parser-generator | PEG/CFG parser generator | Rust |
| codegen-toolkit | Code generation utilities | TypeScript |
| dsl-builder | Domain-specific language toolkit | Rust |
| schema-compiler | Schema to code generator | TypeScript |
| api-generator | OpenAPI to client/server code | TypeScript |

## Creating a Meta Project

```bash
/new-meta <name>
```

See [Contributing Guide](/guide/contributing) for details.
