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

---

## CI/CD Integration

### GitHub Actions - Fast Gate

Use `isFlaky()` for quick CI gates:

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
        run: npm ci

      - name: Check for flaky tests (fast gate)
        run: |
          # Use isFlaky() via CLI for fast feedback (5 runs)
          flaky --test "npm test" --runs 5
        # Exit code 1 = flaky detected, fails the job
        # Exit code 2 = invalid args, fails the job
        # Exit code 0 = no flaky, continues
```

### GitHub Actions - Full Report with Artifacts

Use `detect()` for comprehensive reports:

```yaml
name: Nightly Flakiness Check

on:
  schedule:
    - cron: '0 2 * * *'  # 2 AM daily

jobs:
  flakiness-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm ci

      - name: Deep flakiness detection
        run: flaky --test "npm test" --runs 20

      - name: Upload flakiness report
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: flakiness-report
          path: flakiness-report.json
```

### GitLab CI

```yaml
test:flakiness:
  script:
    - npm ci
    - flaky --test "npm test" --runs 10
  artifacts:
    paths:
      - flakiness-report.json
    when: always
  allow_failure: false  # Fail pipeline if flaky tests detected
```

### Jenkins

```groovy
stage('Flakiness Detection') {
  steps {
    sh 'npm ci'
    sh 'flaky --test "npm test" --runs 15'
    archiveArtifacts artifacts: 'flakiness-report.json', allowEmptyArchive: true
  }
}
```

### CircleCI

```yaml
version: 2.1

jobs:
  flakiness-check:
    docker:
      - image: cimg/node:20.11
    steps:
      - checkout
      - run: npm ci
      - run:
          name: Check for flaky tests
          command: flaky --test "npm test" --runs 10
      - store_artifacts:
          path: flakiness-report.json
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🔍 Checking for flaky tests..."

flaky --test "npm test" --runs 10
EXIT_CODE=$?

if [ $EXIT_CODE -eq 1 ]; then
  echo "❌ Cannot commit: Flaky tests detected!"
  echo "Run 'cat flakiness-report.json' for details"
  exit 1
elif [ $EXIT_CODE -eq 2 ]; then
  echo "❌ Cannot commit: Invalid test configuration"
  exit 1
fi

echo "✅ No flaky tests detected"
```

---

## Library Usage Examples

### Basic TypeScript Integration with `detect()`

```typescript
import { detect } from './test-flakiness-detector/src/index.js'

