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

### 3. File-Based Semaphore - Concurrent snapshot safety

**Why:** When tests run in parallel, multiple processes may try to update the same snapshot simultaneously. Without coordination, this can corrupt snapshot files or cause non-deterministic test results.

**The Problem:**
```
Test A: read snapshot → modify → write
Test B: read snapshot → modify → write  (overwrites A mid-write)
Result: Corrupted or interleaved content
```

**The Solution:**
```bash
# Wrap snapshot updates with sema
sema acquire /tmp/snapshot.lock --tag "test-$PID"
snapcmp update my-snapshot < new-content
sema release /tmp/snapshot.lock
```

**Script:** `scripts/dogfood-sema.sh`

```bash
./scripts/dogfood-sema.sh        # Default: 10 concurrent workers
./scripts/dogfood-sema.sh 20     # Custom: 20 concurrent workers
```

**What it proves:**
| Scenario | Without sema | With sema |
|----------|--------------|-----------|
| Concurrent writes | Possible corruption | Serialized, clean |
| Final state | Non-deterministic | Deterministic |
| File integrity | May be invalid | Always valid |

## Implementation Checklist

- [x] Library integration with output-diffing-utility (core dependency)
- [x] Create composition scripts (`scripts/dogfood-*.sh`)
- [x] Update README with dogfooding section
- [x] Test graceful fallback when tools not available
- [x] Document in this file

## Expected Outcomes

1. **Proves Reliability:** 42 tests × 10 runs = 420 deterministic executions
2. **Proves Concurrency Safety:** N workers × serialized access = no corruption
3. **Demonstrates Composability:** Library (odiff) + CLI (sema, flaky) integration
4. **Real Value:** Both compositions solve actual problems in snapshot testing

## Standalone Behavior

When run outside the monorepo context:

- **Library (odiff):** Required - compilation fails without it
- **Dogfood scripts:** Exit gracefully with informative message

```bash
$ ./scripts/dogfood-sema.sh
Not in monorepo context - file-based-semaphore not available
Skipping concurrent safety validation
```

---

**Status:** Complete - all compositions implemented
