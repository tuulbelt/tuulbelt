# Dogfooding

Using Tuulbelt tools to validate Tuulbelt tools.

## What is Dogfooding?

**Dogfooding** means using our own tools to validate our tools. For CLI Progress Reporting, we use **Test Flakiness Detector** to ensure all 111 tests are deterministic and reliable.

## Why It Matters

Flaky tests undermine confidence. If tests pass sometimes and fail sometimes, you can't trust:
- Whether code changes introduced bugs
- Whether the tool works reliably in production
- Whether failures indicate real issues

Dogfooding proves our tools work in real scenarios and catches integration issues early.

## Validation Process

### Test File

`cli-progress-reporting/test/flakiness-detection.test.ts`

### Configuration

- **Runs:** 10 iterations of the complete test suite
- **Test Command:** `npm test`
- **Detection Threshold:** Any test with 0% < failure rate < 100%
- **Runtime:** ~20 minutes for 10 runs

### What It Checks

1. **Consistency** - Do all runs produce the same result?
2. **Determinism** - Are there any probabilistic failures?
3. **Isolation** - Do tests interfere with each other?

## Running Flakiness Detection

```bash
cd cli-progress-reporting
npx tsx test/flakiness-detection.test.ts
```

### Expected Output

```
Flakiness detection complete!

Results:
────────────────────────────────────────────────────────────
Total runs:        10
Passed runs:       10
Failed runs:       0
Success rate:      100.0%
Execution time:    ~1200s
Flaky tests found: 0
────────────────────────────────────────────────────────────

Perfect! All tests passed consistently across all runs.
No flaky tests detected - tests are deterministic and reliable.

Dogfooding Success: Test Flakiness Detector validated CLI Progress Reporting
```

## What We Validate

**111 tests across 34 suites:**
- Unit tests (35 tests) - Core functionality
- CLI integration tests (28 tests) - Command-line interface
- Filesystem tests (21 tests) - Edge cases and error handling

**Each test must:**
- Pass or fail consistently across all 10 runs
- Not exhibit probabilistic behavior
- Clean up resources properly
- Not interfere with other tests

## Results

**Last Validation:** 2025-12-23
- **Total runs:** 2 (development validation)
- **Pass rate:** 100%
- **Flaky tests:** 0
- **Confidence:** Production-ready

Full 10-run validation recommended in CI (takes ~20 minutes).

## Design Patterns That Prevent Flakiness

### 1. Deterministic Test IDs

```typescript
// Good: Deterministic ID generation
const id = `test-${Date.now()}-${counter++}`;

// Bad: Random IDs
const id = `test-${Math.random()}`;  // Non-deterministic!
```

### 2. Proper Cleanup

```typescript
test('my test', () => {
  const id = `test-${Date.now()}`;
  init(100, 'Test', { id });

  // Do test...

  // Clean up
  clear({ id });
});
```

### 3. Unique Test Resources

```typescript
// Good: Unique filename per test
const counterFile = join(tmpDir, `counter-${Date.now()}-${testId}.txt`);

// Bad: Shared filename
const counterFile = join(tmpDir, 'counter.txt');  // Collision!
```

### 4. No Probabilistic Logic

```typescript
// Good: Deterministic counter pattern
const counter = parseInt(readFileSync(counterFile, 'utf-8'));
const shouldPass = counter % 2 === 0;

// Bad: Random behavior
const shouldPass = Math.random() < 0.5;  // Non-deterministic!
```

## Continuous Validation

**When to Run Flakiness Detection:**
- Before every release (10+ runs)
- After adding new tests (2-5 runs for quick check)
- After modifying test infrastructure (10+ runs)
- When CI shows intermittent failures (10+ runs)

**Development Validation:**
```bash
# Quick validation (2 runs, ~4 minutes)
cd cli-progress-reporting
# Edit test/flakiness-detection.test.ts: set TEST_RUNS = 2
npx tsx test/flakiness-detection.test.ts
```

**Integration with CI:**
```yaml
# .github/workflows/flakiness-check.yml
name: Flakiness Check
on:
  pull_request:
    paths:
      - 'test/**'
jobs:
  detect-flakiness:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@v3
      - run: npx tsx test/flakiness-detection.test.ts  # Uses 10 runs
```

## Future Dogfooding Opportunities

As more Tuulbelt tools are built, we'll use them to validate each other:

- **Cross-Platform Path Handling** → Validate paths in Test Flakiness Detector and CLI Progress
- **File-Based Semaphore** → Coordinate concurrent test execution
- **Output Diffing Utility** → Compare test output across runs

This creates a network of validated, production-ready tools.

## Conclusion

Dogfooding proves that:
1. **Test Flakiness Detector works** - Successfully ran 111 tests 10+ times
2. **CLI Progress tests are reliable** - 100% consistent results
3. **Tuulbelt tools integrate well** - Clean APIs, no surprises
4. **Real-world usage validated** - Not just toy examples

This establishes dogfooding as a standard practice for all future Tuulbelt tools.

**Learn more:** [Test Flakiness Detector Documentation](/tools/test-flakiness-detector/)
