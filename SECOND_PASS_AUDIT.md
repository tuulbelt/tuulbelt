# Second-Pass Documentation Audit

**Date:** 2025-12-31
**Scope:** Comprehensive review of all VitePress documentation pages
**Finding:** First pass only fixed `index.md` files - other pages still have critical issues

---

## Executive Summary

**Status:** 🔴 **CRITICAL ISSUES FOUND**

The first documentation alignment pass (commit 372111c) only fixed the main `index.md` files (10 files). It did NOT fix:
- getting-started.md pages
- library-usage.md pages
- cli-usage.md pages
- examples.md pages
- api-reference.md pages (partially fixed in commit caf0899)

**Total Issues Found:**
- ✅ **15 files** with monorepo clone commands
- ✅ **1 file** with monorepo StackBlitz link
- ✅ **4 files** with wrong API structure (test-flakiness-detector)

---

## Issue 1: Monorepo Clone Commands (15 files)

### Problem

VitePress documentation pages still instruct users to clone the meta repository instead of standalone tool repositories.

### Affected Files

```bash
# test-flakiness-detector (3 files)
docs/tools/test-flakiness-detector/getting-started.md:10
docs/tools/test-flakiness-detector/index.md:37
docs/tools/test-flakiness-detector/library-usage.md:11

# cli-progress-reporting (2 files)
docs/tools/cli-progress-reporting/getting-started.md:10
docs/tools/cli-progress-reporting/library-usage.md:9

# cross-platform-path-normalizer (1 file)
docs/tools/cross-platform-path-normalizer/library-usage.md:9

# structured-error-handler (1 file)
docs/tools/structured-error-handler/getting-started.md:10

# config-file-merger (1 file)
docs/tools/config-file-merger/getting-started.md:10

# file-based-semaphore (2 files)
docs/tools/file-based-semaphore/getting-started.md:10
docs/tools/file-based-semaphore/library-usage.md:9

# output-diffing-utility (1 file)
docs/tools/output-diffing-utility/getting-started.md:10

# snapshot-comparison (2 files)
docs/tools/snapshot-comparison/getting-started.md:10
docs/tools/snapshot-comparison/library-usage.md:9

# file-based-semaphore-ts (1 file)
docs/tools/file-based-semaphore-ts/getting-started.md:10

# port-resolver (0 files) ✅ CLEAN
```

### Current (WRONG):
```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/<tool-name>
```

### Should Be:
```bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
```

### Impact

**CRITICAL** - Users following VitePress documentation will:
1. Clone 33-tool meta repository instead of single tool
2. Navigate to wrong directory structure
3. Get confused about installation instructions
4. Waste bandwidth downloading unnecessary tools

---

## Issue 2: Monorepo StackBlitz Link (1 file)

### Problem

StackBlitz "Try it online" link points to monorepo structure instead of standalone repository.

### Affected Files

```bash
docs/tools/test-flakiness-detector/index.md:79
```

### Current (WRONG):
```html
<a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/test-flakiness-detector">
```

### Should Be:
```html
<a href="https://stackblitz.com/github/tuulbelt/test-flakiness-detector">
```

### Impact

**MEDIUM** - Users clicking "Try it online" will:
1. Load entire meta repository in StackBlitz (slow)
2. See wrong directory structure
3. Potentially get build errors if StackBlitz expects standalone repo

---

## Issue 3: Wrong API Structure - test-flakiness-detector (4 files)

### Problem

Multiple VitePress pages document an API that **does not exist** in the source code. This was partially fixed in `api-reference.md` (commit caf0899) but **THREE other pages still have the wrong API**.

### Affected Files

```bash
docs/tools/test-flakiness-detector/cli-usage.md (lines 88-119, 162-165)
docs/tools/test-flakiness-detector/library-usage.md (lines 27, 36-71, 127-130, 153-167)
docs/tools/test-flakiness-detector/examples.md (lines 80, 105, 127-130, 198, 223-250)
docs/tools/test-flakiness-detector/api-reference.md ✅ FIXED (commit caf0899)
```

### Wrong API (VitePress Documentation)

```typescript
// ❌ These interfaces DO NOT EXIST in source code
interface DetectionOptions {
  testCommand: string
  runs?: number
  verbose?: boolean
}

interface DetectionResult {
  summary: {              // ❌ No nested summary object
    isFlaky: boolean      // ❌ Field doesn't exist
    failureRate: number   // ❌ Field doesn't exist
    totalRuns: number
    passedRuns: number
    failedRuns: number
  }
  runs: Array<{
    runNumber: number     // ❌ Field doesn't exist
    duration: number      // ❌ Field doesn't exist
    timestamp: string     // ❌ Field doesn't exist
    success: boolean
    exitCode: number
    stdout?: string
    stderr?: string
  }>
}
```

### Correct API (Actual Source Code)

```typescript
// ✅ From tools/test-flakiness-detector/src/index.ts

export interface Config {
  runs?: number;
  testCommand: string;
  verbose?: boolean;
}

export interface FlakinessReport {
  success: boolean;           // ✅ Missing from VitePress
  totalRuns: number;
  passedRuns: number;
  failedRuns: number;
  flakyTests: TestFlakiness[]; // ✅ Missing entirely
  runs: TestRunResult[];
  error?: string;              // ✅ Missing entirely
}

export interface TestFlakiness {
  testName: string;
  passed: number;
  failed: number;
  totalRuns: number;
  failureRate: number;
}

export interface TestRunResult {
  success: boolean;
  exitCode: number;
  stdout: string;
  stderr: string;
}
```

### Specific Issues

**Missing Fields:**
- ❌ `success` boolean on FlakinessReport
- ❌ `flakyTests` array (core functionality!)
- ❌ `error` string for error handling

