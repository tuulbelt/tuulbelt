# Test Flakiness Detector

Detect unreliable tests by running them multiple times and tracking failure rates.

## Overview

Test Flakiness Detector helps identify non-deterministic tests by running your test suite multiple times and analyzing the results. It's framework-agnostic and works with any test command.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** TypeScript

**Repository:** [tuulbelt/tuulbelt/test-flakiness-detector](https://github.com/tuulbelt/tuulbelt/tree/main/test-flakiness-detector)

## Features

### <img src="/icons/target.svg" class="inline-icon" alt=""> Framework Agnostic

Works with any test command - Jest, Vitest, Pytest, Cargo, Go tests, or any other test framework. Just provide the test command and run count.

### <img src="/icons/search.svg" class="inline-icon" alt=""> Repeated Execution Analysis

Detects flaky tests by running your test suite multiple times and tracking which tests pass sometimes and fail sometimes.

### <img src="/icons/bar-chart.svg" class="inline-icon" alt=""> Comprehensive JSON Reports

Generates detailed JSON reports with failure rates, individual run results, timestamps, and execution duration for analysis.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Node.js built-ins. No `npm install` required in production.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/test-flakiness-detector

# Install dev dependencies (for TypeScript)
npm install

# Detect flaky tests
flaky --test "npm test" --runs 10
```

## Use Cases

- **CI/CD Validation:** Catch unreliable tests before they merge
- **Test Suite Health:** Identify which tests need fixing
- **Pre-Release Checks:** Ensure test stability before shipping
- **Root Cause Analysis:** Understand why tests fail intermittently
- **Quality Metrics:** Track test reliability over time

## Why Flakiness Detection?

Flaky tests undermine confidence in your test suite:

1. **False Positives:** Tests fail even when code is correct
2. **Wasted Time:** Developers re-run tests or investigate non-issues
3. **Reduced Trust:** Teams start ignoring test failures
4. **Hidden Bugs:** Real failures get dismissed as "just flaky"
5. **CI/CD Friction:** Builds fail randomly, blocking deployments

This tool helps you identify and fix flaky tests systematically.

## Demo

See the tool in action:

![Test Flakiness Detector Demo](/test-flakiness-detector/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/1Swn7Cta5dSsMVLPLPKNYmKcm)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/test-flakiness-detector" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

Run the detector directly in your browser with zero setup. Test different commands, adjust run counts, and explore flakiness detection patterns.

> Demos are automatically generated via GitHub Actions when demo scripts are updated.

## Next Steps

- [Getting Started Guide](/tools/test-flakiness-detector/getting-started) - Installation and setup
- [CLI Usage](/tools/test-flakiness-detector/cli-usage) - Command-line interface
- [Library Usage](/tools/test-flakiness-detector/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/test-flakiness-detector/examples) - Real-world usage patterns
- [API Reference](/tools/test-flakiness-detector/api-reference) - Complete API documentation

## License

MIT License - see repository for details.
