# Protocol Name - Development Context

## Overview

This is a Tuulbelt **protocol** — a wire format specification with a reference implementation.

**Key principle:** The specification (SPEC.md) is the authoritative source. The implementation follows the specification.

## Quick Reference

```bash
cargo test              # Run all tests
cargo test vectors      # Run compliance tests
cargo clippy            # Lint
cargo fmt               # Format
cargo build --release   # Build CLI
```

## Architecture

```
├── SPEC.md             # Protocol specification (REQUIRED)
├── src/
│   ├── lib.rs          # Reference implementation
│   └── main.rs         # CLI for testing
├── tests/
│   └── vectors.rs      # Compliance test vectors
└── examples/
    └── ...             # Usage examples
```

## Protocol Development Workflow

1. **Specification First**
   - All changes start with SPEC.md
   - Define wire format, semantics, error cases
   - Add test vectors for new features

2. **Implementation Second**
   - Implement per specification
   - Match byte layouts exactly
   - Handle all error cases from spec

3. **Test Vectors**
   - Add compliance tests in `tests/vectors.rs`
   - Include both valid and invalid cases
   - Document expected behavior

## Protocol Principles

1. **Zero External Dependencies** — Only Rust std in reference impl
2. **Specification First** — SPEC.md is the source of truth
3. **Test Vectors** — Compliance tests for any implementation
4. **Cross-Language** — Spec is language-agnostic
5. **Explicit Error Handling** — All error cases documented

## Wire Format Guidelines

- Use big-endian for multi-byte integers
- Use UTF-8 for text
- Include length prefixes for variable data
- Reserve values for future extension
- Document all magic numbers

## Test Vector Format

Each test vector should include:
- Input bytes (hex)
- Expected parsed result OR expected error
- Reference to SPEC.md section

```rust
#[test]
fn vector_name() {
    // From SPEC.md Section X.Y
    let bytes = vec![...];
    let result = decode(&bytes);
    assert_eq!(result, expected);
}
```

## Files to Update When Making Changes

1. **SPEC.md** — Update specification FIRST
2. **src/lib.rs** — Implementation changes
3. **tests/vectors.rs** — Add compliance tests
4. **README.md** — Update documentation

## Quality Checklist

Before committing:

- [ ] SPEC.md updated for any format changes
- [ ] `cargo test` passes (all vectors)
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo fmt` applied
- [ ] Zero runtime dependencies
- [ ] Error cases covered in tests

## Part of Tuulbelt

See [Tuulbelt PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md) for ecosystem principles.
