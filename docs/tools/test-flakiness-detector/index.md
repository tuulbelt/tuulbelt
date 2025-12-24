# Test Flakiness Detector

Detect unreliable tests by running them multiple times and tracking failure rates.

## Overview

Test Flakiness Detector helps identify non-deterministic tests by running your test suite multiple times and analyzing the results.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 107 tests across 4 categories

## Features

- <img src="/icons/target.svg" class="inline-icon" alt=""> Framework agnostic - works with any test command
- <img src="/icons/search.svg" class="inline-icon" alt=""> Detects flaky tests through repeated execution
- <img src="/icons/bar-chart.svg" class="inline-icon" alt=""> Comprehensive JSON reports with failure rates
- <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero runtime dependencies
- <img src="/icons/rocket.svg" class="inline-icon" alt=""> 107+ tests with 80%+ coverage

## Quick Start

```bash
cd test-flakiness-detector
npm install
npx tsx src/index.ts --test "npm test" --runs 10
```

## Demo

See the tool in action:

![Test Flakiness Detector Demo](/test-flakiness-detector/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/WgUQsHvqswlUFvfJekVH07jXi)**

> Demos are automatically generated via GitHub Actions when demo scripts are updated.

## Links

- [Getting Started](/tools/test-flakiness-detector/getting-started)
- [CLI Usage](/tools/test-flakiness-detector/cli-usage)
- [Library Usage](/tools/test-flakiness-detector/library-usage)
- [Examples](/tools/test-flakiness-detector/examples)
- [API Reference](/tools/test-flakiness-detector/api-reference)

