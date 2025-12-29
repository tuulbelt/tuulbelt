# Next Tasks

**Last Updated:** 2025-12-29

---

## 🚨 CRITICAL: Meta Repository Migration

**Status:** Phase 2 Wave 2 IN PROGRESS (1/3) 🎯
**Priority:** HIGHEST - Architectural Correction

### Phase 2 Wave 1: Independent Tools Migration ✅ COMPLETE

**Completed Tools (7/7):**
- ✅ **cli-progress-reporting** - https://github.com/tuulbelt/cli-progress-reporting
  - 58 commits, v0.1.0, 121/121 tests passing, CI green
- ✅ **cross-platform-path-normalizer** - https://github.com/tuulbelt/cross-platform-path-normalizer
  - 457 commits, v0.1.0, 141/141 tests passing, CI green
- ✅ **config-file-merger** - https://github.com/tuulbelt/config-file-merger
  - 469 commits, v0.1.0, 144/144 tests passing (Node 18 fix applied), CI green
- ✅ **structured-error-handler** - https://github.com/tuulbelt/structured-error-handler
  - 33 commits, v0.1.0, 88/88 tests passing, CI green
- ✅ **file-based-semaphore** - https://github.com/tuulbelt/file-based-semaphore
  - 53 commits, v0.1.0, 95/95 tests passing, CI green
- ✅ **file-based-semaphore-ts** - https://github.com/tuulbelt/file-based-semaphore-ts
  - 8 commits, v0.1.0, 160/160 tests passing, CI green
- ✅ **output-diffing-utility** - https://github.com/tuulbelt/output-diffing-utility
  - 56 commits, v0.1.0, 108/108 tests passing, CI green

### Phase 2 Wave 2: Required Dependencies Migration

**Migration Order:** Lightest → Heaviest for incremental learning

**Completed Tools (1/3):**
1. ✅ **snapshot-comparison** - https://github.com/tuulbelt/snapshot-comparison
   - 12 commits, v0.1.0, 96/96 tests passing, CI green
   - Dependency: output-diffing-utility (git URL working)

**Remaining Tools (2/3):**
1. [ ] **test-flakiness-detector** (TypeScript) - requires: cli-progress-reporting 🎯 NEXT
   - **NEEDS IMPLEMENTATION REVIEW**: Make cli-progress dependency REQUIRED
   - Review: implementation, tests, spec, architecture
   - Update: integration and documentation to enforce required dependency

2. [ ] **test-port-resolver** (TypeScript) - requires: file-based-semaphore-ts
   - **NEEDS COMPREHENSIVE REVIEW**: Full audit against /new-tool standards
   - Verify: implementation, testing, code quality, security
   - Verify: expanded testing (unit, CLI, integration, performance)
   - Verify: documentation (GitHub + VitePress complete)
   - Verify: demos (asciinema + StackBlitz)
   - Verify: templates and scaffolding adherence
   - Most complex - saved for last

**CRITICAL: GitHub Authentication Pattern** ⚠️
```bash
# EVERY gh command must be chained with source in SAME command
source scripts/setup-github-auth.sh && gh repo create tuulbelt/test-flakiness-detector --public
source scripts/setup-github-auth.sh && gh repo edit tuulbelt/test-flakiness-detector --add-topic typescript
```
**Why**: Claude Code runs each Bash command in separate shell - env vars don't persist

**Next Steps:**
1. Migrate test-flakiness-detector (TypeScript) - review to make cli-progress dependency required 🎯
2. Migrate test-port-resolver - comprehensive /new-tool standards audit
3. Update tracking documents after each migration

**Key Resources:**
- **Authentication guide**: `docs/GH_CLI_AUTH_GUIDE.md` ⭐ NEW - Complete chaining pattern
- Migration command: `.claude/commands/migrate-tool.md`
- Quality checklist: `docs/QUALITY_CHECKLIST.md` (100+ item migration checklist)
- Migration plan: `docs/MIGRATION_TO_META_REPO.md`

---

## 🎯 Short CLI Names Reference

All 10 implemented tools have short CLI names:

| Tool | Short Name | Long Name |
|------|------------|-----------|
| Test Flakiness Detector | `flaky` | `test-flakiness-detector` |
| CLI Progress Reporting | `prog` | `cli-progress-reporting` |
| Cross-Platform Path Normalizer | `normpath` | `cross-platform-path-normalizer` |
| File-Based Semaphore (Rust) | `sema` | `file-semaphore` |
| Output Diffing Utility | `odiff` | `output-diff` |
| Structured Error Handler | `serr` | `structured-error-handler` |
| Configuration File Merger | `cfgmerge` | `config-file-merger` |
| Snapshot Comparison | `snapcmp` | `snapshot-comparison` |
| File-Based Semaphore (TS) | `semats` | `file-semaphore-ts` |
| Test Port Resolver | `portres` | `test-port-resolver` |

**Proposed for next tools:**
- Component Prop Validator → `propval`
- Exhaustiveness Checker → `excheck`
- Content-Addressable Blob Store → `blobstore`

---

## 🚀 New Tools (Priority Order)

### Phase 1: Quick Tools ✅ COMPLETE (5/5)

✅ Test Flakiness Detector (TypeScript)
✅ CLI Progress Reporting (TypeScript)
✅ Cross-Platform Path Normalizer (TypeScript)
✅ File-Based Semaphore (Rust)
✅ Output Diffing Utility (Rust)

### Phase 2: Completed (5/28)

✅ Structured Error Handler (TypeScript)
✅ Configuration File Merger (TypeScript)
✅ Snapshot Comparison (Rust)
✅ File-Based Semaphore (TS) (TypeScript)
✅ Test Port Resolver (TypeScript)

### Phase 2: Next Up

**Recommended Next (after migration):**
- **Component Prop Validator** (`propval`) - TypeScript runtime validation
- **Exhaustiveness Checker** (`excheck`) - Union case coverage for TS/JS
- **Content-Addressable Blob Store** (`blobstore`) - SHA-256 hash-based storage

---

## 📚 Documentation

### Pending Updates

- [ ] Consider adding "Contributing" guide page to VitePress
- [ ] Add troubleshooting sections to tool docs as issues arise
- [ ] Add more visual examples (screenshots, diagrams) over time

---

## ⚙️ Infrastructure

### Active Maintenance

**Documentation Trimming (NEW):**
- Run `/trim-docs` every 5-10 migrations
- Keep HANDOFF.md <150 lines, NEXT_TASKS.md <250 lines, CHANGELOG.md <200 lines
- Reference: `docs/DOCUMENTATION_MAINTENANCE.md`

**Quality Checks:**
- `/quality-check` before every commit
- Includes: build, tests, zero-deps, documentation size check

---

**Last Review:** 2025-12-29
**Next Review:** After structured-error-handler migration (tool 4/7)
