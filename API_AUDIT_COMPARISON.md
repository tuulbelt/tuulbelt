# API Documentation Audit - GitHub vs VitePress

**Date:** 2025-12-31
**Auditor:** Claude
**Scope:** API alignment between GitHub READMEs (source of truth) and VitePress API reference pages

---

## Executive Summary

**STATUS:** ❌ **CRITICAL ISSUE FOUND IN test-flakiness-detector**

- **9/10 tools:** API documentation ALIGNED ✅
- **1/10 tools:** API documentation MISALIGNED ❌ (test-flakiness-detector)

---

## Detailed Findings

### ✅ ALIGNED TOOLS (9/10)

These tools have VitePress API documentation that correctly matches their source code:

#### 1. cli-progress-reporting ✅
**Source Code Exports:**
- `ProgressState` interface
- `ProgressConfig` interface
- `Result<T>` type
- Functions: `init`, `increment`, `set`, `finish`, `get`, `clear`, `formatProgress`

**VitePress API Reference:**
- Documents all interfaces ✓
- Documents all functions ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

#### 2. cross-platform-path-normalizer ✅
**Source Code Exports:**
- `PathFormat` type
- `NormalizeOptions` interface
- `NormalizeResult` interface
- Functions: `normalizePathdetectPathFormat`, `normalizeToUnix`, `normalizeToWindows`

**VitePress API Reference:**
- Documents all types ✓
- Documents all functions ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

#### 3. file-based-semaphore (Rust) ✅
**Source Code Exports:**
- `Semaphore` struct
- `SemaphoreConfig` struct
- `SemaphoreGuard` struct
- `LockInfo` struct
- `SemaphoreError` enum
- Methods: `new`, `with_defaults`, `is_locked`, `lock_info`, `try_acquire`, `acquire`, etc.

**VitePress API Reference:**
- Documents all structs ✓
- Documents all methods ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

### ❌ MISALIGNED TOOL (1/10)

#### test-flakiness-detector ❌ **CRITICAL MISMATCH**

**ACTUAL Source Code (src/index.ts lines 18-75):**

```typescript
export interface Config {
  runs?: number;
  testCommand: string;
  verbose?: boolean;
}

export interface TestRunResult {
  success: boolean;
  exitCode: number;
  stdout: string;
  stderr: string;
}

export interface TestFlakiness {
  testName: string;
  passed: number;
  failed: number;
  totalRuns: number;
  failureRate: number;
}

export interface FlakinessReport {
  success: boolean;
  totalRuns: number;
  passedRuns: number;
  failedRuns: number;
  flakyTests: TestFlakiness[];
  runs: TestRunResult[];
  error?: string;
}
```

**GitHub README (Source of Truth) - Lines 74-99:**

```typescript
import { detectFlakiness } from './src/index.js';

const report = await detectFlakiness({
  testCommand: 'npm test',
  runs: 10,
  verbose: false
});

if (report.success) {
  console.log(`Total runs: ${report.totalRuns}`);
  console.log(`Passed: ${report.passedRuns}, Failed: ${report.failedRuns}`);

  if (report.flakyTests.length > 0) {
    console.log('\nFlaky tests detected:');
    report.flakyTests.forEach(test => {
      console.log(`  ${test.testName}: ${test.failureRate.toFixed(1)}% failure rate`);
    });
  }
} else {
  console.error(`Error: ${report.error}`);
}
```

**VitePress API Reference (WRONG) - Lines 1-146:**

Documents COMPLETELY DIFFERENT interfaces that DON'T EXIST in source code:

```typescript
// ❌ WRONG - doesn't exist in source code
interface DetectionOptions {
  testCommand: string
  runs?: number
  verbose?: boolean
}

// ❌ WRONG - doesn't exist in source code
interface DetectionResult {
  summary: Summary
  runs: RunResult[]
}

// ❌ WRONG - doesn't exist in source code
interface Summary {
  totalRuns: number
  passedRuns: number
  failedRuns: number
  isFlaky: boolean
  failureRate: number
}

// ❌ WRONG - doesn't exist in source code
interface RunResult {
  runNumber: number
  success: boolean
  exitCode: number
  duration: number
  timestamp: string
  stdout?: string
  stderr?: string
}
```

**Problems:**

1. **Wrong interface names:**
   - VitePress docs `DetectionOptions` → Should be `Config`
   - VitePress docs `DetectionResult` → Should be `FlakinessReport`
   - VitePress docs `RunResult` → Should be `TestRunResult`

2. **Wrong structure:**
   - VitePress has nested `summary` object → Should be flat fields on `FlakinessReport`
   - VitePress missing `flakyTests` array → Critical field for identifying flaky tests
   - VitePress missing `success` boolean → Critical field for error handling
   - VitePress missing `error?` string → Critical field for error messages

3. **Extra fields that don't exist:**
   - VitePress has `isFlaky` field → Doesn't exist (user calculates from passedRuns/failedRuns)
   - VitePress has `failureRate` in Summary → Doesn't exist
   - VitePress has `runNumber`, `duration`, `timestamp` in RunResult → Don't exist in TestRunResult

