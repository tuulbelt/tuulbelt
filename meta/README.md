# Meta

Tools and frameworks for building other tools, frameworks, and systems.

## What Belongs Here

Meta projects are **generative** in nature:
- Produce code, scaffolding, or artifacts
- Enable creation of other Tuulbelt projects
- Higher abstraction level
- Compiler/generator tooling

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (dogfood principles) |
| Tuulbelt Dependencies | Allowed |
| Interface | CLI + Library API |
| Output | Generated code/artifacts |
| Documentation | Generation patterns documented |

## Governance

Meta projects require **medium governance**:
- Clear generation patterns
- Customizable output
- Version-aware (handles evolution)
- Well-tested generators

## Examples

- `parser-generator` — PEG/CFG parser generator
- `codegen-toolkit` — Code generation utilities
- `dsl-builder` — Domain-specific language toolkit
- `schema-compiler` — Schema to code generator
- `api-generator` — OpenAPI to client/server code

## Creating a New Meta Project

```bash
/new-meta <name>
```

## Structure

```
meta-name/
├── src/
│   ├── generator/         # Core generation logic
│   ├── templates/         # Output templates
│   └── index.ts
├── test/
├── examples/
│   ├── input/             # Example inputs
│   └── output/            # Expected outputs
├── package.json
└── README.md
```

## Interface Pattern

```bash
# CLI for generation
meta-tool generate --input schema.json --output src/

# Library for programmatic use
import { generate } from '@tuulbelt/codegen-toolkit';
const code = generate(schema, { language: 'typescript' });
```

## Quality Requirements

1. **Deterministic output** — Same input = same output
2. **Customizable** — Templates/options for different needs
3. **Well-documented** — Clear generation patterns
4. **Tested** — Snapshot tests for generated output
