# Contributing to [Tool Name]

Thank you for your interest in contributing! This guide will help you get started.

## Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- Basic understanding of Rust

### Clone and Build

```bash
git clone https://github.com/tuulbelt/[tool-name].git
cd [tool-name]
cargo test  # Run tests
cargo build --release  # Build optimized binary
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Follow Rust idioms and conventions
- Add tests for new functionality
- Update documentation as needed

### 3. Run Quality Checks

Before committing, run:

```bash
cargo test                   # All tests must pass
cargo clippy -- -D warnings  # Zero warnings
cargo fmt                    # Auto-format code
cargo build --release        # Release build succeeds
```

### 4. Commit Changes

Use clear, descriptive commit messages:

```bash
git commit -m "feat: add new validation feature"
git commit -m "fix: handle edge case in parser"
git commit -m "docs: update README with examples"
```

**Commit message format:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions/changes
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then open a Pull Request on GitHub.

## Code Standards

### Rust Style

- Use `?` operator for error propagation (avoid `unwrap()`)
- Explicit error types with `Result<T, E>`
- Document all public items with `///`
- Run `cargo fmt` before committing
- Zero clippy warnings

**Example:**

```rust
use std::fs;
use std::io;

#[derive(Debug)]
pub enum ProcessError {
    Io(io::Error),
    InvalidInput(String),
}

/// Process data from a file
pub fn process_file(path: &str) -> Result<Data, ProcessError> {
    let contents = fs::read_to_string(path)
        .map_err(ProcessError::Io)?;
    
    parse_data(&contents)
}
```

### Testing

- All new features must have tests
- Include unit tests in `src/` with `#[cfg(test)]`
- Include integration tests in `tests/`
- Aim for 80%+ code coverage
- Test error handling

**Example:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let result = process_data("valid");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_input() {
        let result = process_data("");
        assert!(result.is_err());
    }
}
```

### Documentation

- Update README.md for user-facing changes
- Add doc comments (`///`) for public APIs
- Include examples in doc comments with `# Examples`
- Run `cargo doc --open` to verify docs

## Versioning

This tool uses [Semantic Versioning](https://semver.org/):

- **`0.x.y`** — Initial development (API may change)
- **`1.0.0`** — First stable release
- **`2.0.0`** — Breaking changes increment major version

### Release Process

1. Update version in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Increment appropriately
   ```

2. Update `CHANGELOG.md` with changes:
   ```markdown
   ## [0.2.0] - 2025-01-15
   ### Added
   - New validation feature
   ### Fixed
   - Edge case handling bug
   ```

3. Commit and tag:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: release v0.2.0"
   git tag v0.2.0
   ```

4. Push to GitHub:
   ```bash
   git push && git push --tags
   ```

5. (Optional) Publish to crates.io:
   ```bash
   cargo publish
   ```

6. (Optional) GitHub Release created automatically via workflow

## Tuulbelt Principles

This tool follows Tuulbelt design principles:

1. **Single Problem** — Solves one thing well
2. **Zero Dependencies** — No runtime dependencies (only dev-dependencies)
3. **Portable Interface** — CLI + library API
4. **Composable** — Works with pipes and other tools
5. **Proven Implementation** — No experimental features

See [PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md) for details.

## Rust-Specific Guidelines

### Error Handling

- Use `Result<T, E>` for fallible operations
- Define custom error types
- Provide context in error messages
- Use `?` operator for error propagation

### Memory Safety

- No unsafe code unless absolutely necessary and well-documented
- Prefer borrowing over cloning when possible
- Use lifetimes appropriately

### Performance

- Optimize for clarity first, then performance
- Use `cargo flamegraph` for profiling
- Document performance characteristics of public APIs

## Questions?

- **Issues:** [GitHub Issues](https://github.com/tuulbelt/[tool-name]/issues)
- **Meta Repo:** [tuulbelt/tuulbelt](https://github.com/tuulbelt/tuulbelt)
- **Docs:** See README.md

Thank you for contributing! 🦀
