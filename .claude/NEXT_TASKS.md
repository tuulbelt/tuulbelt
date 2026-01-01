# Next Tasks

**Last Updated:** 2026-01-01

---

## 🚀 Unified Workflow Implementation - COMPLETE ✅

**Status:** All 6 Phases Complete - Web Testing Verified
**Archived:** `docs/archive/2026-01-01-workflow-complete/`

### Progress

| Phase | Description | Status |
|-------|-------------|--------|
| **1** | Branch Protection (Universal) | ✅ Complete |
| **2** | CLI Workspace Commands | ✅ Complete - PR #76 |
| **3** | Environment-Aware Commands | ✅ Complete |
| **4** | Session Lifecycle Hooks | ✅ Complete |
| **5** | Documentation | ✅ Complete |
| **6** | Testing & Validation | ✅ Complete (Web verified 2026-01-01) |

**Web Testing Accomplishments (2026-01-01):**
- ✅ All 6 phases verified in Claude Code Web environment
- ✅ Fixed credential loading for Web (env vars vs .env file)
- ✅ Fixed submodule initialization fallback (direct clone)
- ✅ Fixed color output for non-interactive terminals (6 scripts)
- ✅ Real PR creation/cleanup verified (test-flakiness-detector#1)

**Documentation Archived:** Workflow implementation docs moved to `docs/archive/2026-01-01-workflow-complete/`

**Next:** New Tool Development - Component Prop Validator

---

## 🎯 IMMEDIATE PRIORITY: Component Prop Validator

**Status:** Ready to Start
**Language:** TypeScript
**Short Name:** `propval`
**Priority:** HIGH - Next tool in Phase 2 backlog

### Tool Specification

**Purpose:** Runtime prop validation for React/Vue components with TypeScript-first design

**Key Features:**
- Runtime validation of component props
- TypeScript type inference and integration
- Clear validation error messages
- Zero external dependencies
- Framework-agnostic core (works with React, Vue, Svelte)

**Use Cases:**
- Validate props at component boundaries
- Catch type mismatches in development
- Generate helpful error messages for developers
- Optional production validation with minimal overhead

**Implementation Approach:**
- Use `/new-tool component-prop-validator typescript propval` to scaffold
- Follow quality checklist before commit
- Dogfood with test-flakiness-detector and other tools
- Target: 80%+ test coverage, zero runtime dependencies

---

## 🎉 Meta Repository Migration - COMPLETE!

**Status:** Phase 2 Wave 2 COMPLETE (3/3) ✅✅✅
**Priority:** COMPLETED - Architectural Migration Finished

All 10 monorepo tools have been successfully migrated to standalone repositories!

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

### Phase 2 Wave 2: Required Dependencies Migration ✅ COMPLETE

**Migration Order:** Lightest → Heaviest for incremental learning

**Completed Tools (3/3):**
1. ✅ **snapshot-comparison** - https://github.com/tuulbelt/snapshot-comparison
   - 12 commits, v0.1.0, 96/96 tests passing, CI green
   - Dependency: output-diffing-utility (git URL working)

2. ✅ **test-flakiness-detector** - https://github.com/tuulbelt/test-flakiness-detector
   - 92 commits, v0.1.0, 132/132 tests passing, CI green
   - **REQUIRED dependency**: cli-progress-reporting (git URL working)
   - **Implementation corrected**: Changed from optional dynamic import to required ES module import
   - Demonstrates Tuulbelt-to-Tuulbelt composition (PRINCIPLES.md Exception 2)

3. ✅ **test-port-resolver** - https://github.com/tuulbelt/test-port-resolver
   - 3 commits, v0.1.0, 56/56 tests passing, CI green
   - **REQUIRED dependency**: file-based-semaphore-ts (git URL working)
   - Comprehensive pre-migration review completed
   - All quality standards verified and met

**🎉 Migration Complete - All 10 Tools Now Standalone!**

Each tool is now:
- ✅ Standalone GitHub repository
- ✅ Git submodule in meta repo (`tools/`)
- ✅ Using git URL dependencies for composition
- ✅ Independently cloneable and functional
- ✅ CI configured with zero external dependency checks

**Key Resources (for reference):**
- **Authentication guide**: `docs/GH_CLI_AUTH_GUIDE.md`
- Quality checklist: `docs/QUALITY_CHECKLIST.md`
- Migration history: `docs/MIGRATION_TO_META_REPO.md`
- Archived migration command: `docs/archive/migrate-tool-reference.md`

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

**Last Review:** 2025-12-30
**Next Review:** After Phase B completion
