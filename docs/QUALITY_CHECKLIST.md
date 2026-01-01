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
- [ ] **CLI shebang present**: Entry point has `#!/usr/bin/env -S npx tsx` for `npm link` to work

### Rust Specific

- [ ] **Cargo check passes**: `cargo check` (no errors)
- [ ] **Clippy clean**: `cargo clippy -- -D warnings` (no warnings)
- [ ] **Rustfmt applied**: `cargo fmt` (code formatted)
- [ ] **No `unwrap()` in production**: Use `?` operator or proper error handling
- [ ] **Cargo.toml valid**: Proper edition, dependencies empty

### Security Checks (All Languages)

- [ ] **No hardcoded secrets**: No passwords, API keys, tokens in source code
  - Run: `grep -r -iE '(password.*=|api_key.*=|secret.*=|token.*=)' src/ | grep -v test`
- [ ] **No secrets in staged changes**: Check before committing
  - Run: `git diff --cached | grep -iE '(password|api_key|secret|token|private_key|credentials)'`
- [ ] **No .env files tracked**: Sensitive config files not in git
  - Run: `git ls-files | grep -E '\.env'` (should be empty)
- [ ] **No high-severity vulnerabilities**: Dependencies are secure
  - TypeScript: `npm audit --audit-level=high`
  - Rust: `cargo audit` (if installed)
- [ ] **Protected files untouched**: No edits to package-lock.json, Cargo.lock manually
- [ ] **Prototype pollution prevented**: Object keys validated against `__proto__`, `constructor`, `prototype`
- [ ] **Path traversal prevented**: User-provided paths/IDs validated (no `..`, slashes, null bytes)
- [ ] **Stack traces not exposed**: Production APIs use safe serialization (no stack traces)
- [ ] **Input size limits**: Large inputs (files, messages, arrays) have reasonable limits
- [ ] **Security section in README**: Document security considerations for users

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

## Pre-Release Security Scan

**REQUIRED: Run before any version release (v0.x.x, v1.x.x, etc.)**

### When to Run

- Before tagging a new version release
- After major refactoring that touches security-sensitive code
- When adding new input handling (file paths, user IDs, etc.)

### How to Run

Use Claude Code's `/security-scan` command on each tool:

```bash
# In Claude Code
/security-scan

# The scan covers:
# - OWASP Top 10 vulnerabilities
# - Language-specific security patterns
# - Input validation gaps
# - Information disclosure risks
```

### Security Scan Checklist

- [ ] **Run `/security-scan`** on the tool being released
- [ ] **Address all CRITICAL findings** - these block release
- [ ] **Address all HIGH findings** - these block release
- [ ] **Review MEDIUM findings** - fix or document as acceptable
- [ ] **Update Security section in README** if new protections added
- [ ] **Add security tests** for any vulnerabilities fixed

### Common Security Patterns (Quick Reference)

**TypeScript:**
```typescript
// Prototype pollution prevention
const DANGEROUS_KEYS = ['__proto__', 'constructor', 'prototype'];
if (DANGEROUS_KEYS.includes(key)) { throw new Error('Invalid key'); }

// Path traversal prevention
if (!/^[a-zA-Z0-9_-]+$/.test(id)) { return { ok: false, error: 'Invalid ID' }; }

// Safe serialization (no stack traces)
toSafeJSON() { return { ...this, stack: undefined }; }
```

**Rust:**
```rust
// Input validation
if input.contains('\0') || input.contains("..") {
    return Err(Error::InvalidInput("path traversal attempt"));
}

// File size limits
if file_size > MAX_FILE_SIZE {
    return Err(Error::FileTooLarge(file_size, MAX_FILE_SIZE));
}
```

---

## New Tool Completion Checklist

**CRITICAL: When adding a new tool to Tuulbelt, ALL of these must be completed before marking work as done.**

Use TodoWrite to track these items. Do NOT mark the tool as complete until every item is checked.

### Tool Implementation

- [ ] **Tool code complete**: All functionality implemented and tested
- [ ] **Tests passing**: 80%+ coverage, all tests green
- [ ] **README complete**: Installation, usage, API docs, examples
- [ ] **CLI names documented**: Both short and long CLI names documented in README
  - Short name recommended for ease of use
  - Setup instructions for global installation (`npm link` or `cargo install --path .`)
  - Both forms shown in CLI usage examples
