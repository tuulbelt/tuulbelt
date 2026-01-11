# Protocols

Wire formats, communication standards, and protocol specifications with reference implementations.

## What Are Protocols?

Protocols are **specification-first** — the spec is the product, the implementation is secondary. Every protocol must have a formal SPEC.md document that defines the wire format, semantics, error handling, and versioning.

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (reference implementation) |
| Tuulbelt Dependencies | Allowed |
| Scope | Specification-focused |
| Interface | Spec + Reference Implementation |
| Documentation | **SPEC.md required** |

## Governance

Protocols require **high governance**:
- Specification completeness review
- Edge case and error handling coverage
- Versioning and evolution strategy
- Cross-language compatibility verification

## Available Protocols

*No protocols implemented yet. Coming soon!*

## Planned Protocols

| Protocol | Description | Language |
|----------|-------------|----------|
| wire-protocol | Self-describing binary TLV format | Rust |
| message-envelope | Standard request/response wrapper | TypeScript |
| pubsub-protocol | Minimalist pub-sub wire format | Rust |
| rpc-protocol | Simple RPC over TCP/Unix sockets | Rust |

## Creating a Protocol

```bash
/new-protocol <name>
```

See [Contributing Guide](/guide/contributing) for details.
