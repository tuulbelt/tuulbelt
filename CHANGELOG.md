# Tuulbelt Meta Repository Changelog

All notable changes to the Tuulbelt meta repository will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Retention Policy:** Keep only last 60 days of entries. Older entries are in git history (`git log -p CHANGELOG.md`).

---

## [Unreleased]

### Added - Phase 2 Wave 2: test-flakiness-detector Migration Complete ✅ (2025-12-29)

**Second Wave 2 tool with REQUIRED git URL dependency migrated to standalone repository!**
- Repository: https://github.com/tuulbelt/test-flakiness-detector
- Extracted git history: 92 commits processed
- **Added REQUIRED dependency**: `"@tuulbelt/cli-progress-reporting": "git+https://github.com/tuulbelt/cli-progress-reporting.git"`
- **Critical correction**: User requested cli-progress be required, not optional - updated implementation to use standard ES module import instead of dynamic fallback
- Tagged v0.1.0 and pushed to GitHub with koficodedat credentials
- Verified standalone: 132/132 tests passing with required cli-progress dependency
- **Git URL dependency verified**: npm automatically fetched cli-progress-reporting from GitHub
- Added as git submodule: `tools/test-flakiness-detector`
- **Wave 2 Progress**: 2/3 complete (67%)
- **Demonstrates**: Tuulbelt-to-Tuulbelt composition (PRINCIPLES.md Exception 2)

**GitHub Repository Configuration:**
- Issues disabled (centralized to meta repo)
- Wiki disabled
- Projects disabled
- Topics: tuulbelt, typescript, zero-dependencies, testing, flakiness, test-reliability

**Key Lessons:**
- TypeScript tools can have required Tuulbelt dependencies via git URLs
- npm automatically clones and installs git URL dependencies during `npm install`
- Preserves zero external dependency principle (all Tuulbelt tools have zero external deps)

