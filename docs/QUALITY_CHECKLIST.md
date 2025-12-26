# Quality Assurance Checklist

**MANDATORY: Run before every commit and after every implementation.**

This evolving checklist prevents common issues from reaching CI. Add new checks as pitfalls are discovered.

---

## Pre-Commit Checklist

Run these checks **before committing any code**:

### Universal Checks (All Languages)

- [ ] **Branch synced with main**: Run `git fetch origin main && git rev-list --count HEAD..origin/main` (should be 0)
  - If behind, run: `git pull --rebase origin main`
- [ ] **Build succeeds**: `npm run build` or `cargo build`
- [ ] **All tests pass**: `npm test` or `cargo test`
- [ ] **No lint errors**: Language-specific linters pass
- [ ] **Git status clean**: No untracked files or uncommitted changes
- [ ] **Zero runtime dependencies**: Verify `dependencies` section is empty
- [ ] **Temporary files cleaned**: No test artifacts, tmp files, or fixtures left behind

### TypeScript/Node.js Specific

- [ ] **Type check passes**: `npx tsc --noEmit` (no errors)
- [ ] **@types packages installed**: Check devDependencies for needed `@types/*`
- [ ] **ES module imports correct**: Use `import`, not `require()`; use `node:` prefix for built-ins
- [ ] **tsconfig.json valid**: Proper `lib`, `target`, `module` settings
- [ ] **No `any` types**: All types explicitly defined (strict mode)
- [ ] **Import paths correct**: Relative paths use `.js` extension for ES modules

### Rust Specific

- [ ] **Cargo check passes**: `cargo check` (no errors)
- [ ] **Clippy clean**: `cargo clippy -- -D warnings` (no warnings)
- [ ] **Rustfmt applied**: `cargo fmt` (code formatted)
- [ ] **No `unwrap()` in production**: Use `?` operator or proper error handling
- [ ] **Cargo.toml valid**: Proper edition, dependencies empty

### Documentation (VitePress) Specific

- [ ] **Docs build succeeds**: `npm run docs:build` (no errors)
- [ ] **Base URL correct**: Match GitHub Pages deployment path
- [ ] **All referenced pages exist**: No dead links in sidebar/navigation
- [ ] **Icons theme-compatible**: SVG icons work in both light and dark modes
- [ ] **Examples tested**: Code snippets actually work

---

## Post-Implementation Checklist

After completing a feature or fix:

### Functionality Verification

- [ ] **Feature works as expected**: Manual testing confirms behavior
- [ ] **Edge cases covered**: Empty input, invalid input, boundary conditions tested
- [ ] **Error handling tested**: Failures produce clear, helpful error messages
- [ ] **Performance acceptable**: No obvious performance regressions

### Code Quality

- [ ] **Tests added**: New code has corresponding test coverage (≥80%)
- [ ] **Documentation updated**: README, comments, or docs reflect changes
- [ ] **No debug code**: No `console.log`, `println!`, `debugger`, or commented code
- [ ] **Code is simple**: No over-engineering, abstractions justified by actual need
- [ ] **Naming is clear**: Variables, functions, types have descriptive names

### Tuulbelt-Specific

- [ ] **Follows principles**: Adheres to @PRINCIPLES.md (single problem, zero deps, composable)
- [ ] **No frameworks**: Tool is a utility, not a framework
- [ ] **Portable interface**: CLI, files, env vars, or sockets—not proprietary APIs
- [ ] **Works standalone**: Can be cloned and used independently
- [ ] **Dogfooding opportunities considered**: Can this tool use or validate other Tuulbelt tools?
- [ ] **Dogfooding documented**: If tool uses other tools, document in README and VitePress docs

---

## New Tool Completion Checklist

**CRITICAL: When adding a new tool to Tuulbelt, ALL of these must be completed before marking work as done.**

Use TodoWrite to track these items. Do NOT mark the tool as complete until every item is checked.

### Tool Implementation

