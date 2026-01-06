# Tuulbelt Tool Maturity Analysis

**Status:** Research-Backed Gap Analysis
**Reference:** Property Validator (propval) v0.9.3 as Gold Standard
**Date:** 2026-01-05

---

## Executive Summary

This document analyzes all 10 Tuulbelt tools against the Property Validator (propval) gold standard to identify:

1. **Implementation Expansion Opportunities** - Features, APIs, and patterns to adopt
2. **Documentation Expansion Opportunities** - Documentation structure improvements
3. **Benchmarking Opportunities** - Where competitive benchmarks add value

**Key Finding:** Property Validator has evolved through 9+ versions with 595 tests, multi-API design, benchmark CI, and comprehensive documentation. Other tools remain at v0.1.0 with foundational implementations.

---

## Core Philosophy: CLI and Library

**Tuulbelt tools maximize the quality and usefulness of BOTH CLI and library interfaces.**

This is NOT about feature parity — it's about exhausting the potential of each interface:

| Interface | Goal | Approach |
|-----------|------|----------|
| **CLI** | Best possible command-line experience | Optimize for pipes, scripts, human DX |
| **Library** | Best possible programmatic experience | Optimize for type safety, composability, performance |

### Independent Excellence

Each interface should be maximized independently. Some features make sense in one context but not the other:

```
CLI-Specific Opportunities:
├── Interactive modes (prompts, confirmations)
├── Colored/formatted output for humans
├── Progress indicators and spinners
├── Shell completion scripts
├── Streaming output for large results
└── Exit codes for script integration

Library-Specific Opportunities:
├── Complex type inference (generics, conditional types)
├── Builder patterns and fluent APIs
├── Compile-time validation
├── Memory-efficient streaming APIs
├── Callback/async patterns
└── Integration with language ecosystems
```

### Shared Excellence (Both Interfaces)

```
TypeScript Tools:
├── Result types: { ok: true; value } | { ok: false; error }
├── Multi-tier APIs: validate() → check() → compileCheck()
├── Named exports for tree-shaking
├── JSDoc with @example blocks
└── Zero runtime dependencies

Rust Tools:
├── Result<T, E> for all fallible operations
├── Builder pattern for complex configuration
├── Feature flags for optional functionality
├── #[derive] for common traits
├── Zero dependencies (std only)
└── WASM compilation support where applicable
```

### When Expanding Tools

For each new feature, ask:
1. **CLI:** What's the best way to expose this from the command line?
2. **Library:** What's the best API design for programmatic use?
3. **Context:** Does this feature make sense for both, or is it interface-specific?

**Not every feature needs both interfaces.** An interactive wizard makes sense for CLI but not library. Complex generic type inference makes sense for library but not CLI.

---

## Proposed Tool Renames

Based on analysis, three tools should be renamed for clarity:

| Current Name | Issue | New Name | Short Name |
|--------------|-------|----------|------------|
| `cli-progress-reporting` | Sounds CLI-only, conflicts with `cli-progress` npm | **progress-reporter** | `prog` |
| `structured-error-handler` | "handler" implies framework | **structured-error** | `serr` |
| `output-diffing-utility` | "utility" is weak/generic | **output-diff** | `odiff` |

**Migration:** Rename GitHub repos, update all references, preserve git history.

---

## Property Validator Gold Standard Summary

### What Makes Propval the Gold Standard

| Aspect | Implementation | Value |
|--------|----------------|-------|
| **Multi-API Design** | `validate()`, `check()`, `compileCheck()` | Different APIs for different use cases |
| **Entry Points** | `/v` | Named exports for tree-shaking |
| **Version Evolution** | v0.1.0 → v0.9.3 (9 releases) | Continuous improvement pattern |
| **Test Count** | 595 tests | Comprehensive coverage (unit + integration + CLI) |
| **Benchmark CI** | tatami-ng + regression detection | Automatic performance validation |
| **Documentation** | 7+ doc files per standard | README, CLAUDE.md, SPEC.md, CHANGELOG, etc. |
| **Result Type** | `{ ok: true; value } | { ok: false; error }` | Non-throwing, composable |

### Propval Patterns for Replication

