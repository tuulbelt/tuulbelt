# Libraries

Programmatic APIs that provide reusable functionality without CLI overhead.

## What Are Libraries?

Libraries are **API-first** — they're meant to be imported into your code, not executed from the command line. While tools solve problems via CLI, libraries solve problems via programmatic interfaces.

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (standard library only) |
| Tuulbelt Dependencies | Allowed |
| Scope | Focused domain |
| Interface | Programmatic API |
| CLI | Optional (wrapper) |

## Available Libraries

*No libraries implemented yet. Coming soon!*

## Planned Libraries

| Library | Description | Language |
|---------|-------------|----------|
| result-type | Rust-style Result&lt;T,E&gt; for TypeScript | TypeScript |
| option-type | Rust-style Option&lt;T&gt; for TypeScript | TypeScript |
| immutable-collections | Persistent data structures | TypeScript |
| json-pointer | RFC 6901 JSON Pointer implementation | TypeScript |

## Creating a Library

```bash
/new-library <name> <typescript|rust>
```

See [Contributing Guide](/guide/contributing) for details.
