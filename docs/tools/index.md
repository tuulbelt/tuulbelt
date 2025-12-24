# Available Tools

Tuulbelt provides a curated collection of focused, zero-dependency tools for modern software development.

## Completed Tools (2/33)

### Test Flakiness Detector

**Status:** ✅ Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 107 tests across 4 categories

Detect unreliable tests by running them multiple times and tracking failure rates.

**Features:**
- Framework agnostic - works with any test command
- Comprehensive JSON reports with failure rates
- CLI and library API
- Zero runtime dependencies

[View Documentation →](/tools/test-flakiness-detector/)

---

### CLI Progress Reporting

**Status:** ✅ Production Ready (v0.1.0)
**Language:** TypeScript
**Tests:** 93 tests, dogfooding validated

Concurrent-safe progress tracking with file-based atomic writes.

**Features:**
- File-based atomic writes for concurrent safety
- Multiple independent progress trackers
- CLI and library API
- State persistence across processes

[View Documentation →](/tools/cli-progress-reporting/)

---

## Coming Soon

### Phase 1: Quick Tools (3 remaining)

- 🎯 **Cross-Platform Path Handling** (Next)
- ⚪ File-Based Semaphore
- ⚪ Output Diffing Utility

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

✅ **Single Problem** — Solves one specific problem
✅ **Zero Dependencies** — Uses standard library only
✅ **Portable Interface** — CLI, files, sockets
✅ **Composable** — Works via pipes and file I/O
✅ **Independently Cloneable** — Standalone repository
✅ **Proven Implementation** — Production-tested and reliable

[Read Full Principles →](/guide/principles)