async function checkFlakiness() {
  const result = await detect({
    test: 'npm test',
    runs: 20
  })

  if (result.ok === false) {
    console.error(`❌ Detection failed: ${result.error.message}`)
    process.exit(2)
  }

  const report = result.value

  if (report.flakyTests.length > 0) {
    console.error(`⚠️  Flaky tests detected!`)
    report.flakyTests.forEach(test => {
      console.error(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`)
    })
    console.error(`Failed ${report.failedRuns} out of ${report.totalRuns} runs`)
    process.exit(1)
  }

  console.log('✅ No flakiness detected')
}

checkFlakiness()
```

### Fast CI Gate with `isFlaky()`

```typescript
import { isFlaky } from './test-flakiness-detector/src/index.js'

async function ciGate() {
  console.log('🔍 Running quick flakiness check...')

  const result = await isFlaky({
    test: 'npm test',
    runs: 5  // Fast: 5 runs for CI gate
  })

  if (result.ok === false) {
    console.error(`❌ Check failed: ${result.error.message}`)
    process.exit(2)
  }

  if (result.value) {
    console.error('❌ CI GATE FAILED: Flaky tests detected')
    console.error('Please investigate and fix non-deterministic tests')
    process.exit(1)
  }

  console.log('✅ CI gate passed - no flaky tests')
  process.exit(0)
}

ciGate()
```

### Progressive Detection Strategy with `compileDetector()`

```typescript
import { compileDetector } from './test-flakiness-detector/src/index.js'

async function progressiveDetection() {
  const detector = compileDetector({
    test: 'npm test',
    verbose: false
  })

  // Start with quick check
  console.log('Quick check (5 runs)...')
  let result = await detector.run(5)

  if (result.ok && result.value.flakyTests.length === 0) {
    console.log('✅ Quick check passed')
    return
  }

  // Escalate to medium confidence
  console.log('Medium confidence check (15 runs)...')
  result = await detector.run(15)

  if (result.ok && result.value.flakyTests.length === 0) {
    console.log('✅ Medium check passed (false positive avoided)')
    return
  }

  // Confirm with high confidence
  console.log('High confidence check (50 runs)...')
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

  console.log('✅ All checks passed')
}

progressiveDetection()
```

### Advanced Analysis

```typescript
import { detect } from './test-flakiness-detector/src/index.js'
import { writeFile } from 'node:fs/promises'

async function analyzeFlakiness() {
  const result = await detect({
    test: 'npm test',
    runs: 100,
    verbose: true
  })

  if (result.ok === false) {
    console.error('Error:', result.error.message)
    process.exit(2)
  }

  const report = result.value

  // Analyze flaky tests
  const analysis = {
    success: true,
    totalRuns: report.totalRuns,
    passedRuns: report.passedRuns,
    failedRuns: report.failedRuns,
    flakyTestCount: report.flakyTests.length,
    flakyTests: report.flakyTests.map(test => ({
      name: test.testName,
      failureRate: `${test.failureRate.toFixed(2)}%`,
      passed: test.passed,
      failed: test.failed
    }))
  }

  // Save detailed report
  await writeFile(
    'flakiness-analysis.json',
    JSON.stringify(analysis, null, 2)
  )

  console.log('Analysis complete. See flakiness-analysis.json')
}

analyzeFlakiness()
```

### Test Suite Validation

```typescript
import { isFlaky } from './test-flakiness-detector/src/index.js'
import { test } from 'node:test'
import assert from 'node:assert/strict'

test('test suite should be deterministic', async () => {
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
}, { timeout: 600000 }) // 10 minute timeout for slow suites
```

### Multi-Suite Detection

```typescript
import { detect } from './test-flakiness-detector/src/index.js'

async function checkAllSuites() {
  const suites = [
    { name: 'Unit Tests', command: 'npm run test:unit' },
    { name: 'Integration Tests', command: 'npm run test:integration' },
    { name: 'E2E Tests', command: 'npm run test:e2e' },
  ]

  for (const suite of suites) {
    console.log(`\nTesting: ${suite.name}...`)

    const result = await detect({
      test: suite.command,
      runs: 10,
      verbose: false,
    })

    if (result.ok === false) {
      console.error(`  ⚠️  Detection failed: ${result.error.message}`)
      continue
    }

    const report = result.value

    if (report.flakyTests.length > 0) {
      console.error(`  ❌ Flaky tests found: ${report.flakyTests.length}`)
      report.flakyTests.forEach(test => {
        console.error(`    - ${test.testName} (${test.failureRate.toFixed(1)}%)`)
      })
    } else {
      console.log(`  ✅ No flakiness detected`)
    }
  }
}

checkAllSuites()
```

---

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

---

## Best Practices

### Choosing the Right API

**Use `isFlaky()` for:**
- CI/CD pipeline gates
- Pre-merge validation
- Quick yes/no decisions

**Use `detect()` for:**
- Debugging flaky tests
- Generating detailed reports
- Analyzing failure patterns

**Use `compileDetector()` for:**
- Progressive detection strategies
- Running multiple times with different counts
- Caching detector configuration

### Choosing Run Count

- **Highly flaky tests:** 5-10 runs sufficient
- **Occasionally flaky:** 20-50 runs recommended
- **Rarely flaky:** 100+ runs may be needed
- **Production validation:** 50-100 runs for confidence

### Performance Considerations

For slow test suites, use fewer runs:

```bash
# For suites that take > 1 minute
flaky --test "npm test" --runs 5

# For fast suites (< 10 seconds)
flaky --test "npm test" --runs 50
```

---

## See Also

- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
- [Library Usage](/tools/test-flakiness-detector/library-usage) — TypeScript/JavaScript API
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — Command-line interface
- [API Reference](/tools/test-flakiness-detector/api-reference) — Complete API documentation
