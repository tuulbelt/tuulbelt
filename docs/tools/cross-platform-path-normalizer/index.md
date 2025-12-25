# Cross-Platform Path Normalizer

Convert Windows/Unix paths with zero dependencies. Handle UNC paths, mixed separators, and drive letters seamlessly.

## Overview

Cross-Platform Path Normalizer provides bidirectional path conversion between Windows and Unix formats. It automatically detects path formats, handles edge cases like UNC paths and mixed separators, and works entirely with zero runtime dependencies.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** TypeScript

**Repository:** [tuulbelt/tuulbelt/cross-platform-path-normalizer](https://github.com/tuulbelt/tuulbelt/tree/main/cross-platform-path-normalizer)

## Features

### <img src="/icons/link.svg" class="inline-icon" alt=""> Bidirectional Conversion

Convert Windows paths to Unix format and vice versa. Auto-detect source format or force specific output. Handles drive letters (`C:` ↔ `/c`) and path separators (`\` ↔ `/`).

### <img src="/icons/layers.svg" class="inline-icon" alt=""> UNC Path Support

Handle network paths correctly. Convert `\\server\share` ↔ `//server/share` with proper double-slash preservation for network file systems.

### <img src="/icons/sparkles.svg" class="inline-icon" alt=""> Mixed Separator Handling

Normalize paths with both forward and backslashes. Clean up messy paths like `C:/Users\Documents/file.txt` automatically.

### <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Type Safe

Written in TypeScript with strict mode enabled. Full type definitions and Result pattern for error handling with detailed error messages.

### <img src="/icons/tool.svg" class="inline-icon" alt=""> CLI & Library

Use as a command-line tool for quick conversions or integrate as a library in your Node.js/TypeScript projects.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Node.js built-ins. No `npm install` required in production.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/cross-platform-path-normalizer

# Install dev dependencies (for TypeScript)
npm install

# Convert a path
npx tsx src/index.ts --format unix "C:\Users\file.txt"
# Output: /c/Users/file.txt

# Library usage
import { normalizePath } from './src/index.js';
const result = normalizePath('C:\\Users\\file.txt', { format: 'unix' });
// { success: true, path: '/c/Users/file.txt', format: 'unix' }
```

## Use Cases

- **Cross-Platform Testing:** Normalize test fixture paths across Windows/Linux/macOS
- **Build Scripts:** Handle paths consistently in npm scripts and CI/CD pipelines
- **Path Validation:** Convert user input paths to canonical format
- **Documentation Examples:** Show paths in consistent format regardless of platform
- **Git Workflows:** Normalize paths before committing or comparing diffs

## Why Cross-Platform Path Normalizer?

Building cross-platform tools requires handling different path conventions:

1. **Incompatible Formats:** Windows uses `\` and drive letters, Unix uses `/`
2. **Mixed Input:** Users provide paths in inconsistent formats
3. **Heavy Dependencies:** Existing solutions pull in large dependency trees
4. **Edge Cases:** UNC paths, relative paths, and mixed separators need special handling
5. **Type Safety:** Runtime validation needed to catch path errors early

This tool provides zero-dependency path normalization with comprehensive edge case handling.

## Dogfooding: Test Reliability Validation

We use **[Test Flakiness Detector](/tools/test-flakiness-detector/)** (another Tuulbelt tool) to validate that all 145 tests are deterministic and reliable:

```bash
npm run test:dogfood
```

This runs the entire test suite 10 times (1,450 total test executions) to detect any non-deterministic behavior. Zero flaky tests detected across all runs, proving test suite reliability.

## Next Steps

- [Getting Started Guide](/tools/cross-platform-path-normalizer/getting-started) - Installation and setup
- [CLI Usage](/tools/cross-platform-path-normalizer/cli-usage) - Command-line interface
- [Library Usage](/tools/cross-platform-path-normalizer/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/cross-platform-path-normalizer/examples) - Real-world usage patterns
- [API Reference](/tools/cross-platform-path-normalizer/api-reference) - Complete API documentation

## License

MIT License - see repository for details.