```
Implementation Patterns:
├── Multi-API tiers (validate → check → compileCheck)
├── Result type (non-throwing)
├── Named exports for tree-shaking
├── Entry point `/v` for named exports
├── JSDoc with @example blocks
├── CLI entry point with npm link support
└── Performance optimization phases (documented)

Documentation Patterns:
├── README.md (14+ sections)
├── CLAUDE.md (development context)
├── SPEC.md (formal behavior specification)
├── CHANGELOG.md (Keep a Changelog format)
├── DOGFOODING_STRATEGY.md
├── benchmarks/README.md (results + methodology)
└── examples/ (basic.ts, advanced.ts)

CI/CD Patterns:
├── test.yml (tsc → test → build → dogfood)
├── benchmark.yml (PR regression detection)
├── benchmark-update-baseline.yml (version tracking)
├── Multi-Node matrix (18, 20, 22)
└── Slack notifications (org secret)
```

---

## Tool-by-Tool Gap Analysis

### 1. Test Flakiness Detector (`flaky`)

**Current State:** v0.1.0 | 132 tests | Uses cli-progress-reporting

**Competitive Position:** ✅ **STRONG** — No standalone OSS tool exists for flaky test detection

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Multi-API Design** | HIGH | Add `detect()` (full report) + `isFlaky()` (boolean) + `compileDetector()` (pre-compiled) |
| **Streaming Results** | MEDIUM | Emit results as tests run (not just at end) |
| **Machine-Readable Output** | MEDIUM | JSON schema for CI integration |
| **Configurable Thresholds** | LOW | Allow `--threshold 0.1` for custom flakiness detection |

**Proposed API Expansion:**
```typescript
// Current (single API)
flaky --test "npm test" --runs 10

// Expanded (multi-tier)
import { detect, isFlaky, compileDetector } from 'test-flakiness-detector';

// Full detection with detailed report
const report = await detect({ test: 'npm test', runs: 10 });

// Boolean check (faster for CI gates)
const hasFlaky = await isFlaky({ test: 'npm test', runs: 5 });

// Pre-compiled for repeated use
const detector = compileDetector({ test: 'npm test' });
await detector.run(10); // Run 10 times
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ❌ Missing | Define flakiness detection algorithm, output format |
| benchmarks/README.md | ❌ Missing | Not applicable (tool is inherently slow) |
| Advanced Examples | ❌ Missing | CI integration, parallel test suites |

#### Benchmarking Opportunity

**Verdict:** ❌ **NOT RECOMMENDED** — Tool is inherently slow (runs tests N times). No meaningful performance comparison.

---

### 2. CLI Progress Reporting → **Progress Reporter** (`prog`)

**Current State:** v0.1.0 | 121 tests | Foundation tool

**Proposed Rename:** `cli-progress-reporting` → `progress-reporter` (avoids conflict with `cli-progress` npm package)

**Competitive Position:** ⚠️ **MODERATE** — ora (spinners), listr2 (task lists), cli-progress dominate

**Differentiator:** Concurrent-safe, zero dependencies (competitors have deps)

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Multi-API Design** | HIGH | Add `createProgress()` (builder) + `progress.update()` (instance) + `progress.done()` |
| **Concurrent Progress Tracking** | HIGH | Multiple progress bars simultaneously (listr2's strength) |
| **Template System** | MEDIUM | Customizable output format (like ora's spinners) |
| **Streaming API** | LOW | Generator-based progress for async iterators |

**Proposed API Expansion:**
```typescript
// CLI (enhanced)
prog init --total 100 --message "Processing" --format json
prog update 50
prog multi add --id download --total 50 --message "Downloading"
prog multi add --id process --total 100 --message "Processing"
prog multi update download 25
prog multi update process 75

// Library (parallel API)
import { createProgress, MultiProgress } from 'progress-reporter';

// Single progress
const bar = createProgress({ total: 100, message: 'Processing' });
bar.update(50);
bar.done();

// Multiple concurrent (differentiator)
const multi = new MultiProgress();
const bar1 = multi.add({ total: 50, message: 'Downloading' });
const bar2 = multi.add({ total: 100, message: 'Processing' });
bar1.update(25);
bar2.update(75);
multi.done();
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ❌ Missing | Define output format, escape sequences, concurrent model |
| examples/concurrent.ts | ❌ Missing | Multi-progress bar demo |
| examples/streaming.ts | ❌ Missing | Async iterator integration |

#### Benchmarking Opportunity

**Verdict:** ❌ **NOT RECOMMENDED** — Visual output tool, not performance-critical.

---

### 3. Cross-Platform Path Normalizer (`normpath`)

**Current State:** v0.1.0 | 141 tests | Stable, proven

**Competitive Position:** ✅ **GOOD** — normalize-path is 8+ years old, upath adds unwanted features

