# Dogfooding Strategy: Config File Merger

This document outlines how Config File Merger leverages other Tuulbelt tools to demonstrate composability.

## High-Value Compositions

### 1. Test Flakiness Detector - Validate test reliability

**Why:** Configuration merging involves parsing, type coercion, and precedence rules. Tests must be 100% deterministic to ensure consistent behavior across environments.

**Script:** `scripts/dogfood-flaky.sh`

```bash
./scripts/dogfood-flaky.sh [runs]
# Validates all 135 tests are deterministic across N runs
# Default: 10 runs = 1,350 test executions
```

**Expected outcome:** Zero flaky tests detected.

### 2. Output Diffing Utility - Prove deterministic outputs

**Why:** Configuration merging must produce identical output for identical input. This validates that type coercion, precedence, and source tracking are deterministic.

**Script:** `scripts/dogfood-diff.sh`

```bash
./scripts/dogfood-diff.sh
# Runs the same merge operation twice and compares outputs
# Proves JSON output is byte-for-byte identical
```

**Expected outcome:** `IDENTICAL` status for all test cases.

## Implementation Checklist

- [x] Identify high-value compositions (focus on REAL utility, not checkboxes)
- [x] Create composition scripts (`scripts/dogfood-*.sh`)
- [x] Update README with dogfooding section
- [x] Update GH Pages docs (if applicable)
  - Use GitHub links: `[DOGFOODING_STRATEGY.md](https://github.com/tuulbelt/tuulbelt/blob/main/config-file-merger/DOGFOODING_STRATEGY.md)`
- [x] Test graceful fallback when tools not available
- [x] Document in this file

## Expected Outcomes

1. **Proves Reliability:** 135 tests × 10 runs = 1,350 deterministic executions
2. **Demonstrates Composability:** CLI-based validation works across tool boundaries
3. **Real Value:** Catches edge cases in type coercion and precedence handling

## Graceful Fallback

Both scripts exit gracefully (exit 0) when not in monorepo context:

```bash
# When run standalone (not in monorepo)
./scripts/dogfood-flaky.sh
# Output: "Not in monorepo context, skipping dogfooding"
# Exit: 0 (success)
```

This ensures config-file-merger works independently while gaining validation benefits when used within the Tuulbelt ecosystem.

---

**Guidelines:**
- Only implement compositions that provide REAL value
- Don't dogfood just for the sake of dogfooding
- Focus on 2-4 high-impact compositions
- Keep tools standalone (graceful fallback)
- Prioritize: Test validation > Output consistency > Domain-specific needs

**Status:** Implemented (2 high-value compositions)
