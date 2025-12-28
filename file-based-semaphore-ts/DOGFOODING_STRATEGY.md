# Dogfooding Strategy: File-Based Semaphore (TypeScript)

This document outlines how `semats` leverages other Tuulbelt tools to demonstrate composability.

## High-Value Compositions

### 1. Test Flakiness Detector - Validate concurrent test reliability

**Why:** File-based semaphores are inherently about concurrency and timing. Flaky tests could hide race conditions that would cause issues in production. Running tests multiple times validates that our concurrent lock acquisition is truly deterministic.

**Script:** `scripts/dogfood-flaky.sh`

```bash
./scripts/dogfood-flaky.sh 10
# Validates all 160 tests are deterministic across 10 runs
# ✅ NO FLAKINESS DETECTED (160 tests × 10 runs = 1,600 executions)
```

### 2. Cross-Language Validation with Rust sema

**Why:** `semats` is designed to be compatible with the Rust `sema` tool. Both use the same lock file format. This composition validates that locks created by TypeScript can be read by Rust and vice versa.

**Script:** `scripts/dogfood-sema.sh`

```bash
./scripts/dogfood-sema.sh
# Creates lock with semats, verifies with Rust sema
# Creates lock with Rust sema, verifies with semats
```

## Implementation Checklist

- [x] Identify high-value compositions (focus on REAL utility)
- [x] Create composition scripts (`scripts/dogfood-*.sh`)
- [x] Update README with dogfooding section
- [x] Update GH Pages docs
- [x] Test graceful fallback when tools not available
- [x] Document in this file

## Expected Outcomes

1. **Proves Reliability:** 160 concurrent-safety tests run deterministically
2. **Proves Compatibility:** Lock files work across TypeScript and Rust
3. **Demonstrates Composability:** Shows Tuulbelt tools validating each other

## Cross-Language Lock Format

Both `semats` (TypeScript) and `sema` (Rust) use this format:

```
pid=12345
timestamp=1735420800
tag=optional-description
```

This enables mixed-language deployments where Node.js and Rust services coordinate via shared lock files.

---

**Status:** Complete
