# Library Name - Development Context

## Overview

This is a Tuulbelt **library** — a programmatic API that provides reusable functionality.

**Key difference from tools:** Libraries are API-first, not CLI-first. The value is in the programmatic interface.

## Quick Reference

```bash
npm test              # Run tests
npm run build         # Build to dist/
npm run test:watch    # Watch mode
npm run dogfood       # Flakiness detection
```

## Architecture

```
src/
├── index.ts          # Main exports (public API)
├── types.ts          # Additional type exports
└── internal/         # Internal modules (not exported)

test/
└── index.test.ts     # Tests

examples/
├── basic.ts          # Basic usage
└── advanced.ts       # Advanced patterns
```

## Library Principles

1. **Zero External Dependencies** — Only Node.js built-ins
2. **Type-Safe API** — Comprehensive TypeScript types
3. **Result Pattern** — Use `Result<T, E>` instead of exceptions
4. **Focused Domain** — Solve one problem well
5. **Tree-Shakeable** — ESM exports for optimal bundle size

## API Design Guidelines

### Use Result Types

```typescript
// Good: Explicit error handling
function process(input: string): Result<Output, ProcessError>

// Bad: Throwing exceptions
function process(input: string): Output // throws!
```

### Export Types

```typescript
// Export all public types
export type { Result, LibraryOptions };

// Use declaration files for consumers
// tsconfig: "declaration": true
```

### Document Everything

- JSDoc on all public functions
- API.md for complete reference
- Examples for common patterns

## Testing

- Use Node.js native test runner (`node:test`)
- Target 80%+ coverage
- Test error cases explicitly
- Run flakiness detection before release

## Files to Update When Implementing

1. `src/index.ts` — Main implementation
2. `src/types.ts` — Additional types
3. `test/index.test.ts` — Tests
4. `API.md` — API documentation
5. `README.md` — Usage documentation
6. `examples/` — Usage examples

## Quality Checklist

Before committing:

- [ ] `npm test` passes
- [ ] `npm run build` succeeds
- [ ] No `any` types (use `unknown` and type guards)
- [ ] API.md updated for new functions
- [ ] Examples work
- [ ] Zero runtime dependencies

## Part of Tuulbelt

See [Tuulbelt PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md) for ecosystem principles.
