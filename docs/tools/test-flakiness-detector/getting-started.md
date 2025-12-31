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

## Quick Example

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

## Next Steps

- [CLI Usage](/tools/test-flakiness-detector/cli-usage) — All CLI commands and options
- [Library Usage](/tools/test-flakiness-detector/library-usage) — TypeScript/JavaScript API
- [Examples](/tools/test-flakiness-detector/examples) — Real-world usage patterns

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
