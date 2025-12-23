# Quality Assurance Checklist

**MANDATORY: Run before every commit and after every implementation.**

This evolving checklist prevents common issues from reaching CI. Add new checks as pitfalls are discovered.

---

## Pre-Commit Checklist

Run these checks **before committing any code**:

### Universal Checks (All Languages)

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
// ✅ Correct (ES modules)
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

// ❌ Wrong (CommonJS in ES module)
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
// ✅ Correct - explicit file list
"test": "node --import tsx --test test/index.test.ts test/integration.test.ts"

// ❌ Wrong - glob in quotes
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
  // ✅ Correct - matches deployment URL
  base: '/tuulbelt/',  // For https://tuulbelt.github.io/tuulbelt/

  // ❌ Wrong - mismatched path
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
// ✅ Create ALL pages before adding to sidebar
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
<!-- ✅ Correct - file exists in docs/public/ -->
![Logo](/logo.svg)

<!-- ❌ Wrong - file doesn't exist -->
![Logo](/missing-file.svg)

<!-- ✅ Better - use icon objects for features -->
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

### Rust Issues

#### Cargo.lock Conflicts

**Symptom:**
Merge conflicts in `Cargo.lock` during git operations.

**Root Cause:**
Manually editing `Cargo.lock` instead of using cargo commands.

**Prevention:**
```bash
# ✅ Correct
cargo add package-name
cargo update

# ❌ Wrong
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
// ✅ Deterministic counter pattern
const counterFile = `/tmp/counter-${Date.now()}.txt`;
writeFileSync(counterFile, '0');
// Use counter % 2 for predictable pass/fail

// ❌ Probabilistic
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
// ✅ Unique filenames with timestamp
const counterFile = join(tmpDir, `counter-${Date.now()}-${testId}.txt`);

// ❌ Shared filename
const counterFile = join(tmpDir, 'counter.txt');  // Collision!
```

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
echo "✅ All quality checks passed!"
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

**Last Updated:** 2025-12-23
**Next Review:** Add new pitfalls as discovered during development
