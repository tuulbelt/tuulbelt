# Protocols

Wire formats, communication standards, and protocol specifications with reference implementations.

## What Belongs Here

Protocols are **specification-first**:
- Formal SPEC.md document (mandatory)
- Reference implementation (minimal, correct)
- Test vectors for compliance
- Cross-language compatibility

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (reference implementation) |
| Tuulbelt Dependencies | Allowed |
| Interface | Specification + Implementation |
| SPEC.md | **Required** |
| Test Vectors | Required |

## Governance

Protocols require **high governance**:
- Specification completeness review
- Edge case and error handling coverage
- Versioning and evolution strategy
- Cross-language compatibility verification

## Examples

- `wire-protocol` — Self-describing binary TLV format
- `message-envelope` — Standard request/response wrapper
- `pubsub-protocol` — Minimalist pub-sub wire format
- `rpc-protocol` — Simple RPC over TCP/Unix sockets

## Creating a New Protocol

```bash
/new-protocol <name>
```

## Structure

```
protocol-name/
├── SPEC.md                # Formal specification (REQUIRED)
├── src/
│   ├── lib.rs             # Reference implementation
│   └── ...
├── tests/
│   └── vectors/           # Compliance test vectors
├── examples/
├── Cargo.toml
└── README.md
```

## Specification Requirements

Every SPEC.md must include:
1. **Overview** — What problem this solves
2. **Wire Format** — Byte-level specification
3. **Semantics** — Meaning of fields and operations
4. **Error Handling** — All error conditions
5. **Versioning** — How the protocol evolves
6. **Test Vectors** — Reference inputs/outputs