**Differentiator:** Zero dependencies, modern ES modules, actively maintained

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Format Options** | HIGH | `--format unix|windows|native` for explicit conversion |
| **Batch Processing** | MEDIUM | Process multiple paths efficiently |
| **Validation Mode** | LOW | `--validate` to check path validity without normalizing |

**Proposed API Expansion:**
```typescript
// Current
normpath "C:\Users\file.txt"

// Expanded
import { normalize, toUnix, toWindows, isValid, batch } from 'cross-platform-path-normalizer';

// Explicit format
toUnix('C:\\Users\\file.txt');     // '/c/Users/file.txt'
toWindows('/home/user/file.txt');  // '\\home\\user\\file.txt'

// Validation
isValid('../../../etc/passwd');    // false (path traversal)

// Batch processing
batch(['path1', 'path2', 'path3'], { format: 'unix' });
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ❌ Missing | Define normalization rules, edge cases |
| Security section | ❌ Missing | Path traversal prevention documentation |

#### Benchmarking Opportunity

**Verdict:** ❌ **NOT RECOMMENDED** — String manipulation, trivial performance.

---

### 4. Configuration File Merger (`cfgmerge`)

**Current State:** v0.1.0 | 144 tests | Comprehensive

**Competitive Position:** ⚠️ **WEAK** — cosmiconfig has 64M weekly downloads

**Differentiator:** Zero dependencies, explicit merging (not discovery-based like cosmiconfig)

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Schema Validation** | HIGH | Integrate with propval for type-safe config |
| **Layered Configs** | HIGH | `--layers defaults,user,env,cli` for explicit precedence |
| **Watch Mode** | MEDIUM | `--watch` for hot config reloading |
| **Dry Run** | LOW | `--dry-run` to preview merged result |

**Proposed API Expansion:**
```typescript
// Current
cfgmerge --file config.json --env --prefix APP_

// Expanded
import { merge, MergeConfig, watchConfig } from 'config-file-merger';

// Explicit layer control
const config = await merge({
  layers: [
    { type: 'file', path: 'defaults.json' },
    { type: 'file', path: 'config.json' },
    { type: 'env', prefix: 'APP_' },
    { type: 'args', source: process.argv }
  ],
  schema: UserConfigSchema  // propval integration
});

// Watch mode
const watcher = watchConfig({ path: 'config.json' }, (newConfig) => {
  console.log('Config updated:', newConfig);
});
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ❌ Missing | Define merge precedence, conflict resolution |
| examples/layered.ts | ❌ Missing | Multi-layer configuration demo |
| Integration guide | ❌ Missing | Using with propval for validation |

#### Benchmarking Opportunity

**Verdict:** ❌ **NOT RECOMMENDED** — I/O bound (file reading).

---

### 5. Structured Error Handler → **Structured Error** (`serr`)

**Current State:** v0.1.0 | 88 tests | Focused utility

**Proposed Rename:** `structured-error-handler` → `structured-error` ("handler" implies framework behavior)

**Competitive Position:** ✅ **MODERATE** — VError, Boom are alternatives but have dependencies

**Differentiator:** Zero dependencies, serialization-focused (context preservation)

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Error Chaining** | HIGH | Chain errors with full context (like VError) |
| **HTTP Integration** | MEDIUM | Built-in HTTP error factories |
| **Serialization Formats** | MEDIUM | JSON, YAML, plain text output |
| **Stack Trace Filtering** | LOW | Filter internal frames for cleaner output |

**Proposed API Expansion:**
```bash
# CLI (new capabilities)
serr create "Database connection failed" --context '{"host":"localhost"}'
serr chain "User service unavailable" --cause db-error.json --context '{"userId":123}'
serr http 404 "User not found" --context '{"userId":123}'
serr serialize error.json --format json --no-stack

# Pipe-friendly
echo '{"message":"error"}' | serr chain "Wrapper error" | serr serialize --format text
```