- [ ] **Tool code complete**: All functionality implemented and tested
- [ ] **Tests passing**: 80%+ coverage, all tests green
- [ ] **README complete**: Installation, usage, API docs, examples
- [ ] **Zero dependencies**: `dependencies` object empty in package.json or Cargo.toml
- [ ] **Runs /quality-check**: Tool-level quality check passes

### Dogfooding Strategy

**CRITICAL: Only implement compositions that provide REAL value, not checkboxes.**

- [ ] **DOGFOODING_STRATEGY.md created**: Document high-value compositions (2-4 maximum)
- [ ] **High-value scripts implemented**: Focus on meaningful compositions:
  - **Test Flakiness Detector** (almost always valuable) - Validates test determinism
  - **Output Diffing Utility** (if outputs matter) - Proves consistent behavior
  - **Tool-specific compositions** (domain-relevant only) - Real utility, not forced
- [ ] **README dogfooding section**: Documents all composition scripts with examples
- [ ] **GH Pages dogfooding section**: Mirrors README documentation
- [ ] **Graceful fallback tested**: Tool works standalone when dependencies unavailable
- [ ] **Scripts executable**: All `scripts/dogfood-*.sh` have execute permissions

**Guidelines:**
- Don't dogfood just to dogfood - focus on REAL utility
- Typical high-value: dogfood-flaky.sh (test validation), dogfood-diff.sh (output consistency)
- Optional: Tool-specific compositions (only if genuinely useful)
- See existing tools for patterns: Test Flakiness Detector, CLI Progress, Path Normalizer, Semaphore, Output Diffing

### GitHub Pages Documentation

**This is where we failed - don't skip these:**

- [ ] **VitePress config updated**: Added to `docs/.vitepress/config.ts`
  - Added to `/tools/` sidebar items list
  - Created dedicated sidebar section for tool (e.g., `/tools/{tool-name}/`)
- [ ] **Docs directory created**: `docs/tools/{tool-name}/` exists
- [ ] **Docs pages copied**: At minimum: index.md, getting-started.md, cli-usage.md, library-usage.md, examples.md, api-reference.md
- [ ] **Internal links fixed**: All links use `/tools/{tool-name}/` paths, not `/guide/` or `/api/`
- [ ] **Placeholder demo.gif created**: Create `docs/public/{tool-name}/demo.gif` (1x1 transparent GIF)
  - VitePress validates image imports at build time - missing images FAIL the build
  - Run: `mkdir -p docs/public/{tool-name} && echo "R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" | base64 -d > docs/public/{tool-name}/demo.gif`
  - The create-demos workflow will replace this with the real demo after merge
- [ ] **Docs build succeeds**: Run `npm run docs:build` from root - must pass with no dead links
- [ ] **Tools index updated**: `docs/tools/index.md` shows correct count (e.g., "3/33") and includes new tool card

### GitHub Workflows

- [ ] **Demo recording script created**: `scripts/record-{tool-name}-demo.sh` exists with `--title` flag
- [ ] **Demo workflow path filter added**: Add tool path to `.github/workflows/create-demos.yml` under `paths:` section
  - Example: `- '{tool-name}/**'`
  - This enables smart detection to only record demo when tool changes
- [ ] **Demo script tested locally**: Run recording script and verify `demo.cast` and `docs/demo.gif` exist

> **Note:** `deploy-docs.yml` auto-discovers tool directories via `*/docs/**` and `*/README.md` patterns (no manual update).
> **Important:** `create-demos.yml` requires manual path filter addition for efficient demo recording.

- [ ] **Test workflow updated** (if needed): Any tool-specific CI configuration added

### Root Repository Updates

- [ ] **Root README updated**: Tool added to appropriate category with status badge
- [ ] **HANDOFF.md updated**: Current session reflects completed work
- [ ] **NEXT_TASKS.md updated**: Tool moved from "Coming Soon" to "Completed"

### Final Verification

