# Contributing to Tuulbelt

Thanks for contributing! This guide explains how to create and maintain tools.

## Before You Start

Read [/guide/principles](/guide/principles). If your tool doesn't fit those principles, it doesn't belong in Tuulbelt.

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

### Step 2: Set Up Tool Directory

**Important:** All tools are developed in the [tuulbelt/tuulbelt](https://github.com/tuulbelt/tuulbelt) monorepo, not separate repositories.

Create a new tool directory:
```bash
cd tuulbelt
mkdir <tool-name>
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

- **TypeScript tools:** Discovered by finding `package.json` files
- **Rust tools:** Discovered by finding `Cargo.toml` files
- **When:** On push to main, pull requests, and nightly at 2 AM UTC
- **What gets tested:**
  - TypeScript: `npm ci`, `npm test`, `npm run build`
  - Rust: `cargo test`, `cargo clippy`, `cargo fmt --check`, `cargo build --release`

Just create your tool directory with `package.json` or `Cargo.toml` and CI will find it!

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

### Step 9: GitHub Pages Documentation

**CRITICAL:** Add your tool to the VitePress documentation site. This is mandatory and often missed!

#### 9.1 Update VitePress Configuration

Edit `docs/.vitepress/config.ts`:

```typescript
sidebar: {
  '/tools/': [
    {
      text: 'Available Tools',
      items: [
        { text: 'Overview', link: '/tools/' },
        { text: 'Your Tool Name', link: '/tools/your-tool-name/' }  // Add here
      ]
    }
  ],
  '/tools/your-tool-name/': [  // Add entire section
    {
      text: 'Your Tool Name',
      items: [
        { text: 'Overview', link: '/tools/your-tool-name/' },
        { text: 'Getting Started', link: '/tools/your-tool-name/getting-started' },
        { text: 'CLI Usage', link: '/tools/your-tool-name/cli-usage' },
        { text: 'Library Usage', link: '/tools/your-tool-name/library-usage' },
        { text: 'Examples', link: '/tools/your-tool-name/examples' },
        { text: 'API Reference', link: '/tools/your-tool-name/api-reference' }
      ]
    }
  ]
}
```

#### 9.2 Create Documentation Directory

```bash
mkdir -p docs/tools/your-tool-name
```

#### 9.3 Create Documentation Pages

Copy and customize these pages from an existing tool (e.g., test-flakiness-detector):
- `index.md` - Overview page (use standard template, NOT home layout)
- `getting-started.md` - Installation and setup
- `cli-usage.md` - Command-line examples
- `library-usage.md` - TypeScript/Rust API examples
- `examples.md` - Real-world use cases
- `api-reference.md` - Complete API documentation

**Template Structure for `index.md`:**
```markdown
# Tool Name

One-sentence tagline.

## Overview

Paragraph description.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** TypeScript

**Repository:** [link]

## Features

### <img src="/icons/icon.svg" class="inline-icon" alt=""> Feature Name

Description.

## Quick Start

Code examples.

## Use Cases

Bullet points.

## Why Tool Name?

Explanation.

## Demo

Demo GIF and link.

## Next Steps

Documentation links.

## License

MIT License - see repository for details.
```

#### 9.4 Update Tools Index

Edit `docs/tools/index.md` to add your tool card and update the count (e.g., "4/33").

#### 9.5 Update GitHub Workflow

Edit `.github/workflows/deploy-docs.yml` to add your tool's docs to the paths trigger:

```yaml
paths:
  - 'docs/**'
  - 'your-tool-name/**/*.md'  # Add this line
```

#### 9.6 Verify Documentation Build

**MANDATORY before committing:**

```bash
cd /path/to/tuulbelt/root
npm run docs:build
```

This MUST succeed with zero dead links. Fix any broken links before proceeding.

### Step 10: Update Meta Repo

Once your tool is ready for initial release:

1. Update root `README.md` — change `(TBD)` to `(✓ v0.1.0)` and link to tool
2. Add a one-line entry to the appropriate category
3. Update `docs/index.md` - Add tool card to Available Tools section
4. Update `docs/guide/getting-started.md` - Add Quick Start example and table entry

Example:
```markdown
- **[Test Flakiness Detector](https://github.com/tuulbelt/tuulbelt/tree/main/test-flakiness-detector)** — Identify unreliable tests ✓ v0.1.0
```

### Step 11: Pre-Commit Quality Checks

**MANDATORY before every commit:**

Run the quality check command:
```bash
/quality-check
```

This validates:
- Build succeeds
- All tests pass
- TypeScript/Clippy checks pass
- Zero runtime dependencies
- **Documentation builds without errors**

See [Quality Checklist](/QUALITY_CHECKLIST) for complete requirements.

### Step 12: Use TodoWrite for Task Tracking

For any multi-step task (3+ steps), use TodoWrite to track progress:

```typescript
// Example: Creating a new tool
TodoWrite([
  { content: "Implement core functionality", status: "completed" },
  { content: "Write tests (80%+ coverage)", status: "completed" },
  { content: "Create README", status: "in_progress" },
  { content: "Add to VitePress config", status: "pending" },
  { content: "Create docs pages", status: "pending" },
  { content: "Update workflows", status: "pending" },
  { content: "Run npm run docs:build", status: "pending" }
]);
```

This prevents missing critical steps like GitHub Pages integration.

### Step 13: Consider Dogfooding

Can your tool:
- **Use other Tuulbelt tools?** (e.g., test-flakiness-detector uses cli-progress-reporting)
- **Validate other tools?** (e.g., cross-platform-path-normalizer validated by test-flakiness-detector)

Document dogfooding relationships in README and VitePress docs.

### Step 14: Maintenance

- Respond to issues within a week
- Keep README updated with new features
- Run `/quality-check` before every merge
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