```typescript
// Library (parallel API)
import {
  createError,
  chain,
  httpError,
  serialize,
  ErrorChain
} from 'structured-error';

// Error chaining (VError-like)
const dbError = createError('Database connection failed', { host: 'localhost' });
const serviceError = chain(dbError, 'User service unavailable', { userId: 123 });

// HTTP errors
const notFound = httpError(404, 'User not found', { userId: 123 });

// Serialization
serialize(serviceError, { format: 'json', includeStack: false });
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ❌ Missing | Define error format, serialization schema |
| examples/chaining.ts | ❌ Missing | Error chain demo |
| examples/http.ts | ❌ Missing | HTTP error integration |

#### Benchmarking Opportunity

**Verdict:** ❌ **NOT RECOMMENDED** — Error creation is rare, not hot path.

---

### 6. File-Based Semaphore TypeScript (`semats`)

**Current State:** v0.1.0 | 160 tests | Highest test count

**Competitive Position:** ✅ **STRONG** — proper-lockfile is best but not zero-dependency

**Differentiator:** Zero dependencies, TypeScript-native, network FS considerations

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Async/Await Native** | HIGH | Full Promise-based API (not callbacks) |
| **Resource Pool** | MEDIUM | Multiple resources with semaphore (like database connections) |
| **Distributed Locking** | LOW | Optional Redis/file backend abstraction |

**Proposed API Expansion:**
```typescript
// Current
import { Semaphore } from 'file-based-semaphore-ts';
const sem = new Semaphore('/tmp/lock');
await sem.acquire();
// ... work
await sem.release();

// Expanded
import {
  createSemaphore,
  withLock,
  ResourcePool
} from 'file-based-semaphore-ts';

// RAII pattern (recommended)
await withLock('/tmp/lock', async () => {
  // Lock acquired automatically
  // Released on scope exit (even on error)
});

// Resource pool
const pool = new ResourcePool({ max: 5, lockDir: '/tmp/pool' });
const resource = await pool.acquire();
// Use resource
pool.release(resource);
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | Likely exists (Rust version has it) | Verify TypeScript SPEC |
| examples/raii.ts | ❌ Missing | withLock pattern demo |
| Network FS guide | ❌ Missing | NFS/CIFS considerations |

#### Benchmarking Opportunity

**Verdict:** ⚠️ **OPTIONAL** — Concurrent lock acquisition speed matters for test infrastructure.

---

### 7. Port Resolver (`portres`)

**Current State:** v0.1.0 | 56 tests | Uses file-based-semaphore-ts

**Competitive Position:** ✅ **STRONG** — No tool handles concurrent test port allocation safely

**Differentiator:** Concurrent-safe via semaphore integration

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Batch Allocation** | HIGH | Request N ports atomically |
| **Port Ranges** | MEDIUM | `--range 8000-9000` for specific range |
| **Release Tracking** | MEDIUM | Track which ports are in use by which tests |
| **Health Check** | LOW | Verify port is actually free before returning |

**Proposed API Expansion:**
```typescript
// Current
portres get --tag "api-server"

// Expanded
import {
  getPort,
  getPorts,
  reserveRange,
  PortManager
} from 'port-resolver';

// Single port
const port = await getPort({ tag: 'api-server' });

// Multiple ports (atomic, no race conditions)
const [http, grpc, metrics] = await getPorts(3, { tags: ['http', 'grpc', 'metrics'] });

// Range reservation
const range = await reserveRange({ start: 8000, count: 10 });

// Port manager for test suites
const manager = new PortManager({ baseDir: '/tmp/ports' });
const port1 = await manager.allocate('test-1');
const port2 = await manager.allocate('test-2');
manager.releaseAll();  // Cleanup
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ❌ Missing | Define allocation algorithm, locking strategy |
| examples/parallel-tests.ts | ❌ Missing | Jest/Vitest parallel test demo |
| CI integration guide | ❌ Missing | GitHub Actions parallel job setup |

#### Benchmarking Opportunity

**Verdict:** ⚠️ **OPTIONAL** — Concurrent allocation speed matters for test suite performance.

---

### 8. File-Based Semaphore Rust (`sema`)

**Current State:** v0.1.0 | 95 tests | Criterion benchmarks exist

**Competitive Position:** ✅ **STRONG** — fs2/fs4 use advisory locks, this uses atomic file creation

**Differentiator:** Zero dependencies, atomic mkdir strategy, stale lock detection

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Async Runtime Support** | HIGH | Tokio/async-std compatibility |
| **Counting Semaphore** | MEDIUM | Multiple resources (not just binary lock) |
| **Metrics Export** | LOW | Lock contention statistics |

**Proposed API Expansion:**
```rust
// Current
let sem = Semaphore::new("/tmp/lock")?;
let guard = sem.acquire()?;
// ... work
// guard drops, lock released

// Expanded
use file_based_semaphore::{Semaphore, CountingSemaphore, SemaphoreMetrics};

