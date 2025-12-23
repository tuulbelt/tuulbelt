# Tuulbelt Quick Reference & Templates

## Quick Start Checklist (For Creating Each Tool)

```
☐ Step 1: File GitHub issue in tuulbelt/tuulbelt with tool proposal
☐ Step 2: Create new repo under github.com/tuulbelt/<tool-name>
☐ Step 3: Clone repo locally
☐ Step 4: Copy template from tuulbelt/templates/tool-repo-template/
☐ Step 5: Open in Claude Code at https://claude.ai/code
☐ Step 6: Implement src/index.ts using Claude Code
☐ Step 7: Write tests in test/index.test.ts
☐ Step 8: Create examples/basic.ts
☐ Step 9: Update README.md and SPEC.md
☐ Step 10: Run npm test (all pass)
☐ Step 11: Commit: git commit -m "Initial implementation v0.1.0"
☐ Step 12: Tag: git tag v0.1.0
☐ Step 13: Push: git push origin main --tags
☐ Step 14: Update meta repo README.md
☐ Step 15: Create GitHub release with notes
```

---

## Tool Repo Template Files (Copy to New Tools)

### package.json Template

```json
{
  "name": "@tuulbelt/[TOOL-NAME]",
  "version": "0.1.0",
  "description": "[ONE SENTENCE PROBLEM STATEMENT]",
  "main": "src/index.ts",
  "type": "module",
  "scripts": {
    "build": "tsc",
    "test": "node --test test/**/*.test.ts",
    "test:watch": "node --test --watch test/**/*.test.ts"
  },
  "keywords": [
    "tuulbelt",
    "[CATEGORY]"
  ],
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/tuulbelt/[TOOL-NAME].git"
  },
  "devDependencies": {
    "typescript": "^5.3.0"
  }
}
```

### tsconfig.json Template

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ES2020",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "moduleResolution": "node"
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "test", "dist"]
}
```

### .github/workflows/test.yml Template

```yaml
name: Test

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [18, 20]
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Use Node.js ${{ matrix.node-version }}
        uses: actions/setup-node@v3
        with:
          node-version: ${{ matrix.node-version }}
      
      - name: Run tests
        run: npm test
      
      - name: Check TypeScript
        run: npx tsc --noEmit
```

### README.md Template for Tools

```markdown
# [Tool Name]

[ONE SENTENCE DESCRIPTION OF WHAT IT DOES]

## Problem

[What problem does this solve? Why does it exist?]

## Features

- Zero dependencies
- [Feature 1]
- [Feature 2]

## Installation

Clone the repository:

```bash
git clone https://github.com/tuulbelt/[TOOL-NAME].git
cd [TOOL-NAME]
```

No npm install needed. This tool has zero runtime dependencies.

## Usage

### As a Library

```typescript
import { myFunction } from './src/index.ts';

const result = myFunction({ /* config */ });
console.log(result);
```

### As a CLI

```bash
node src/index.js --flag value --option another-value
```

## Examples

See `examples/` for runnable examples:

```bash
npx ts-node examples/basic.ts
```

## Testing

```bash
npm test              # Run all tests
npm test -- --watch  # Watch mode
```

## Specification

See [SPEC.md](SPEC.md) for detailed specification.

## Error Handling

[Document exit codes and common errors]

- Exit 0: Success
- Exit 1: [Error case]
- Exit 2: [Error case]

## Compatibility

- **Node.js:** 18+ (uses ES2020 features)
- **Browsers:** Not intended for browser use
- **Deno:** Should work, untested

## License

MIT
```

### SPEC.md Template (For Protocols/Formats)

```markdown
# [Tool Name] Specification

## Overview

[1-2 sentence description of the tool and what it does]

## Problem

[Why this tool exists]

## Design Goals

1. [Goal 1]
2. [Goal 2]
3. [Goal 3]

## Interface

### Input Format

[Description of input]

Example:
```
[Example input]
```

### Output Format

[Description of output]

Example:
```
[Example output]
```

### CLI Flags

```
--flag NAME       Description (default: value)
--option VALUE    Description
--help            Show help message
--version         Show version
```

## Algorithm / Wire Format

[If applicable: exact algorithm, state machine, or binary format]

### State Machine (Example)

```
[State] --[Event]--> [New State]
Start  --init--->   Initialized
```

### Binary Format (Example)

```
[1 byte]   Type (0x01, 0x02, etc.)
[4 bytes]  Length (big-endian uint32)
[N bytes]  Data
```

## Examples

### Example 1

Input:
```
[example input]
```

Output:
```
[example output]
```

### Example 2

Input:
```
[example input]
```

Output:
```
[example output]
```

## Error Cases

### Invalid Input

Input:
```
[invalid input]
```

Error:
```
Error: [error message]
```

### Missing Required Field

Error:
```
Error: Required field 'X' is missing
```

## Security Considerations

[Any security notes relevant to this tool]

## Performance

[Expected performance characteristics]

## Future Extensions

[Possible future additions without breaking changes]
```

### src/index.ts Template

```typescript
/**
 * [Tool Name]
 * 
 * [One sentence description]
 */

export interface Config {
  // Configuration options
}

export interface Result {
  // Result structure
}

/**
 * Main function that does the thing
 * @param input - The input to process
 * @param config - Configuration options
 * @returns The result
 */
export function myFunction(input: string, config: Config): Result {
  // Implementation
  return {
    // result
  };
}

