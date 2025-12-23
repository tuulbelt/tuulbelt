# Code Style Guide

Code conventions for Tuulbelt TypeScript and Rust tools.

---

## TypeScript Style

### File Naming

- Source files: `kebab-case.ts`
- Test files: `kebab-case.test.ts`
- Types/Interfaces: Defined in same file or `types.ts`

### Code Style

```typescript
// Explicit return types
export function processData(input: string): Result {
  // Implementation
}

// Result pattern for errors
export type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: Error };

// Async/await (not callbacks or raw Promises)
export async function fetchData(): Promise<Result<Data>> {
  try {
    const response = await fetch(url);
    const data = await response.json();
    return { ok: true, value: data };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}
```

### Testing Pattern

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';

test('should process valid input', () => {
  const result = processData('input');
  assert.strictEqual(result.ok, true);
});

test('should handle errors gracefully', () => {
  const result = processData('');
  assert.strictEqual(result.ok, false);
});
```

---

## Rust Style

### File Naming

- Library: `src/lib.rs`
- Binary: `src/main.rs`
- Modules: `src/module_name.rs` or `src/module_name/mod.rs`
- Tests: `tests/integration_test.rs`

### Code Style

```rust
// Explicit error types
pub enum ProcessError {
    InvalidInput(String),
    IoError(std::io::Error),
}

// Result pattern for all fallible operations
pub fn process_data(input: &str) -> Result<Data, ProcessError> {
    if input.is_empty() {
        return Err(ProcessError::InvalidInput("empty input".to_string()));
    }
    // Implementation
    Ok(data)
}

// Async with tokio (if needed)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = fetch_data().await?;
    Ok(())
}
```

### Testing Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let result = process_data("input");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_input() {
        let result = process_data("");
        assert!(result.is_err());
    }
}
```

---

## Additional Conventions

### TypeScript

- Always use `node:` prefix for built-in modules: `import { readFileSync } from 'node:fs'`
- ES modules only: Use `import`, never `require()`
- Include `@types/node` in devDependencies for type definitions
- No `any` types (use `unknown` and narrow with type guards)
- Explicit return types on all exported functions

### Rust

- Use `?` operator for error propagation (avoid `unwrap()` in production)
- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` (zero warnings)
- All public items must have documentation comments (`///`)
- Prefer `&str` over `String` for function parameters when not taking ownership

---

## See Also

- @.claude/skills/typescript-patterns/ - TypeScript best practices skill
- @.claude/skills/rust-idioms/ - Rust idioms skill
- @docs/QUALITY_CHECKLIST.md - Quality assurance checks