- [ ] **All tests pass**: Run `/test-all` from root
- [ ] **TypeScript compiles**: Run `npx tsc --noEmit` in all TS tools
- [ ] **Docs deploy**: Verify `npm run docs:build` succeeds
- [ ] **Visual check**: Preview docs with `npm run docs:preview` and navigate to new tool
- [ ] **Git status clean**: No uncommitted changes
- [ ] **All TodoWrite items completed**: Don't commit until every todo is done

**Example TodoWrite for New Tool:**

```
1. [x] Implement tool functionality
2. [x] Write tests (80%+ coverage)
3. [x] Create tool README
4. [ ] Create demo recording script (scripts/record-{tool-name}-demo.sh)
5. [ ] Add path filter to create-demos.yml workflow
6. [ ] Add to VitePress config (docs/.vitepress/config.ts)
7. [ ] Create docs/tools/{tool-name}/ directory
8. [ ] Copy docs pages and fix links
9. [ ] Create placeholder demo.gif in docs/public/{tool-name}/
10. [ ] Update docs/tools/index.md
11. [ ] Update root README.md
12. [ ] Run npm run docs:build (verify - MUST PASS)
13. [ ] Run /quality-check
14. [ ] Update HANDOFF.md and NEXT_TASKS.md
```

---

## Common Pitfalls Database

### TypeScript/Node.js Issues

#### Missing `@types/node` (2025-12-23)

**Symptom:**
```
error TS2307: Cannot find module 'child_process'
error TS2584: Cannot find name 'console'
error TS7017: Element implicitly has an 'any' type for globalThis
error TS2339: Property 'url' does not exist on ImportMeta
```

**Root Cause:**
TypeScript can't resolve Node.js built-in types without `@types/node` package.

**Prevention:**
```bash
# Always include in devDependencies
npm install --save-dev @types/node@20
```

**Verification:**
```bash
npx tsc --noEmit  # Must pass with no errors
npm run build     # Must succeed
```

---

#### ES Module Import Errors

**Symptom:**
```
ReferenceError: require is not defined in ES module scope
```

**Root Cause:**
Using CommonJS `require()` in ES module context.

**Prevention:**
```typescript
// Correct (ES modules)
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

// Wrong (CommonJS in ES module)
const { readFileSync } = require('fs');
```

**Verification:**
- Check `package.json` has `"type": "module"`
- All imports use `import` syntax
- Built-in modules use `node:` prefix

---

#### Test Glob Patterns Not Expanding

**Symptom:**
Tests don't run in CI; glob pattern `'test/*.test.ts'` not recognized.

**Root Cause:**
Shell doesn't expand globs in quoted strings.

**Prevention:**
```json
// Correct - explicit file list
"test": "node --import tsx --test test/index.test.ts test/integration.test.ts"

// Wrong - glob in quotes
"test": "node --import tsx --test 'test/*.test.ts'"
```

---

### VitePress/Documentation Issues

#### Incorrect Base URL Configuration (2025-12-23)

**Symptom:**
All documentation links return 404 errors; site loads but navigation fails.

**Root Cause:**
VitePress `base` config doesn't match GitHub Pages deployment path.

**Prevention:**
```typescript
// docs/.vitepress/config.ts
export default defineConfig({
  // Correct - matches deployment URL
  base: '/tuulbelt/',  // For https://tuulbelt.github.io/tuulbelt/

  // Wrong - mismatched path
  base: '/test-flakiness-detector/',  // Doesn't match actual URL
})
```

**Verification:**
```bash
npm run docs:build  # Check build output paths
# Verify URLs start with correct base path
```

---

#### Missing Referenced Documentation Pages (2025-12-23)

**Symptom:**
VitePress build fails with dead link errors.

**Root Cause:**
Sidebar references pages that don't exist yet.

**Prevention:**
```typescript
// Create ALL pages before adding to sidebar
sidebar: {
  '/guide/': [
    { text: 'Getting Started', link: '/guide/getting-started' }  // Must exist!
  ]
}
```

**Verification:**
```bash
# Before committing sidebar changes, verify all files exist
ls docs/guide/getting-started.md
ls docs/guide/installation.md
# etc.

npm run docs:build  # Must succeed with no dead link errors
```

---

#### Icons Not Working in Dark Mode (2025-12-23)

**Symptom:**
SVG icons invisible or low contrast in dark theme.

**Root Cause:**
Icons use `currentColor` but VitePress dark mode needs explicit styling.

**Prevention:**
```css
/* docs/.vitepress/theme/custom.css */
.dark .vp-feature-icon svg {
  stroke: rgba(255, 255, 255, 0.9);
}

