# Next Tasks

**Last Updated:** 2026-01-04

---

## 🏷️ Property Validator v0.7.5 - COMPLETE & TAGGED ✅

**Status:** v0.7.5 TAGGED and pushed to origin
**Tag:** `v0.7.5`
**Language:** TypeScript
**Short Name:** `propval`

### v0.7.5 Final Summary

All 6 optimization phases addressed (5 implemented, 1 rejected):

| Phase | Status | Actual Impact |
|-------|--------|---------------|
| Phase 1: Skip empty refinement loop | ✅ COMPLETE | +8-20% |
| Phase 2: Eliminate Fast API Result allocation | ✅ COMPLETE | +12-22% |
| Phase 3: Inline primitive validation | ❌ REJECTED | -24% union regression |
| Phase 4: Lazy path building | ✅ COMPLETE | +24-30% |
| Phase 5: Shared primitive validator functions | ✅ COMPLETE | No runtime benefit |
| Phase 6: Inline validateWithPath for plain objects | ✅ COMPLETE | **+214% (+3.1x)** |

### v0.7.5 vs Competition

**vs Zod: 6/6 categories won** ✅

**vs Valibot:**
| Category | propval | valibot | Winner |
|----------|---------|---------|--------|
| Simple objects | 120 ns | 207 ns | **propval 1.7x** ✅ |
| Unions | 107 ns | 450 ns | **propval 4.5x** ✅ |
| Primitives | 180 ns | 101 ns | valibot 1.8x |
| Complex nested | 2.5 µs | 1.05 µs | valibot 2.4x |
| Primitive arrays | 1.1 µs | 296 ns | valibot 3.8x |

**Score: 2 wins, 3 losses (Valibot-tier performance)**

---

## 🏆 Property Validator v0.8.0 - JIT Bypass Pattern ✅ COMPLETE

**Status:** v0.8.0 COMPLETE - Now the baseline for future optimizations
**Achievement:** 6 wins, 1 near-tie vs Valibot (was 2 wins, 3 losses in v0.7.5)

### v0.8.0 vs Valibot Final Results

| Category | propval | valibot | Winner |
|----------|---------|---------|--------|
| Primitives (string) | 62 ns | 80 ns | **propval 1.30x** ✅ |
| Primitives (number) | 59 ns | 85 ns | **propval 1.44x** ✅ |
| Simple Object | 62 ns | 215 ns | **propval 3.46x** ✅ |
| Complex Nested | 62 ns | 1.08 µs | **propval 17.5x** ✅ |
| Union (3 types) | 88 ns | 83 ns | valibot 1.05x (near-tie) |

### v0.8.0 Implementation Summary

- **Phase 8:** JIT object validator bypass via `_compiled` property
- **Phase 9:** JIT array validator bypass
- **Phase 10:** Recursive JIT bypass for nested objects
- **Phase 11:** JIT bypass for unions, primitives, and literals

**Key Insight:** JIT compilation already existed; the bottleneck was wrapper overhead.

---

## 🎯 Property Validator v0.8.5 - TypeBox-Level Performance 📋 NEXT

**Status:** Roadmap complete, ready for implementation
**Goal:** Achieve TypeBox-level performance (~16M ops/sec) while maintaining Zod-like DX
**Roadmap:** `/tmp/property-validator/docs/V0_8_5_PERFORMANCE_ROADMAP.md`

### v0.8.5 Target APIs

| API | Current (v0.8.0) | v0.8.5 Target | Description |
|-----|------------------|---------------|-------------|
| `v.validate()` | ~17M ops/sec | Maintain | Full error details |
| `v.check()` | N/A | 12-15M ops/sec | Boolean-only, no errors |
| `v.compile()` | Partial | 15-18M ops/sec | Pre-compiled for hot paths |

### v0.8.5 Phases

1. **Phase 1: v.check() API** - Boolean-only fast path
   - Skip Result allocation entirely
   - Use `_compiled` directly
   - Target: 12-15M ops/sec

2. **Phase 2: Inlined Primitive JIT** - Use `new Function()`
   - V8 optimizes standalone functions better
   - No closure scope overhead
   - Target: +30-50% on primitives

3. **Phase 3: Fully Inlined Object Validation** - Single-function JIT
   - Generate `return typeof data.name === 'string' && ...`
   - Zero function call overhead
   - Target: +50-100% on objects

4. **Phase 4: v.compile() API** - Explicit compilation
   - `const checker = v.compile(schema)`
   - Maximum speed for hot paths
   - Target: 15-18M ops/sec (TypeBox territory)

### Union Status in v0.8.5

Unions already win 3/4 scenarios vs valibot after v0.8.0 Phase 11.
The new APIs (`v.check()`, `v.compile()`) will naturally benefit unions.
Dedicated union optimization can be added as Phase 5 if needed.

---

## 🚀 Unified Workflow Implementation - COMPLETE ✅

**Status:** All 6 Phases Complete - Web Testing Verified
**Archived:** `docs/archive/2026-01-01-workflow-complete/`

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

All 11 implemented tools have short CLI names:

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
| Property Validator | `propval` | `property-validator` |

**Proposed for next tools:**
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

### Phase 2: Completed (6/28)

✅ Structured Error Handler (TypeScript)
✅ Configuration File Merger (TypeScript)
✅ Snapshot Comparison (Rust)
✅ File-Based Semaphore (TS) (TypeScript)
✅ Test Port Resolver (TypeScript)
✅ Property Validator (TypeScript) - v0.7.5, 537 tests, Valibot-tier performance

### Phase 2: Next Up

**Recommended Next:**
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
