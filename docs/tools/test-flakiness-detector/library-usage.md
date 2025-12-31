# Library Usage

Use Test Flakiness Detector programmatically in your TypeScript/JavaScript projects.

## Installation

Since this is a standalone tool with zero runtime dependencies, you can either:

1. **Clone and import directly:**
```bash
git clone https://github.com/tuulbelt/test-flakiness-detector.git
```

2. **Copy the source files** into your project

## Basic Usage

```typescript
import { detectFlakiness } from './src/index.ts'

const result = await detectFlakiness({
  testCommand: 'npm test',
  runs: 10,
  verbose: false
})

if (result.success && result.flakyTests.length > 0) {
  console.log(`⚠️  Flaky tests detected!`)
  result.flakyTests.forEach(test => {
    console.log(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`)
  })
} else if (result.success) {
  console.log('✓ No flakiness detected')
} else {
  console.error(`Error: ${result.error}`)
}
```

## API Reference

### `detectFlakiness(options: Config): Promise<FlakinessReport>`

Main function to detect test flakiness.

**Parameters:**

```typescript
interface Config {
  testCommand: string    // Test command to run (required)
  runs?: number          // Number of runs (default: 10, max: 1000)
  verbose?: boolean      // Enable verbose output (default: false)
}
```

**Returns:**

```typescript
interface FlakinessReport {
  success: boolean              // Whether detection completed successfully
  totalRuns: number            // Total number of test runs
  passedRuns: number           // Number of runs that passed
  failedRuns: number           // Number of runs that failed
  flakyTests: TestFlakiness[]  // List of flaky tests detected
  runs: TestRunResult[]        // All test run results
  error?: string               // Error message if detection failed
}

interface TestFlakiness {
  testName: string      // Name of the flaky test
  passed: number        // Number of times it passed
  failed: number        // Number of times it failed
  totalRuns: number     // Total runs for this test
  failureRate: number   // Failure rate percentage (0-100)
}

interface TestRunResult {
  success: boolean   // Whether this run passed
  exitCode: number   // Exit code from test command
  stdout: string     // Standard output
  stderr: string     // Standard error
}
```

## Advanced Examples

### With Custom Configuration

```typescript
const result = await detectFlakiness({
  testCommand: 'npm run test:integration',
  runs: 50,
  verbose: true
})

// Access detailed run information
result.runs.forEach((run, index) => {
  if (!run.success) {
    console.log(`Run ${index + 1} failed:`)
    console.log(run.stderr)
  }
})
```

### Error Handling

```typescript
try {
  const result = await detectFlakiness({
    testCommand: 'npm test',
    runs: 20
  })

  // Save report to file
  await writeFile(
    'flakiness-report.json',
    JSON.stringify(result, null, 2)
  )
} catch (error) {
  console.error('Flakiness detection failed:', error.message)
  process.exit(1)
}
```

### Integration with Test Suites

```typescript
import { detectFlakiness } from './test-flakiness-detector/src/index.ts'
import { test } from 'node:test'
import assert from 'node:assert/strict'

test('CI test suite should not be flaky', async () => {
  const result = await detectFlakiness({
    testCommand: 'npm test',
    runs: 10
  })

  assert.strictEqual(
    result.flakyTests.length,
    0,
    `Tests are flaky! ${result.flakyTests.length} flaky tests detected`
  )
})
```

### Custom Analysis

```typescript
const result = await detectFlakiness({
  testCommand: 'npm test',
  runs: 100
})

// Analyze flaky tests
if (result.flakyTests.length > 0) {
  console.log(`Found ${result.flakyTests.length} flaky tests:`)

  result.flakyTests.forEach(test => {
    console.log(`\n${test.testName}:`)
    console.log(`  Passed: ${test.passed}/${test.totalRuns}`)
    console.log(`  Failed: ${test.failed}/${test.totalRuns}`)
    console.log(`  Failure rate: ${test.failureRate.toFixed(2)}%`)
  })
}

// Find failure patterns
const failures = result.runs.filter(r => !r.success)
console.log(`\nTotal failures: ${failures.length} out of ${result.totalRuns} runs`)
```

## Type Definitions

The tool provides full TypeScript type definitions. Import them as needed:

```typescript
import type {
  Config,
  FlakinessReport,
  TestFlakiness,
  TestRunResult
} from './src/index.ts'
```

## See Also

- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns
- [API Reference](/tools/test-flakiness-detector/api-reference) — Complete API documentation