.vp-feature-icon svg {
  stroke: currentColor;
}
```

```typescript
// docs/.vitepress/theme/index.ts
import DefaultTheme from 'vitepress/theme'
import './custom.css'

export default DefaultTheme
```

**Verification:**
- Build docs: `npm run docs:build`
- Preview: `npm run docs:preview`
- Toggle dark mode and verify icon visibility

---

#### Broken Logo/Asset References (2025-12-23)

**Symptom:**
Question mark or broken image icon showing instead of logo.

**Root Cause:**
Referenced file doesn't exist in `/public` directory.

**Prevention:**
```markdown
<!-- Correct - file exists in docs/public/ -->
![Logo](/logo.svg)

<!-- Wrong - file doesn't exist -->
![Logo](/missing-file.svg)

<!-- Better - use icon objects for features -->
features:
  - icon:
      src: /icons/target.svg
    title: Feature
```

**Verification:**
```bash
# Verify all referenced assets exist
ls docs/public/logo.svg
ls docs/public/icons/*.svg

npm run docs:build  # Check for warnings about missing files
```

---

#### Missing Demo GIF in docs/public/ (2025-12-26)

**Symptom:**
```
[vite]: Rollup failed to resolve import "/{tool-name}/demo.gif" from '...docs/tools/{tool-name}/index.md'.
```

**Root Cause:**
VitePress validates image imports at build time. When index.md references `![Demo](/{tool-name}/demo.gif)`, the file must exist in `docs/public/{tool-name}/demo.gif` BEFORE the build runs.

This is a chicken-and-egg problem: the demo.gif is generated by the create-demos workflow, but the docs build runs first.

**Prevention:**
```bash
# Create placeholder demo.gif (42-byte transparent 1x1 GIF)
mkdir -p docs/public/{tool-name}
echo "R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" | base64 -d > docs/public/{tool-name}/demo.gif

# The create-demos workflow will replace this with real demo after merge
```

**Verification:**
```bash
# ALWAYS run before committing new tool documentation
npm run docs:build

# If this fails with "failed to resolve import", create the placeholder
```

**CRITICAL:** This check MUST be part of quality-check for any new tool. The docs build failing in CI after merge is a serious issue.

---

### Rust Issues

#### Cargo.lock Conflicts

**Symptom:**
Merge conflicts in `Cargo.lock` during git operations.

**Root Cause:**
Manually editing `Cargo.lock` instead of using cargo commands.

**Prevention:**
```bash
# Correct
cargo add package-name
cargo update

# Wrong
vim Cargo.lock  # Never manually edit
```

---

### General Issues

#### Probabilistic Tests

**Symptom:**
Tests pass sometimes, fail sometimes (flaky tests).

**Root Cause:**
Using `Math.random()`, `process.hrtime()`, or non-deterministic patterns.

**Prevention:**
```typescript
// Deterministic counter pattern
const counterFile = `/tmp/counter-${Date.now()}.txt`;
writeFileSync(counterFile, '0');
// Use counter % 2 for predictable pass/fail

// Probabilistic
if (Math.random() < 0.5) fail();  // Non-deterministic!
```

**Principle:**
Tests must produce identical results on every run.

---

#### File-Based State Conflicts

**Symptom:**
Tests expecting exact counts get different values; state persists across runs.

**Root Cause:**
Multiple tests sharing same counter/state filenames.

**Prevention:**
```typescript
// Unique filenames with timestamp
const counterFile = join(tmpDir, `counter-${Date.now()}-${testId}.txt`);

// Shared filename
const counterFile = join(tmpDir, 'counter.txt');  // Collision!
```

---

#### Merge Conflicts After CI Updates (2025-12-26)

**Symptom:**
After merging a PR, CI runs update files (like demo.gif). Your next commit from a feature branch creates merge conflicts.

**Root Cause:**
CI workflows commit changes to main (demo recordings, documentation updates). Feature branches diverge from main.

**Prevention:**
```bash
# Before making changes, always sync with main
git fetch origin main
BEHIND=$(git rev-list --count HEAD..origin/main)
if [ "$BEHIND" != "0" ]; then
  echo "Behind by $BEHIND commits - rebasing..."
  git pull --rebase origin main
fi
```

**Verification:**
- `/quality-check` now automatically checks if branch is behind main
- Run `git rev-list --count HEAD..origin/main` - should return 0

---

### Best Practices

#### Dogfooding Pattern: Dynamic Imports with Graceful Fallback (2025-12-24)

**Pattern:**
Tools can use other Tuulbelt tools while maintaining standalone independence.

**Use Case:**
test-flakiness-detector integrates cli-progress-reporting for progress tracking, but works fine without it.

**Implementation:**
```typescript
import { existsSync } from 'node:fs';
import { join } from 'node:path';

/**
 * Try to load optional Tuulbelt tool (monorepo context)
 * Returns null if not available (standalone context)
 */
