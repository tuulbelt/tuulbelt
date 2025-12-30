# {{TOOL_NAME}} - Development Guide

**Tool:** {{tool-name}} (`{{short-name}}`)
**Language:** Rust
**Type:** CLI + Library
**Repository:** https://github.com/tuulbelt/{{tool-name}}
**Part of:** [Tuulbelt](https://github.com/tuulbelt/tuulbelt)

---

## Quick Reference

**Commands:**
```bash
cargo test                    # Run all tests
cargo clippy -- -D warnings   # Lint (zero warnings)
cargo fmt                     # Format code
cargo build --release         # Build release binary
./target/release/{{short-name}} --help  # CLI help
```

**CLI Names:**
- Short: `{{short-name}}` (recommended)
- Long: `{{tool-name}}`

**Test Count:** {{X}} tests (library + integration + CLI)

---

## What This Tool Does

{{One sentence description of what this tool does and why it exists.}}

**Use Cases:**
- {{Use case 1}}
- {{Use case 2}}
- {{Use case 3}}

**Key Features:**
- Zero runtime dependencies (uses only Rust standard library)
- Cross-platform support (Linux, macOS, Windows)
- Both library and CLI interfaces
- Comprehensive error handling

---

## Architecture

**Core Components:**

1. **`src/lib.rs`** - Main library implementation
   - Core types and functions
   - Public API
   - Error types

2. **`src/main.rs`** - CLI interface (both `{{short-name}}` and `{{tool-name}}`)
   - Argument parsing
   - Error handling and exit codes
   - Output formatting

3. **`tests/`** - Integration tests
   - End-to-end CLI testing
   - Library integration tests
   - Error scenario validation

**Key Algorithms/Patterns:**
{{Describe main algorithms or patterns used}}

---

## Development Workflow

### Adding New Features

1. **Update library** (`src/lib.rs`)
2. **Add tests** (unit tests in `lib.rs`, integration tests in `tests/`)
3. **Update CLI** if needed (`src/main.rs`)
4. **Run quality checks:**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   cargo build --release
   ```
5. **Update README** with examples
6. **Test dogfooding scripts** if applicable

### Testing Strategy

**Unit Tests (in `src/lib.rs`):**
- Test each function independently
- Cover edge cases: empty input, invalid input, boundary conditions
- Test error handling

**Integration Tests (in `tests/`):**
- End-to-end CLI testing
- Library usage scenarios
- Error code verification

**Run tests:**
```bash
cargo test                    # All tests
cargo test --lib              # Library tests only
cargo test --test '*'         # Integration tests only
cargo test test_name          # Specific test
```

### Code Style

**Rust Standards:**
- Follow Rust idioms (use `?` operator, avoid `unwrap()`)
- Run `cargo fmt` before committing
- Zero clippy warnings: `cargo clippy -- -D warnings`
- Document public items with `///`

**Error Handling Pattern:**
```rust
pub enum {{Tool}}Error {
    InvalidInput(String),
    IoError(std::io::Error),
    // ... other error types
}

pub fn do_something(input: &str) -> Result<Output, {{Tool}}Error> {
    if input.is_empty() {
        return Err({{Tool}}Error::InvalidInput("input required".to_string()));
    }

    // Use ? for error propagation
    let result = process_input(input)?;

    Ok(result)
}
```

---

## Zero Dependencies Principle

**This tool has ZERO runtime dependencies.**

- Uses only Rust standard library (`std`)
- No external crates in `[dependencies]` section
- `cargo build` requires no network after first toolchain install

**Verification:**
```bash
# CI automatically checks this
awk '/^\[dependencies\]/,/^\[/ {if (!/^\[/ && !/^#/ && NF > 0) print}' Cargo.toml | wc -l
# Should output: 0
```

---

## Dogfooding

{{Describe how this tool is used by other Tuulbelt tools or validates itself}}

### Used By (Library Dependencies)

{{List tools that use this as a library dependency}}

Example:
```toml
[dependencies]
{{tool-name}} = { git = "https://github.com/tuulbelt/{{tool-name}}" }
```

### Uses

{{List Tuulbelt tools this depends on}}

### Dogfooding Scripts

**Test Flakiness Detection:**
```bash
./scripts/dogfood-flaky.sh  # Validate test determinism (20 runs)
```

**Output Consistency Validation:**
```bash
./scripts/dogfood-diff.sh   # Verify consistent outputs (100 runs)
```

These scripts require meta repo context but tool works standalone.

---

## Release Checklist

Before tagging a new version:

- [ ] All tests pass: `cargo test`
- [ ] Zero clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt`
- [ ] Zero runtime dependencies verified
- [ ] README updated with new features
- [ ] CHANGELOG.md updated
- [ ] Version bumped in `Cargo.toml`
- [ ] Tag created: `git tag vX.Y.Z`
- [ ] Pushed to GitHub: `git push origin main --tags`

---

## Common Tasks

**Add new CLI subcommand:**
1. Add to CLI argument parsing in `src/main.rs`
2. Implement handler function
3. Add validation logic
4. Update help text
5. Add integration tests
6. Update README

**Add new public function:**
1. Add function to `src/lib.rs`
2. Add documentation with `///`
3. Export if needed
4. Add unit tests
5. Add to README API section

**Performance optimization:**
1. Benchmark current implementation
2. Profile with `cargo build --release && perf record ...`
3. Optimize algorithm or data structures
4. Verify tests still pass
5. Document improvement in CHANGELOG

---

## Troubleshooting

**Tests fail with "No such file or directory":**
- Integration tests expect certain paths
- Run tests from repository root: `cargo test`
- Check test fixtures exist

**Clippy warnings:**
- Run `cargo clippy --fix` for auto-fixes
- Manually fix remaining warnings
- Ensure `cargo clippy -- -D warnings` passes

**Binary size too large:**
- Use release profile: `cargo build --release`
- Strip symbols: already enabled in `Cargo.toml` (`strip = true`)
- Check with: `ls -lh target/release/{{short-name}}`

**Cross-compilation issues:**
- Use cross: `cargo install cross`
- Build for target: `cross build --target x86_64-unknown-linux-musl`

---

## Related Tools

**In Tuulbelt:**
{{List related Tuulbelt tools}}

**External:**
{{List external tools that solve similar problems and how this differs}}

---

## Links

- **Repository:** https://github.com/tuulbelt/{{tool-name}}
- **Meta Repo:** https://github.com/tuulbelt/tuulbelt
- **Issues:** https://github.com/tuulbelt/tuulbelt/issues (centralized)
- **Documentation:** https://tuulbelt.github.io/tuulbelt/tools/{{tool-name}}/
- **Principles:** https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md

---

**Last Updated:** {{YYYY-MM-DD}}
**Version:** 0.1.0
**Status:** Active Development
