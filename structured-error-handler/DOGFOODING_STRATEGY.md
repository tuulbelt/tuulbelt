# Dogfooding Strategy: Structured Error Handler

This document outlines how Structured Error Handler leverages other Tuulbelt tools to demonstrate composability.

## High-Value Compositions

### 1. Test Flakiness Detector - Validate test reliability

**Why:** Error serialization and context chaining must be deterministic. Any timestamp handling or context ordering issues could cause flaky tests.

**Script:** `scripts/dogfood-flaky.sh`

```bash
./scripts/dogfood-flaky.sh 10
# ✅ NO FLAKINESS DETECTED (68 tests × 10 runs = 680 executions)
```

**What it validates:**
- Error creation is deterministic
- Serialization/deserialization round-trips are consistent
- Context chaining produces predictable results
- CLI commands behave identically across runs

### 2. Output Diffing Utility - Prove deterministic serialization

**Why:** Error serialization must produce identical JSON output for identical errors (excluding timestamps). This is critical for logging systems and error transmission.

**Script:** `scripts/dogfood-diff.sh`

```bash
./scripts/dogfood-diff.sh
# Compares serialized error outputs between runs
# Timestamps are masked for comparison
# ✅ Serialization is deterministic
```

**What it validates:**
- toJSON() produces stable output structure
- Field ordering is consistent
- Metadata serialization is deterministic
- No unintended state leakage between errors

## Implementation Checklist

- [x] Identify high-value compositions
- [ ] Create `scripts/dogfood-flaky.sh` for test validation
- [ ] Create `scripts/dogfood-diff.sh` for serialization verification
- [ ] Update README with dogfooding section
- [ ] Test graceful fallback when tools not available

## Expected Outcomes

1. **Proves Reliability:** Tests are deterministic across runs
2. **Proves Consistency:** Error serialization produces identical output
3. **Demonstrates Composability:** Shows tools working together via CLI
4. **Validates Error Handling Philosophy:** Clean separation of concerns

## Potential Future Compositions

### Using Structured Error Handler in Other Tools

Other Tuulbelt tools can use Structured Error Handler for better error reporting:

```typescript
// Example: Enhanced error handling in Test Flakiness Detector
import { StructuredError } from '../structured-error-handler/src/index.js';

try {
  await runTestCommand(command);
} catch (err) {
  throw StructuredError.wrap(err, 'Test execution failed', {
    code: 'TEST_EXEC_FAILED',
    category: 'test',
    operation: 'runTestCommand',
    metadata: { command, runs }
  });
}
```

This demonstrates the tool's value as foundational infrastructure for the entire Tuulbelt collection.

---

**Guidelines:**
- Only implement compositions that provide REAL value
- Focus on test validation and output consistency
- Keep tool standalone (graceful fallback when dependencies unavailable)
- Prioritize: Test reliability > Serialization consistency

**Status:** Strategy defined, scripts to be implemented
