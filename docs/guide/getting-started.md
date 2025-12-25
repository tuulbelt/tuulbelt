# Getting Started

Start using Tuulbelt tools in minutes.

## Quick Start

### Clone the Repository

```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt
```

### Try Test Flakiness Detector

```bash
cd test-flakiness-detector
npm install  # Dev dependencies only
npx tsx src/index.ts --test "npm test" --runs 10
```

### Try CLI Progress Reporting

```bash
cd cli-progress-reporting
npm install  # Dev dependencies only

# Initialize progress
npx tsx src/index.ts init --total 100 --message "Processing files"

# Update progress
npx tsx src/index.ts increment --amount 10

# Check status
npx tsx src/index.ts get
```

### Try Cross-Platform Path Normalizer

```bash
cd cross-platform-path-normalizer
npm install  # Dev dependencies only

# Convert Windows path to Unix format
npx tsx src/index.ts --format unix "C:\Users\file.txt"
# Output: /c/Users/file.txt

# Auto-detect format
npx tsx src/index.ts "/home/user/file.txt"
# Output: /home/user/file.txt (unix)
```

## Available Tools

| Tool | Status | Description |
|------|--------|-------------|
| [Test Flakiness Detector](/tools/test-flakiness-detector/) | <img src="/icons/check-circle.svg" class="inline-icon" alt=""> v0.1.0 | Detect unreliable tests |
| [CLI Progress Reporting](/tools/cli-progress-reporting/) | <img src="/icons/check-circle.svg" class="inline-icon" alt=""> v0.1.0 | Concurrent-safe progress tracking |
| [Cross-Platform Path Normalizer](/tools/cross-platform-path-normalizer/) | <img src="/icons/check-circle.svg" class="inline-icon" alt=""> v0.1.0 | Convert Windows/Unix paths |
| File-Based Semaphore | <img src="/icons/target.svg" class="inline-icon" alt=""> Next | Process synchronization |
| Output Diffing Utility | <img src="/icons/circle.svg" class="inline-icon" alt=""> Planned | Semantic diff for test output |

**Progress:** 3 of 33 tools (9%)

## Installation Models

### 1. Clone Individual Tools

Each tool is independently cloneable:

```bash
# Clone just one tool
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/test-flakiness-detector
npm test && npm run build
```

### 2. Clone Entire Collection

Get all tools at once:

```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt
```

### 3. Use Without Cloning

Many tools can be used directly via `npx` (once published to npm):

```bash
# Future: Direct usage
npx @tuulbelt/test-flakiness-detector --test "npm test" --runs 10
```

## Zero Dependencies Philosophy

**Tuulbelt tools have ZERO runtime dependencies.**

When you run `npm install` in a tool directory, you're only installing:
- TypeScript compiler (devDependency)
- Test runner (devDependency)
- Type definitions (devDependency)

The tools themselves use **only Node.js built-in modules** or **Rust standard library**.

## Development Workflow

### 1. Explore a Tool

```bash
cd test-flakiness-detector
cat README.md  # Read documentation
ls examples/   # Check example outputs
```

### 2. Run Tests

```bash
npm test  # TypeScript tools
cargo test  # Rust tools
```

### 3. Try Examples

```bash
# Run tool with example data
npx tsx src/index.ts --test "echo test" --runs 5
```

### 4. Integrate Into Your Project

```typescript
// Import as library
import { detectFlakiness } from './path/to/tool/src/index.js';

// Or use via CLI
import { execSync } from 'child_process';
execSync('npx tsx ../test-flakiness-detector/src/index.ts --test "npm test"');
```

## Next Steps

### Learn the Philosophy
- [Philosophy](/guide/philosophy) - Why Tuulbelt exists
- [Principles](/guide/principles) - Design principles guiding every tool

### Explore Tools
- [Test Flakiness Detector](/tools/test-flakiness-detector/) - Comprehensive docs
- [CLI Progress Reporting](/tools/cli-progress-reporting/) - Full API reference
- [Cross-Platform Path Normalizer](/tools/cross-platform-path-normalizer/) - Path conversion guide

### Contribute
- [Contributing Guide](/guide/contributing) - How to build new tools
- [Quality Checklist](/guide/quality-checklist) - Pre-commit checks
- [Testing Standards](/guide/testing-standards) - Test requirements

## Common Questions

### Do I need to install dependencies?

**For development:** Yes, dev dependencies (TypeScript, test runners)
**For production:** No, zero runtime dependencies

### Can I use these in production?

Yes! All production-ready tools are battle-tested with 80%+ test coverage and dogfooding validation.

### Which programming languages are supported?

**TypeScript tools** work with Node.js 18+
**Rust tools** work with Rust 1.70+

All tools provide **CLI interfaces** that work from any language.

### Can I contribute?

Absolutely! See the [Contributing Guide](/guide/contributing) for how to propose and build new tools.

### Where are the tools hosted?

All tools live in the [tuulbelt/tuulbelt](https://github.com/tuulbelt/tuulbelt) monorepo. Each tool is independently documented and can be extracted to its own repository if needed.

## Support

- **Issues:** [GitHub Issues](https://github.com/tuulbelt/tuulbelt/issues)
- **Documentation:** This site
- **Source:** [GitHub Repository](https://github.com/tuulbelt/tuulbelt)
