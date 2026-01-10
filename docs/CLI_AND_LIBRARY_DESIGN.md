# CLI and Library Design Principles

**Status:** Active Standard
**Applies To:** All Tuulbelt tools (TypeScript and Rust)
**Date:** 2026-01-06

---

## Core Philosophy

**Tuulbelt tools maximize the quality and usefulness of BOTH CLI and library interfaces.**

This is NOT about feature parity — it's about exhausting the potential of each interface independently:

| Interface | Goal | Approach |
|-----------|------|----------|
| **CLI** | Best possible command-line experience | Optimize for pipes, scripts, human DX, shell integration |
| **Library** | Best possible programmatic experience | Optimize for type safety, composability, performance, ergonomics |

### Not Every Feature Needs Both

Some features make sense for CLI but not library (and vice versa):

```
CLI-Appropriate Features:
├── Interactive wizards and prompts
├── Colored/formatted human output
├── Progress spinners and animations
├── Shell completion scripts
├── `--watch` modes with live updates
└── Human-friendly error formatting

Library-Appropriate Features:
├── Complex generic type inference
├── Builder patterns with method chaining
├── Compile-time validation
├── Async iterators and streams
├── Callback-based APIs
└── Integration with language ecosystems
```

---

## CLI Excellence

### 1. Pipes-Friendly

Support stdin/stdout for composability:

```bash
# Input from pipe
cat tests.txt | flaky --stdin

# Output to pipe
flaky --test "npm test" --format json | jq '.flakyTests'

# Chain tools
flaky --test "npm test" --format json | serr wrap "Test analysis failed"
```

### 2. Machine-Readable Output

Always provide structured output option:

```bash
# Human-readable (default)
flaky --test "npm test"

# Machine-readable
flaky --test "npm test" --format json
odiff file1.json file2.json --format json
sema status --format json
```

### 3. Good DX

- Clear `--help` with examples
- Meaningful error messages with exit codes
- Short (`-v`) and long (`--verbose`) flags
- Sensible defaults
- Shell completion support

```bash
# Help shows examples
flaky --help

# Clear error messages
flaky --test ""
# Error: --test is required. Usage: flaky --test "<command>" --runs <n>

# Exit codes (document these per tool)
# 0 = success
# 1 = tool-specific failure (flaky tests found, diff detected, etc.)
# 2 = invalid arguments
# 3 = system error
```

### 4. Interactive Features (CLI-Specific)

These make sense for CLI but not library:

```bash
# Interactive mode (prompts user)
snapcmp update --interactive
# [?] Update snapshot "test-output"? (y/n/d)

# Progress indicators
flaky --test "npm test" --runs 20
# ████████░░░░░░░░ 8/20 runs complete

# Colored diff output
odiff expected.json actual.json --color
```

### 5. Close to the Metal

Minimal abstraction, direct access:

```bash
# Direct, predictable behavior
odiff file1.json file2.json --format unified

# Not hidden behind "smart" defaults
# Not requiring configuration files
# Not depending on environment magic
```

---

## Library Excellence

### TypeScript Libraries

#### 1. Type-Safe APIs

Full TypeScript inference:

```typescript
// Inferred types throughout
const result = detect({ test: 'npm test', runs: 10 });
// result is typed as Result<DetectionReport>

if (result.ok) {
  // result.value is DetectionReport
  console.log(result.value.flakyTests);
}
```

#### 2. Multiple API Tiers

Inspired by propval's pattern:

```typescript
// Full API (detailed results)
const report = await detect({ test: 'npm test', runs: 10 });

// Fast API (boolean check)
const hasFlaky = await isFlaky({ test: 'npm test', runs: 5 });

// Compiled API (pre-optimized for repeated use)
const detector = compileDetector({ test: 'npm test' });
await detector.run(10);
await detector.run(20);  // Reuses compiled configuration
```

#### 3. Non-Throwing Result Types

```typescript
type Result<T> = { ok: true; value: T } | { ok: false; error: Error };

const result = detect(config);
if (result.ok) {
  console.log(result.value.flakyTests);
} else {
  console.error(result.error.message);
}
```

#### 4. Tree-Shakeable Exports

```typescript
// Named exports for optimal bundling
import { detect, isFlaky } from 'test-flakiness-detector';

// Multiple entry points for different use cases (propval v0.10.0+)
import { v, validate } from 'property-validator';           // Full API
import type { Validator, Result } from 'property-validator/types';  // Types only
import { toJsonSchema } from 'property-validator/json-schema';      // Interop
```

#### 5. JSDoc with Examples

```typescript
/**
 * Detect flaky tests by running the test command multiple times.
 *
 * @param config - Detection configuration
 * @returns Detection result with flaky test details
 *
 * @example
 * ```typescript
 * const result = await detect({ test: 'npm test', runs: 10 });
 * if (result.ok && result.value.flakyTests.length > 0) {
 *   console.log('Found flaky tests:', result.value.flakyTests);
 * }
 * ```
 */
export function detect(config: DetectConfig): Promise<Result<DetectionReport>>;
```

#### 6. Interoperability

Consider export formats for ecosystem integration:

```typescript
// JSON Schema export for OpenAPI/Swagger compatibility (propval v0.10.0+)
import { toJsonSchema } from 'property-validator/json-schema';

const UserSchema = v.object({
  name: v.string(),
  email: v.pipe(v.string(), v.email()),
  age: v.optional(v.number())
});

// Export for API documentation, code generation, or cross-language validation
const jsonSchema = toJsonSchema(UserSchema);
// { type: 'object', properties: { name: { type: 'string' }, ... }, required: ['name', 'email'] }
```

---

### Rust Libraries

#### 1. Result Types with Custom Errors

