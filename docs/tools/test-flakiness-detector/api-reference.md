# API Reference

Complete API documentation for Test Flakiness Detector.

## Main Function

### `detectFlakiness(config: Config): Promise<FlakinessReport>`

Runs a test command multiple times and detects flaky behavior.

**Parameters:**
- `config` - Configuration object (see [Config](#config))

**Returns:** `Promise<FlakinessReport>` - Detection results (see [FlakinessReport](#flakinessreport))

**Example:**
```typescript
import { detectFlakiness } from './src/index.js'

const report = await detectFlakiness({
  testCommand: 'npm test',
  runs: 20,
  verbose: false
})

if (report.success) {
  console.log(`Total runs: ${report.totalRuns}`)
  console.log(`Passed: ${report.passedRuns}, Failed: ${report.failedRuns}`)

  if (report.flakyTests.length > 0) {
    console.log('\nFlaky tests detected:')
    report.flakyTests.forEach(test => {
      console.log(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`)
      console.log(`    Passed: ${test.passed}/${test.totalRuns}`)
      console.log(`    Failed: ${test.failed}/${test.totalRuns}`)
    })
  } else {
    console.log('No flaky tests detected')
  }
} else {
  console.error(`Error: ${report.error}`)
}
```

---

## Type Definitions

### `Config`

Configuration options for flakiness detection.

```typescript
interface Config {
  /** Test command to execute (required) */
  testCommand: string

  /** Number of times to run the test (default: 10) */
  runs?: number

  /** Enable verbose output showing each run (default: false) */
  verbose?: boolean
}
```

**Properties:**

| Property | Type | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `testCommand` | `string` | ✅ Yes | - | Shell command to execute for testing |
| `runs` | `number` | ❌ No | `10` | Number of test executions (1-1000) |
| `verbose` | `boolean` | ❌ No | `false` | Log each test run to stderr |

**Example:**
```typescript
const config: Config = {
  testCommand: 'npm test',
  runs: 50,
  verbose: true
}
```

---

### `FlakinessReport`

Complete flakiness detection report returned by `detectFlakiness()`.

```typescript
interface FlakinessReport {
  /** Whether the detection completed successfully */
  success: boolean

  /** Total number of test runs performed */
  totalRuns: number

  /** Number of runs that passed */
  passedRuns: number

  /** Number of runs that failed */
  failedRuns: number

  /** List of flaky tests (tests with 0 < failure rate < 100) */
  flakyTests: TestFlakiness[]

  /** All test run results */
  runs: TestRunResult[]

  /** Error message if detection failed */
  error?: string
}
```

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `success` | `boolean` | `true` if detection completed, `false` if error occurred |
| `totalRuns` | `number` | Total number of executions performed |
| `passedRuns` | `number` | Number of runs with exit code 0 |
| `failedRuns` | `number` | Number of runs with non-zero exit code |
| `flakyTests` | `TestFlakiness[]` | Tests that passed sometimes and failed sometimes |
| `runs` | `TestRunResult[]` | Detailed results for each execution |
| `error` | `string \| undefined` | Error message if `success` is `false` |

**Example (all passing):**
```json
{
  "success": true,
  "totalRuns": 10,
  "passedRuns": 10,
  "failedRuns": 0,
  "flakyTests": [],
  "runs": [
    {
      "success": true,
      "exitCode": 0,
      "stdout": "Test passed\n",
      "stderr": ""
    }
    // ... 9 more runs
  ]
}
```

**Example (flaky test detected):**
```json
{
  "success": true,
  "totalRuns": 20,
  "passedRuns": 12,
  "failedRuns": 8,
  "flakyTests": [
    {
      "testName": "Test Suite",
      "passed": 12,
      "failed": 8,
      "totalRuns": 20,
      "failureRate": 40.0
    }
  ],
  "runs": [
    // ... 20 run results
  ]
}
```

**Example (error):**
```json
{
  "success": false,
  "totalRuns": 0,
  "passedRuns": 0,
  "failedRuns": 0,
  "flakyTests": [],
  "runs": [],
  "error": "Test command must be a non-empty string"
}
```

---

### `TestFlakiness`

Flakiness statistics for a single test.

```typescript
interface TestFlakiness {
  /** Name or identifier of the test */
  testName: string

  /** Number of times the test passed */
  passed: number

  /** Number of times the test failed */
  failed: number

  /** Total number of runs */
  totalRuns: number

  /** Failure rate as a percentage (0-100) */
  failureRate: number
}
```

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `testName` | `string` | Test identifier (currently always "Test Suite") |
| `passed` | `number` | Count of successful runs (exit code 0) |
| `failed` | `number` | Count of failed runs (non-zero exit code) |
| `totalRuns` | `number` | Total runs (should equal `passed + failed`) |
| `failureRate` | `number` | Percentage of failed runs (0-100) |

**Example:**
```json
{
  "testName": "Test Suite",
  "passed": 7,
  "failed": 3,
  "totalRuns": 10,
  "failureRate": 30.0
}
```

**Note:** Currently, flakiness detection works at the suite level (entire test command pass/fail). Individual test names are not yet parsed from test runner output.

---

### `TestRunResult`

Result of a single test execution.

```typescript
interface TestRunResult {
  /** Whether the test command succeeded */
  success: boolean

