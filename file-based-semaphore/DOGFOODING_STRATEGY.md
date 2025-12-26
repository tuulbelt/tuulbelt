# Dogfooding Strategy: File-Based Semaphore

## High-Value Compositions

### 1. Test Flakiness Detector - Validate concurrent safety

**Why:** Semaphore must be 100% reliable under concurrent load

**Script:** `scripts/dogfood-flaky.sh`

```bash
./scripts/dogfood-flaky.sh 10
# ✅ 85 tests × 10 runs = 850 executions
```

### 2. Output Diffing Utility - Prove deterministic outputs

**Why:** Lock operations must be predictable

**Script:** `scripts/dogfood-diff.sh`

```bash
./scripts/dogfood-diff.sh
# Test outputs must be identical
```

## Expected Outcomes

- **Proves Concurrent Safety**: No race conditions under load
- **Validates Lock Reliability**: Critical for other tools
- **Used By**: Output Diffing (cache locking demo)

## Implementation Checklist

- [ ] `scripts/dogfood-flaky.sh` - Test Flakiness Detector validation
- [ ] `scripts/dogfood-diff.sh` - Output Diffing validation
- [ ] Update README + GH Pages docs

---

**Status:** Ready for implementation
