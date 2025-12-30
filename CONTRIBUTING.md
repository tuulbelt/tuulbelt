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

### Step 2: Create Tool Repository

**Recommended: Use `/new-tool` command**

In Claude Code, run:
```bash
/new-tool <tool-name> <typescript|rust>
```

This command automatically:
1. Creates standalone GitHub repository
2. Scaffolds from appropriate template (TypeScript or Rust)
3. Configures CI/CD, metadata, and documentation
4. Tags v0.1.0 and pushes to GitHub
5. Adds as git submodule to meta repo

See [@.claude/commands/new-tool.md](.claude/commands/new-tool.md) for complete documentation.

**Alternative: Manual Setup** (not recommended)

If you need manual control:
1. Create GitHub repo: `gh repo create tuulbelt/<tool-name> --public`
2. Clone locally: `git clone https://github.com/tuulbelt/<tool-name>.git`
3. Copy template files:
   - TypeScript: `cp -r templates/tool-repo-template/* <tool-name>/`
   - Rust: `cp -r templates/rust-tool-template/* <tool-name>/`
4. Customize package.json/Cargo.toml (name, description, repository URLs)
5. Add as submodule to meta repo: `git submodule add https://github.com/tuulbelt/<tool-name>.git tools/<tool-name>`

### Step 3: Implement Core Functionality

Open the tool repository in Claude Code and implement:

**Implementation checklist:**
- [ ] Core logic in `src/index.ts` (or `src/lib.rs`)
- [ ] At least 3-5 test cases in `test/index.test.ts`
- [ ] Example usage in `examples/`
- [ ] README with clear usage instructions
- [ ] Run `/quality-check` before committing
- [ ] All tests pass: `npm test` (or `cargo test`)

### Step 4: Testing Standards

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

See [@docs/testing-standards.md](docs/testing-standards.md) for complete requirements.

**CI/CD Automatic Testing:**

Each standalone tool repository has its own CI/CD workflow (`.github/workflows/test.yml`) that runs on every push and pull request.

- **TypeScript tools:** `npm ci`, `npm test`, `npm run build`
- **Rust tools:** `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `cargo build --release`

### Step 5: Documentation

**README.md** — One page, includes:
- 1-sentence description
- Why it exists (what problem it solves)
- Installation (clone + `npm install` or `cargo build`)
- Usage examples (CLI and library)
- Exit codes / error handling (if applicable)

**CLAUDE.md** — Tool-specific development context for Claude Code

**SPEC.md** (optional) — Formal wire format, algorithm, or data structure definition

**ARCHITECTURE.md** (optional) — Design decisions, key functions, interaction flow

### Step 6: Code Quality

Run `/quality-check` before every commit. This verifies:
- Build succeeds
- Tests pass (100%)
- Zero runtime dependencies
- TypeScript compilation (for TS tools)
- Clippy warnings (for Rust tools)

**TypeScript:**
- `strict: true` in tsconfig.json
- No `any` types (use `unknown` and type guards)
- Explicit return types on all exported functions
- ES modules only (`import`, not `require()`)

**Rust:**
- Use `?` operator for error propagation (avoid `unwrap()`)
- Run `cargo fmt` and `cargo clippy -- -D warnings`
- All public items documented with `///`

See [@.claude/rules/code-style.md](.claude/rules/code-style.md) for detailed examples.

### Step 7: Versioning

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

### Step 8: Update Meta Repo

Once your tool is ready for initial release:

1. Update meta `README.md`:
   - Change `(TBD)` to `🟢 v0.1.0`
   - Add tool to appropriate category
   - Include link to standalone repository

Example:
```markdown
- **[Test Flakiness Detector](https://github.com/tuulbelt/test-flakiness-detector)** — Identify unreliable tests 🟢 v0.1.0 | [📖 Docs](https://github.com/tuulbelt/test-flakiness-detector#readme) | [🚀 Examples](https://github.com/tuulbelt/test-flakiness-detector/tree/main/examples/)
```

2. Ensure tool is added as git submodule in `tools/` (done automatically by `/new-tool`)

### Step 9: Maintenance

- Respond to issues within a week
- Keep README updated with new features
- Run `/quality-check` before every merge
- Consider backwards compatibility; avoid breaking changes in patch versions
- Update git submodule reference in meta repo when releasing new versions

---

## Working with Git Submodules

Tools are referenced as git submodules in the meta repository:

```bash
# Clone meta repo with all tools
git clone --recursive https://github.com/tuulbelt/tuulbelt.git

# Initialize submodules after clone
git submodule update --init --recursive

# Update all submodules to latest
git submodule update --remote

# Update specific tool
cd tools/test-flakiness-detector
git pull origin main
cd ../..
git add tools/test-flakiness-detector
git commit -m "chore: update test-flakiness-detector submodule"
```

See [@ARCHITECTURE.md](ARCHITECTURE.md) for complete submodule documentation.

---

## Pull Request Process

1. Create feature branch: `git checkout -b feature/description`
2. Run `/quality-check` to verify all checks pass
3. Commit with semantic messages: `/git-commit <type> <scope> <message>`
4. Push: `git push origin feature/description`
5. Open PR on GitHub with:
   - What changed
   - Why it changed
   - Tests added
   - Any breaking changes

---

## Code Review Checklist

Reviewers check:
- [ ] Solves the stated problem
- [ ] No new runtime dependencies
- [ ] Tests cover core logic + edge cases (80%+ coverage)
- [ ] README is clear and complete
- [ ] No console logs or debug code
- [ ] Follows the style of the existing codebase
- [ ] `/quality-check` passes

See [@docs/QUALITY_CHECKLIST.md](docs/QUALITY_CHECKLIST.md) for complete checklist.

---

## Issues & Feedback

**For tool-specific issues:** Open issues in the tool's standalone repository

**For meta repo issues:** Open issues at https://github.com/tuulbelt/tuulbelt/issues

- Bug reports: Include steps to reproduce
- Feature requests: Explain the use case
- Questions: Check README and existing issues first

---

## Questions?

- Check existing issues first
- Read [@ARCHITECTURE.md](ARCHITECTURE.md) for repository structure
- Read [@PRINCIPLES.md](PRINCIPLES.md) for design philosophy
- Ask in meta repo discussions if unsure

---

## Useful Commands

```bash
# Quality checks (MANDATORY before commit)
/quality-check

# Create new tool
/new-tool <tool-name> <typescript|rust>

# Run all tests
/test-all

# Security analysis
/security-scan

# Semantic commit
/git-commit <type> <scope> <message>
```

See [@.claude/commands/](.claude/commands/) for complete command documentation.
