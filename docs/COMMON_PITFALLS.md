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

#### Missing Shebang for npm link (2025-12-27)

**Symptom:**
```
/opt/hostedtoolcache/node/20.19.6/x64/bin/prog: line 9: import: command not found
/opt/hostedtoolcache/node/20.19.6/x64/bin/prog: line 10: import: command not found
```

**Root Cause:**
The `bin` entry in package.json points to a TypeScript file, but without a shebang, `npm link` creates a symlink that tries to execute the file directly as a shell script.

**Prevention:**
```typescript
#!/usr/bin/env -S npx tsx
/**
 * Tool Name
 * ...
 */
import { something } from 'node:module';
```

Add `#!/usr/bin/env -S npx tsx` as the **first line** of your TypeScript entry point.

**Verification:**
```bash
cd your-tool
npm install
npm link
your-short-name --help  # Should work, not show "import: command not found"
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

#### CI Zero-Dependency Check Failing (False Positives)

**Symptom:**
CI fails with "Runtime dependencies found!" but Cargo.toml has no dependencies.

**Root Cause:**
Grep patterns using `-A 10` capture lines from subsequent TOML sections like `[profile.release]`, causing false matches.

**Prevention:**
```bash
# Correct - uses awk to properly handle TOML sections
COUNT=$(awk '/^\[dependencies\]/,/^\[/ {if (!/^\[/ && !/^#/ && NF > 0) print}' Cargo.toml | wc -l)

# Wrong - captures too many lines
grep -A 10 '^\[dependencies\]' Cargo.toml | grep -q '^[a-z]'
```

**Verification:**
- Rust template now includes correct pattern
- Test locally: Check if awk command returns empty output for zero-dep tools

---

### CI/CD Issues

#### Branch Protection Blocking Automated Workflows (2026-01-10)

**Symptom:**
```
remote: error: GH006: Protected branch update failed for refs/heads/main.
remote: - Changes must be made through a pull request.
remote: - 3 of 3 required status checks are expected.
! [remote rejected] main -> main (protected branch hook declined)
error: failed to push some refs
```

Or in PR UI: `"testExpected — Waiting for status to be reported"` (check never arrives)

Automated workflows (update-baseline, create-demo) fail when trying to push to main branch, or PRs get stuck waiting for non-existent status checks.

**Root Cause:**
GitHub branch protection with "Require pull request reviews" and "Require status checks" blocks ALL direct pushes to main, including automated workflows using `GITHUB_TOKEN`. The default `GITHUB_TOKEN` cannot bypass branch protection.

**Why This Happens:**
1. Workflow runs on main after PR merge
2. Workflow creates new commit (baseline/demo update)
3. Workflow tries to push commit to main
4. Branch protection rejects push (requires PR + status checks)
5. Commit has `[skip ci]` but this doesn't exempt from protection

**Solution:**
Remove `required_pull_request_reviews` and `required_status_checks` from branch protection while keeping other protections:

```bash
gh api --method PUT repos/tuulbelt/REPO_NAME/branches/main/protection --input - << 'EOF'
{
  "required_status_checks": null,
  "enforce_admins": false,
  "required_pull_request_reviews": null,
  "restrictions": null,
  "required_linear_history": true,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "block_creations": false,
  "required_conversation_resolution": true,
  "lock_branch": false,
  "allow_fork_syncing": false
}
EOF
```

**Why This Is Safe:**
- ✅ **Write access is controlled**: Public repos don't grant write access - only collaborators added by admin can push/merge
- ✅ Pre-commit hooks prevent developers from pushing to main directly
- ✅ Workflow conventions enforce feature branch development (documented + enforced by hooks)
- ✅ Status checks still run on all PRs automatically
- ✅ GitHub PR UI requires checks pass before enabling merge button (visual feedback)
- ✅ Code reviews are **recommended best practice** (shown in PR UI, just not enforced)
- ✅ Only automated documentation workflows push directly (baselines, demos - low risk changes)

**Alternative Solutions (Not Recommended for Tuulbelt):**
1. **Use GitHub App or PAT** - Requires additional setup and secret management
2. **Create PRs from workflows** - Adds complexity for low-risk documentation updates
3. **Disable workflows** - Loses automated baseline tracking and demo generation

**Prevention:**
When setting up new tool repositories:
- Use simplified branch protection (linear history + conversation resolution only)
- Don't enable PR/status check requirements for repos with automated workflows
- Document in tool's README that automated workflows push directly to main

**Verification:**
```bash
# Check protection configuration
gh api repos/tuulbelt/REPO_NAME/branches/main/protection --jq '{required_status_checks, required_pull_request_reviews}'

# Both should be null or absent for repos with automated workflows

