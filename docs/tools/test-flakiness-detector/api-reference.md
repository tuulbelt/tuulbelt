# API Reference

Complete API documentation for Test Flakiness Detector.

## Multi-Tier API Design

Test Flakiness Detector provides three APIs optimized for different use cases:

1. **`detect()`** - Full detection with detailed reports (default: 10 runs)
2. **`isFlaky()`** - Fast boolean check for CI gates (default: 5 runs)
3. **`compileDetector()`** - Pre-compiled detector for repeated use

All APIs use a non-throwing `Result<T>` pattern for predictable error handling.

---

## Core APIs

### `detect(config: DetectConfig): Promise<Result<DetectionReport>>`

Comprehensive flakiness detection with detailed report generation.

**Purpose:** Debugging, analysis, generating reports

**Default runs:** 10 (balances speed and accuracy)

**Parameters:**

```typescript
interface DetectConfig {
  test: string          // Test command to run (required)
  runs?: number         // Number of runs (default: 10, range: 1-1000)
  verbose?: boolean     // Show progress during execution (default: false)
  threshold?: number    // Failure rate threshold % (default: 0.01 = 1%)
}
```

**Returns:** `Promise<Result<DetectionReport>>`

**Example:**

```typescript
import { detect } from 'test-flakiness-detector'

const result = await detect({
  test: 'npm test',
  runs: 20,
  verbose: true,
  threshold: 0.01  // Flag tests with ≥1% failure rate
})

if (result.ok === false) {
  console.error('Detection failed:', result.error.message)
  process.exit(2)
}

const report = result.value
console.log(`Total runs: ${report.totalRuns}`)
console.log(`Passed: ${report.passedRuns}, Failed: ${report.failedRuns}`)

if (report.flakyTests.length > 0) {
  console.log('⚠️ Flaky tests detected:')
  report.flakyTests.forEach(test => {
    console.log(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`)
  })
  process.exit(1)
}

console.log('✅ No flaky tests detected')
process.exit(0)
```

---

### `isFlaky(config: DetectConfig): Promise<Result<boolean>>`

Fast boolean check optimized for CI/CD pipeline gates.

**Purpose:** Quick CI gate, pre-merge validation

**Default runs:** 5 (faster than detect's 10)

**Parameters:** Same as `detect()` (see [DetectConfig](#detectconfig))

**Returns:** `Promise<Result<boolean>>`
- `true` = Flaky tests detected (fail the build)
- `false` = No flakiness detected (pass)

**Example:**

```typescript
import { isFlaky } from 'test-flakiness-detector'

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
} else {
  console.log('✅ No flakiness detected')
  process.exit(0)
}
```

---

### `compileDetector(config: DetectConfig): CompiledDetector`

Pre-compile detector configuration for repeated use with different run counts.

**Purpose:** Progressive detection (5 → 15 → 50 runs), caching

**Parameters:** Same as `detect()` (see [DetectConfig](#detectconfig))

**Returns:** `CompiledDetector` object with methods:
- `run(runs: number): Promise<Result<DetectionReport>>`
- `getCommand(): string`
- `getOptions(): Readonly<DetectConfig>`

**Example:**

```typescript
import { compileDetector } from 'test-flakiness-detector'

// Compile detector once with test command
const detector = compileDetector({
  test: 'npm test',
  verbose: false,
})

console.log(`Compiled detector for: ${detector.getCommand()}`)

// Run with different counts
console.log('Quick check (5 runs)...')
const quickResult = await detector.run(5)

if (quickResult.ok && !quickResult.value.flakyTests.length) {
  console.log('✅ Quick check passed')
} else {
  console.log('⚠️ Running thorough check (20 runs)...')
  const thoroughResult = await detector.run(20)

  if (thoroughResult.ok === false) {
    console.error('Error:', thoroughResult.error.message)
    process.exit(2)
  }

  const report = thoroughResult.value
  console.log(`Found ${report.flakyTests.length} flaky tests`)
}
```

---

## Type Definitions

### `Result<T>`

Non-throwing result type used by all APIs.

```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: Error }
```

**Usage pattern:**

```typescript
const result = await detect({ test: 'npm test', runs: 10 })

if (result.ok === false) {
  // Handle error
  console.error('Error:', result.error.message)
  return
}

