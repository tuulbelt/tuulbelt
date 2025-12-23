# Claude Code Workflow for Tuulbelt

This guide explains how to use Claude Code (https://claude.ai/code) effectively for Tuulbelt development.

## Setup

### Initial Setup (First Time)

1. Go to https://claude.ai/code
2. Click "Clone Repository"
3. Paste: `https://github.com/tuulbelt/tuulbelt.git`
4. Authenticate with GitHub when prompted
5. Select folder location: `~/code/tuulbelt` (or your preference)
6. Claude Code clones the meta repo

### For Each New Tool

1. In Claude Code: "New Project"
2. Clone: `https://github.com/tuulbelt/<tool-name>.git`
3. Or: File → Open → select tool directory

## Workflow

### Phase 1: Scaffolding (Start Here)

When creating a new tool, Claude Code should:

```bash
# Copy template
cp -r ../tuulbelt/templates/tool-repo-template/* .

# Structure becomes:
src/index.ts          # Main implementation (empty)
test/index.test.ts    # Test file (empty)
examples/basic.ts     # Example (empty)
package.json          # Pre-configured
tsconfig.json         # Pre-configured
README.md             # Template with placeholders
SPEC.md               # Optional, for protocols
```

Typical Claude Code prompt:
```
Copy the template structure from ../tuulbelt/templates/tool-repo-template/ into this repo.
Update package.json name to @tuulbelt/<tool-name>.
Leave src/index.ts and test/index.test.ts empty for now.
Create directory structure.
```

### Phase 2: Specification (Before Coding)

Before implementing, write SPEC.md (if applicable):

```markdown
# [Tool Name] Specification

## Problem
[1-2 sentence description]

## Interface
[How users interact: CLI flags, file format, input/output examples]

## Behavior
[Precise description of what happens in each case]

## Wire Format (if applicable)
[Exact bytes/JSON structure]

## Examples
[Real usage examples]

## Error Cases
[What happens on invalid input]
```

Typical Claude Code prompt:
```
Write SPEC.md for [tool name] based on these requirements:
- Problem: [description]
- Input: [format]
- Output: [format]
- Examples: [2-3 examples]

Use the specification template from docs/tool-template.md.
```

### Phase 3: Core Implementation

Implement in `src/index.ts`:

```typescript
// src/index.ts

// Exports the main API
export interface Config {
  // ...
}

export function process(input: string, config: Config): string {
  // Implementation
}

// CLI entry point (if this is a CLI tool)
if (import.meta.main) {
  // CLI logic
}
```

Typical Claude Code prompt:
```
Implement src/index.ts for [tool name]:
- Input: [description]
- Output: [description]
- Key logic: [description]
- Reference SPEC.md for exact behavior
- Use TypeScript strict mode
- Add JSDoc comments for all exports
```

### Phase 4: Testing

Write tests in `test/index.test.ts`:

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { process } from '../src/index.ts';

test('basic functionality', () => {
  const result = process('input', { /* config */ });
  assert.strictEqual(result, 'expected');
});

test('edge case: empty input', () => {
  const result = process('', { /* config */ });
  assert(result !== null);
});

// More tests...
```

Run tests in Claude Code terminal:
```bash
npm test
```

Typical Claude Code prompt:
```
Write comprehensive tests for src/index.ts in test/index.test.ts:
- Test 1: Basic functionality with example input
- Test 2: Edge case — empty input
- Test 3: Edge case — malformed input
- Test 4: [specific behavior]
- Test 5: [error case]

Use Node's native test runner (node:test).
All tests should pass after implementation.
```

### Phase 5: Examples

Create runnable example in `examples/basic.ts`:

```typescript
import { process } from '../src/index.ts';

// Example usage
const result = process('example input', { /* config */ });
console.log(result);
```

Run in Claude Code:
```bash
npx ts-node examples/basic.ts
```

### Phase 6: Documentation

Update README.md with:
- What problem it solves
- Why it exists (existing solutions and their gaps)
- Installation (clone the repo)
- Usage (CLI flags or API)
- Examples (copy from examples/)
- Error handling (exit codes, error messages)

Typical Claude Code prompt:
```
Update README.md with complete usage documentation:
- Problem statement: [one sentence]
- Why it exists: [why existing tools aren't sufficient]
- Installation: [git clone instructions]
- Usage: [CLI example or API example]
- Examples: [2-3 realistic use cases]
- Error handling: [exit codes and messages]
```

## Claude Code Tips

### Running Commands

In Claude Code terminal:
```bash
npm test                    # Run tests
npm run build              # Compile TypeScript
npx ts-node src/index.ts   # Run directly
node src/index.js          # Run compiled JS
```

### File Navigation

- `Ctrl+P` (Cmd+P) — Quick file open
- `Ctrl+Shift+P` (Cmd+Shift+P) — Command palette
- `Ctrl+`` (Cmd+`) — Toggle terminal

### Debugging

In Claude Code:
1. Add `debugger;` statement where you want to pause
2. Run: `node --inspect src/index.ts`
3. Open `chrome://inspect` in browser
4. Click "Inspect" next to the process

## Best Practices

### Keep It Simple

```typescript
// Good: Clear, simple logic
export function parse(input: string): Data {
  const lines = input.split('\n');
  return lines.map(line => JSON.parse(line));
}

// Bad: Over-engineered
export class Parser {
  private cache = new Map();
  async parse(input: string, config?: ParserConfig): Promise<Data> {
    // 50 lines of logic
  }
}
```

### Test First, Code Second

Write test cases before implementation:
```typescript
test('should detect flaky test', () => {
  // Define expected behavior
});

// Then implement to pass test
```

### Use TypeScript Strict Mode

In tsconfig.json:
```json
{
  "compilerOptions": {
    "strict": true  // Catches most bugs at compile time
  }
}
```

### Avoid Console.log for Logic

```typescript
// For debugging only:
console.error('[DEBUG] Value:', value);

// For output, use structured returns:
export function process(input: string): { success: boolean; data?: string } {
  // ...
}
```

### Commit Often

In Claude Code terminal:
```bash
git add .
git commit -m "Add core parsing logic"
git push origin main
```

## Troubleshooting

### "Module not found" error

```bash
npm install  # Should be empty for tuulbelt tools
npm test     # Should pass without install
```

If npm install is needed, you've added an external dependency (don't).

### Tests failing

1. Read error message carefully
2. Add `console.log()` to debug
3. Check SPEC.md — are you matching spec?
4. Ask Claude Code: "Why is this test failing?"

### TypeScript compilation errors

Run: `npx tsc --noEmit` to see all errors
Ask Claude Code: "Fix TypeScript errors"

## Final Checklist Before Release

- [ ] `npm test` passes
- [ ] `npx tsc --noEmit` has no errors
- [ ] Examples run without errors
- [ ] README is complete
- [ ] SPEC.md (if applicable) matches implementation
- [ ] No external dependencies in package.json
- [ ] GitHub Actions test.yml configured
- [ ] Git history is clean (logical commits)
- [ ] Version bumped in package.json
- [ ] Git tag created: `git tag v0.1.0`
