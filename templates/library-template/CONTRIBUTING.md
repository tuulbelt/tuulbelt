# Contributing

Thank you for your interest in contributing to this library!

## Development Setup

```bash
git clone https://github.com/tuulbelt/library-name.git
cd library-name
npm install
npm test
```

## Making Changes

1. Create a feature branch: `git checkout -b feature/my-feature`
2. Make your changes
3. Run tests: `npm test`
4. Run type check: `npx tsc --noEmit`
5. Build: `npm run build`
6. Commit with semantic message: `git commit -m "feat: add feature"`
7. Push: `git push origin feature/my-feature`
8. Open a Pull Request

## Code Style

- Use TypeScript strict mode
- No `any` types (use `unknown` and type guards)
- Use Result types instead of exceptions
- Document all public functions with JSDoc
- Keep functions small and focused

## Testing

- Write tests for all new functionality
- Use Node.js native test runner
- Test error cases explicitly
- Run flakiness detection: `npm run dogfood`

## Library Principles

This library follows Tuulbelt principles:

1. **Zero External Dependencies** — Only Node.js built-ins allowed
2. **Type-Safe API** — Comprehensive TypeScript types required
3. **Result Pattern** — Use `Result<T, E>` instead of exceptions
4. **Focused Domain** — Solve one problem well

## Pull Request Checklist

- [ ] Tests pass (`npm test`)
- [ ] Type check passes (`npx tsc --noEmit`)
- [ ] Build succeeds (`npm run build`)
- [ ] API.md updated for new functions
- [ ] No runtime dependencies added
- [ ] Commit messages follow semantic convention

## Questions?

Open an issue at https://github.com/tuulbelt/tuulbelt/issues
