# CLI Usage

Command-line interface for Test Flakiness Detector.

## Basic Syntax

```bash
flaky [options]
```

## Options

### `--test <command>`

**Required.** The test command to execute repeatedly.

```bash
flaky --test "npm test"
```

### `--runs <number>`

**Optional.** Number of times to run the tests. Default: `10`, Max: `1000`

```bash
flaky --test "npm test" --runs 20
```

### `--verbose`

**Optional.** Enable verbose output showing each run's result. Default: `false`

```bash
flaky --test "npm test" --verbose
```

### `--help`

Display help message with all available options.

```bash
flaky --help
```

## Examples

### Basic Flakiness Detection

```bash
flaky --test "npm test" --runs 10
```

### With Verbose Output

```bash
flaky --test "npm test" --runs 20 --verbose
```

### Different Test Frameworks

**Jest:**
```bash
flaky --test "npm run test:jest" --runs 15
```

**Pytest:**
```bash
flaky --test "pytest tests/" --runs 20
```

**Cargo (Rust):**
```bash
flaky --test "cargo test" --runs 10
```

**Go:**
```bash
flaky --test "go test ./..." --runs 15
```

**Vitest:**
```bash
flaky --test "vitest run" --runs 20
```

## Output Format

The tool outputs JSON to `flakiness-report.json`:

```json
{
  "success": true,
  "totalRuns": 10,
  "passedRuns": 8,
  "failedRuns": 2,
  "flakyTests": [
    {
      "testName": "Test Suite",
      "passed": 8,
      "failed": 2,
      "totalRuns": 10,
      "failureRate": 20.0
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

## Exit Codes

- `0` — Success: Detection completed, no flaky tests found
- `1` — Flaky Detected: One or more flaky tests found
- `2` — Invalid Args: Invalid arguments or validation error

**Example:**
```bash
flaky --test "npm test" --runs 10
echo $?  # 0 = no flaky, 1 = flaky found, 2 = invalid args
```

**Use in CI/CD:**
```bash
# Fail the build if flaky tests are detected
flaky --test "npm test" --runs 20
EXIT_CODE=$?

if [ $EXIT_CODE -eq 1 ]; then
  echo "❌ Flaky tests detected - failing the build"
  exit 1
elif [ $EXIT_CODE -eq 2 ]; then
  echo "❌ Invalid arguments or execution error"
  exit 2
else
  echo "✅ No flaky tests detected"
fi
```

## Integration with CI/CD

### GitHub Actions

```yaml
name: Flakiness Detection

on: [push, pull_request]

jobs:
  detect-flakiness:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm install

      - name: Run flakiness detection
        run: |
          cd test-flakiness-detector
          flaky --test "npm test" --runs 20

      - name: Upload report
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: flakiness-report
          path: test-flakiness-detector/flakiness-report.json
```

### GitLab CI

```yaml
test:flakiness:
  script:
    - cd test-flakiness-detector
    - npm install
    - flaky --test "npm test" --runs 20
  artifacts:
    paths:
      - test-flakiness-detector/flakiness-report.json
    when: always
```

### Jenkins

```groovy
stage('Flakiness Detection') {
  steps {
    dir('test-flakiness-detector') {
      sh 'npm install'
      sh 'flaky --test "npm test" --runs 20'
      archiveArtifacts artifacts: 'flakiness-report.json'
    }
  }
}
```

## Tips & Best Practices

### Choosing Run Count

- **Highly flaky tests:** 5-10 runs sufficient
- **Occasionally flaky:** 20-50 runs recommended
- **Rarely flaky:** 100+ runs may be needed
- **Production validation:** 50-100 runs for confidence

### Use Verbose Mode for Debugging

```bash
flaky --test "npm test" --runs 10 --verbose
```

This shows the output of each individual run, helpful for diagnosing why tests are flaky.

### Analyzing Results

```bash
# Run detection
flaky --test "npm test" --runs 20

# Check the report
cat flakiness-report.json | jq '.summary'

# Count flaky runs
cat flakiness-report.json | jq '.runs | map(select(.success == false)) | length'
```

### Performance Considerations

For slow test suites, use fewer runs:

```bash
# For suites that take > 1 minute
flaky --test "npm test" --runs 5

# For fast suites (< 10 seconds)
flaky --test "npm test" --runs 50
```

## See Also

- [Getting Started](/tools/test-flakiness-detector/getting-started) — Installation and setup
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage examples
- [API Reference](/tools/test-flakiness-detector/api-reference) — Programmatic API
- [Library Usage](/tools/test-flakiness-detector/library-usage) — TypeScript/JavaScript integration
