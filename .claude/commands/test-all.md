---
description: Run all tests (TypeScript and Rust templates)
argument-hint: "[test-filter]"
---

Run comprehensive test suite for both TypeScript and Rust tool templates.

## Test Execution

Please run the following test commands in sequence:

### TypeScript Template Tests

```bash
cd templates/tool-repo-template
npm test -- $1 2>&1
```

### Rust Template Tests

```bash
cd templates/rust-tool-template
cargo test $1 -- --nocapture 2>&1
```

## Success Criteria

- All tests pass with exit code 0
- No panics or unhandled errors
- Coverage meets minimum thresholds (80%+)

## Output

Provide a summary of:
- Total tests run
- Pass/fail counts
- Any failing test details
- Coverage percentages (if available)