// CLI entry point (if this tool is a CLI)
if (import.meta.main) {
  const args = process.argv.slice(2);
  
  try {
    const result = myFunction('input', {});
    console.log(JSON.stringify(result, null, 2));
  } catch (error) {
    if (error instanceof Error) {
      console.error(`Error: ${error.message}`);
      process.exit(1);
    }
    throw error;
  }
}
```

### test/index.test.ts Template

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { myFunction } from '../src/index.ts';

test('myFunction', async (t) => {
  await t.test('basic case', () => {
    const result = myFunction('input', {});
    assert(result);
  });

  await t.test('handles empty input', () => {
    const result = myFunction('', {});
    assert(result);
  });

  await t.test('throws on invalid input', () => {
    assert.throws(
      () => myFunction('invalid', {}),
      Error
    );
  });
});
```

### examples/basic.ts Template

```typescript
/**
 * Basic usage example for [Tool Name]
 */

import { myFunction } from '../src/index.ts';

// Example 1: Basic usage
const result1 = myFunction('example input', {
  // config options
});

console.log('Example 1 result:', result1);

// Example 2: With options
const result2 = myFunction('another input', {
  // different config
});

console.log('Example 2 result:', result2);
```

---

## GitHub Issue Templates

### bug_report.md

```markdown
---
name: Bug Report
about: Report a bug in this tool
title: '[BUG] '
labels: bug
---

## Description

[Describe the bug]

## Steps to Reproduce

1. [Step 1]
2. [Step 2]
3. [Step 3]

## Expected Behavior

[What should happen]

## Actual Behavior

[What actually happens]

## Environment

- Node.js version: [e.g., 20.0.0]
- OS: [e.g., macOS 13.0]

## Additional Context

[Any other information]
```

### feature_request.md

```markdown
---
name: Feature Request
about: Suggest an improvement
title: '[FEATURE] '
labels: enhancement
---

## Description

[Describe the feature]

## Use Case

[Why would this feature be useful?]

## Proposed Solution

[How might this be implemented?]

## Alternatives

[Are there alternative approaches?]
```

### tool_proposal.md

```markdown
---
name: New Tool Proposal
about: Propose a new tool for Tuulbelt
title: '[PROPOSAL] [Tool Name]'
labels: proposal
---

## Problem Statement

[One sentence describing the problem]

## Why It's Needed

[Why existing solutions don't work]

## Proposed Interface

[How would users interact with this tool?]

## Proposed Implementation

[Is there a proven way to implement this?]

## Examples

[What would using this tool look like?]
```

---

## Rust Tool Template (Cargo.toml)

```toml
[package]
name = "tuulbelt-[tool-name]"
version = "0.1.0"
edition = "2021"
description = "[ONE SENTENCE DESCRIPTION]"
license = "MIT"
repository = "https://github.com/tuulbelt/[tool-name]"

[[bin]]
name = "[tool-name]"
path = "src/main.rs"

[dependencies]

[dev-dependencies]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

## Recommended First 3 Tools (In Order)

### 1. Structured Error Handler (Week 1)
- **Scope:** Error format + serialization
- **Complexity:** Quick (days)
- **Output:** Library + examples
- **Why first:** Simplest, establishes patterns

### 2. Test Flakiness Detector (Week 2)
- **Scope:** Run tests N times, detect failures
- **Complexity:** Quick (days)
- **Output:** CLI tool + library
- **Why second:** Builds on first, simple logic

### 3. Output Diffing Utility (Week 2-3)
- **Scope:** Compare JSON/text outputs semantically
- **Complexity:** Quick-Medium (1 week)
- **Output:** Library + CLI
- **Why third:** Practical, combines concepts from first two

---

## Claude Code Initialization Prompt

Once you've created the meta repo in GitHub and cloned it in Claude Code, use this prompt:

```
You are setting up the Tuulbelt meta repository for the first time.

Task: Initialize the complete directory structure and documentation as specified in META_REPO_SETUP.md.

Steps:
1. Create all directories and files listed in Part 3 (Meta Repo Structure)
2. Copy exact file contents from Part 4 (File Contents)
3. Create all template files from the tools section
4. Create GitHub issue templates
5. Initialize git: git add . && git commit -m "Initialize meta repo"
6. Push to main: git push origin main

When complete, confirm all files are created and the repository is pushed to GitHub.
```

---

## Development Workflow Once Meta Repo Is Ready

### For Each New Tool:

1. **Create repo on GitHub** (github.com/tuulbelt/[tool-name])
2. **Clone locally** and open in Claude Code
3. **Copy template:**
   ```bash
   cp -r ../tuulbelt/templates/tool-repo-template/* .
   ```
4. **Scaffold in Claude Code:** Update package.json, create directories
5. **Implement:** Write src/index.ts
6. **Test:** Write test/index.test.ts
7. **Document:** Update README.md, SPEC.md
8. **Verify:** `npm test` passes
9. **Publish:**
   ```bash
   git tag v0.1.0
   git push origin main --tags
   ```
10. **Update meta repo:** Link to new tool in README.md

---

## File Checklist For Each Tool

Before declaring a tool "done":

- [ ] src/index.ts — Core implementation
- [ ] test/index.test.ts — Comprehensive tests
- [ ] examples/basic.ts — Usage example
- [ ] README.md — Complete documentation
- [ ] SPEC.md — Formal specification (if applicable)
- [ ] package.json — Correct metadata
- [ ] tsconfig.json — Strict mode enabled
- [ ] .github/workflows/test.yml — CI/CD configured
- [ ] LICENSE — MIT license
- [ ] .gitignore — Excludes node_modules, dist, etc.
- [ ] npm test — All pass
- [ ] npx tsc --noEmit — No TypeScript errors
- [ ] Git history — Clean, logical commits