// Counting semaphore (N resources)
let pool = CountingSemaphore::new("/tmp/pool", 5)?;
let guard1 = pool.acquire()?;  // 4 remaining
let guard2 = pool.acquire()?;  // 3 remaining

// Async support
let guard = sem.acquire_async().await?;

// Metrics
let metrics: SemaphoreMetrics = sem.metrics();
println!("Contention events: {}", metrics.contention_count);
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ✅ Exists | Already comprehensive |
| benchmarks/README.md | ❌ Missing | Document benchmark results |
| Async guide | ❌ Missing | Tokio integration |

#### Benchmarking Opportunity

**Verdict:** ✅ **RECOMMENDED** — Compare with fs2/fs4 for lock acquisition speed.

**Competitors to benchmark against:**
- fs2 (flock-based)
- fs4 (rustix-based)
- advisory-lock

---

### 9. Output Diffing Utility → **Output Diff** Rust (`odiff`)

**Proposed Rename:** `output-diffing-utility` → `output-diff` (matches short name, removes weak "utility")

**Current State:** v0.1.0 | 108 tests | Criterion benchmarks exist

**Competitive Position:** ⚠️ **MODERATE** — similar, imara-diff are established Rust diff libraries

**Differentiator:** Multi-format (text, JSON, binary), zero dependencies, semantic JSON diff

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Streaming Diff** | HIGH | Large file support without loading entirely into memory |
| **Custom Comparators** | MEDIUM | User-defined equality (e.g., ignore whitespace) |
| **Patch Generation** | MEDIUM | Generate patches that can be applied |
| **Histogram Algorithm** | LOW | imara-diff's algorithm is 10-100% faster than Myers |

**Proposed API Expansion:**
```rust
// Current
let result = diff_text(expected, actual, &config)?;

// Expanded
use output_diffing_utility::{
    diff_text, diff_json, diff_binary,
    stream_diff, generate_patch,
    DiffConfig, Algorithm
};

// Streaming for large files
let diff = stream_diff(
    File::open("large1.txt")?,
    File::open("large2.txt")?,
    &config
)?;

// Algorithm selection
let config = DiffConfig {
    algorithm: Algorithm::Histogram,  // Faster than Myers
    ..Default::default()
};

// Patch generation
let patch = generate_patch(&diff, PatchFormat::Unified)?;
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ✅ Exists | Already comprehensive |
| benchmarks/README.md | ❌ Missing | Document benchmark results vs competitors |
| Algorithm comparison | ❌ Missing | Myers vs Patience vs Histogram |

#### Benchmarking Opportunity

**Verdict:** ✅ **CRITICAL** — Performance is a key differentiator.

**Competitors to benchmark against:**
- similar (Rust, used by insta)
- imara-diff (Rust, optimized)
- dissimilar (Rust, Google algorithm)
- jsdiff (Node.js baseline)

---

### 10. Snapshot Comparison Rust (`snapcmp`)

**Current State:** v0.1.0 | 96 tests | Uses output-diffing-utility

**Competitive Position:** ⚠️ **WEAK** — insta (Rust) and Jest (JS) dominate

**Differentiator:** Zero dependencies (via Tuulbelt composition), CLI-first design

#### Implementation Expansion Opportunities

| Opportunity | Priority | Rationale |
|-------------|----------|-----------|
| **Inline Snapshots** | HIGH | Store snapshots in source code (like expect-test) |
| **Multiple Formats** | MEDIUM | YAML, TOML, RON support (like insta) |
| **Update Modes** | MEDIUM | Interactive review workflow |
| **Test Framework Integration** | LOW | Macro for easier test writing |

**Proposed API Expansion:**
```rust
// Current
let store = SnapshotStore::new("/snapshots")?;
let result = store.check("test-name", actual_output)?;

// Expanded
use snapshot_comparison::{
    SnapshotStore, inline_snapshot,
    SnapshotFormat, UpdateMode
};

