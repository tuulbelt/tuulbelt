---
description: Run comprehensive quality checks before committing
argument-hint: "[language]"
---

Run all quality assurance checks from the QUALITY_CHECKLIST.md to ensure code is ready for commit.

## Pre-Flight Checks

Before running checks:
1. Identify the project language (TypeScript/Node.js or Rust)
2. Ensure you're in the correct directory (tool root)
3. Load the checklist: @docs/QUALITY_CHECKLIST.md

## Check Execution

### Universal Checks (All Languages)

```bash
# 0. Check if branch is behind main (CRITICAL - prevents merge conflicts)
git fetch origin main --quiet 2>/dev/null || true
BEHIND=$(git rev-list --count HEAD..origin/main 2>/dev/null || echo "0")
if [ "$BEHIND" != "0" ]; then
  echo "⚠️  WARNING: Your branch is $BEHIND commit(s) behind origin/main"
  echo "   Run: git pull --rebase origin main"
fi

# 1. Check git status for uncommitted changes
git status --porcelain

# 2. Look for temporary files or test artifacts
find . -type f \( -name "*.tmp" -o -name "*counter*" \) -not -path "*/node_modules/*" -not -path "*/target/*"
```

### TypeScript/Node.js Projects

If `package.json` exists, run:

```bash
# 1. Type check (must pass)
npx tsc --noEmit 2>&1

# 2. Build (must succeed)
npm run build 2>&1

# 3. Tests (must pass 100%)
npm test 2>&1

# 4. Verify zero runtime dependencies
echo "Checking for runtime dependencies..."
cat package.json | grep -A10 '"dependencies"'

# 5. Check for @types/node in devDependencies
cat package.json | grep '@types/node'
```

### Rust Projects

If `Cargo.toml` exists, run:

```bash
# 1. Cargo check (must pass)
cargo check 2>&1

# 2. Clippy (must have zero warnings)
cargo clippy -- -D warnings 2>&1

# 3. Format check
cargo fmt -- --check 2>&1

# 4. Tests (must pass 100%)
cargo test 2>&1

# 5. Verify zero runtime dependencies
echo "Checking for runtime dependencies..."
grep -A10 '^\[dependencies\]' Cargo.toml
```

### Documentation Checks (If at Root Level)

If you're at the root of the Tuulbelt repo (contains `docs/.vitepress/`), also run:

```bash
# 1. Verify VitePress documentation builds without errors
cd /path/to/tuulbelt/root
npm run docs:build 2>&1

# 2. Check for dead links in output
# VitePress will report dead links during build

# 3. Verify all tools are in VitePress config
grep -r "Cross-Platform Path Normalizer\|Test Flakiness Detector\|CLI Progress Reporting" docs/.vitepress/config.ts
```

**CRITICAL for new tools:**
- Tool MUST appear in `docs/.vitepress/config.ts` sidebar
- Tool docs MUST exist in `docs/tools/{tool-name}/`
- VitePress build MUST succeed with no dead links
- Tool MUST be listed in `docs/tools/index.md`

## Validation Criteria

### MUST PASS (Blocking)

- [ ] Build succeeds (exit code 0)
- [ ] All tests pass (exit code 0)
- [ ] No TypeScript/Clippy errors or warnings
- [ ] Zero runtime dependencies in `dependencies` section
- [ ] **Documentation builds** (if at root: `npm run docs:build` succeeds)
- [ ] **No dead links** in VitePress output

### MUST REVIEW (Warnings)

- [ ] **Branch is up-to-date with main** (run `git pull --rebase origin main` if behind)
- [ ] No uncommitted changes or untracked files (unless intentional)
- [ ] No temporary test files or artifacts
- [ ] No debug code (`console.log`, `println!`, etc.)
- [ ] Code follows Tuulbelt principles (@PRINCIPLES.md)
- [ ] **New tools added to VitePress config** (if adding a tool)
- [ ] **New tools listed in docs/tools/index.md** (if adding a tool)

## Common Pitfalls to Check

Reference @docs/QUALITY_CHECKLIST.md for detailed pitfalls. Specifically verify:

**TypeScript/Node.js:**
- `@types/node` is in devDependencies
- ES module imports use `import`, not `require()`
- No `any` types in code
- Test commands use explicit file lists, not globs in quotes

**Rust:**
- No manual edits to `Cargo.lock`
- No `unwrap()` in production code paths
- Proper error handling with `?` operator

**General:**
- No probabilistic tests (Math.random(), etc.)
- Unique filenames for test state (no conflicts)
- Test cleanup removes all temporary files

## Output Format

Provide a clear summary:

```
✅ Quality Check Summary

Language: [TypeScript|Rust]
Location: [current directory]

CHECKS PASSED:
  ✓ Build successful
  ✓ Tests passed (X/X tests, 100%)
  ✓ Type/lint checks passed
  ✓ Zero runtime dependencies verified

WARNINGS:
  ⚠ [Any warnings or items to review]

READY TO COMMIT: [YES|NO]

[If NO, explain what needs to be fixed]
```

## On Failure

If any check fails:
1. **Stop immediately** - Do not proceed with commit
2. **Identify root cause** - Check error messages against QUALITY_CHECKLIST.md
3. **Fix the issue** - Apply prevention steps from checklist
4. **Re-run** - Execute `/quality-check` again to verify
5. **Document** - If this is a new issue, add it to QUALITY_CHECKLIST.md

## References

- @docs/QUALITY_CHECKLIST.md - Full checklist and pitfalls database
- @PRINCIPLES.md - Tuulbelt design principles
- @CONTRIBUTING.md - Contribution workflow
