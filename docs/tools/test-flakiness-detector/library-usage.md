# Library Usage

Use Test Flakiness Detector programmatically in your TypeScript/JavaScript projects.

## Installation

Since this is a standalone tool with zero runtime dependencies, you can either:

1. **Clone and import directly:**
```bash
git clone https://github.com/tuulbelt/test-flakiness-detector.git
```

2. **Copy the source files** into your project

## Three-Tier API Design

Test Flakiness Detector provides three APIs optimized for different use cases:

- **`detect()`** - Full detection with detailed reports (default: 10 runs)
- **`isFlaky()`** - Fast boolean check for CI gates (default: 5 runs)
- **`compileDetector()`** - Pre-compiled detector for repeated use

All APIs use a non-throwing `Result&lt;T&gt;` pattern for predictable error handling.

---

## Basic Usage

### `detect()` - Full Detection

```typescript
import { detect } from './src/index.js'

const result = await detect({
  test: 'npm test',
  runs: 10,
  verbose: false
})

if (result.ok === false) {
  console.error('❌ Detection failed:', result.error.message)
  process.exit(2)
}

const report = result.value

if (report.flakyTests.length > 0) {
  console.log(`⚠️  Flaky tests detected!`)
  report.flakyTests.forEach(test => {
    console.log(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`)
  })
  process.exit(1)
} else {
  console.log('✓ No flakiness detected')
  process.exit(0)
}
```

### `isFlaky()` - Fast Boolean Check

```typescript
import { isFlaky } from './src/index.js'

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

### `compileDetector()` - Pre-compiled Detector

```typescript
import { compileDetector } from './src/index.js'

// Compile detector once
const detector = compileDetector({
  test: 'npm test',
  verbose: false,
})

// Run with different counts
const quickResult = await detector.run(5)
const thoroughResult = await detector.run(20)
```

---

## Result Type Pattern

All APIs return `Result&lt;T&gt;` which never throws errors:

```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: Error }
```

**Always check `result.ok` before accessing `result.value`:**

```typescript
const result = await detect({ test: 'npm test', runs: 10 })

// ✅ Correct: Check .ok === false for proper type narrowing
if (result.ok === false) {
  console.error('Error:', result.error.message)
  return
}

// Safe: TypeScript knows result.value is DetectionReport
const report = result.value
console.log(report.totalRuns)
```

**Why Result type?**
- No try/catch required
- Type-safe error handling
- Explicit success/failure states
- Never throws unexpected errors

---

## Advanced Examples

### With Custom Configuration

```typescript
import { detect } from './src/index.js'

const result = await detect({
  test: 'npm run test:integration',
  runs: 50,
  verbose: true,
  threshold: 5.0  // Only flag tests with ≥5% failure rate
})

if (result.ok === false) {
  console.error('Error:', result.error.message)
  return
}

const report = result.value

// Access detailed run information
report.runs.forEach((run, index) => {
  if (!run.success) {
    console.log(`Run ${index + 1} failed:`)
    console.log(run.stderr)
  }
})
```

### Progressive Detection Strategy

```typescript
import { compileDetector } from './src/index.js'

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

  const report = result.value
  if (report.flakyTests.length > 0) {
    console.log(`⚠️ Confirmed flaky: ${report.flakyTests.length} tests`)
    process.exit(1)
  }
}
```

### Error Handling

```typescript
import { detect } from './src/index.js'
import { writeFile } from 'node:fs/promises'

async function runDetection() {
  const result = await detect({
    test: 'npm test',
    runs: 20
  })

  if (result.ok === false) {
    console.error('Detection failed:', result.error.message)
    process.exit(2)
  }

  // Save report to file
  await writeFile(
    'flakiness-report.json',
    JSON.stringify(result.value, null, 2)
  )

  console.log('Report saved to flakiness-report.json')
}

runDetection()
```

### Integration with Test Suites

```typescript
import { isFlaky } from './test-flakiness-detector/src/index.js'
import { test } from 'node:test'
import assert from 'node:assert/strict'

test('CI test suite should not be flaky', async () => {
  const result = await isFlaky({
    test: 'npm test',
    runs: 10
  })

  if (result.ok === false) {
    assert.fail(`Flakiness check failed: ${result.error.message}`)
  }

  assert.strictEqual(
    result.value,
    false,
    'Tests are flaky! Please fix non-deterministic tests'
  )
})
```