// Inline snapshots (in source code)
#[test]
fn test_output() {
    let result = my_function();
    inline_snapshot!(result, @r###"
    expected output
    goes here
    "###);
}

// Multiple formats
let store = SnapshotStore::new("/snapshots")?
    .with_format(SnapshotFormat::Yaml);

// Interactive update mode
let result = store.check_interactive("test-name", actual)?;
```

#### Documentation Expansion

| Document | Current | Needed |
|----------|---------|--------|
| SPEC.md | ✅ Exists | Already comprehensive |
| Comparison guide | ❌ Missing | vs insta, vs jest snapshots |
| Migration guide | ❌ Missing | From insta to snapcmp |

#### Benchmarking Opportunity

**Verdict:** ❌ **NOT RECOMMENDED** — Snapshot I/O is not performance-critical.

---

## Consolidated Recommendations

### Implementation Priority (Option A: Competitive Advantage First)

Tools ordered by competitive positioning (strongest market position first):

| Order | Tool | New Name | Competitive Position | Priority |
|-------|------|----------|----------------------|----------|
| 1 | **test-flakiness-detector** | (keep) | ✅ **STRONG** — No OSS competitor | 🔴 FIRST |
| 2 | **port-resolver** | (keep) | ✅ **STRONG** — Concurrent-safe unique | 🔴 FIRST |
| 3 | **file-based-semaphore** | (keep) | ✅ **STRONG** — Zero-dep atomic locks | 🟡 SECOND |
| 4 | **file-based-semaphore-ts** | (keep) | ✅ **STRONG** — TypeScript-native | 🟡 SECOND |
| 5 | **output-diffing-utility** | → **output-diff** | ⚠️ Moderate — Needs benchmarks | 🟡 SECOND |
| 6 | **structured-error-handler** | → **structured-error** | ⚠️ Moderate — Niche | 🟢 THIRD |
| 7 | **cli-progress-reporting** | → **progress-reporter** | ⚠️ Moderate — Crowded | 🟢 THIRD |
| 8 | **cross-platform-path-normalizer** | (keep) | ✅ Good — Modern replacement | 🟢 THIRD |
| 9 | **config-file-merger** | (keep) | ⚠️ Weak — cosmiconfig dominates | 🟢 THIRD |
| 10 | **snapshot-comparison** | (keep) | ⚠️ Weak — insta/jest dominate | 🟢 THIRD |

### Documentation Priority Matrix

| Tool | SPEC.md | Benchmark README | Advanced Examples | Priority |
|------|---------|------------------|-------------------|----------|
| **test-flakiness-detector** | ❌ Need | N/A | ❌ Need | 🔴 HIGH |
| **cli-progress-reporting** | ❌ Need | N/A | ❌ Need | 🔴 HIGH |
| **port-resolver** | ❌ Need | Optional | ❌ Need | 🔴 HIGH |
| **config-file-merger** | ❌ Need | N/A | ❌ Need | 🟡 MEDIUM |
| **structured-error-handler** | ❌ Need | N/A | ❌ Need | 🟡 MEDIUM |
| **file-based-semaphore-ts** | ✅ Verify | Optional | ❌ Need | 🟡 MEDIUM |
| **cross-platform-path-normalizer** | ❌ Need | N/A | ✅ OK | 🟢 LOW |
| **file-based-semaphore** | ✅ Exists | ❌ Need | ✅ OK | 🟢 LOW |
| **output-diffing-utility** | ✅ Exists | ❌ Need | ✅ OK | 🟢 LOW |
| **snapshot-comparison** | ✅ Exists | N/A | ✅ OK | 🟢 LOW |

### Benchmarking Priority Matrix

| Tool | Should Benchmark? | Competitors | Differentiator |
|------|-------------------|-------------|----------------|
| **output-diffing-utility** | ✅ CRITICAL | similar, imara-diff, jsdiff | Multi-format, zero deps |
| **file-based-semaphore** | ✅ RECOMMENDED | fs2, fs4, advisory-lock | Atomic mkdir, stale detection |
| **port-resolver** | ⚠️ OPTIONAL | get-port, detect-port | Concurrent-safe |
| **file-based-semaphore-ts** | ⚠️ OPTIONAL | proper-lockfile | Zero deps, TypeScript-native |
| Others | ❌ NOT RECOMMENDED | N/A | Not performance-critical |

---

## Implementation Roadmap

### Approach: Vertical (One Tool at a Time)

Complete all improvements for each tool before moving to the next:
- CLI enhancements + Library enhancements (equal attention)
- Documentation (SPEC.md, examples)
- Benchmarks (if applicable)
- Release v0.2.0

### Phase 0: Foundation (Horizontal)

**Goal:** Establish standards before tool work begins

- [x] CLI-also philosophy documented
- [x] Tool renames decided
- [ ] Create rename migration plan (GitHub repo renames)
- [ ] Update all cross-references in meta repo

### Phase 1: High Competitive Advantage Tools (v0.2.0)

**Order:** test-flakiness-detector → port-resolver

**Per Tool Deliverables:**
1. **CLI enhancements** — New commands, better output formats
2. **Library enhancements** — Multi-tier APIs, Result types
3. **Documentation** — SPEC.md, advanced examples
4. **Tests** — +50% coverage for new features
5. **Release** — Tag v0.2.0

### Phase 2: Strong Foundation Tools (v0.2.0)

**Order:** file-based-semaphore → file-based-semaphore-ts → output-diff

**Additional:** Benchmark CI for output-diff and file-based-semaphore

### Phase 3: Remaining Tools (v0.2.0)

**Order:** structured-error → progress-reporter → cross-platform-path-normalizer → config-file-merger → snapshot-comparison

### Per-Tool Checklist Template

```
[ ] CLI: Review current commands, identify gaps
[ ] CLI: Implement new commands with --help
[ ] CLI: Add machine-readable output (--format json)
[ ] Library: Add multi-tier API
[ ] Library: Add Result type if not present
[ ] Docs: Create/update SPEC.md
[ ] Docs: Add examples/advanced.ts
[ ] Tests: Add unit tests for new features
[ ] Tests: Verify dogfooding passes
[ ] Benchmarks: Add if applicable (output-diff, semaphores)
[ ] Release: Tag v0.2.0
```

### Documentation Standardization

**Required documents per tool:**
- [ ] SPEC.md (formal behavior specification)
- [ ] CHANGELOG.md (Keep a Changelog format)
- [ ] DOGFOODING_STRATEGY.md
- [ ] examples/basic.ts + examples/advanced.ts
- [ ] 14+ section README.md (propval template)

### Phase 3: Benchmark CI Infrastructure (v0.3.0 for applicable tools)

**Goal:** Automatic performance regression detection

**Applicable tools:**
1. output-diffing-utility (CRITICAL)
2. file-based-semaphore (RECOMMENDED)
3. port-resolver (OPTIONAL)
4. file-based-semaphore-ts (OPTIONAL)

**Infrastructure per tool:**
- [ ] benchmarks/ directory with tatami-ng (TS) or Criterion (Rust)
- [ ] .github/workflows/benchmark.yml (PR regression detection)
- [ ] .github/workflows/benchmark-update-baseline.yml (version tracking)
- [ ] benchmarks/README.md (results + methodology)

### Phase 4: Competitive Benchmarking (v0.4.0)

**Goal:** Document performance vs established alternatives

**Required comparisons:**
1. **output-diffing-utility** vs similar, imara-diff, jsdiff
2. **file-based-semaphore** vs fs2, fs4
3. **propval** (already done) vs Zod, Valibot, TypeBox

---

## Version Target Summary

| Tool | Current | Target v0.2.0 | Target v0.3.0 | Target v1.0.0 |
|------|---------|---------------|---------------|---------------|
| property-validator | v0.9.3 | N/A (already gold) | N/A | v1.0.0 (stable API) |
| test-flakiness-detector | v0.1.0 | Multi-API + SPEC | Docs complete | v1.0.0 |
| cli-progress-reporting | v0.1.0 | Multi-API + concurrent | SPEC + examples | v1.0.0 |
| port-resolver | v0.1.0 | Batch allocation | SPEC + CI guide | v1.0.0 |
| config-file-merger | v0.1.0 | Schema validation | SPEC + examples | v1.0.0 |
| structured-error-handler | v0.1.0 | Error chaining | SPEC + examples | v1.0.0 |
| file-based-semaphore-ts | v0.1.0 | RAII pattern | Benchmark CI | v1.0.0 |
| cross-platform-path-normalizer | v0.1.0 | Format options | SPEC | v1.0.0 |
| file-based-semaphore | v0.1.0 | Async support | Benchmark CI | v1.0.0 |
| output-diffing-utility | v0.1.0 | Streaming diff | Benchmark CI | v1.0.0 |
| snapshot-comparison | v0.1.0 | Inline snapshots | Docs complete | v1.0.0 |

---

## Appendix A: Gold Standard Patterns by Language

### TypeScript Patterns

#### Result Type
```typescript
export type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: Error };
```

#### Multi-API Design
```typescript
// Full validation (detailed errors)
export function validate<T>(data: unknown, schema: Schema<T>): Result<T>;