// Access value safely
const report = result.value
console.log(report.totalRuns)
```

**Why Result type?**
- No try/catch required
- Type-safe error handling
- Explicit success/failure states
- Consistent pattern across all APIs

---

### `DetectConfig`

Configuration object for all detection APIs.

```typescript
interface DetectConfig {
  test: string          // Test command to run (required)
  runs?: number         // Number of runs (default: 10, range: 1-1000)
  verbose?: boolean     // Show progress during execution (default: false)
  threshold?: number    // Failure rate threshold % (default: 0.01 = 1%)
}
```

**Field details:**

- **`test`** (required): Shell command to execute your tests
  - Example: `'npm test'`, `'cargo test'`, `'pytest tests/'`

- **`runs`** (optional): Number of times to run the test command
  - Default: `10` for `detect()`, `5` for `isFlaky()`
  - Range: 1-1000
  - Higher values detect rarer flakiness but take longer

- **`verbose`** (optional): Show execution progress
  - Default: `false`
  - When `true`: Shows each run's result in real-time
  - Useful for debugging slow test suites

- **`threshold`** (optional): Minimum failure rate to flag as flaky
  - Default: `0.01` (1%)
  - Range: 0.0-100.0
  - Example: `5.0` = only flag tests with ≥5% failure rate

---

### `DetectionReport`

Detailed report returned by `detect()` and `compileDetector().run()`.

```typescript
interface DetectionReport {
  totalRuns: number              // Total test runs executed
  passedRuns: number             // Number of runs that passed
  failedRuns: number             // Number of runs that failed
  flakyTests: TestFlakiness[]    // Array of flaky tests detected
  runs: TestRunResult[]          // Individual run results
}
```

**Example report:**

```json
{
  "totalRuns": 10,
  "passedRuns": 7,
  "failedRuns": 3,
  "flakyTests": [
    {
      "testName": "Test Suite",
      "passed": 7,
      "failed": 3,
      "totalRuns": 10,
      "failureRate": 30.0
    }
  ],
  "runs": [
    { "success": true, "exitCode": 0, "stdout": "...", "stderr": "" },
    { "success": false, "exitCode": 1, "stdout": "...", "stderr": "Error: ..." }
  ]
}
```

---

### `TestFlakiness`

Information about a single flaky test.

```typescript
interface TestFlakiness {
  testName: string      // Name of the flaky test
  passed: number        // Number of times it passed
  failed: number        // Number of times it failed
  totalRuns: number     // Total runs for this test
  failureRate: number   // Failure rate percentage (0-100)
}
```

---

### `TestRunResult`

Result of a single test execution.

```typescript
interface TestRunResult {
  success: boolean   // Whether this run passed (exit code 0)
  exitCode: number   // Exit code from test command
  stdout: string     // Standard output captured
  stderr: string     // Standard error captured
}
```

---

### `CompiledDetector`

Pre-compiled detector object returned by `compileDetector()`.

```typescript
interface CompiledDetector {
  run(runs: number): Promise<Result<DetectionReport>>
  getCommand(): string
  getOptions(): Readonly<DetectConfig>
}
```

**Methods:**

- **`run(runs: number)`**: Execute detection with specified run count
  - Returns same `Result<DetectionReport>` as `detect()`
  - Run count overrides any `runs` specified in initial config

- **`getCommand()`**: Get the test command this detector will run
  - Returns: `string`

- **`getOptions()`**: Get the detector's configuration
  - Returns: `Readonly<DetectConfig>`
  - Useful for introspection and logging

---

## API Selection Guide

### When to use `detect()`

✅ **Use when:**
- Debugging flaky tests
- Generating detailed reports
- Analyzing failure patterns
- Need comprehensive run history

❌ **Don't use when:**
- Need fast CI gate (use `isFlaky()`)
- Running multiple times with different counts (use `compileDetector()`)

### When to use `isFlaky()`

✅ **Use when:**
- CI/CD pipeline gate
- Pre-merge validation
- Need yes/no answer quickly
- Don't need detailed report

❌ **Don't use when:**
- Need to know which specific tests are flaky (use `detect()`)
- Want detailed failure analysis (use `detect()`)

### When to use `compileDetector()`

✅ **Use when:**
- Progressive detection strategy (start with 5 runs, escalate to 50)
- Running same test command multiple times
- Want to cache detector configuration
- Need detector introspection

❌ **Don't use when:**
- Only running detection once (use `detect()` or `isFlaky()`)
- Need simplest possible API (use `detect()`)

---

## Error Handling

All APIs return `Result<T>` which never throws errors.

### Validation Errors

```typescript
const result = await detect({
  test: '',  // Invalid: empty string
  runs: 0    // Invalid: must be >= 1
})

if (result.ok === false) {
  console.error(result.error.message)
  // Output: "Test command must be a non-empty string"
}
```

### Execution Errors

```typescript
const result = await detect({
  test: 'invalid-command-that-does-not-exist',
  runs: 10
})

if (result.ok === false) {
  console.error(result.error.message)
  // Output: "spawn invalid-command-that-does-not-exist ENOENT"
}
```

### Best Practice

Always check `result.ok` before accessing `result.value`:

```typescript
const result = await detect({ test: 'npm test', runs: 10 })

// ✅ Correct: Check .ok first
if (result.ok === false) {
  console.error('Error:', result.error.message)
  process.exit(2)
}

const report = result.value  // TypeScript knows this is DetectionReport
console.log(report.totalRuns)

// ❌ Wrong: Don't use try/catch
try {
  const report = (await detect({ test: 'npm test', runs: 10 })).value
} catch (e) {
  // This will never catch errors - Result type doesn't throw!
}
```

---

## Exit Codes

When using as a library, you control exit codes. The recommended pattern:

```typescript
const result = await detect({ test: 'npm test', runs: 10 })

if (result.ok === false) {
  console.error('Detection failed:', result.error.message)
  process.exit(2)  // Invalid arguments or execution error
}

if (result.value.flakyTests.length > 0) {
  console.error('Flaky tests detected')
  process.exit(1)  // Flaky tests found
}

console.log('No flaky tests detected')
process.exit(0)  // Success
```

**Exit code meanings:**
- `0` - Success: Detection completed, no flaky tests found
- `1` - Flaky Detected: One or more flaky tests found
- `2` - Invalid Args: Invalid arguments or validation error

---

## See Also

- [Library Usage](/tools/test-flakiness-detector/library-usage) — Integration patterns
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns
- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
