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

## 🎯 Property Validator v0.8.0 - JIT Compilation 📋 NEXT

**Status:** Research complete, planning done, ready for implementation
**Goal:** Close performance gaps with Valibot on primitives, complex objects, and arrays
**Target:** Match or beat Valibot in 4/6 categories (currently 2/6)

### Competitor Landscape (Research Complete)

| Library | Ops/sec | Approach | DX Trade-off |
|---------|---------|----------|--------------|
| **Typia** | 9.6M | AOT (TypeScript transformer) | Requires build step |
| **TypeBox** | 16.5M | JIT (`new Function()`) | JSON Schema only |
| **ArkType** | ~10M | JIT (`new Function()`) | Different API |
| **Valibot** | 4.1M | Closure-based | Good DX, modular |
| **Zod** | 2.0M | Closure-based | Best DX, slowest |
| **property-validator** | ~5M | Closure-based | Zod-like DX |

### v0.8.0 Phases

1. **Phase 7: JIT Primitive Validators** 🔥 HIGHEST
   - Expected: +50-100% on primitives (close 1.8x gap with Valibot)
   - Use `new Function()` instead of closures
   - V8 can better optimize JIT-generated code

2. **Phase 8: JIT Object Validators** 🔥 HIGH
   - Expected: +30-50% on complex objects (close 2.4x gap)
   - Generate custom validation code per schema shape
   - Direct property access (no dynamic lookup)

3. **Phase 9: JIT Array Validators** MEDIUM
   - Expected: +20-40% on primitive arrays (close 3.8x gap)
   - Loop unrolling for small arrays
   - Specialized code generation

### v0.8.0 Research Tasks (Before Implementation)

- [ ] Profile current primitive validators with `node --prof`
- [ ] Benchmark `new Function()` vs closure in isolation
- [ ] Study TypeBox's TypeCompiler source code
- [ ] Study ArkType's shift-reduce parser approach
- [ ] Test JIT approach in browsers with CSP
- [ ] Measure memory impact of JIT code strings

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