async function loadOptionalTool(): Promise<any | null> {
  try {
    // Check if sibling directory exists
    const toolPath = join(process.cwd(), '..', 'tool-name', 'src', 'index.ts');
    if (!existsSync(toolPath)) {
      return null; // Not in monorepo, that's fine
    }

    // Dynamic import
    const module = await import(`file://${toolPath}`);
    return module;
  } catch {
    return null; // Import failed, that's fine - tool is optional
  }
}

// Use optional tool with graceful fallback
export async function myFunction() {
  const optionalTool = await loadOptionalTool();

  if (optionalTool) {
    // Enhanced functionality when available
    optionalTool.doSomething();
  }

  // Core functionality works regardless
  return coreLogic();
}
```

**Checklist:**
- [ ] Function is `async` if using dynamic imports
- [ ] Check file exists before importing (`existsSync`)
- [ ] Wrap import in try-catch
- [ ] Return `null` on failure (graceful fallback)
- [ ] Core functionality works without optional tool
- [ ] Document in README that tool uses other Tuulbelt tools when in monorepo
- [ ] Add `test:dogfood` script if tool validates other tools

**Benefits:**
- **Monorepo**: Tools enhance each other
- **Standalone**: Tools work independently
- **Zero Impact**: No runtime dependencies, no breaking changes

---

#### Dogfooding Pattern: CLI-Based Cross-Language Validation (2025-12-25)

**Pattern:**
Tools can validate each other across languages (TypeScript ↔ Rust) via CLI.

**Use Case:**
- TypeScript test-flakiness-detector validates Rust file-based-semaphore tests
- Rust CLI tools can be called from TypeScript tools

**Implementation (TypeScript → Rust via Shell Script):**

For Rust tools, add `scripts/dogfood.sh`:

```bash
#!/bin/bash
# Validate Rust tests using TypeScript test-flakiness-detector

RUNS="${1:-10}"
DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"

# Exit gracefully if not in monorepo
if [ ! -d "$DETECTOR_DIR" ]; then
    echo "Not in monorepo context, skipping dogfooding"
    exit 0
fi

cd "$DETECTOR_DIR"
npx tsx src/index.ts \
    --test "cd '$TOOL_DIR' && cargo test 2>&1" \
    --runs "$RUNS"
```

**Implementation (TypeScript → Rust CLI):**

For TypeScript tools needing Rust CLI:

```typescript
import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

function useRustTool(args: string): string | null {
  const binaryPath = join(process.cwd(), '..', 'rust-tool', 'target', 'release', 'rust-tool');

  if (!existsSync(binaryPath)) {
    return null; // Not available, graceful fallback
  }

  try {
    return execSync(`${binaryPath} ${args}`, { encoding: 'utf-8' });
  } catch {
    return null;
  }
}
```

**Implementation (Rust → TypeScript via Shell):**

For Rust tools needing TypeScript functionality:

```rust
use std::process::Command;