// Boolean check (faster, no error details)
export function check<T>(data: unknown, schema: Schema<T>): boolean;

// Pre-compiled (fastest for repeated use)
export function compileCheck<T>(schema: Schema<T>): (data: unknown) => boolean;
```

#### CLI Entry Point
```typescript
#!/usr/bin/env -S npx tsx
const argv1 = globalThis.process?.argv?.[1];
if (argv1) {
  const realPath = realpathSync(argv1);
  if (import.meta.url === `file://${realPath}`) {
    main();
  }
}
```

#### Benchmark CI (TypeScript)
```yaml
name: Benchmark
on:
  pull_request:
    paths: ['src/**', 'benchmarks/**']
jobs:
  benchmark:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [18, 20, 22]
    steps:
      - uses: actions/checkout@v4
      - run: cd benchmarks && npm ci
      - run: npm run bench:ci
```

---

### Rust Patterns

#### Result Type
```rust
// Use std::result::Result<T, E> with custom error types
pub enum DiffError {
    IoError(std::io::Error),
    InvalidFormat(String),
    SizeLimitExceeded { actual: usize, limit: usize },
}

pub type DiffResult<T> = Result<T, DiffError>;
```

#### Builder Pattern
```rust
// Complex configuration via builder
pub struct DiffConfig {
    algorithm: Algorithm,
    context_lines: usize,
    ignore_whitespace: bool,
}

