# Examples

Real-world examples of using Test Flakiness Detector.

## CLI Examples

### Basic Detection

```bash
# Run your test suite 10 times
flaky --test "npm test"
```

### High-Confidence Detection

```bash
# Run 50 times for rare flaky tests
flaky --test "npm test" --runs 50
```

### Verbose Debugging

```bash
# See output from each run
flaky --test "npm test" --runs 10 --verbose
```

### Different Test Frameworks

```bash
# Jest
flaky --test "npm run test:jest" --runs 20

# Vitest
flaky --test "vitest run" --runs 20

# Rust
flaky --test "cargo test" --runs 15

# Python
flaky --test "pytest tests/" --runs 20

# Go
flaky --test "go test ./..." --runs 15
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Flakiness Check

on: [pull_request]

jobs:
  flakiness:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: |
          cd test-flakiness-detector
          npm install

      - name: Detect flaky tests
        run: |
          cd test-flakiness-detector
          flaky --test "npm test" --runs 20

      - name: Check for flakiness
        run: |
          cd test-flakiness-detector
          if [ $(jq '.flakyTests | length' flakiness-report.json) -gt 0 ]; then
            echo "❌ Flaky tests detected!"
            exit 1
          fi
          echo "✅ No flaky tests"

      - name: Upload report
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: flakiness-report
          path: test-flakiness-detector/flakiness-report.json
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🔍 Checking for flaky tests..."

cd test-flakiness-detector
flaky --test "npm test" --runs 10

if [ $(jq '.flakyTests | length' flakiness-report.json) -gt 0 ]; then
  echo "❌ Cannot commit: Flaky tests detected!"
  echo "Run 'cat test-flakiness-detector/flakiness-report.json' for details"
  exit 1
fi

echo "✅ No flaky tests detected"
```

## Library Usage Examples

### Basic TypeScript Integration

```typescript
import { detectFlakiness } from './test-flakiness-detector/src/index.ts'

async function checkFlakiness() {
  const result = await detectFlakiness({
    testCommand: 'npm test',
    runs: 20
  })

  if (result.success && result.flakyTests.length > 0) {
    console.error(`⚠️  Flaky tests detected!`)
    result.flakyTests.forEach(test => {
      console.error(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`)
    })
    console.error(`Failed ${result.failedRuns} out of ${result.totalRuns} runs`)
    process.exit(1)
  } else if (!result.success) {
    console.error(`Error: ${result.error}`)
    process.exit(1)
  }

  console.log('✅ No flakiness detected')
}

checkFlakiness()
```

### Advanced Analysis

```typescript
import { detectFlakiness } from './test-flakiness-detector/src/index.ts'
import { writeFile } from 'node:fs/promises'

async function analyzeFlakiness() {
  const result = await detectFlakiness({
    testCommand: 'npm test',
    runs: 100,
    verbose: true
  })

  // Analyze flaky tests
  const report = {
    success: result.success,
    totalRuns: result.totalRuns,
    passedRuns: result.passedRuns,
    failedRuns: result.failedRuns,
    flakyTestCount: result.flakyTests.length,
    flakyTests: result.flakyTests.map(test => ({
      name: test.testName,
      failureRate: `${test.failureRate.toFixed(2)}%`,
      passed: test.passed,
      failed: test.failed
    }))
  }

  // Save detailed report
  await writeFile(
    'flakiness-analysis.json',
    JSON.stringify(report, null, 2)
  )

  console.log('Analysis complete. See flakiness-analysis.json')
}

analyzeFlakiness()
```

### Test Suite Validation

```typescript
import { detectFlakiness } from './test-flakiness-detector/src/index.ts'
import { test } from 'node:test'
import assert from 'node:assert/strict'

test('test suite should be deterministic', async () => {
  const result = await detectFlakiness({
    testCommand: 'npm test',
    runs: 10
  })

  assert.strictEqual(
    result.flakyTests.length,
    0,
    `Tests are flaky! ${result.failedRuns}/${result.totalRuns} runs failed`
  )
}, { timeout: 600000 }) // 10 minute timeout for slow suites
```

## Analyzing Results

### Reading the JSON Report

```bash
# Check if flaky tests detected
cat flakiness-report.json | jq '.flakyTests | length'

# List all flaky tests
cat flakiness-report.json | jq '.flakyTests[]'

# List all failed runs
cat flakiness-report.json | jq '.runs[] | select(.success == false)'

# Get overall statistics
cat flakiness-report.json | jq '{totalRuns, passedRuns, failedRuns, flakyTestCount: (.flakyTests | length)}'
```

### Example Report

```json
{
  "success": true,
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
    {
      "success": true,
      "exitCode": 0,
      "stdout": "...",
      "stderr": ""
    },
    {
      "success": false,
      "exitCode": 1,
      "stdout": "...",
      "stderr": "Error: test failed"
    }
    // ... more runs
  ]
}
```

## See Also

- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [Library Usage](/tools/test-flakiness-detector/library-usage) — TypeScript/JavaScript API
- [API Reference](/tools/test-flakiness-detector/api-reference) — Complete API documentation
