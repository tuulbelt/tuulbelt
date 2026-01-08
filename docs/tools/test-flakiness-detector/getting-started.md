# Getting Started

Get up and running with Test Flakiness Detector in minutes.

## Installation

### Clone the Repository

```bash
git clone https://github.com/tuulbelt/test-flakiness-detector.git
cd test-flakiness-detector
```

### Install Dev Dependencies

```bash
npm install
npm link     # Enable the 'flaky' command globally
```

:::tip Zero Runtime Dependencies
Test Flakiness Detector has **zero runtime dependencies**. The `npm install` step only installs development tools (TypeScript compiler, test runner). The tool itself uses only Node.js built-in modules. The `npm link` command creates a global symlink so you can use the `flaky` command from anywhere.
:::

## Quick Start: CLI

### Basic Flakiness Detection

Run your test command 10 times to detect flaky tests:

```bash
flaky --test "npm test"
```

Output:
```
🔍 Running test 10 times: npm test
⠋ Run 1/10...
✓ All runs completed

📊 Results:
  Successes: 8
  Failures: 2
  ⚠️  FLAKY TESTS DETECTED!
```

### With Custom Run Count

```bash
flaky --test "npm test" --runs 20
```

### Verbose Mode

See detailed execution logs:

```bash
flaky --test "npm test" --verbose
```

## Quick Start: Library

Test Flakiness Detector provides three APIs optimized for different use cases.

### 1. `detect()` — Full Detection API

Best for: Debugging, generating detailed reports, analyzing failure patterns

```typescript
import { detect } from './test-flakiness-detector/src/index.js'

const result = await detect({
  test: 'npm test',
  runs: 10
})

if (result.ok === false) {
  console.error('Detection failed:', result.error.message)
  process.exit(2)
}

const report = result.value

if (report.flakyTests.length > 0) {
  console.error('⚠️ Flaky tests detected!')
  report.flakyTests.forEach(test => {
    console.error(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`)
  })
  process.exit(1)
}

console.log('✅ No flakiness detected')
```

### 2. `isFlaky()` — Fast Boolean Check

Best for: CI/CD pipeline gates, pre-merge validation, quick yes/no decisions

```typescript
import { isFlaky } from './test-flakiness-detector/src/index.js'

const result = await isFlaky({
  test: 'npm test',
  runs: 5  // Faster: default is 5 for quick feedback
})

if (result.ok === false) {
  console.error('Check failed:', result.error.message)
  process.exit(2)
}

if (result.value) {
  console.error('⚠️ Flakiness detected!')
  process.exit(1)
}

console.log('✅ No flakiness detected')
```

### 3. `compileDetector()` — Pre-Compiled Detector

Best for: Progressive detection strategies, running multiple times with different counts

```typescript
import { compileDetector } from './test-flakiness-detector/src/index.js'

const detector = compileDetector({
  test: 'npm test',
  verbose: false
})

// Start with quick check
console.log('Quick check (5 runs)...')
let result = await detector.run(5)

if (result.ok && result.value.flakyTests.length === 0) {
  console.log('✅ Quick check passed')
} else {
  // Escalate to thorough check
  console.log('Running thorough check (50 runs)...')
  result = await detector.run(50)

  if (result.ok === false) {
    console.error('Error:', result.error.message)
    process.exit(2)
  }

  if (result.value.flakyTests.length > 0) {
    console.log(`⚠️ Confirmed flaky: ${result.value.flakyTests.length} tests`)
    process.exit(1)
  }
}
```

### Result Type Pattern

All APIs use a non-throwing `Result<T>` type for predictable error handling:

```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: Error }
```

**Always check `result.ok === false` for proper TypeScript type narrowing:**

```typescript
const result = await detect({ test: 'npm test', runs: 10 })

// ✅ Correct: Proper type narrowing
if (result.ok === false) {
  console.error('Error:', result.error.message)
  return
}

// Safe: TypeScript knows result.value is DetectionReport
const report = result.value
console.log(report.totalRuns)
```

## CLI Options

- `--test <command>` — Test command to run (required)
- `--runs <number>` — Number of times to run (default: 10, max: 1000)
- `--verbose` — Show detailed execution logs
- `--help` — Show help message

## Understanding Results

The tool categorizes test behavior into three patterns:

### ✅ All Pass
No flakiness detected. Your tests are reliable!

```
📊 Results:
  Successes: 10
  Failures: 0
  ✓ No flakiness detected
```

### ❌ All Fail
Consistent failure (not flaky). Your tests have a deterministic bug.

```
📊 Results:
  Successes: 0
  Failures: 10
  ✓ No flakiness (consistent failure)
```

### ⚠️ Mixed Results
Flaky tests detected! Your tests pass sometimes and fail sometimes.

```
📊 Results:
  Successes: 7
  Failures: 3
  ⚠️  FLAKY TESTS DETECTED!
```

## Choosing the Right API

| Use Case | Recommended API | Default Runs |
|----------|----------------|--------------|
| CI/CD pipeline gate | `isFlaky()` | 5 |
| Debugging flaky tests | `detect()` | 10 |
| Progressive detection | `compileDetector()` | Variable |
| Quick yes/no answer | `isFlaky()` | 5 |
| Detailed reports | `detect()` | 10-50 |

## Next Steps

- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — All CLI commands and options
- [Library Usage](/tools/test-flakiness-detector/library-usage) — TypeScript/JavaScript API
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns
- [API Reference](/tools/test-flakiness-detector/api-reference) — Complete API documentation

## Troubleshooting

### Tests Pass Locally But Fail in CI

This is a classic sign of flakiness! Run the tool with a higher number of iterations:

```bash
flaky --test "npm test" --runs 50
```

### Permission Denied

If you get permission errors when running tests, check that your test command itself works:

```bash
# Verify your test command works independently
npm test
```

### Out of Memory

For very long-running test suites, limit the run count:

```bash
flaky --test "npm test" --runs 5
```