- [ ] **Zero dependencies**: `dependencies` object empty in package.json or Cargo.toml
- [ ] **Runs /quality-check**: Tool-level quality check passes

### Dogfooding Strategy

**CRITICAL: Only implement compositions that provide REAL value, not checkboxes.**

**How It Works:**
- Tools add other Tuulbelt tools as `devDependencies` via git URLs
- Dogfood runs in CI automatically via `npm run dogfood`
- See `DOGFOODING_STRATEGY.md` template for full decision tree

**Step 1: Default Setup (All Tools)**

All tools include test-flakiness-detector as a devDependency by default:

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

**Step 2: Additional Compositions (Answer These Questions)**

| Question | If YES | If NO |
|----------|--------|-------|
| Does your tool produce deterministic output? | Add output-diffing-utility | Skip (e.g., port-resolver) |
| Does your tool use file-based-semaphore-ts? | Test concurrent scenarios | Skip |
| Would your tool validate itself? | DON'T add that tool | (circular validation) |

**Step 3: Verification**

- [ ] **DOGFOODING_STRATEGY.md created**: Answer questions, document decisions
- [ ] **npm run dogfood**: Passes locally and in CI
- [ ] **CI runs dogfood**: test.yml workflow includes dogfood step
- [ ] **README dogfooding section**: Documents what tools are used

**Anti-Patterns:**
- ❌ Adding output-diffing to non-deterministic tools (port-resolver)
- ❌ Self-validation (output-diffing-utility validating itself)
- ❌ Adding all tools "just in case"

**Testing Dogfood:**
```bash
npm run dogfood    # Runs flaky detection (10 runs for TS, 20 for Rust)
```

### Library Composition Documentation (PRINCIPLES.md Exception 2)

**If your tool uses another Tuulbelt tool as a library dependency:**

- [ ] **README section added**: "Library Composition" section explaining the integration
- [ ] **VitePress index.md callout**: Add tip box highlighting library composition
  ```markdown
  ::: tip <img src="/icons/package.svg" class="inline-icon" alt=""> Uses [OtherTool] Library
  This tool uses [other-tool](/tools/other-tool/) as a library dependency...
  :::
  ```
- [ ] **VitePress library-usage.md updated**: Explain how the dependency is used
- [ ] **Root README.md Dogfooding section**: Add example showing the integration
- [ ] **docs/index.md card**: Add badge with icon (use `.library-badge` class with package.svg icon)
- [ ] **DOGFOODING_STRATEGY.md**: Document the library integration value

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
- [ ] **Demo section uses correct structure**: Use this template in VitePress index.md:
  ```markdown
  ## Demo

  See the tool in action:

  ![{Tool Name} Demo](/{tool-name}/demo.gif)

  **[▶ View interactive recording on asciinema.org](#)**

  <div style="margin: 20px 0;">
    <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
      <strong>Try it online:</strong>
    </span>
    <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/{tool-name}" style="display: inline-block; vertical-align: middle;">
      <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
    </a>
  </div>
  ```
  - The `(#)` placeholder link will be auto-updated by create-demos workflow
  - The placeholder demo.gif will be replaced with the real recording
- [ ] **Docs build succeeds**: Run `npm run docs:build` from root - must pass with no dead links
- [ ] **Tools index updated**: `docs/tools/index.md` shows correct count (e.g., "3/33") and includes new tool card
- [ ] **Home page updated**: `docs/index.md` updated with:
  - "View All X Tools" link shows correct count
  - "More Tools" section includes new tool card
  - "Progress" section shows correct phase counts and tool list

### GitHub Workflows