### Custom Analysis

```typescript
import { detect } from './src/index.js'

const result = await detect({
  test: 'npm test',
  runs: 100
})

if (result.ok === false) {
  console.error('Error:', result.error.message)
  process.exit(2)
}

const report = result.value

// Analyze flaky tests
if (report.flakyTests.length > 0) {
  console.log(`Found ${report.flakyTests.length} flaky tests:`)

  report.flakyTests.forEach(test => {
    console.log(`\n${test.testName}:`)
    console.log(`  Passed: ${test.passed}/${test.totalRuns}`)
    console.log(`  Failed: ${test.failed}/${test.totalRuns}`)
    console.log(`  Failure rate: ${test.failureRate.toFixed(2)}%`)
  })
}

// Find failure patterns
const failures = report.runs.filter(r => !r.success)
console.log(`\nTotal failures: ${failures.length} out of ${report.totalRuns} runs`)
```

### Detector Introspection

```typescript
import { compileDetector } from './src/index.js'

const detector = compileDetector({
  test: 'npm test',
  verbose: true,
  threshold: 1.0
})

// Inspect detector configuration
console.log('Detector command:', detector.getCommand())
console.log('Detector options:', detector.getOptions())

// Run detection
const result = await detector.run(10)
```

---

## Type Definitions

The tool provides full TypeScript type definitions:

```typescript
import type {
  // Configuration
  DetectConfig,

  // Results
  Result,
  DetectionReport,
  TestFlakiness,
  TestRunResult,

  // Compiled detector
  CompiledDetector
} from './src/index.js'
```

### Key Types

**DetectConfig:**
```typescript
interface DetectConfig {
  test: string          // Test command (required)
  runs?: number         // Run count (default: 10, range: 1-1000)
  verbose?: boolean     // Show progress (default: false)
  threshold?: number    // Failure rate % (default: 0.01 = 1%)
}
```

**DetectionReport:**
```typescript
interface DetectionReport {
  totalRuns: number              // Total runs executed
  passedRuns: number             // Successful runs
  failedRuns: number             // Failed runs
  flakyTests: TestFlakiness[]    // Flaky tests found
  runs: TestRunResult[]          // All run results
}
```

**Result&lt;T&gt;:**
```typescript
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: Error }
```

---

## API Selection Guide

### Use `detect()` when:
- Debugging flaky tests
- Need detailed reports
- Analyzing failure patterns
- Generating comprehensive documentation

### Use `isFlaky()` when:
- CI/CD pipeline gate
- Pre-merge validation
- Need quick yes/no answer
- Don't need detailed analysis

### Use `compileDetector()` when:
- Progressive detection (5 → 15 → 50 runs)
- Running same test command multiple times
- Caching detector configuration
- Need detector introspection

---

## Best Practices

### 1. Always Check Result Type

```typescript
// ✅ Good
if (result.ok === false) {
  console.error(result.error.message)
  return
}

// ❌ Bad
try {
  const report = (await detect({ test: 'npm test', runs: 10 })).value
} catch (e) {
  // This will never catch errors - Result type doesn't throw!
}
```

### 2. Use Appropriate API

```typescript
// ✅ Good: Fast CI gate
const result = await isFlaky({ test: 'npm test', runs: 5 })

// ❌ Bad: Overkill for CI gate
const result = await detect({ test: 'npm test', runs: 50 })
```

### 3. Handle All Error Cases

```typescript
const result = await detect({ test: 'npm test', runs: 10 })

if (result.ok === false) {
  console.error('Detection failed:', result.error.message)
  process.exit(2)  // Invalid arguments
}

if (result.value.flakyTests.length > 0) {
  console.error('Flaky tests detected')
  process.exit(1)  // Flakiness found
}

console.log('No flakiness detected')
process.exit(0)  // Success
```

### 4. Use Verbose Mode for Debugging

```typescript
const result = await detect({
  test: 'npm test',
  runs: 10,
  verbose: true  // Shows progress for slow test suites
})
```

---

## See Also

- [API Reference](/tools/test-flakiness-detector/api-reference) — Complete API documentation
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns
- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