impl DiffConfig {
    pub fn builder() -> DiffConfigBuilder {
        DiffConfigBuilder::default()
    }
}

// Usage
let config = DiffConfig::builder()
    .algorithm(Algorithm::Histogram)
    .context_lines(3)
    .build();
```

#### Multi-API Design (Rust)
```rust
// Full diff (detailed output)
pub fn diff(expected: &str, actual: &str, config: &DiffConfig) -> DiffResult<DiffOutput>;

// Boolean check (faster, no output)
pub fn has_changes(expected: &str, actual: &str) -> bool;

// Streaming for large files
pub fn diff_streaming<R: Read>(
    expected: R,
    actual: R,
    config: &DiffConfig
) -> DiffResult<impl Iterator<Item = DiffChunk>>;
```

#### CLI Entry Point (Rust)
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "odiff", version, about = "Semantic diff utility")]
struct Cli {
    #[arg(short, long)]
    format: Option<OutputFormat>,

    #[arg(short, long, default_value = "3")]
    context: usize,

    expected: PathBuf,
    actual: PathBuf,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
```

#### Benchmark CI (Rust)
```yaml
name: Benchmark
on:
  pull_request:
    paths: ['src/**', 'benches/**']
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo bench --bench main -- --save-baseline pr
      - run: cargo bench --bench main -- --baseline main --save-baseline pr
```

#### Feature Flags
```toml
# Cargo.toml
[features]
default = []
async = ["tokio"]
json = []  # Enable JSON diff support
binary = [] # Enable binary diff support

[dependencies]
tokio = { version = "1", optional = true, features = ["fs"] }
```

```rust
// Conditional compilation
#[cfg(feature = "async")]
pub async fn diff_async(/* ... */) -> DiffResult<DiffOutput> {
    // Async implementation
}
```

#### RAII Guards
```rust
// Automatic cleanup via Drop
pub struct LockGuard<'a> {
    semaphore: &'a Semaphore,
    path: PathBuf,
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir(&self.path);
    }
}

// Usage - lock released automatically
let guard = semaphore.acquire()?;
// ... do work ...
// guard dropped here, lock released
```

---

## Appendix B: Competitive Landscape Summary

| Category | Leader | Tuulbelt Opportunity |
|----------|--------|----------------------|
| Schema Validation | Zod (DX), AJV (perf) | ✅ Performance differentiation |
| Test Flakiness | None (SaaS only) | ✅ **STRONG** — No standalone OSS |
| CLI Progress | ora, listr2 | ⚠️ Concurrent-safe differentiator |
| Path Normalization | normalize-path (8yr old) | ✅ Modern, maintained |
| Config Merging | cosmiconfig (64M/week) | ⚠️ Niche (explicit merging) |
| Error Handling | VError, Boom | ✅ Zero-dep differentiator |
| File Locking | proper-lockfile | ✅ Zero-dep, TypeScript-native |
| Diffing | similar, imara-diff | ⚠️ Multi-format differentiator |
| Snapshots | insta (Rust), Jest (JS) | ⚠️ Weak position |
| Port Allocation | get-port, detect-port | ✅ **STRONG** — Concurrent-safe |

---

**Document Version:** 1.0.0
**Last Updated:** 2026-01-05
**Maintained By:** Tuulbelt Core Team