**Non-Existent Fields:**
- ❌ `summary` nested object
- ❌ `isFlaky` boolean
- ❌ `failureRate` on summary (exists on TestFlakiness, not summary)
- ❌ `runNumber` on runs
- ❌ `duration` on runs
- ❌ `timestamp` on runs

**Wrong Interface Names:**
- ❌ `DetectionOptions` → Should be `Config`
- ❌ `DetectionResult` → Should be `FlakinessReport`

### Impact

**CRITICAL** - Developers using VitePress documentation will:
1. Write code that **FAILS TO COMPILE**
2. Try to access fields that don't exist (`result.summary.isFlaky`)
3. Miss critical fields (`result.flakyTests`, `result.success`, `result.error`)
4. Get TypeScript errors on every example
5. Lose trust in documentation accuracy

### Examples of Broken Code in VitePress

**cli-usage.md line 93-98** (BROKEN):
```typescript
{
  "summary": {              // ❌ Doesn't exist
    "isFlaky": true,        // ❌ Doesn't exist
    "failureRate": 20       // ❌ Doesn't exist
  }
}
```

**library-usage.md line 27** (BROKEN):
```typescript
if (result.summary.isFlaky) {  // ❌ TypeError: summary doesn't exist
```

**examples.md line 127-130** (BROKEN):
```typescript
if (result.summary.isFlaky) {  // ❌ TypeError
  console.error(`Failure rate: ${result.summary.failureRate}%`)  // ❌ TypeError
}
```

---

## Issue 4: Other Tools API Alignment

### Status

**INCOMPLETE** - Second pass audit focused on test-flakiness-detector due to known critical issues.

### Next Steps

Should verify API alignment for remaining 9 tools:
- cli-progress-reporting (sampled - appears correct)
- cross-platform-path-normalizer
- structured-error-handler
- config-file-merger
- file-based-semaphore
- output-diffing-utility
- snapshot-comparison
- file-based-semaphore-ts
- port-resolver

**Recommendation:** Perform systematic API verification for each tool's library-usage.md and examples.md against actual source code exports.

---

## Root Cause Analysis

### Why First Pass Missed These Issues

1. **Scope Limited to index.md:**
   First pass script (`scripts/align-vitepress-docs.sh`) only targeted `docs/tools/*/index.md` files

2. **Pattern Incomplete:**
   Script fixed repository links but didn't check:
   - StackBlitz links in index.md
   - Clone commands in getting-started.md
   - Clone commands in library-usage.md
   - API examples in multiple pages

3. **API Issue Pre-Existing:**
   test-flakiness-detector API documentation was wrong BEFORE migration, not caused by migration

---

## Recommended Fix Strategy

### Phase 1: Fix Clone Commands (15 files)

**Automated fix:**
```bash
# For each affected file
sed -i 's|git clone https://github.com/tuulbelt/tuulbelt.git|git clone https://github.com/tuulbelt/TOOLNAME.git|g' FILE
sed -i 's|cd tuulbelt/TOOLNAME|cd TOOLNAME|g' FILE
```

**Files to fix:**
- 9 tools × getting-started.md (9 files)
- 5 tools × library-usage.md (5 files)
- 1 tool × index.md (1 file)

### Phase 2: Fix StackBlitz Link (1 file)

**Manual fix:**
```bash
docs/tools/test-flakiness-detector/index.md:79
```

Change:
```html
<a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/test-flakiness-detector">
```

To:
```html
<a href="https://stackblitz.com/github/tuulbelt/test-flakiness-detector">
```

### Phase 3: Fix test-flakiness-detector API (3 files)

**Files to rewrite:**
- cli-usage.md (output format examples)
- library-usage.md (all code examples)
- examples.md (all code examples, JSON examples)

**Approach:**
1. Replace all instances of `DetectionOptions` with `Config`
2. Replace all instances of `DetectionResult` with `FlakinessReport`
3. Remove `summary` nesting - flatten to top level
4. Remove non-existent fields: `isFlaky`, `failureRate` (on summary), `runNumber`, `duration`, `timestamp`
5. Add missing fields: `success`, `flakyTests`, `error`
6. Update all code examples to match actual API

### Phase 4: Verification

1. **Build test:** `npm run docs:build` (must pass)
2. **TypeScript compile:** Verify examples would compile
3. **Visual check:** Preview docs with `npm run docs:preview`
4. **Cross-reference:** Compare each fixed page with GitHub README

---

## Testing Checklist

Before committing fixes:

- [ ] All 15 clone command references updated
- [ ] StackBlitz link fixed
- [ ] test-flakiness-detector API fixed in all 3 remaining files
- [ ] VitePress build passes
- [ ] No dead links
- [ ] Examples match actual source code
- [ ] Git status clean

---

## Statistics

**First Pass (commit 372111c):**
- Files fixed: 10 (index.md only)
- Issues remaining: 16 files

**Second Pass (this audit):**
- Files to fix: 19 total
  - 15 clone command issues
  - 1 StackBlitz issue
  - 3 API structure issues (test-flakiness-detector)

**Coverage:**
- First pass coverage: 38% (10/26 affected files)
- Remaining work: 62% (16/26 files)

---

## References

- First pass commit: `372111c` - "docs: align VitePress with standalone repo URLs"
- API fix commit: `caf0899` - "fix(docs): rewrite test-flakiness-detector API reference"
- First pass audit: `DOCUMENTATION_ALIGNMENT_AUDIT.md`
- API audit: `API_AUDIT_COMPARISON.md`
- Source code: `tools/test-flakiness-detector/src/index.ts` (lines 18-75)

---

**End of Second-Pass Audit**