  /** Exit code from the test command */
  exitCode: number

  /** Standard output from the test command */
  stdout: string

  /** Standard error from the test command */
  stderr: string
}
```

**Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `success` | `boolean` | `true` if exit code was 0, `false` otherwise |
| `exitCode` | `number` | Process exit code (0 = success, non-zero = failure) |
| `stdout` | `string` | Standard output captured from the command |
| `stderr` | `string` | Standard error captured from the command |

**Example (success):**
```json
{
  "success": true,
  "exitCode": 0,
  "stdout": "✓ All tests passed (5/5)\n",
  "stderr": ""
}
```

**Example (failure):**
```json
{
  "success": false,
  "exitCode": 1,
  "stdout": "",
  "stderr": "AssertionError: Expected true to equal false\n"
}
```

---

## Usage Examples

### Basic Flakiness Detection

```typescript
import { detectFlakiness } from './src/index.js'

const report = await detectFlakiness({
  testCommand: 'npm test',
  runs: 10
})

if (!report.success) {
  console.error('Detection failed:', report.error)
  process.exit(1)
}

if (report.flakyTests.length > 0) {
  console.error('❌ Flaky tests detected!')
  process.exit(1)
}

console.log('✅ All tests are reliable')
```

### Analyzing Failure Patterns

```typescript
const report = await detectFlakiness({
  testCommand: 'cargo test',
  runs: 50,
  verbose: true
})

if (report.success) {
  const failures = report.runs.filter(r => !r.success)

  console.log(`Failure rate: ${report.failedRuns}/${report.totalRuns}`)
  console.log(`Unique errors:`)

  const errors = new Set(failures.map(r => r.stderr))
  errors.forEach(err => console.log(`  - ${err}`))
}
```

### CI/CD Integration

```typescript
#!/usr/bin/env tsx
import { detectFlakiness } from './src/index.js'

async function main() {
  const report = await detectFlakiness({
    testCommand: process.argv[2] || 'npm test',
    runs: 20
  })

  if (!report.success) {
    console.error('ERROR:', report.error)
    process.exit(2)
  }

  if (report.flakyTests.length > 0) {
    console.error('FLAKY TESTS DETECTED:')
    report.flakyTests.forEach(test => {
      console.error(`  ${test.testName}: ${test.failureRate}% failure rate`)
    })
    process.exit(1)
  }

  console.log('All tests passed reliably')
  process.exit(0)
}

main()
```

### Checking Specific Failure Rate

```typescript
const report = await detectFlakiness({
  testCommand: 'pytest tests/',
  runs: 100
})

if (report.success) {
  const failureRate = (report.failedRuns / report.totalRuns) * 100

  if (failureRate > 5) {
    console.error(`Unacceptable failure rate: ${failureRate}%`)
    process.exit(1)
  }

  if (failureRate > 0) {
    console.warn(`Warning: ${failureRate}% failure rate detected`)
  }
}
```

---

## Error Handling

The `detectFlakiness()` function never throws errors. Instead, it returns a `FlakinessReport` with `success: false` and an `error` message.

**Common Error Scenarios:**

### Invalid Test Command

```typescript
const report = await detectFlakiness({
  testCommand: '',  // Empty string
  runs: 10
})

// report.success === false
// report.error === "Test command must be a non-empty string"
```

### Execution Errors

```typescript
const report = await detectFlakiness({
  testCommand: 'nonexistent-command',
  runs: 10
})

// report.success === true (detection completed)
// report.failedRuns === 10 (all runs failed)
// report.runs[0].stderr contains error message
```

**Best Practice:**
```typescript
const report = await detectFlakiness(config)

if (!report.success) {
  console.error('Configuration error:', report.error)
  return
}

if (report.failedRuns === report.totalRuns) {
  console.error('All tests failed - check test command')
  return
}

// ... analyze flakiness
```

---

## CLI Exit Codes

When used as a CLI tool (`flaky --test "npm test" --runs 10`), the process exits with:

- `0` — All tests passed consistently (no flakiness)
- `1` — Flaky tests detected, or all tests failed consistently
- `2` — Invalid arguments or execution error

---

## Performance Considerations

- **Time Complexity:** O(N × T) where N = runs, T = test execution time
- **Space Complexity:** O(N × S) where N = runs, S = output size per run
- **Resource Limits:**
  - Maximum runs: 1000 (prevents resource exhaustion)
  - Maximum buffer per run: 10MB (stdout + stderr combined)
- **Execution:** Tests run sequentially (not parallel) to avoid false flakiness from resource contention

**Example:**
```typescript
// For tests that take 2 seconds each:
const report = await detectFlakiness({
  testCommand: 'npm test',
  runs: 50  // Will take ~100 seconds (50 × 2s)
})
```

---

## See Also

- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [Library Usage](/tools/test-flakiness-detector/library-usage) — TypeScript/JavaScript integration
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns
