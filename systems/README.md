# Systems

Complex, integrated infrastructure projects like programming languages, databases, and runtimes.

## What Belongs Here

Systems are **large-scope infrastructure**:
- Multiple components working together
- Long development timeline
- May have own build system, tooling
- Often written in Rust for performance

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Minimal (pragmatic) |
| Tuulbelt Dependencies | Allowed |
| Interface | Varies (CLI, REPL, server, client) |
| Scope | Large, integrated |
| Documentation | ARCHITECTURE.md + SPEC.md required |

## Governance

Systems require **high governance**:
- Architecture review before acceptance
- Clear module boundaries
- Incremental development milestones
- Performance benchmarks required

## Examples

- `tuu-lang` — Small, embeddable scripting language
- `embedded-db` — Single-file embedded database
- `key-value-store` — Persistent key-value storage
- `virtual-machine` — Stack-based bytecode VM
- `build-system` — Declarative build system

## Creating a New System

```bash
/new-system <name>
```

## Structure

```
system-name/
├── ARCHITECTURE.md        # Required detailed design
├── SPEC.md                # Format/language specification
├── src/
│   ├── module-a/
│   ├── module-b/
│   └── main.rs
├── tests/
├── benches/               # Performance benchmarks required
├── examples/
├── Cargo.toml
└── README.md
```

## Development Guidelines

1. **Start small** — Build minimal viable version first
2. **Document architecture** — Design before implementation
3. **Benchmark early** — Performance is a feature
4. **Module boundaries** — Clear separation of concerns
5. **Incremental milestones** — Ship working versions frequently
