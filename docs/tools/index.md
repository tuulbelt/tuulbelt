# Available Tools

Tuulbelt provides a curated collection of focused, zero-dependency tools for modern software development.

## Completed Tools (8/33)

### Test Flakiness Detector

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 111 tests, dogfooding validated

Detect unreliable tests by running them multiple times and tracking failure rates.

**Features:**
- Framework agnostic - works with any test command
- Comprehensive JSON reports with failure rates
- CLI and library API
- Zero runtime dependencies
- Real-time progress tracking (uses CLI Progress Reporting)

[View Documentation →](/tools/test-flakiness-detector/)

---

### CLI Progress Reporting

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 111 tests, dogfooding validated

Concurrent-safe progress tracking with file-based atomic writes.

**Features:**
- File-based atomic writes for concurrent safety
- Multiple independent progress trackers
- CLI and library API
- State persistence across processes
- Validates test suite reliability (validated by Test Flakiness Detector)

[View Documentation →](/tools/cli-progress-reporting/)

---

### Cross-Platform Path Normalizer

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 128 tests, dogfooding validated

Convert Windows/Unix paths with zero dependencies. Handle UNC paths, mixed separators, and drive letters seamlessly.

**Features:**
- Bidirectional conversion (Windows ↔ Unix)
- UNC path support (\\server\share ↔ //server/share)
- Mixed separator handling
- Auto-format detection
- CLI and library API
- Zero runtime dependencies

[View Documentation →](/tools/cross-platform-path-normalizer/)

---

### File-Based Semaphore

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** Rust
**Tests:** 85 tests

Cross-platform file-based semaphore for process coordination. Lock resources reliably across processes with zero dependencies.

**Features:**
- Atomic locking with O_CREAT | O_EXCL
- Stale lock detection with configurable timeout
- RAII-style guards for automatic release
- Both CLI and library interfaces
- Cross-platform (Linux, macOS, Windows)
- Zero runtime dependencies

[View Documentation →](/tools/file-based-semaphore/)

---

### Output Diffing Utility

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** Rust
**Tests:** 99 tests

Semantic diff for JSON, text, and binary files. Compare structured data, configuration files, or binary blobs with smart file type detection.

**Features:**
- Smart file type detection (JSON, text, binary)
- Multiple output formats (unified, JSON, summary)
- Structural JSON comparison with path-based change reporting
- File size safety limits (configurable)
- Both CLI and library interfaces
- Zero runtime dependencies

[View Documentation →](/tools/output-diffing-utility/)

---

### Structured Error Handler

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 81 tests

Structured error format with context preservation and serialization. Chain context through call stacks and serialize errors for logging and debugging.

**Features:**
- Context chaining through call stacks
- Full serialization/deserialization support
- Cause chain analysis (getRootCause, getCauseChain)
- Programmatic error routing (hasCode, hasCategory)
- Human-readable and JSON output formats
- CLI and library API
- Zero runtime dependencies

[View Documentation →](/tools/structured-error-handler/)

---

### Config File Merger

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 135 tests

Merge configuration from ENV variables, config files, and CLI arguments with clear precedence rules and source tracking.

**Features:**
- Clear precedence: CLI > ENV > File > Defaults
- Source tracking for configuration provenance
- Automatic type coercion (strings to booleans, numbers, null)
- Environment variable prefix filtering
- CLI and library API
- Zero runtime dependencies

[View Documentation →](/tools/config-file-merger/)

---

### Snapshot Comparison

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)
**Language:** Rust
**Tests:** 42 tests

Snapshot testing utility for regression detection with integrated diff output. Compare current output against stored "golden" snapshots.

**Features:**
- Integrated with output-diffing-utility for semantic diffs
- Text, JSON, and binary snapshot support
- Hash-based fast comparison with detailed diff on mismatch
- Update mode for development workflows
- Orphan cleanup for snapshot management
- CLI and library API
- Zero runtime dependencies (uses odiff as path dependency)

[View Documentation →](/tools/snapshot-comparison/)

---

## Coming Soon

### Phase 1: Quick Tools — Complete! ✨

All 5 Phase 1 tools have been implemented.

### All Planned Tools

See the [full roadmap](https://github.com/tuulbelt/tuulbelt/blob/main/ROADMAP.md) for the complete list of 33 planned tools across these categories:

- **CLI/DevTools** (4 tools)
- **Testing & Observability** (8 tools)
- **Frontend** (2 tools)
- **Data & Protocol** (4 tools)
- **APIs & Integration** (4 tools)
- **Security & Networking** (4 tools)
- **Utilities & Infrastructure** (6 tools)
- **Observability** (2 tools)
- **Interoperability** (1 tool)

## Tool Selection Criteria

Each Tuulbelt tool must meet these requirements:

<img src="/icons/check-circle.svg" class="inline-icon" alt=""> **Single Problem** — Solves one specific problem

<img src="/icons/check-circle.svg" class="inline-icon" alt=""> **Zero Dependencies** — Uses standard library only

<img src="/icons/check-circle.svg" class="inline-icon" alt=""> **Portable Interface** — CLI, files, sockets

<img src="/icons/check-circle.svg" class="inline-icon" alt=""> **Composable** — Works via pipes and file I/O

<img src="/icons/check-circle.svg" class="inline-icon" alt=""> **Independently Cloneable** — Standalone repository

<img src="/icons/check-circle.svg" class="inline-icon" alt=""> **Proven Implementation** — Production-tested and reliable

[Read Full Principles →](/guide/principles)