4. **Missing interface:**
   - VitePress doesn't document `TestFlakiness` interface → Critical for `flakyTests` array

**Impact:**

- **CRITICAL:** Developers using VitePress API reference will write code that DOESN'T WORK
- **CRITICAL:** API structure is completely incompatible with actual code
- **HIGH:** Examples in VitePress show non-existent fields
- **HIGH:** Type imports will fail (interfaces don't exist)

**Root Cause:**

VitePress API documentation was written for a DIFFERENT/PLANNED API that was never implemented, or the implementation was changed but VitePress docs weren't updated.

---

#### 4. structured-error-handler ✅
**Source Code Exports:**
- `StructuredError` class with static methods (`wrap`, `from`, `fromJSON`)
- `ErrorContext` interface
- `SerializedError` interface
- `StructuredErrorOptions` interface
- Functions: `serializeError`, `deserializeError`, `formatError`, etc.

**VitePress API Reference:**
- Documents all types and classes ✓
- Documents all methods and functions ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

#### 5. config-file-merger ✅
**Source Code Exports:**
- `ConfigSource` type
- `ConfigValue` interface
- `TrackedConfig`, `SimpleConfig` types
- `MergeOptions`, `MergeResult`, `MergeError` interfaces
- Functions: `mergeConfig`, `getValue`, `parseJsonFile`, `parseEnv`, `parseCliArgs`, etc.

**VitePress API Reference:**
- Documents all types ✓
- Documents all functions ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

#### 6. output-diffing-utility (Rust) ✅
**Source Code Exports:**
- `DiffConfig` struct
- `OutputFormat`, `FileType` enums
- `TextDiffResult`, `BinaryDiffResult` structs
- `DiffError`, `LineChange`, `JsonChange` enums
- Functions: `diff_text`, `diff_json`, `diff_binary`, `detect_file_type`, etc.

**VitePress API Reference:**
- Documents all structs and enums ✓
- Documents all functions ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

#### 7. snapshot-comparison (Rust) ✅
**Source Code Exports:**
- `SnapshotStore` struct
- `SnapshotConfig`, `Snapshot`, `SnapshotMetadata` structs
- `SnapshotError`, `CompareResult`, `DiffOutput` enums
- Methods: `new`, `create`, `read`, `check`, `update`, `delete`, `exists`

**VitePress API Reference:**
- Documents all structs and enums ✓
- Documents all methods ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

#### 8. file-based-semaphore-ts ✅
**Source Code Exports:**
- `Semaphore` class
- `SemaphoreConfig`, `LockInfo` interfaces
- `SemaphoreError` interface, `SemaphoreErrorType` type
- `SemaphoreResult<T>` type
- Functions: `createLockInfo`, `parseLockInfo`, `isLockStale`, `isProcessRunning`

**VitePress API Reference:**
- Documents all types and classes ✓
- Documents all methods and functions ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

#### 9. port-resolver ✅
**Source Code Exports:**
- `PortResolver` class
- `PortConfig`, `PortEntry`, `PortRegistry`, `PortAllocation`, `RegistryStatus` interfaces
- `Result<T, E>` type
- Function: `isPortAvailable`

**VitePress API Reference:**
- Documents all types and classes ✓
- Documents all methods and functions ✓
- Signatures match source code ✓

**Status:** CORRECT - No action needed

---

## Alignment Plan

### Phase 1: Fix test-flakiness-detector (CRITICAL)

**Strategy:** Rewrite VitePress API reference to match actual source code

**Tasks:**
1. Replace `DetectionOptions` with `Config`
2. Replace `DetectionResult` with `FlakinessReport`
3. Replace `RunResult` with `TestRunResult`
4. Add `TestFlakiness` interface documentation
5. Remove nested `Summary` structure
6. Update all examples to use correct API
7. Remove non-existent fields (`isFlaky`, `failureRate`, `runNumber`, `duration`, `timestamp`)
8. Add missing fields (`success`, `error`, `flakyTests`)

**Testing:**
- Compare every example in VitePress with GitHub README
- Verify all type definitions exist in src/index.ts
- Test code snippets actually compile

### Phase 2: Establish Verification Process (Future Enhancement)

Create automated checks to prevent future drift:
- Script to extract exported types from source code
- Script to verify VitePress documents all exports
- CI check that fails if API docs drift from source

---

## Next Steps

1. ✅ **Complete audit of all 10 tools** - DONE
2. **Fix test-flakiness-detector VitePress API documentation** - Execute Phase 1
3. **Test updated documentation** - Verify examples compile
4. **Commit and push fixes** - Single commit with alignment changes

---

## Summary

**✅ AUDIT COMPLETE - 10/10 tools audited**

- **9 tools:** API documentation correctly aligned with source code ✓
- **1 tool:** test-flakiness-detector requires complete API reference rewrite ❌

**Recommendation:** Proceed with Phase 1 to fix test-flakiness-detector VitePress API documentation by rewriting it to match the actual source code (FlakinessReport, Config, TestRunResult, TestFlakiness interfaces).

---

**Status:** AUDIT COMPLETE - Ready to execute alignment plan