fn use_typescript_tool(args: &[&str]) -> Option<String> {
    let output = Command::new("npx")
        .args(["tsx", "../ts-tool/src/index.ts"])
        .args(args)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None // Graceful fallback
    }
}
```

**Checklist:**
- [ ] Exit gracefully when not in monorepo (exit 0, return null)
- [ ] Check binary/script exists before executing
- [ ] Wrap execution in try-catch or error handling
- [ ] Document in README that dogfooding requires monorepo context
- [ ] Add `scripts/dogfood.sh` for Rust tools
- [ ] Add `test/flakiness-detection.test.ts` for TypeScript tools

**Template Files:**
- Rust: `templates/rust-tool-template/scripts/dogfood.sh`
- TypeScript: `templates/tool-repo-template/test/flakiness-detection.test.ts`

---

## Adding New Checks

When you discover a new issue, add it here using this template:

### [Issue Name] (YYYY-MM-DD)

**Symptom:**
```
[Error message or behavior]
```

**Root Cause:**
[Brief explanation of why this happens]

**Prevention:**
```bash
[Code or command to prevent this issue]
```

**Verification:**
```bash
[How to verify the fix works]
```

---

## Automation Script

Run this to execute all checks:

```bash
#!/bin/bash
# Save as: scripts/quality-check.sh

echo "Running quality checks..."

# Detect language
if [ -f "package.json" ]; then
  echo "→ TypeScript/Node.js detected"

  echo "  ✓ Checking TypeScript compilation..."
  npx tsc --noEmit || exit 1

  echo "  ✓ Running build..."
  npm run build || exit 1

  echo "  ✓ Running tests..."
  npm test || exit 1

  # Check for VitePress docs
  if [ -d "docs" ] && grep -q '"docs:build"' package.json; then
    echo "  ✓ Building documentation..."
    npm run docs:build || exit 1
  fi

  echo "  ✓ Verifying zero dependencies..."
  if grep -q '"dependencies".*{.*[^}]' package.json; then
    echo "  ✗ ERROR: Runtime dependencies found!"
    exit 1
  fi

elif [ -f "Cargo.toml" ]; then
  echo "→ Rust detected"

  echo "  ✓ Running cargo check..."
  cargo check || exit 1

  echo "  ✓ Running clippy..."
  cargo clippy -- -D warnings || exit 1

  echo "  ✓ Running tests..."
  cargo test || exit 1

  echo "  ✓ Verifying zero dependencies..."
  if grep -A5 '^\[dependencies\]' Cargo.toml | grep -q '^[a-z]'; then
    echo "  ✗ ERROR: Runtime dependencies found!"
    exit 1
  fi
fi

echo "  ✓ Checking git status..."
if [ -n "$(git status --porcelain)" ]; then
  echo "  ⚠ WARNING: Uncommitted changes or untracked files"
  git status --short
fi

echo ""
echo "All quality checks passed!"
```

**Usage:**
```bash
chmod +x scripts/quality-check.sh
./scripts/quality-check.sh
```

---

## Tool-Specific Usage

This checklist is maintained at the root level and applies to all Tuulbelt tools. When working on a specific tool:

**For TypeScript tools:**
- Use Universal Checks + TypeScript Specific sections
- If tool has docs, use Documentation section

**For Rust tools:**
- Use Universal Checks + Rust Specific sections

**Adding tool-specific checks:**
If a tool needs unique checks, add them to this file under a new subsection (e.g., "### Test Flakiness Detector Specific") rather than creating per-tool checklists.

---

## References

- @PRINCIPLES.md - Zero dependencies, single problem, composable
- @CONTRIBUTING.md - Contribution workflow and standards
- @docs/testing-standards.md - Test coverage and patterns
- @docs/security-guidelines.md - Security checklist

---

**Last Updated:** 2025-12-26
**Next Review:** Add new pitfalls as discovered during development
