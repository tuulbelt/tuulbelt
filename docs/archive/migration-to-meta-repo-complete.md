# Migration Plan: Monorepo to Meta Repository

**Created:** 2025-12-29
**Status:** Phase 2 Wave 1 In Progress (1/7 tools complete)
**Priority:** Critical (Architectural Correction)

---

## Executive Summary

Tuulbelt was designed as a **meta repository** but incorrectly implemented as a **monorepo**. This migration corrects that architectural mistake by:
- Moving each tool to its own GitHub repository
- Using git submodules in the meta repo to reference tools
- Switching from path dependencies to git URL dependencies
- Enabling true standalone tool usage

**Current Progress:**
- ✅ Phase 0: Preparation (Complete)
- ✅ Phase 1: Automation (Complete - `/migrate-tool` command created)
- 🔄 Phase 2 Wave 1: Independent tools (1/7 complete - cli-progress-reporting done)

---

## Why This Matters

**The Problem:**
```
tuulbelt/tuulbelt/
├── test-port-resolver/           # Tool code here (WRONG)
└── file-based-semaphore-ts/      # Dependency: file:../semats (BREAKS standalone)
```

**The Solution:**
```
tuulbelt/tuulbelt/                # Meta repo - coordination only
└── tools/                        # Git submodules
    ├── test-port-resolver/       → github.com/tuulbelt/test-port-resolver
    └── file-based-semaphore-ts/  → github.com/tuulbelt/file-based-semaphore-ts

# Each tool repo has git URL dependency:
"@tuulbelt/semats": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
```

---

## Completed Work

### Phase 0: Preparation ✅
- Created ROADMAP.md, updated documentation
- Documented migration decisions (centralized issues, post-migration tagging)
- Created TOOL_REPO_SETTINGS.md template
- Added CONTRIBUTING.md to templates

### Phase 1: Automation ✅
- **MCP Server:** `.claude/mcp/tuulbelt-github/` for GitHub API operations
- **Commands:** `/migrate-tool`, `/new-tool`, `/release-tool`, `/add-tool-dependency`
- **Agent:** `tool-creator` for comprehensive tool setup
- **Scripts:** `setup-github-auth.sh`, `commit.sh`, `push.sh`

### Phase 2 Wave 1: First Tool ✅
- **cli-progress-reporting** migrated to standalone repository
- 58 commits extracted with full git history
- v0.1.0 tagged and released
- 121/121 tests passing standalone
- CI workflow configured (Node 18, 20, 22 + zero-dep check)

---

## Migration Order (Dependency-Aware)

**Wave 1: Independent Tools (7 tools)** ✅ COMPLETE
1. ✅ cli-progress-reporting
2. ✅ cross-platform-path-normalizer
3. ✅ config-file-merger
4. ✅ structured-error-handler
5. ✅ file-based-semaphore (Rust)
6. ✅ file-based-semaphore-ts
7. ✅ output-diffing-utility

**Wave 2: Required Dependencies (3 tools)**

Ordered from lightest to heaviest for incremental learning:

8. **snapshot-comparison** (requires: output-diffing-utility)
   - Rust tool with single dependency
   - Straightforward migration pattern
   - Good warm-up for dependency handling

9. **test-flakiness-detector** (requires: cli-progress-reporting)
   - TypeScript tool with required dependency
   - **NEEDS IMPLEMENTATION REVIEW**: Make cli-progress dependency required
   - Review implementation, tests, spec, architecture to enforce required dependency
   - Update integration and documentation

10. **test-port-resolver** (requires: file-based-semaphore-ts)
    - TypeScript tool with required dependency
    - **NEEDS COMPREHENSIVE REVIEW**: Full audit against /new-tool standards
    - Verify: implementation, testing, code quality, security
    - Verify: expanded testing categories (unit, CLI, integration, performance)
    - Verify: documentation (GitHub README + VitePress full site)
    - Verify: demos (asciinema recordings + StackBlitz links)
    - Verify: templates and scaffolding adherence
    - Most complex - saved for last with full pattern knowledge

---

## Using /migrate-tool Command

The `/migrate-tool` command automates the entire migration process:

