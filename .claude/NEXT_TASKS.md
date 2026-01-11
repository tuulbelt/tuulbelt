# Next Tasks

**Last Updated:** 2026-01-11

---

## 🎯 Property Validator v0.11.0 - Test Data Generation (Proposed)

**Status:** Proposed - Pending Phase 3 validation completion
**Language:** TypeScript
**Short Name:** `propval`

### v0.11.0 Proposed Features

| Feature | Status | Description |
|---------|--------|-------------|
| Test data generation from schemas | 📋 Proposed | Generate random valid data from propval schemas |
| CLI support | 📋 Proposed | `propval generate --schema user.ts --count 10` |
| fast-check integration | 📋 Proposed | `toGenerator(schema)` → fast-check arbitrary |

### Rationale

Discovered during Phase 3 validation research:
- Typia does this with compile-time transformers (complex setup)
- Effect Schema does schema → arbitrary (well-integrated)
- Zod has fragmented third-party solutions
- propval already has `toJsonSchema()` which captures constraints
- Straightforward mapping: `v.string().email()` → `fc.emailAddress()`

### API Design (Draft)

```typescript
// Library API
import { toGenerator } from 'property-validator/generate';
import * as fc from 'fast-check';

const userArb = toGenerator(UserSchema);
fc.assert(fc.property(userArb, (user) => {
  // user is guaranteed to match UserSchema
}));

// CLI
propval generate --schema ./user.schema.ts --count 10 --format json
```

### Open Questions

- **Naming:** `toGenerator`, `toArbitrary`, `generateFrom`, `toTestData`?
- **CLI integration:** Separate command or flag on existing `propval` CLI?
- **fast-check as peer dep:** Required or optional?

---

## 🏷️ Property Validator v0.10.0 - COMPLETE ✅

**Status:** v0.10.0 Ready for Tag & Merge
**Language:** TypeScript
**Short Name:** `propval`
**Tests:** 898 passing

### v0.10.0 Features

| Feature | Status |
|---------|--------|
| Full modularization (index.ts: 3744→149 lines) | ✅ |
| JSON Schema Export (`toJsonSchema()`) | ✅ |
| `record()` validator | ✅ |
| `discriminatedUnion()` validator | ✅ |
| `strict()` / `passthrough()` object modes | ✅ |
| Extended string validators (cuid, ulid, nanoid, base64, hex, jwt) | ✅ |
| Extended number validators (port, latitude, longitude, percentage) | ✅ |
| Array JIT for objects optimization | ✅ |

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

## 🏆 Property Validator v0.8.5 - check() and compileCheck() APIs ✅ COMPLETE

**Status:** v0.8.5 COMPLETE - PRs created for merge
**PRs:** property-validator #9, tuulbelt #95

### v0.8.5 What Was Delivered

**New APIs:**
- `check(schema, data)` — Boolean-only validation (~10-18% faster)
- `compileCheck(schema)` — Pre-compiled boolean validator (+5-15% on unions)

**Benchmark Restructuring:**
- Internal: `benchmarks/internal/` (API tier comparison)
- External: `benchmarks/external/` (competitor comparison with API equivalence)

### v0.8.5 Performance Results

| Scenario | validate() | check() | compileCheck() |
|----------|------------|---------|----------------|
| Simple Object | ~62 ns | ~57 ns | ~57 ns |
| Complex Nested | ~154 ns | ~145 ns | ~143 ns |
| Union (3 types) | ~74 ns | ~62 ns | ~55 ns |
| Invalid Data | ~357 ns | ~55 ns | ~55 ns |

**Key Insight:** Invalid data is **6.4x faster** with check/compileCheck (skip error path)

**vs Competitors:**
- vs Zod: 3.4-25.6x faster across categories
- vs Valibot: 1.3-7x faster across categories

---

## 🏆 Property Validator v0.9.0-0.9.3 - COMPLETE ✅

**Status:** v0.9.3 COMPLETE - Ready for PR & Tag
**Tags:** v0.9.2 (current), v0.9.3 (pending)

### v0.9.0-0.9.2: Modularization & Tree-Shaking ✅

- ✅ Named exports for tree-shaking
- ✅ Functional refinement API
- ✅ Entry points (`/v`, `/lite`)
- ✅ TypeBox comparison benchmarks

### v0.9.3: Benchmark CI Infrastructure ✅

- ✅ GitHub Action for automatic regression detection on PRs
- ✅ Alert/notification system (Slack via `TUULBELT_SLACK_WEBHOOK`)
- ✅ Trend tracking across versions (historical baselines)
- ✅ Multi-Node matrix (18, 20, 22)
- ✅ PR comments with results summary
- ✅ Templates updated (TypeScript + Rust)

---

## ✅ Property Validator v0.9.5 → v0.10.0 - Extended Validators COMPLETE

**Status:** All features merged into v0.10.0

Extended validators originally planned for v0.9.5 are now part of v0.10.0:
- ✅ String: `cuid()`, `cuid2()`, `ulid()`, `nanoid()`, `base64()`, `hex()`, `jwt()`
- ✅ Number: `port()`, `latitude()`, `longitude()`, `percentage()`

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
✅ Property Validator (TypeScript) - v0.9.3, 595 tests, Benchmark CI

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

**Last Review:** 2026-01-05
**Next Review:** After v0.9.5 or next tool implementation
