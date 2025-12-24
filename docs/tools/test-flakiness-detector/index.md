# Test Flakiness Detector

Detect unreliable tests by running them multiple times and tracking failure rates.

## Overview

Test Flakiness Detector helps identify non-deterministic tests by running your test suite multiple times and analyzing the results.

**Status:** ✅ Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 107 tests across 4 categories

## Features

- 🎯 Framework agnostic - works with any test command
- 🔍 Detects flaky tests through repeated execution
- 📊 Comprehensive JSON reports with failure rates
- ⚡ Zero runtime dependencies
- 🚀 107+ tests with 80%+ coverage

## Quick Start

```bash
cd test-flakiness-detector
npm install
npx tsx src/index.ts --test "npm test" --runs 10
```

##Links

- [Getting Started](/tools/test-flakiness-detector/getting-started)
- [CLI Usage](/tools/test-flakiness-detector/cli-usage)
- [Library Usage](/tools/test-flakiness-detector/library-usage)
- [Examples](/tools/test-flakiness-detector/examples)
- [API Reference](/tools/test-flakiness-detector/api-reference)

*Full documentation coming soon. See the [tool README](https://github.com/tuulbelt/tuulbelt/tree/main/test-flakiness-detector) for complete information.*