```rust
pub enum DiffError {
    IoError(std::io::Error),
    InvalidFormat(String),
    SizeLimitExceeded { actual: usize, limit: usize },
}

pub type DiffResult<T> = Result<T, DiffError>;

// Usage
let result = diff(expected, actual, &config)?;
```

#### 2. Builder Pattern

```rust
let config = DiffConfig::builder()
    .algorithm(Algorithm::Histogram)
    .context_lines(3)
    .ignore_whitespace(true)
    .build();

let result = diff(expected, actual, &config)?;
```

#### 3. Multiple API Tiers

```rust
// Full diff (detailed output)
pub fn diff(expected: &str, actual: &str, config: &DiffConfig) -> DiffResult<DiffOutput>;

// Boolean check (faster, no output)
pub fn has_changes(expected: &str, actual: &str) -> bool;

// Streaming for large files
pub fn diff_streaming<R: Read>(
    expected: R,
    actual: R,
    config: &DiffConfig
) -> DiffResult<impl Iterator<Item = DiffChunk>>;
```

#### 4. RAII Guards

```rust
// Automatic cleanup via Drop
let guard = semaphore.acquire()?;
// ... do work ...
// guard dropped automatically, lock released
```

#### 5. Feature Flags

```toml
[features]
default = []
async = ["tokio"]
json = []
binary = []
```

```rust
#[cfg(feature = "async")]
pub async fn diff_async(/* ... */) -> DiffResult<DiffOutput>;
```

#### 6. Rustdoc with Examples

```rust
/// Compute the diff between two strings.
///
/// # Arguments
/// * `expected` - The expected content
/// * `actual` - The actual content
/// * `config` - Diff configuration
///
/// # Examples
/// ```
/// let config = DiffConfig::default();
/// let result = diff("hello", "hello world", &config)?;
/// assert!(result.has_changes());
/// ```
pub fn diff(expected: &str, actual: &str, config: &DiffConfig) -> DiffResult<DiffOutput>;
```

---

## Feature Review Checklist

When adding a new feature, consider both interfaces independently:

```
CLI Questions:
[ ] Would a CLI user benefit from this feature?
[ ] What's the best flag/command design?
[ ] Should it support --format json?
[ ] Does it need interactive mode?
[ ] What exit code for success/failure?

Library Questions:
[ ] Would a developer benefit from this feature?
[ ] What's the best API signature?
[ ] Should it have multiple tiers (full/fast/compiled)?
[ ] Does it follow Result pattern?
[ ] Is it well-typed (TS) or use appropriate traits (Rust)?

Documentation Questions:
[ ] Is the CLI documented in --help?
[ ] Is the library documented in JSDoc/rustdoc?
[ ] Are there examples for both?
```

---

## Context-Appropriate Features

### CLI-Only is OK

```bash
# Interactive wizard - makes no sense as library API
snapcmp update --interactive

# Progress spinner - visual feedback for humans
flaky --test "npm test" --progress

# Shell completion - IDE/shell integration
flaky --completions bash > /etc/bash_completion.d/flaky
```

### Library-Only is OK

```typescript
// Complex type inference - no CLI equivalent
type Infer<T extends Schema> = T extends ObjectSchema<infer P> ? P : never;

// Builder pattern with method chaining
const schema = v.object({ name: v.string() }).optional().default({ name: '' });

// Callback-based API
detector.onProgress((run, total) => updateUI(run, total));
```

### Different Behavior is Sometimes Correct

```bash
# CLI might have colored output by default
odiff file1.json file2.json
# Shows colored diff

# Library returns structured data, no colors
const diff = diffJson(obj1, obj2);
// Returns DiffResult object
```

---

## Anti-Patterns

### Don't: Gratuitous Inconsistency

```typescript
// Bad: Different defaults without reason
// CLI defaults to 10 runs
flaky --test "npm test"

// Library defaults to 5 runs (why?)
detect({ test: 'npm test' })
```

### Don't: Missing Core Functionality

```bash
# Bad: Can't do basic operations from CLI
# Only library can parse JSON diffs
odiff file1.json file2.json  # No JSON support??
```

### Don't: Poor DX in Either Interface

```bash
# Bad: Unhelpful error
flaky
# Error: invalid arguments

# Good: Helpful error with usage
flaky
# Error: --test is required
# Usage: flaky --test "<command>" --runs <n>
# Example: flaky --test "npm test" --runs 10
```

---

## Reference Implementations

### Property Validator (TypeScript)

```bash
# CLI - optimized for human use
propval validate schema.json data.json
propval check schema.json data.json --quiet
```

```typescript
// Library - optimized for programmatic use
import { validate, check, compileCheck } from 'property-validator';

const result = validate(data, schema);      // Full validation
const isValid = check(data, schema);        // Boolean
const checker = compileCheck(schema);       // Pre-compiled
```

### File-Based Semaphore (Rust)

```bash
# CLI - shell scripting use cases
sema acquire /tmp/my-lock --timeout 5s
sema release /tmp/my-lock
sema status /tmp/my-lock --format json
```

```rust
// Library - programmatic use cases
let sem = Semaphore::new("/tmp/my-lock")?;
let guard = sem.acquire()?;  // RAII pattern
// guard dropped = lock released

// Or with timeout
let guard = sem.try_acquire(Duration::from_secs(5))?;
```

---

## Related Documents

- [PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md) — Tuulbelt design philosophy
- [TOOL_MATURITY_ANALYSIS.md](./TOOL_MATURITY_ANALYSIS.md) — Tool gap analysis
- [ARCHITECTURE.md](https://github.com/tuulbelt/tuulbelt/blob/main/ARCHITECTURE.md) — Repository structure

---

**Last Updated:** 2026-01-10
**Maintained By:** Tuulbelt Core Team
