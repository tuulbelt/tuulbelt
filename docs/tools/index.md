# Available Tools

Tuulbelt provides a curated collection of focused, zero-dependency tools for modern software development.

## Completed Tools (4/33)

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

## Coming Soon

### Phase 1: Quick Tools (1 remaining)

- <img src="/icons/target.svg" class="inline-icon" alt=""> **Output Diffing Utility** (Next)

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
