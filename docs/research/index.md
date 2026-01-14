# Research

Experimental projects pushing boundaries and exploring novel approaches.

## What Is Research?

Research projects are **exploratory** — they test hypotheses, explore novel ideas, and may fail (that's okay and expected). These are experimental playgrounds with relaxed principles.

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Relaxed (pragmatism over purity) |
| Tuulbelt Dependencies | Allowed |
| Scope | Exploratory |
| Interface | Whatever works |
| Documentation | HYPOTHESIS.md required |

## Governance

Research requires **low governance** — exploration is the goal:
- Clear hypothesis required
- Failure is acceptable
- Document findings (positive and negative)
- May graduate to other categories when proven

## Status Labels

| Status | Meaning |
|--------|---------|
| `active` | Currently being explored |
| `paused` | Temporarily on hold |
| `concluded` | Exploration complete (success or failure) |
| `graduated` | Moved to another category |

## Available Research Projects

*No research projects yet. Coming soon!*

## Planned Research

| Project | Description | Language |
|---------|-------------|----------|
| zero-copy-parsing | Zero-copy parser experiments | Rust |
| wasm-runtime | WebAssembly runtime experiments | Rust |
| effect-system | Effect system for TypeScript | TypeScript |
| incremental-compute | Incremental computation framework | Rust |
| formal-verification | Proof-carrying code experiments | Rust |

## Graduation Path

When a research project proves successful:

```
research/effect-system → libraries/effect-system
(hypothesis validated, implementation stable)
```

## Creating a Research Project

```bash
/new-research <name>
```

See [Contributing Guide](/guide/contributing) for details.