```bash
# 1. Configure authentication
source scripts/setup-github-auth.sh

# 2. Run migration
/migrate-tool <tool-name>
```

**What it does:**
1. Extracts git history using `git subtree split`
2. Creates GitHub repository
3. Updates package.json/Cargo.toml (URLs, repository fields)
4. Updates CI workflow for standalone operation
5. Updates README.md (absolute URLs, standalone instructions)
6. Creates tool-specific CLAUDE.md
7. Commits with correct author (from .env) using auth scripts
8. Tags v0.1.0 and pushes to GitHub
9. Verifies standalone functionality (tests, build)
10. Adds as git submodule to meta repo
11. Updates tracking documents (HANDOFF.md, STATUS.md, CHANGELOG.md, NEXT_TASKS.md)

**See:** `.claude/commands/migrate-tool.md` for complete details.

---

## Target Architecture

### Meta Repository Structure

```
tuulbelt/tuulbelt/
├── .claude/                    # Claude Code configuration
├── .github/workflows/          # Meta repo CI/CD
├── docs/                       # VitePress documentation site
├── templates/                  # Tool scaffolding templates
├── tools/                      # Git submodules (NEW)
│   ├── cli-progress-reporting/    → tuulbelt/cli-progress-reporting
│   ├── test-port-resolver/        → tuulbelt/test-port-resolver
│   └── ... (all 10 tools)
├── scripts/
│   ├── setup-github-auth.sh
│   ├── commit.sh
│   └── push.sh
├── CLAUDE.md
├── PRINCIPLES.md
└── README.md
```

### Individual Tool Repository Structure

```
tuulbelt/<tool-name>/
├── .github/workflows/test.yml  # Standalone CI
├── src/
├── test/
├── docs/                       # Tool-specific docs
├── scripts/
│   ├── dogfood-*.sh
│   └── record-demo.sh
├── package.json                # With git URL dependencies
├── README.md
├── CLAUDE.md                   # Tool-specific context
└── LICENSE
```

---

## Key Dependency Changes

**Before (Monorepo - BROKEN):**
```json
"dependencies": {
  "@tuulbelt/file-based-semaphore-ts": "file:../file-based-semaphore-ts"
}
```

**After (Git URL - WORKS EVERYWHERE):**
```json
"dependencies": {
  "@tuulbelt/file-based-semaphore-ts": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
}
```

**Rust:**
```toml
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility" }
```

When running `npm install` or `cargo build`, dependencies are automatically cloned from GitHub. No manual setup required.

---

## Next Steps for Remaining Tools

**For each Wave 1 tool (6 remaining):**

1. Start fresh session for tool
2. Run: `source scripts/setup-github-auth.sh`
3. Run: `/migrate-tool <tool-name>`
4. Verify:
   ```bash
   # Fresh clone test
   cd /tmp
   git clone https://github.com/tuulbelt/<tool-name>.git
   cd <tool-name>
   npm install && npm test  # or cargo build && cargo test
   ```
5. Update tracking documents (automated by `/migrate-tool`)
6. Commit and push

**After Wave 1 completes (DONE ✅):**
- Migrate Wave 2 tools in order: snapshot-comparison, test-flakiness-detector, test-port-resolver
- Update dependencies to git URLs
- Test standalone with auto-fetched dependencies
- For test-flakiness-detector: Review and update to make cli-progress dependency required
- For test-port-resolver: Comprehensive review against /new-tool standards

---

## Post-Migration Restructure

After all 10 tools are migrated:

**Phase 5: Meta Repo Restructure**
1. Remove tool directories from root (they're now submodules in tools/)
2. Update VitePress to reference tools/ submodules
3. Update workflows for submodule paths
4. Create `scripts/prepare-docs.sh` to copy docs from submodules

**Phase 6: Verification**
1. Clone meta repo with submodules: `git clone --recursive https://github.com/tuulbelt/tuulbelt.git`
2. Run all tests via submodules
3. Verify docs build
4. Verify total test count: 1,141 across all tools

---

## Rollback Plan

If migration fails:

1. **Restore from backup:**
   ```bash
   git checkout backup/pre-meta-repo-migration
   git branch -D main
   git checkout -b main
   git push -f origin main
   ```

2. **Delete created repositories** (if needed)

3. **Document issues** in KNOWN_ISSUES.md

Individual tool migrations are isolated - if one fails, keep it in monorepo temporarily and retry.

---

## Testing Strategy

**Pre-Migration:** Record baseline test counts (done - 1,141 total tests)

**Per-Tool Verification:**
1. Fresh clone from GitHub
2. Run `npm install && npm test` (or cargo equivalents)
3. Verify test count matches baseline
4. For dependent tools: verify dependency auto-fetches during install

**Post-Migration:**
1. Clone meta repo: `git clone --recursive https://github.com/tuulbelt/tuulbelt.git`
2. Run all tests via `scripts/test-all-tools.sh`
3. Verify VitePress builds
4. Verify total: 1,141 tests passing

---

## Authentication Scripts

All migrations use these scripts for correct credentials:

- **setup-github-auth.sh** - Sources GITHUB_TOKEN, GITHUB_USERNAME, GITHUB_EMAIL from .env
- **commit.sh** - Commits with correct author (from .env), no Claude attribution
- **push.sh** - Pushes with correct credentials, auto-pulls before push to avoid conflicts

**Usage:**
```bash
# Always run first in new session
source scripts/setup-github-auth.sh

# Then use for commits (via /migrate-tool)
scripts/commit.sh "commit message"
scripts/push.sh
```

---

## Known Issues & Fixes

**Git Push Conflicts:**
- **Cause:** GitHub Actions workflows auto-commit (create-demos.yml, update-dashboard.yml)
- **Fix:** scripts/push.sh now auto-pulls before push
- **Status:** Resolved

**Stop Hook Path Failure:**
- **Cause:** Hardcoded `/home/user/tuulbelt` path
- **Fix:** Dynamic path detection using `git rev-parse --show-toplevel`
- **Status:** Resolved

---

## Lessons Learned from First Two Migrations

**Source:** cli-progress-reporting (Wave 1, Tool 1/7) and cross-platform-path-normalizer (Wave 1, Tool 2/7) migrations completed 2025-12-29

### Critical Success Factors

**1. Comprehensive Gap Analysis is Essential**
- Initial migration appeared complete but missed 6 critical gaps
- Second-pass review discovered 5 additional potential gaps
- **Takeaway:** Always run through complete checklist twice before marking complete
- Reference: `docs/QUALITY_CHECKLIST.md` - Meta Repository Migration Checklist

**2. All Four Tracking Documents Must Be Updated**
- **HANDOFF.md**: Session summary, progress, next tool
- **STATUS.md**: Current phase, tool count
- **CHANGELOG.md**: Complete migration entry
- **NEXT_TASKS.md**: Move tool to completed
- **Why:** Session continuity - we clear context after every migration
- Missing any document loses critical information

**3. GitHub Configuration is NOT Automatic**
- Creating repo doesn't automatically configure settings
- Must explicitly disable: issues, wiki, projects
- Must explicitly add: topics, description
- **Verification required:** Always check GitHub web UI or use `gh repo view`

### Authentication Challenges & Solutions

**Problem:** gh CLI Authentication Cache
- `gh` CLI caches credentials in system keyring
- Setting GITHUB_TOKEN doesn't override cached credentials
- Caused recurring "wrong account does not have the correct permissions" errors

**Solutions (in order of preference):**
1. **Use GitHub MCP Server** (`.claude/mcp/tuulbelt-github/`)
   - Reads directly from `.env` file
   - No CLI authentication cache issues
   - Recommended for all GitHub operations

2. **Use GH_TOKEN environment variable**
   ```bash
   unset GITHUB_TOKEN
   export GH_TOKEN="ghp_token_here"
   gh auth status  # Now shows correct account
   ```
   - GH_TOKEN takes precedence over GITHUB_TOKEN
   - Avoids keyring cache

3. **Fallback to curl REST API**
   ```bash
   curl -X POST -H "Authorization: token ghp_token" \
     https://api.github.com/orgs/tuulbelt/repos -d '{...}'
   ```
   - Direct GitHub API access
   - No CLI cache issues

### README Badge Updates are Critical

**Problem:** Badges pointed to monorepo workflows after migration
- Test badge: `tuulbelt/tuulbelt/.../test-all-tools.yml`
- Should be: `tuulbelt/{tool-name}/.../test.yml`

**Why this matters:** Shows incorrect CI status to users

**Fix pattern:**
```markdown
# Before (WRONG):
[![Tests](https://github.com/tuulbelt/tuulbelt/actions/workflows/test-all-tools.yml/badge.svg)]

# After (CORRECT):
[![Tests](https://github.com/tuulbelt/{tool-name}/actions/workflows/test.yml/badge.svg)]
```

**Meta repo links OK:** Keeping "Part of [Tuulbelt](https://github.com/tuulbelt/tuulbelt)" is appropriate

### Testing Verification Pattern

**Fresh Clone is Non-Negotiable:**
```bash
# Don't trust local state
cd /tmp/verify-$(date +%s)
git clone https://github.com/tuulbelt/{tool-name}.git
cd {tool-name}
npm ci  # or cargo build
npm test  # or cargo test
```

**Why:** Local state may have:
- Symlinks to monorepo tools
- Cached node_modules
- Git worktree artifacts

**Record everything:**
- Commit count extracted
- Test count passing
- Build success confirmation

### CI Workflow Updates

**Required changes:**
1. Remove `working-directory:` references
2. Add multi-version matrix (Node 18, 20, 22)
3. Add zero-dependency check
4. Remove `cache-dependency-path` (just use `cache: 'npm'`)

**Verification:**
- Wait for first CI run in standalone repo
- Don't rely on local testing alone

### Git Submodule Integration

**Correct order:**
1. Complete all standalone repo work first
2. Verify standalone tests pass
3. Then add as submodule to meta repo
4. Commit submodule addition separately

**Why:** Separates concerns - if submodule fails, standalone repo is still complete

### Documentation Updates

**Second-pass review findings:**
- Labels: GitHub adds 9 default labels automatically (no action needed)
- Releases: Git tags sufficient, GitHub Releases not required
- Cache config: Verified correctly removed
- Meta repo README: Found monorepo paths still present (needs fix)

### Session Continuity Strategy

**Problem:** Context cleared after every migration
**Solution:** Document everything in permanent files

**Updated files this migration:**
- `.claude/commands/migrate-tool.md` - Added explicit steps and lessons learned
- `docs/QUALITY_CHECKLIST.md` - Added complete migration checklist (100+ items)
- `docs/MIGRATION_TO_META_REPO.md` - This lessons learned section

**Future migrations:** Reference these three files

### What Worked Well

1. **Authentication scripts** (setup-github-auth.sh, commit.sh, push.sh)
   - Centralized credential management
   - Auto-rebase before push prevents conflicts

2. **Git subtree split**
   - Preserves full commit history
   - 58 commits (cli-progress) and 457 commits (path-normalizer) extracted cleanly

3. **Incremental verification**
   - Test at each step
   - Don't batch multiple changes

4. **CLAUDE.md in standalone repos**
   - Provides tool-specific context
   - Helps future development sessions

### What Needs Improvement

1. **Automate GitHub configuration**
   - Current: Manual `gh repo edit` commands
   - Future: MCP server batch operation or script

2. **Automate badge URL updates**
   - Current: Manual search/replace in README
   - Future: Script to update badge patterns

3. **Automated tracking document updates**
   - Current: Manual edits to 4 files
   - Future: Script or template expansion

4. **Pre-flight verification**
   - Add checklist verification before starting migration
   - Prevent "oops I forgot" scenarios

---

## References

- **Automation:** `.claude/commands/migrate-tool.md` - Complete automation spec
- **Quality Standards:** `docs/QUALITY_CHECKLIST.md` - Pre-commit requirements
- **Tracking:** `.claude/HANDOFF.md`, `.claude/NEXT_TASKS.md` - Current status
- **Authentication:** `scripts/setup-github-auth.sh` - Credential management

---

**Document Version:** 2.1 (Added Lessons Learned from First Two Migrations)
**Last Updated:** 2025-12-29
