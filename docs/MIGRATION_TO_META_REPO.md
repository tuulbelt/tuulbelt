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

**Wave 1: Independent Tools (7 tools)**
1. ✅ cli-progress-reporting
2. cross-platform-path-normalizer
3. config-file-merger
4. structured-error-handler
5. file-based-semaphore (Rust)
6. file-based-semaphore-ts
7. output-diffing-utility

**Wave 2: Optional Dependencies (1 tool)**
8. test-flakiness-detector (optional: cli-progress-reporting)

**Wave 3: Required Dependencies (2 tools)**
9. snapshot-comparison (requires: output-diffing-utility)
10. test-port-resolver (requires: file-based-semaphore-ts)

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
7. Commits with correct author (koficodedat) using auth scripts
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

**After Wave 1 completes:**
- Migrate Wave 2 tool (test-flakiness-detector)
- Migrate Wave 3 tools (snapshot-comparison, test-port-resolver)
  - Update dependencies to git URLs
  - Test standalone with auto-fetched dependencies

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
- **commit.sh** - Commits with koficodedat author, no Claude attribution
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

## References

- **Automation:** `.claude/commands/migrate-tool.md` - Complete automation spec
- **Quality Standards:** `docs/QUALITY_CHECKLIST.md` - Pre-commit requirements
- **Tracking:** `.claude/HANDOFF.md`, `.claude/NEXT_TASKS.md` - Current status
- **Authentication:** `scripts/setup-github-auth.sh` - Credential management

---

**Document Version:** 2.0 (Streamlined)
**Last Updated:** 2025-12-29