- [ ] **Demo recording script created**: `scripts/record-{tool-name}-demo.sh` using demo-framework.sh
  - Copy template from `templates/{language}-tool-template/scripts/record-demo.sh`
  - Set `TOOL_NAME`, `SHORT_NAME`, `LANGUAGE`
  - Implement `demo_commands()` function with tool's demo flow
  - See `scripts/lib/README.md` for framework documentation
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
3. [x] Create tool README with demo section structure
4. [ ] Create demo recording script using demo-framework.sh
5. [ ] Add path filter to create-demos.yml workflow
6. [ ] Add to VitePress config (docs/.vitepress/config.ts)
7. [ ] Create docs/tools/{tool-name}/ directory
8. [ ] Copy docs pages and fix links
9. [ ] Create placeholder demo.gif in docs/public/{tool-name}/
10. [ ] Add Demo section with correct structure (gif reference + (#) placeholder link)
11. [ ] Update docs/tools/index.md (tool count, add tool card)
12. [ ] Update docs/index.md (home page: tool count, More Tools, Progress)
13. [ ] Update root README.md
14. [ ] Run npm run docs:build (verify - MUST PASS)
15. [ ] Run /quality-check
16. [ ] Update HANDOFF.md and NEXT_TASKS.md
```

---

## Meta Repository Migration Checklist

**CRITICAL: For migrating tools from monorepo to standalone repositories (Phase 2 Wave 1-3)**

Use this checklist when running `/migrate-tool` to ensure complete migration. Reference: `.claude/commands/migrate-tool.md`

### Step 1: Extract Git History

- [ ] **Run git subtree split**: `git subtree split -P {tool-name} -b {tool-name}-history`
- [ ] **Verify commit count**: Compare with `git log --oneline {tool-name}/ | wc -l`
- [ ] **Check branch created**: `git branch | grep {tool-name}-history`

### Step 2: Create GitHub Repository

- [ ] **Create repository**: `gh repo create tuulbelt/{tool-name} --public`
- [ ] **Set description**: Short tool summary (one sentence)
- [ ] **Add topics**: tuulbelt, language (typescript/rust), zero-dependencies, tool-specific keywords
- [ ] **Disable issues**: `gh repo edit tuulbelt/{tool-name} --enable-issues=false`
- [ ] **Disable wiki**: `gh repo edit tuulbelt/{tool-name} --enable-wiki=false`
- [ ] **Disable projects**: `gh repo edit tuulbelt/{tool-name} --enable-projects=false`
- [ ] **Verify on GitHub**: Check web UI shows all settings correct

### Step 3: Prepare Standalone Repository

- [ ] **Clone to /tmp**: `cd /tmp && git clone https://github.com/tuulbelt/{tool-name}.git`
- [ ] **Pull history**: `git pull /path/to/monorepo {tool-name}-history`
- [ ] **Update package.json/Cargo.toml**:
  - [ ] `repository.url` points to standalone repo
  - [ ] `homepage` points to standalone repo README
  - [ ] `bugs.url` points to meta repo issues
- [ ] **Update CI workflow**:
  - [ ] Remove `working-directory` references
  - [ ] Add multi-version matrix (Node 18, 20, 22 or Rust stable)
  - [ ] Add zero-dependency verification step
  - [ ] Remove `cache-dependency-path` (use just `cache: 'npm'` or cargo default)
- [ ] **Update README.md**:
  - [ ] Badge URLs point to standalone repo workflows (not monorepo)
  - [ ] Installation instructions work standalone
  - [ ] Meta repo link kept ("Part of Tuulbelt" is appropriate)
- [ ] **Create CLAUDE.md**: Tool-specific development context

### Step 4: Commit and Release

- [ ] **Run npm install** (or cargo build): Verify dependencies install
- [ ] **Commit changes**: Use `scripts/commit.sh` with correct author
- [ ] **Tag v0.1.0**: `git tag v0.1.0`
- [ ] **Push to GitHub**: Use `scripts/push.sh` with correct credentials
- [ ] **Verify pushed**: Check GitHub shows v0.1.0 tag

### Step 5: Verify Standalone Functionality

- [ ] **Fresh clone**: `cd /tmp/verify && git clone https://github.com/tuulbelt/{tool-name}.git`
- [ ] **Install dependencies**: `npm ci` or `cargo build`
- [ ] **Run tests**: All tests pass (record count: X/X passing)
- [ ] **TypeScript compile** (if TS): `npx tsc --noEmit` passes
- [ ] **Build succeeds**: `npm run build` or `cargo build --release` passes
- [ ] **CLI works** (if applicable): Test short and long CLI names

### Step 6: Add Git Submodule

- [ ] **Add submodule**: `git submodule add https://github.com/tuulbelt/{tool-name}.git tools/{tool-name}`
- [ ] **Verify .gitmodules**: Check entry created correctly
- [ ] **Commit submodule**: Use `scripts/commit.sh`
- [ ] **Push to meta repo**: Use `scripts/push.sh`

### Step 7: Update Tracking Documents

- [ ] **Update .claude/HANDOFF.md**:
  - [ ] Update session title with tool name and progress (e.g., 2/7 complete)
  - [ ] Add migration accomplishments section
  - [ ] Update "NEXT SESSION" to next tool
  - [ ] Update Wave 1 progress checklist
- [ ] **Update STATUS.md**:
  - [ ] Update "Current Phase" with new progress (e.g., 2/7 complete)
  - [ ] Update percentage (e.g., 28%)
  - [ ] Add tool to completed list
- [ ] **Update CHANGELOG.md**:
  - [ ] Add new section with date: "### Added - Phase 2 Wave 1: {tool-name} Migration Complete ✅"
  - [ ] Document commit count, test results, repository URL
  - [ ] Document GitHub configuration applied
- [ ] **Update .claude/NEXT_TASKS.md**:
  - [ ] Move tool from pending to completed (with ✅)
  - [ ] Update remaining count (e.g., "Remaining Tools (5/7)")
- [ ] **Commit tracking updates**: Use `scripts/commit.sh`
- [ ] **Push to meta repo**: Use `scripts/push.sh`

### Post-Migration Verification

- [ ] **GitHub repository**: Visit https://github.com/tuulbelt/{tool-name}
  - [ ] README renders correctly
  - [ ] CI workflow runs and passes
  - [ ] Topics visible
  - [ ] Issues disabled message shows
  - [ ] **Description matches package.json/Cargo.toml** (not leftover text)
- [ ] **Meta repo submodule**: `cd tools/{tool-name} && git log --oneline -5`
- [ ] **Clean git status**: `git status` in meta repo shows clean
- [ ] **All 4 tracking docs updated**: HANDOFF, STATUS, CHANGELOG, NEXT_TASKS
- [ ] **Temporary branch deleted**: `git branch | grep {tool-name}-history` returns nothing
- [ ] **CLI functionality tested** (if applicable): Both short and long CLI names work via `npm link`

### Authentication Checklist

**Unified Credential Loading (MANDATORY for all scripts using gh/git):**

All scripts that use `gh` CLI or git operations MUST source the unified credential loader:

```bash
# At the top of your script, after REPO_ROOT detection
source "$REPO_ROOT/scripts/lib/load-credentials.sh"
```

This automatically:
- Loads credentials from `.env`
- Exports `GH_TOKEN` (for gh CLI)
- Exports `GITHUB_TOKEN` (for git/MCP)
- Sets git user.name and user.email
- Validates required variables exist

**Checklist:**
- [ ] **All gh/git scripts source load-credentials.sh**: Check `scripts/lib/load-credentials.sh` is loaded
- [ ] **MCP server uses .env**: Custom GitHub MCP server reads from `.env` automatically (already configured)
- [ ] **Verify auth**: `gh auth status` shows correct account (koficodedat from .env)
- [ ] **Test GitHub operation**: Try `gh repo view tuulbelt/tuulbelt` to verify access

**Authentication Priority (gh CLI):**
1. `GH_TOKEN` environment variable ✅ (what we use)
2. `GITHUB_TOKEN` environment variable
3. Stored credentials in `~/.config/gh/hosts.yml` ⚠️ (can be wrong account)

**Never:**
- ❌ Call `gh` directly without loading credentials
- ❌ Manually source `.env` in scripts (use load-credentials.sh instead)
- ❌ Rely on stored gh credentials (may be wrong account)

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

# Security checks (all languages)
echo ""
echo "→ Running security checks..."

echo "  ✓ Checking for hardcoded secrets..."
if grep -r -iE '(password.*=|api_key.*=|secret.*=|token.*=)' src/ 2>/dev/null | grep -v test | grep -v '.md' | grep -q .; then
  echo "  ⚠ WARNING: Potential secrets found in source files!"
  grep -r -iE '(password.*=|api_key.*=|secret.*=|token.*=)' src/ 2>/dev/null | grep -v test | grep -v '.md'
fi

echo "  ✓ Checking for tracked .env files..."
if git ls-files | grep -qE '\.env'; then
  echo "  ✗ ERROR: .env files are tracked by git!"
  git ls-files | grep -E '\.env'
  exit 1
fi

echo "  ✓ Running dependency vulnerability scan..."
if [ -f "package.json" ]; then
  npm audit --audit-level=high 2>&1 || echo "  ⚠ Review vulnerabilities above"
fi

echo ""
echo "All quality checks passed!"
```

> **Note:** For comprehensive security analysis, also run `/security-scan` in Claude Code.

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
