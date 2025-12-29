# Tuulbelt Meta Repository Changelog

All notable changes to the Tuulbelt meta repository will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Retention Policy:** Keep only last 60 days of entries. Older entries are in git history (`git log -p CHANGELOG.md`).

---

## [Unreleased]

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
- Prevents gh CLI from using cached kofirc credentials
- Project-specific credentials always used (koficodedat)

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
