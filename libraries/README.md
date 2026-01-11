# Libraries

Programmatic APIs that provide reusable functionality without CLI overhead.

## What Belongs Here

Libraries are **API-first** (not CLI-first like tools). They provide:
- Reusable functionality as importable modules
- Type-safe, well-documented APIs
- Composability with other Tuulbelt libraries

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (standard library only) |
| Tuulbelt Dependencies | Allowed (git URL) |
| Interface | Programmatic API |
| CLI | Optional (wrapper) |
| Documentation | API.md required |

## Examples

- `result-type` — Rust-style Result<T,E> for TypeScript
- `option-type` — Rust-style Option<T> for TypeScript
- `immutable-collections` — Persistent data structures
- `json-pointer` — RFC 6901 JSON Pointer implementation

## Creating a New Library

```bash
/new-library <name> <typescript|rust>
```

Or use the template manually:
```bash
cp -r templates/library-template/ libraries/<name>/
```

## Structure

```
library-name/
├── src/
│   ├── index.ts           # Main exports
│   ├── types.ts           # Type definitions
│   └── internal/          # Internal modules
├── test/
├── examples/
├── package.json           # No bin entry (not CLI)
├── README.md
└── API.md                 # Detailed API documentation
```