**Gap Analysis & Documentation Fixes:**
After migration, comprehensive review identified 6 documentation gaps:
1. SPEC.md: "Zero dependencies" → "Zero external dependencies"
2. DOGFOODING_STRATEGY.md: Updated from optional to REQUIRED pattern
3. CI workflow: Fixed zero-dep check to allow @tuulbelt/* deps
4. README.md: Fixed 4 misleading statements
5. VitePress docs: Fixed 3 files (index.md, what-is-it.md, installation.md)
6. CI Fix: Corrected grep pattern matching devDependencies (false positive)

**Commits:**
- `34e629d` - chore: prepare for standalone release
- `fb3bd1e` - feat: make cli-progress-reporting a required dependency
- `344e957` - chore: add test-flakiness-detector as git submodule
- `82f8cec` - chore: update test-flakiness-detector submodule (required dependency)
- `25edae8` - docs: update all documentation to reflect required cli-progress dependency
- `2179606` - chore: update test-flakiness-detector submodule (gap fixes)
- `1f6b6a8` - fix(ci): correct zero-dep check to exclude devDependencies
- `f7d6a57` - chore: update test-flakiness-detector submodule (CI fix)

### Added - Phase 2 Wave 2: snapshot-comparison Migration Complete ✅ (2025-12-29)

**First Wave 2 tool with git URL dependency migrated to standalone repository!**
- Repository: https://github.com/tuulbelt/snapshot-comparison
- Extracted git history: 12 commits processed
- **Updated dependency**: Changed from `path = "../output-diffing-utility"` to `git = "https://github.com/tuulbelt/output-diffing-utility"`
- Tagged v0.1.0 and pushed to GitHub with koficodedat credentials
- Verified standalone: 96/96 tests passing (33 lib + 59 integration + 4 doc tests)
- **Git URL dependency verified**: Cargo automatically fetched output-diffing-utility from GitHub
- Added as git submodule: `tools/snapshot-comparison`
- **Wave 2 Progress**: 1/3 complete (33%)

**GitHub Repository Configuration:**
- Issues disabled (centralized to meta repo)
- Wiki disabled
- Projects disabled
- Topics: tuulbelt, rust, zero-dependencies, snapshot-testing, testing, diff
- Description: "Snapshot testing with integrated diffs - Part of Tuulbelt"

**Key Achievements:**
- First tool to use git URL dependency pattern successfully
- Proves dependency resolution works for standalone tools
- Establishes pattern for remaining Wave 2 migrations
- CI workflow correctly configured (no zero-dep check for Wave 2 tools)

**Next**: test-flakiness-detector migration (TypeScript, requires cli-progress-reporting)

---

### Fixed - Documentation Cleanup & Authentication Pattern Finalized ✅ (2025-12-29)

**Removed real credentials from all documentation:**
- Cleaned 10 files: GH_CLI_AUTH_GUIDE.md, CHANGELOG.md, HANDOFF.md, MIGRATION_REVIEW.md, migrate-tool.md, KNOWN_ISSUES.md, MIGRATION_TO_META_REPO.md, QUALITY_CHECKLIST.md, STATUS.md
- Replaced real usernames with generic placeholders
- Replaced specific file paths with generic examples
- Replaced email addresses with environment variable references

**Finalized authentication pattern for future migrations:**
- **CRITICAL Discovery**: Claude Code Bash commands run in separate shells
- **Solution**: Chain `source scripts/setup-github-auth.sh && gh ...` for every gh command
- **Documentation updated**:
  - `docs/GH_CLI_AUTH_GUIDE.md` - Complete usage guide with chaining pattern
  - `.claude/commands/migrate-tool.md` - Lessons learned section updated with authentication requirements
  - All migration documentation now reflects correct pattern

**Why this matters:**
- Environment variables don't persist between separate Bash tool calls in Claude Code
- Chaining with `&&` keeps environment in same shell session
- Ensures `gh` CLI always uses project credentials from `.env`
- Prevents recurring authentication failures in future migrations

**Impact:**
- All documentation now credential-free and safe for public repository
- Clear, verified pattern for all future GitHub operations
- Wave 2 migrations can proceed without authentication issues

---

### Added - Phase 2 Wave 1: output-diffing-utility (Rust) Migration Complete ✅ (2025-12-29)

**Migrated output-diffing-utility to standalone repository - WAVE 1 COMPLETE!**
- Repository: https://github.com/tuulbelt/output-diffing-utility
- Extracted git history: 56 commits processed
- Tagged v0.1.0 and pushed to GitHub
- Verified standalone: 108/108 tests passing (76 lib + 27 integration + 5 doc tests)
- Added as git submodule: `tools/output-diffing-utility`

**Repository Configuration:**
- Repository already existed (configured previously)
- Description: "Semantic diff for JSON, text, binary files - Part of Tuulbelt"
- Homepage: https://tuulbelt.github.io/tuulbelt/tools/output-diffing-utility/

**Technical Details:**
- CI workflow: Removed monorepo path filters, added zero-dependency verification
- Created tool-specific CLAUDE.md for development guidance
- Build verified: cargo build --release successful
- No README badges to update (tool doesn't use badges)

**🎉 Phase 2 Wave 1 Progress: 7/7 complete (100%) - WAVE 1 COMPLETE!**

This completes all independent tools in Wave 1. Next up: Wave 2 (test-flakiness-detector with optional dependencies).

---

### Added - Phase 2 Wave 1: file-based-semaphore-ts (TypeScript) Migration Complete ✅ (2025-12-29)

**Migrated file-based-semaphore-ts to standalone repository:**
- Repository: https://github.com/tuulbelt/file-based-semaphore-ts
- Extracted git history: 8 commits processed
- Tagged v0.1.0 and pushed to GitHub
- Verified standalone: 160/160 tests passing (42 suites)
- Added as git submodule: `tools/file-based-semaphore-ts`

**Repository Configuration:**
- Issues/Wiki/Projects DISABLED (centralized in meta repo)
- Topics: tuulbelt, typescript, zero-dependencies, semaphore, file-locking, process-synchronization, concurrency, cross-platform (8 total)
- Homepage: https://tuulbelt.github.io/tuulbelt/tools/file-based-semaphore-ts/

**Technical Details:**
- CI workflow: Multi-version matrix (Node 18, 20, 22), zero-dependency verification
- README badges: Updated to point to standalone repository workflows
- Created tool-specific CLAUDE.md for development guidance
- Build verified: npm run build successful, TypeScript compilation clean

**Gap Analysis and Quality Fixes:**
- First-pass review: 3 gaps fixed (temp branch cleanup, GitHub description, commit method docs)
- Second-pass review: Comprehensive verification passed (all 7 steps + post-migration checks)
- Third-pass review: **Gap 4 discovered and fixed** - 6 broken README links (commit `81e4d33`)
  - Fixed 3 dogfooding links (test-flakiness-detector, output-diffing-utility, file-based-semaphore)
  - Fixed 2 related tools links (corrected port-conflict-resolver → test-port-resolver)
  - Fixed StackBlitz demo link (monorepo → standalone repo)

**Wave 1 Progress:** 6/7 complete (86%)

---

### Added - Phase 2 Wave 1: file-based-semaphore (Rust) Migration Complete ✅ (2025-12-29)

**Migrated file-based-semaphore to standalone repository:**
- Repository: https://github.com/tuulbelt/file-based-semaphore
- Extracted git history: 53 commits processed
- Tagged v0.1.0 and pushed to GitHub
- Verified standalone: 95/95 tests passing (33 + 39 + 19 integration tests + 4 doc tests)
- Added as git submodule: `tools/file-based-semaphore`

**Repository Configuration:**
- Issues/Wiki/Projects DISABLED (centralized in meta repo)
- Topics: tuulbelt, rust, zero-dependencies, file-locking, semaphore, cross-platform, process-synchronization, concurrency (8 total)
- Homepage: https://tuulbelt.github.io/tuulbelt/tools/file-based-semaphore/

**Technical Details:**
- CI workflow: Removed monorepo paths, added zero-dependency verification
- README badges: Updated to point to standalone repository workflows
- Created tool-specific CLAUDE.md for development guidance
- Build verified: cargo build --release successful

**Wave 1 Progress:** 5/7 complete (71%)

---

### Added - Phase 2 Wave 1: structured-error-handler Migration Complete ✅ (2025-12-29)

**Migrated structured-error-handler to standalone repository:**
- Repository: https://github.com/tuulbelt/structured-error-handler
- Extracted git history: 33 commits processed
- Tagged v0.1.0 and pushed to GitHub
- Verified standalone: 88/88 tests passing on Node 18, 20, 22
- Added as git submodule: `tools/structured-error-handler`

**Repository Configuration:**
- Issues/Wiki/Projects DISABLED (centralized in meta repo)
- Topics: tuulbelt, typescript, zero-dependencies, error-handling, serialization, logging, context-preservation, nodejs (8 total)
- Homepage: https://tuulbelt.github.io/tuulbelt/tools/structured-error-handler/

**Technical Details:**
- CI workflow: Multi-version matrix (Node 18, 20, 22) with zero-dependency verification
- README badges: Updated to point to standalone repository
- Created tool-specific CLAUDE.md for development guidance

**Wave 1 Progress:** 4/7 complete (57%)

---

### Added - Phase 2 Wave 1: config-file-merger Migration Complete ✅ (2025-12-29)

**Migrated config-file-merger to standalone repository:**
- Repository: https://github.com/tuulbelt/config-file-merger
- Extracted git history: 469 commits processed
- Tagged v0.1.0 and pushed to GitHub
- Verified standalone: 144/144 tests passing on Node 18, 20, 22
- Added as git submodule: `tools/config-file-merger`

**Repository Configuration:**
- Issues/Wiki/Projects DISABLED (centralized in meta repo)
- Topics: tuulbelt, typescript, zero-dependencies, config, cli, env-vars, configuration, config-management (8 total)
- Homepage: https://tuulbelt.github.io/tuulbelt/tools/config-file-merger/

**Node 18 Compatibility Fix:**
- Fixed `import.meta.dirname` usage (undefined in Node 18)
- Replaced with `dirname(fileURLToPath(import.meta.url))`
- All 144 tests now pass on Node 18, 20, and 22

**Wave 1 Progress:** 3/7 complete (42%)

---

### Added - Authentication Workflow Improvements ✅ (2025-12-29)

**Implemented direnv support for automatic credential loading:**
- Created `.envrc` for auto-loading credentials on `cd` to project
- Exports GH_TOKEN, GITHUB_TOKEN, GITHUB_ORG, GIT_AUTHOR_*, GIT_COMMITTER_*
- Works like `.npmrc` for project-specific configuration

**Enhanced scripts/setup-github-auth.sh:**
- Now exports `GH_TOKEN` (takes precedence over gh CLI keyring cache)
- Matches `.envrc` behavior for consistency

**Impact:**
- Solves recurring "wrong username" authentication issues
- Prevents gh CLI from using cached global credentials
- Project-specific credentials always used (from .env)

---

### Added - Migration Learnings Documentation ✅ (2025-12-29)

**Enhanced `.claude/commands/migrate-tool.md`:**
- Added explicit GitHub configuration steps
- Added comprehensive "Lessons Learned" from first 2 migrations
- Documented authentication challenges and solutions
- Total: +77 lines of critical guidance

**Added Migration Checklist to `docs/QUALITY_CHECKLIST.md`:**
- Complete 7-step checklist with 100+ verification items
- Covers git history, GitHub config, standalone prep, verification, submodules, tracking docs
- Total: +107 lines of systematic verification

**Added Lessons Learned to `docs/MIGRATION_TO_META_REPO.md`:**
- Critical Success Factors, Authentication Challenges, Testing Patterns
- Session Continuity Strategy, What Worked vs. Needs Improvement
- Total: +181 lines of strategic guidance

**Impact:**
- Prevents gaps in future migrations
- Session continuity across tool migrations
- Automated verification of migration completeness

---

### Added - Phase 2 Wave 1: cli-progress-reporting Migration Complete ✅ (2025-12-29)

**First Tool Migrated to Standalone Repository:**
- Created GitHub repository: https://github.com/tuulbelt/cli-progress-reporting
- Extracted 58 commits with full git history
- Tagged v0.1.0, verified standalone: 121/121 tests passing

**Authentication Infrastructure:**
- Created scripts/setup-github-auth.sh, commit.sh, push.sh
- All scripts source credentials from meta repo .env

**Git Submodule Integration:**
- Added as git submodule: tools/cli-progress-reporting

**Impact:**
- First tool FULLY migrated (1/7 Wave 1 complete)
- Established migration workflow for remaining tools

---

### Added - Phase 2 Wave 1: cross-platform-path-normalizer Migration Complete ✅ (2025-12-29)

**Second Tool Migrated to Standalone Repository:**
- Created GitHub repository: https://github.com/tuulbelt/cross-platform-path-normalizer
- Extracted 457 commits with full git history
- Tagged v0.1.0, verified standalone: 141/141 tests passing

**GitHub Repository Configuration:**
- Disabled issues, wiki, projects
- Added 8 topics including tuulbelt, typescript, zero-dependencies

**Git Submodule Integration:**
- Added as git submodule: tools/cross-platform-path-normalizer

**Impact:**
- Second tool FULLY migrated (2/7 Wave 1 complete - 28%)
- Migration automation validated and working

---

## Historical Entries

**Phase 0 and Phase 1 entries have been archived.**

To view historical changelog entries:
```bash
# View all changelog history
git log -p CHANGELOG.md | less

# Find specific entries
git log --grep="Phase 1" --oneline
git show <commit-hash>:CHANGELOG.md
```

---

**Last Updated:** 2025-12-29
**Next Pruning:** After 5-10 more migrations or when file exceeds 200 lines
