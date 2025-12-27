# Dogfooding Strategy: Snapshot Comparison

This document outlines how snapshot-comparison leverages other Tuulbelt tools to demonstrate composability.

## High-Value Compositions

### 1. Output Diffing Utility - Semantic diff rendering (Library Integration)

**Why:** Snapshot testing requires rich, human-readable diffs when content differs. Instead of implementing our own diff algorithms, we use odiff's proven LCS-based text diff, semantic JSON comparison, and binary hex diff.

**Type:** Library dependency (compile-time)

**Implementation:**
```toml
# Cargo.toml
[dependencies]
output-diffing-utility = { path = "../output-diffing-utility" }
```

```rust
use output_diffing_utility::{diff_text, diff_json, diff_binary};
```

**Value:** Core functionality - not optional. Demonstrates Tuulbelt-to-Tuulbelt library composition as defined in PRINCIPLES.md Exception 2.

### 2. Test Flakiness Detector - Validate test reliability

**Why:** Snapshot operations must be deterministic. A flaky test could falsely report mismatches or miss regressions.

**Script:** `scripts/dogfood-flaky.sh`

```bash
./scripts/dogfood-flaky.sh       # Default: 10 runs
./scripts/dogfood-flaky.sh 20    # Custom: 20 runs
```

### 3. Output Diffing Utility - Prove deterministic outputs (CLI)

**Why:** Snapshot creation and comparison must produce identical results across runs. Any non-determinism would undermine the tool's purpose.

**Script:** `scripts/dogfood-diff.sh`

```bash
./scripts/dogfood-diff.sh
# Compares CLI outputs between runs
```

## Implementation Checklist

- [x] Library integration with output-diffing-utility (core dependency)
- [x] Create composition scripts (`scripts/dogfood-*.sh`)
- [x] Update README with dogfooding section
- [x] Test graceful fallback when tools not available
- [x] Document in this file

## Expected Outcomes

1. **Proves Reliability:** 42 tests × 10 runs = 420 deterministic executions
2. **Demonstrates Composability:** Library + CLI integration patterns
3. **Real Value:** Semantic diffs are essential for snapshot testing UX

## Standalone Behavior

When run outside the monorepo context:

- **Library (odiff):** Required - compilation fails without it
- **Dogfood scripts:** Exit gracefully with informative message

```bash
$ ./scripts/dogfood-flaky.sh
Not in monorepo context - test-flakiness-detector not available
Skipping dogfooding validation
```

---

**Status:** Complete - all compositions implemented
