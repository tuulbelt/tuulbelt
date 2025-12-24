# Contributing to Tuulbelt

Thanks for contributing! This guide explains how to create and maintain tools.

## Before You Start

Read [PRINCIPLES.md](PRINCIPLES.md). If your tool doesn't fit those principles, it doesn't belong in Tuulbelt.

## Creating a New Tool

### Step 1: Proposal

Open an issue on the meta repo (tuulbelt/tuulbelt):
- Title: "Proposal: [Tool Name]"
- Include:
  - Problem statement (1-2 sentences)
  - Use case(s)
  - Proposed interface (CLI flags, input/output format)
  - Implementation approach (proven?)
  - Why existing solutions don't work

Example:
```markdown
## Proposal: Test Flakiness Detector

**Problem:** Tests that pass sometimes, fail sometimes are hard to identify.

**Use case:** Catch intermittent failures in CI.

**Interface:**
$ flakiness-detector --runs 10 --test "npm test"
Output: JSON list of unreliable tests

**Why it's needed:** Jest/Vitest don't have built-in flakiness detection.
```

Get feedback before building.

### Step 2: Set Up Tool Repository

Use the template in `templates/tool-repo-template/` as your starting point.

Create a new repo under https://github.com/tuulbelt:
- Repo name: `<category>-<tool-name>` (e.g., `testing-flakiness-detector`)
- Visibility: Public
- Initialize with: README, LICENSE (MIT)

Clone locally:
```bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
```

### Step 3: Copy Template Files

For Node.js/TypeScript tools:

```bash
cp -r ../tuulbelt/templates/tool-repo-template/* .
```

For Rust tools:

```bash
cp -r ../tuulbelt/templates/rust-tool-template/* .
```

Customize:
- Update `package.json` (or `Cargo.toml`) name, description
- Update README.md with your tool's purpose
- Remove example code, keep structure

### Step 4: Implement in Claude Code

Open the tool repo in Claude Code:
1. In Claude Code, click "Open folder"
2. Clone: `https://github.com/tuulbelt/<tool-name>.git`
3. Start implementing following the template structure

Implementation checklist:
- [ ] Core logic in `src/index.ts` (or `src/lib.rs`)
- [ ] At least 3-5 test cases in `test/index.test.ts`
- [ ] Example usage in `examples/`
- [ ] README with clear usage instructions
- [ ] Run `npm test` (or `cargo test`) successfully

### Step 5: Testing Standards

All tools must have:

**Unit tests** — Core logic validation
```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';

test('core functionality', () => {
  const result = myFunction('input');
  assert.strictEqual(result, 'expected');
});
```

**Integration tests** — CLI behavior validation
```typescript
import { execSync } from 'child_process';

test('CLI works end-to-end', () => {
  const output = execSync('node src/index.js --flag value').toString();
  assert(output.includes('expected-output'));
});
```

**Edge cases** — Malformed input, empty input, boundary conditions
```typescript
test('handles empty input', () => {
  const result = myFunction('');
  assert(result !== null); // Doesn't crash
});
```

**CI/CD Automatic Testing:**

Your tool will be **automatically discovered and tested** by CI workflows - no configuration needed!

**How it works:**
- **TypeScript tools:** Discovered by finding `package.json` files (excluding root, docs, templates, .github, .claude, scripts)
- **Rust tools:** Discovered by finding `Cargo.toml` files (excluding templates)
- **Workflows:** `test-all-tools.yml` and `update-dashboard.yml` auto-discover all tools
- **When:** On push to main, pull requests, and nightly at 2 AM UTC
- **What gets tested:**
  - TypeScript: `npm ci`, `npm test`, `npm run build`
  - Rust: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `cargo build --release`

**Just create your tool directory with `package.json` or `Cargo.toml` and CI will find it!**

Example discovery output:
```
Found 2 TypeScript tools: cli-progress-reporting test-flakiness-detector
Found 0 Rust tools:
Total: 2 tools
```

### Step 6: Documentation

**README.md** — One page, includes:
- 1-sentence description
- Why it exists (what problem it solves)
- Installation (just: clone the repo)
- Usage (with examples)
- Exit codes / error handling (if applicable)

**SPEC.md** (optional, for protocols/formats) — Formal wire format, algorithm, or data structure definition.

**ARCHITECTURE.md** (optional, for complex tools) — Design decisions, key functions, interaction flow.

### Step 7: Code Quality

- TypeScript: `strict: true` in tsconfig.json
- No `any` types
- No external dependencies (runtime)
- Clear variable names
- Comments for non-obvious logic

Example:

```typescript
// Detect if a test is flaky by running it multiple times
export function detectFlakiness(
  testFn: () => Promise<void>,
  runs: number = 10
): Promise<FlakeReport> {
  // ...
}
```

### Step 8: Versioning

Start at `0.1.0`. Increment:
- `0.1.x` for bug fixes (internal only)
- `0.2.0` for minor features
- `1.0.0` when API is stable and production-ready
- `2.0.0` for breaking changes

Tag releases on GitHub:
```bash
git tag v0.1.0
git push origin v0.1.0
```

### Step 9: Update Meta Repo

Once your tool is ready for initial release:

1. Update meta `README.md` — change `(TBD)` to `(✓ v0.1.0)` and link to repo
2. Add a one-line entry to the appropriate category

Example:
```markdown
- **[Test Flakiness Detector](https://github.com/tuulbelt/test-flakiness-detector)** — Identify unreliable tests ✓ v0.1.0
```

### Step 10: Maintenance

- Respond to issues within a week
- Keep README updated with new features
- Run tests before every merge
- Consider backwards compatibility; avoid breaking changes in patch versions

---

## Pull Request Process

1. Create feature branch: `git checkout -b feature/description`
2. Commit with clear messages: `git commit -m "Add feature: description"`
3. Push: `git push origin feature/description`
4. Open PR on GitHub with:
   - What changed
   - Why it changed
   - Tests added
   - Any breaking changes

---

## Code Review Checklist

Reviewers check:
- [ ] Solves the stated problem
- [ ] No new runtime dependencies
- [ ] Tests cover core logic + edge cases
- [ ] README is clear and complete
- [ ] No console logs or debug code
- [ ] Follows the style of the existing codebase

---

## Issues & Feedback

- Bug reports: Include steps to reproduce
- Feature requests: Explain the use case
- Questions: Check README and existing issues first

---

## Questions?

Check existing issues first. If unsure, ask in the meta repo discussion.
