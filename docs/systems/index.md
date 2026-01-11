# Systems

Complex, integrated infrastructure projects like programming languages, databases, and runtimes.

## What Are Systems?

Systems are **large-scope infrastructure** — multiple components working together, long development timelines, and significant architectural design. These are the most ambitious projects in the Tuulbelt ecosystem.

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Minimal (pragmatic) |
| Tuulbelt Dependencies | Allowed |
| Scope | Large, integrated |
| Interface | Varies (CLI, REPL, server, client) |
| Documentation | ARCHITECTURE.md + SPEC.md required |

## Governance

Systems require **high governance**:
- Architecture review before acceptance
- Clear module boundaries
- Incremental development milestones
- Performance benchmarks required

## Available Systems

*No systems implemented yet. Coming soon!*

## Planned Systems

| System | Description | Language |
|--------|-------------|----------|
| tuu-lang | Small, embeddable scripting language | Rust |
| embedded-db | Single-file embedded database | Rust |
| key-value-store | Persistent key-value storage | Rust |
| virtual-machine | Stack-based bytecode VM | Rust |
| build-system | Declarative build system | Rust |

## Creating a System

```bash
/new-system <name>
```

See [Contributing Guide](/guide/contributing) for details.
