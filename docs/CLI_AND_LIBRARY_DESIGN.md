# CLI and Library Design Principles

**Status:** Active Standard
**Applies To:** All Tuulbelt tools
**Date:** 2026-01-06

---

## Core Philosophy

**Tuulbelt tools provide equal attention to both CLI and library interfaces.**

This is "CLI-also" — both interfaces are first-class citizens, not an afterthought.

| Interface | Purpose | Priority |
|-----------|---------|----------|
| **CLI** | Pipes, shell scripts, standalone use, composability | ✅ First-class |
| **Library** | Embedding in other tools, programmatic access | ✅ First-class |

---

## CLI Excellence

Every CLI should embody these qualities:

### 1. Full Functionality

All features available via command-line — no library-only capabilities.

```bash
# Good: Feature available in both CLI and library
flaky --test "npm test" --runs 10 --threshold 0.1

# Bad: Feature only in library
# (no CLI equivalent for some library function)
```

### 2. Pipes-Friendly

Support stdin/stdout for composability:

```bash
# Input from pipe
cat tests.txt | flaky --stdin

# Output to pipe
flaky --test "npm test" --format json | jq '.flakyTests'

# Chain tools
flaky --test "npm test" --format json | serr wrap "Test analysis failed"
```

### 3. Machine-Readable Output

Always provide structured output option:

```bash
# Human-readable (default)
flaky --test "npm test"

# Machine-readable
flaky --test "npm test" --format json
flaky --test "npm test" --format csv
```

### 4. Good DX

- Clear `--help` with examples
- Meaningful error messages with exit codes
- Short (`-v`) and long (`--verbose`) flags
- Sensible defaults

```bash
# Help shows examples
flaky --help

# Clear error messages
flaky --test ""
# Error: --test is required. Usage: flaky --test "<command>" --runs <n>

# Exit codes
# 0 = success
# 1 = flaky tests found
# 2 = invalid arguments
# 3 = system error
```

### 5. Close to the Metal

Minimal abstraction, direct access to underlying operations:

```bash
# Direct, predictable behavior
odiff file1.json file2.json --format unified

# Not hidden behind "smart" defaults
# Not requiring configuration files
# Not depending on environment magic
```

---

## Library Excellence

Every library should embody these qualities:

### 1. Type-Safe APIs

Full TypeScript inference or Rust generics:

```typescript
// TypeScript: Inferred types
const result = detect({ test: 'npm test', runs: 10 });
// result.flakyTests is typed as FlakyTest[]

// Rust: Generic results
let result: DetectionResult = detect(&config)?;
```

### 2. Multiple API Tiers

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

### 3. Non-Throwing Result Types

Return `Result` instead of throwing exceptions:

```typescript
// Good: Result type
type Result<T> = { ok: true; value: T } | { ok: false; error: Error };

const result = detect(config);
if (result.ok) {
  console.log(result.value.flakyTests);
} else {
  console.error(result.error.message);
}

// Bad: Throwing
try {
  const report = detect(config);  // Might throw!
} catch (e) {
  // Hidden control flow
}
```

### 4. Tree-Shakeable Exports

Named exports for optimal bundling:

```typescript
// Good: Named exports
import { detect, isFlaky } from 'test-flakiness-detector';

// Avoid: Default exports or namespace imports
import detector from 'test-flakiness-detector';  // Can't tree-shake
import * as detector from 'test-flakiness-detector';  // Bundles everything
```

### 5. Comprehensive Documentation

JSDoc/rustdoc with examples:

```typescript
/**
 * Detect flaky tests by running the test command multiple times.
 *
 * @param config - Detection configuration
 * @param config.test - Test command to run
 * @param config.runs - Number of times to run tests
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

---

## Feature Parity Checklist

When adding any new feature, verify both interfaces:

```
[ ] CLI: How would a user invoke this from the command line?
[ ] CLI: What flags/options are needed?
[ ] CLI: What output format(s) should be supported?
[ ] Library: What function signature?
[ ] Library: What types are needed?
[ ] Library: Does it follow Result pattern?
[ ] Docs: Is the CLI documented in --help?
[ ] Docs: Is the library documented in JSDoc/rustdoc?
[ ] Tests: Are there CLI tests?
[ ] Tests: Are there library tests?
```

---

## Anti-Patterns

### Don't: Library-Only Features

```typescript
// Bad: No CLI equivalent
export function advancedAnalysis(config: AdvancedConfig): Result<Report>;
// Users can't access this from shell scripts
```

### Don't: CLI-Only Features

```bash
# Bad: No library equivalent
flaky --interactive
# Developers can't use this programmatically
```

### Don't: Different Behavior

```typescript
// Bad: CLI and library behave differently
// CLI returns summary, library returns detailed report
// Users get confused switching between interfaces
```

### Don't: Magic Defaults

```bash
# Bad: Different defaults in CLI vs library
flaky --test "npm test"  # Defaults to 10 runs
detect({ test: 'npm test' })  # Defaults to 5 runs???
```

---

## Reference Implementation

Property Validator (propval) demonstrates these principles:

```bash
# CLI
propval validate --schema schema.json data.json
propval check --schema schema.json data.json  # Boolean only
propval --format json  # Machine-readable
```

```typescript
// Library (parallel APIs)
import { validate, check, compileCheck } from 'property-validator';

const result = validate(data, schema);      // Full validation
const isValid = check(data, schema);        // Boolean
const checker = compileCheck(schema);       // Pre-compiled
```

---

## Related Documents

- [PRINCIPLES.md](../PRINCIPLES.md) — Tuulbelt design philosophy
- [TOOL_MATURITY_ANALYSIS.md](TOOL_MATURITY_ANALYSIS.md) — Tool gap analysis
- [ARCHITECTURE.md](../ARCHITECTURE.md) — Repository structure

---

**Last Updated:** 2026-01-06
**Maintained By:** Tuulbelt Core Team
