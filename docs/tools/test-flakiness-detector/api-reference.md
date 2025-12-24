# API Reference

Complete API documentation for Test Flakiness Detector.

## Main Function

### `detectFlakiness(options: DetectionOptions): Promise<DetectionResult>`

Runs a test command multiple times and detects flaky behavior.

**Example:**
```typescript
import { detectFlakiness } from './src/index.ts'

const result = await detectFlakiness({
  testCommand: 'npm test',
  runs: 20,
  verbose: false
})
```

## Type Definitions

### `DetectionOptions`

Configuration for flakiness detection.

```typescript
interface DetectionOptions {
  /** Test command to execute (required) */
  testCommand: string

  /** Number of times to run the test (default: 10, max: 1000) */
  runs?: number

  /** Enable verbose output showing each run's stdout/stderr (default: false) */
  verbose?: boolean
}
```

**Properties:**

- `testCommand` (required) — The shell command to execute for testing
- `runs` (optional) — Number of test executions. Must be between 1 and 1000. Default: 10
- `verbose` (optional) — When true, includes stdout/stderr in results. Default: false

### `DetectionResult`

Result object returned by `detectFlakiness()`.

```typescript
interface DetectionResult {
  /** Summary of all test runs */
  summary: Summary

  /** Individual run results */
  runs: RunResult[]
}
```

### `Summary`

Aggregate statistics across all runs.

```typescript
interface Summary {
  /** Total number of test runs executed */
  totalRuns: number

  /** Number of runs that passed */
  passedRuns: number

  /** Number of runs that failed */
  failedRuns: number

  /** True if any runs failed (indicating flakiness or consistent failure) */
  isFlaky: boolean

  /** Percentage of runs that failed (0-100) */
  failureRate: number
}
```

**Example:**
```json
{
  "totalRuns": 10,
  "passedRuns": 7,
  "failedRuns": 3,
  "isFlaky": true,
  "failureRate": 30
}
```

### `RunResult`

Result of a single test execution.

```typescript
interface RunResult {
  /** Sequential run number (1-indexed) */
  runNumber: number

  /** True if test exited with code 0 */
  success: boolean

  /** Process exit code */
  exitCode: number

  /** Test duration in milliseconds */
  duration: number

  /** ISO 8601 timestamp when the run started */
  timestamp: string

  /** Standard output (only if verbose=true) */
  stdout?: string

  /** Standard error (only if verbose=true) */
  stderr?: string
}
```

**Example (success):**
```json
{
  "runNumber": 1,
  "success": true,
  "exitCode": 0,
  "duration": 1234,
  "timestamp": "2025-01-01T00:00:00.000Z"
}
```

**Example (failure with verbose):**
```json
{
  "runNumber": 2,
  "success": false,
  "exitCode": 1,
  "duration": 1456,
  "timestamp": "2025-01-01T00:00:01.500Z",
  "stdout": "",
  "stderr": "AssertionError: Expected true to equal false"
}
```

## CLI Exit Codes

When used as a CLI tool, the process exits with:

- `0` — All tests passed consistently (no flakiness)
- `1` — Flaky tests detected, or all tests failed consistently
- `2` — Invalid arguments or execution error

## File Outputs

### `flakiness-report.json`

The tool writes a JSON report to `flakiness-report.json` in the current working directory.

**Structure:**
```json
{
  "summary": {
    "totalRuns": 10,
    "passedRuns": 8,
    "failedRuns": 2,
    "isFlaky": true,
    "failureRate": 20
  },
  "runs": [
    {
      "runNumber": 1,
      "success": true,
      "exitCode": 0,
      "duration": 1234,
      "timestamp": "2025-01-01T00:00:00.000Z"
    }
    // ... more runs
  ]
}
```

## Error Handling

The function throws errors for:

- **Invalid `runs` value:** Must be between 1 and 1000
- **Missing `testCommand`:** Test command is required
- **Execution errors:** If the test command cannot be executed

**Example:**
```typescript
try {
  const result = await detectFlakiness({
    testCommand: 'npm test',
    runs: 20
  })
} catch (error) {
  if (error.message.includes('runs')) {
    console.error('Invalid run count')
  } else {
    console.error('Execution failed:', error.message)
  }
}
```

## Usage Examples

### Check for Any Flakiness

```typescript
const result = await detectFlakiness({
  testCommand: 'npm test',
  runs: 10
})

if (result.summary.isFlaky) {
  console.error('Flaky tests detected!')
  process.exit(1)
}
```

### Analyze Failure Pattern

```typescript
const result = await detectFlakiness({
  testCommand: 'npm test',
  runs: 50,
  verbose: true
})

const failures = result.runs.filter(r => !r.success)
console.log(`${failures.length} failures out of ${result.summary.totalRuns} runs`)

failures.forEach(run => {
  console.log(`Run ${run.runNumber}: ${run.stderr}`)
})
```

### Calculate Statistics

```typescript
const result = await detectFlakiness({
  testCommand: 'npm test',
  runs: 100
})

const durations = result.runs.map(r => r.duration)
const avgDuration = durations.reduce((a, b) => a + b, 0) / durations.length

console.log(`Average duration: ${avgDuration}ms`)
console.log(`Failure rate: ${result.summary.failureRate}%`)
```

## See Also

- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [Library Usage](/tools/test-flakiness-detector/library-usage) — TypeScript/JavaScript integration
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns
