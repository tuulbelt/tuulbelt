---
layout: home

hero:
  name: Tuulbelt
  text: Zero-Dependency Tools
  tagline: Focused, zero-dependency tools and utilities for modern software development
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: View Tools
      link: /tools/
    - theme: alt
      text: GitHub
      link: https://github.com/tuulbelt/tuulbelt

features:
  - icon:
      src: /icons/target.svg
    title: Single Problem Per Tool
    details: Each tool solves one specific problem and solves it well. No framework bloat, no feature creep.

  - icon:
      src: /icons/zap.svg
    title: Zero External Dependencies
    details: Tools use only standard library features. No dependency management, no supply chain vulnerabilities.

  - icon:
      src: /icons/plug.svg
    title: Portable Interface
    details: CLI, files, sockets—not proprietary APIs. Works across languages, shells, and systems.

  - icon:
      src: /icons/link.svg
    title: Composable
    details: Tools chain together via pipes, environment variables, and file I/O. Unix philosophy applied.

  - icon:
      src: /icons/package.svg
    title: Independently Cloneable
    details: Each tool is a standalone repository. Clone what you need, nothing more.

  - icon:
      src: /icons/check-circle.svg
    title: Proven Implementation
    details: No moonshots, no "works 80%" solutions. Every tool is production-tested and reliable.
---

## Available Tools

### Featured Tools

<div class="tool-card">

#### <img src="/icons/sparkles.svg" class="inline-icon" alt=""> TypeScript: [Test Flakiness Detector](/tools/test-flakiness-detector/)

Identify unreliable tests by running them multiple times and tracking failure rates.

- <img src="/icons/target.svg" class="inline-icon" alt=""> Framework agnostic - works with any test command
- <img src="/icons/bar-chart.svg" class="inline-icon" alt=""> Comprehensive JSON reports with failure statistics
- <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero runtime dependencies

[Get Started →](/tools/test-flakiness-detector/getting-started)

</div>

<div class="tool-card">

#### <img src="/icons/tool.svg" class="inline-icon" alt=""> Rust: [File-Based Semaphore](/tools/file-based-semaphore/)

Cross-platform file-based semaphore for process coordination with stale lock detection.

- <img src="/icons/lock.svg" class="inline-icon" alt=""> Atomic locking with O_CREAT | O_EXCL
- <img src="/icons/clock.svg" class="inline-icon" alt=""> Stale lock detection and recovery
- <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero runtime dependencies

[Get Started →](/tools/file-based-semaphore/getting-started)

</div>

[**View All 6 Tools →**](/tools/)

### More Tools

<div class="tool-card">

#### [CLI Progress Reporting](/tools/cli-progress-reporting/)

Concurrent-safe progress tracking with file-based atomic writes.

- <img src="/icons/lock.svg" class="inline-icon" alt=""> Concurrent-safe operations
- <img src="/icons/database.svg" class="inline-icon" alt=""> State persistence across processes

[Get Started →](/tools/cli-progress-reporting/getting-started)

</div>

<div class="tool-card">

#### [Cross-Platform Path Normalizer](/tools/cross-platform-path-normalizer/)

Convert Windows/Unix paths with zero dependencies.

- <img src="/icons/link.svg" class="inline-icon" alt=""> Bidirectional Windows ↔ Unix conversion
- <img src="/icons/layers.svg" class="inline-icon" alt=""> UNC path and mixed separator support

[Get Started →](/tools/cross-platform-path-normalizer/getting-started)

</div>

<div class="tool-card">

#### [Output Diffing Utility](/tools/output-diffing-utility/)

Semantic diff for JSON, text, and binary files with zero dependencies.

- <img src="/icons/code.svg" class="inline-icon" alt=""> Smart file type detection
- <img src="/icons/file-diff.svg" class="inline-icon" alt=""> Multiple output formats (unified, JSON, summary)
- <img src="/icons/layers.svg" class="inline-icon" alt=""> Structural JSON comparison

[Get Started →](/tools/output-diffing-utility/getting-started)

</div>

<div class="tool-card">

#### [Structured Error Handler](/tools/structured-error-handler/)

Structured error format with context preservation and serialization.

- <img src="/icons/layers.svg" class="inline-icon" alt=""> Context chain preservation through call stacks
- <img src="/icons/code.svg" class="inline-icon" alt=""> Full JSON serialization/deserialization
- <img src="/icons/search.svg" class="inline-icon" alt=""> Error codes and categories for routing

[Get Started →](/tools/structured-error-handler/getting-started)

</div>

## Progress

**Phase 1: Quick Tools** — 5 of 5 complete (100%) ✅

- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Test Flakiness Detector (TypeScript)
- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> CLI Progress Reporting (TypeScript)
- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Cross-Platform Path Normalizer (TypeScript)
- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> File-Based Semaphore (Rust)
- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Output Diffing Utility (Rust)

**Phase 2: Started** — 1 of 28

- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Structured Error Handler (TypeScript) ✨ NEW

**Overall Progress:** 6 of 33 tools (18%)

## Quick Start

### TypeScript Tools

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt

# Try Test Flakiness Detector
cd test-flakiness-detector
npm install
npx tsx src/index.ts --test "npm test" --runs 10
```

### Rust Tools

```bash
# Try File-Based Semaphore
cd file-based-semaphore
cargo build --release

# Acquire a lock
./target/release/file-semaphore try /tmp/my.lock

# Check status
./target/release/file-semaphore status /tmp/my.lock

# Release
./target/release/file-semaphore release /tmp/my.lock
```

> See each tool's documentation for full usage guides and API references.

## Philosophy

Tuulbelt tools follow strict principles:

1. **Single Problem Per Tool** — Narrow, well-defined scope
2. **Zero External Dependencies** — Standard library only
3. **Portable Interface** — CLI, files, sockets; not proprietary APIs
4. **Composable** — Works via pipes, environment variables, file I/O
5. **Independently Cloneable** — Each tool is a standalone repository
6. **Proven Implementation** — No moonshots, no "works 80%" solutions

[Read Full Principles →](/guide/principles)

<style>
.tool-card {
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  padding: 1.5rem;
  margin: 1rem 0;
  background: var(--vp-c-bg-soft);
}

.tool-card h4 {
  margin-top: 0;
}

.tool-card ul {
  margin: 1rem 0;
}
</style>
