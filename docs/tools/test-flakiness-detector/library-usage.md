# Library Usage

Use Test Flakiness Detector programmatically in your TypeScript/JavaScript projects.

## Installation

Since this is a standalone tool with zero runtime dependencies, you can either:

1. **Clone and import directly:**
```bash
git clone https://github.com/tuulbelt/tuulbelt.git
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

if (result.summary.isFlaky) {
  console.log(`⚠️  Flaky tests detected! Failure rate: ${result.summary.failureRate}%`)
} else {
  console.log('✓ No flakiness detected')
}
```

## API Reference

### `detectFlakiness(options: DetectionOptions): Promise<DetectionResult>`

Main function to detect test flakiness.

**Parameters:**

```typescript
interface DetectionOptions {
  testCommand: string    // Test command to run (required)
  runs?: number          // Number of runs (default: 10, max: 1000)
  verbose?: boolean      // Enable verbose output (default: false)
}
```

**Returns:**

```typescript
interface DetectionResult {
  summary: {
    totalRuns: number
    passedRuns: number
    failedRuns: number
    isFlaky: boolean
    failureRate: number  // Percentage (0-100)
  }
  runs: Array<{
    runNumber: number
    success: boolean
    exitCode: number
    duration: number     // Milliseconds
    timestamp: string    // ISO 8601
    stdout?: string      // Only if verbose
    stderr?: string      // Only if verbose
  }>
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
result.runs.forEach(run => {
  if (!run.success) {
    console.log(`Run ${run.runNumber} failed:`)
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
    result.summary.isFlaky,
    false,
    `Tests are flaky! Failure rate: ${result.summary.failureRate}%`
  )
})
```

### Custom Analysis

```typescript
const result = await detectFlakiness({
  testCommand: 'npm test',
  runs: 100
})

// Calculate statistics
const avgDuration = result.runs.reduce((sum, r) => sum + r.duration, 0) / result.runs.length
const maxDuration = Math.max(...result.runs.map(r => r.duration))
const minDuration = Math.min(...result.runs.map(r => r.duration))

console.log(`Average duration: ${avgDuration}ms`)
console.log(`Max duration: ${maxDuration}ms`)
console.log(`Min duration: ${minDuration}ms`)

// Find failure patterns
const failures = result.runs.filter(r => !r.success)
console.log(`Failures occurred at runs: ${failures.map(r => r.runNumber).join(', ')}`)
```

## Type Definitions

The tool provides full TypeScript type definitions. Import them as needed:

```typescript
import type {
  DetectionOptions,
  DetectionResult,
  RunResult,
  Summary
} from './src/types.ts'
```

## See Also

- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns
- [API Reference](/tools/test-flakiness-detector/api-reference) — Complete API documentation