# Test workflows
gh workflow run "Update Benchmark Baseline" --repo tuulbelt/REPO_NAME
gh workflow run "Create Demo Recording" --repo tuulbelt/REPO_NAME
```

**Affected Repositories:**
- **Meta repo (tuulbelt/tuulbelt)** ✅ Fixed 2026-01-10
  - Issue: Required non-existent "test" status check
  - Fix: Removed status check and PR review requirements
- **All 11 tool repos** ✅ Fixed 2026-01-10
  - test-flakiness-detector, property-validator (with benchmark workflows)
  - cli-progress-reporting, config-file-merger, cross-platform-path-normalizer
  - file-based-semaphore, file-based-semaphore-ts, output-diffing-utility
  - port-resolver, snapshot-comparison, structured-error-handler (all with demo workflows)
- **All future tools** with automated workflows will use simplified protection from templates

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

#### GitHub Authentication with Wrong Account (2025-12-31)

**Symptom:**
```
pull request create failed: GraphQL: must be a collaborator (createPullRequest)
```
Or `gh auth status` shows wrong account:
```
✓ Logged in to github.com account kofirc (GITHUB_TOKEN)
  - Active account: true
```

**Root Cause:**
Scripts call `gh` CLI directly without loading credentials from `.env` first. `gh` falls back to stored credentials in `~/.config/gh/hosts.yml` which may be for a different account.

**Authentication Priority (gh CLI):**
1. `GH_TOKEN` environment variable (correct - uses .env)
2. `GITHUB_TOKEN` environment variable (correct - uses .env)
3. Stored credentials in `~/.config/gh/hosts.yml` (wrong - may be different account)

**Prevention:**
```bash
#!/bin/bash
# At the top of any script using gh or git commands

# Detect repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$HOME/tuulbelt")"

# Load GitHub credentials from .env
source "$REPO_ROOT/scripts/lib/load-credentials.sh"

# Now gh and git use correct credentials
gh pr create --title "..." --body "..."
```

**What load-credentials.sh does:**
- Sources `.env` file from repository root
- Exports both `GH_TOKEN` and `GITHUB_TOKEN`
- Sets git user.name and user.email
- Validates required variables exist

**Verification:**
```bash
# After sourcing load-credentials.sh
echo $GH_TOKEN  # Should show token from .env

gh auth status  # Should show:
# ✓ Logged in to github.com account koficodedat (GH_TOKEN)

gh repo view tuulbelt/tuulbelt  # Should succeed without errors
```

**Fixed Scripts:**
- `scripts/cli/create-cli-prs.sh`
- `scripts/web/create-web-prs.sh`
- `scripts/web/show-status.sh`
- `scripts/cli/cleanup-cli-workspace.sh`
- `scripts/web/cleanup-web-session.sh`
- `scripts/create-all-repos.sh`

**Never:**
- ❌ Call `gh` directly without loading credentials
- ❌ Manually source `.env` in scripts (use load-credentials.sh instead)
- ❌ Rely on stored gh credentials (can be wrong account)

---

### Best Practices

#### Dogfooding Pattern: DevDependency Approach (2025-12-30)

**Pattern:**
Tools validate each other by adding Tuulbelt tools as devDependencies via git URLs.

**Use Case:**
All tools use test-flakiness-detector to validate test determinism. This runs in CI automatically.

**Implementation (TypeScript):**

Add to `package.json`:

```json
{
  "scripts": {
    "dogfood": "flaky --test 'npm test' --runs 10"
  },
  "devDependencies": {
    "@tuulbelt/test-flakiness-detector": "git+https://github.com/tuulbelt/test-flakiness-detector.git"
  }
}
```

CI workflow includes:

```yaml
- name: Dogfood (flakiness detection)
  run: npm run dogfood
```

**Implementation (Rust):**

Add a minimal `package.json` for dogfooding:

```json
{
  "private": true,
  "scripts": {
    "dogfood": "flaky --test 'cargo test' --runs 20"
  },
  "devDependencies": {
    "@tuulbelt/test-flakiness-detector": "git+https://github.com/tuulbelt/test-flakiness-detector.git"
  }
}
```

CI workflow includes:

```yaml
- name: Setup Node.js (for dogfooding)
  uses: actions/setup-node@v4

- name: Install dogfood dependencies
  run: npm install

- name: Dogfood (flakiness detection)
  run: npm run dogfood
```

**Checklist:**
- [ ] `package.json` has test-flakiness-detector as devDependency
- [ ] `npm run dogfood` script is defined
- [ ] CI workflow includes dogfood step
- [ ] Dogfood passes locally before pushing

**Benefits:**
- **CI Integration**: Runs automatically on every push/PR
- **Standalone**: Each tool is independently testable
- **No Sibling Dependencies**: Works without meta repo context

---

