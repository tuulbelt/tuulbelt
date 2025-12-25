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

### Testing & Observability

<div class="tool-card">

#### [Test Flakiness Detector](/tools/test-flakiness-detector/)

Identify unreliable tests by running them multiple times and tracking failure rates.

- <img src="/icons/target.svg" class="inline-icon" alt=""> Framework agnostic
- <img src="/icons/bar-chart.svg" class="inline-icon" alt=""> Comprehensive JSON reports
- <img src="/icons/rocket.svg" class="inline-icon" alt=""> 148 tests, production-ready
- <img src="/icons/book.svg" class="inline-icon" alt=""> Full documentation with examples

[Get Started →](/tools/test-flakiness-detector/getting-started)

</div>

<div class="tool-card">

#### [CLI Progress Reporting](/tools/cli-progress-reporting/)

Concurrent-safe progress tracking with file-based atomic writes.

- <img src="/icons/lock.svg" class="inline-icon" alt=""> Concurrent-safe operations
- <img src="/icons/hash.svg" class="inline-icon" alt=""> Multiple independent trackers
- <img src="/icons/database.svg" class="inline-icon" alt=""> State persistence
- <img src="/icons/beaker.svg" class="inline-icon" alt=""> Dogfooding validated

[Get Started →](/tools/cli-progress-reporting/getting-started)

</div>

### Utilities & Infrastructure

<div class="tool-card">

#### [Cross-Platform Path Normalizer](/tools/cross-platform-path-normalizer/)

Convert Windows/Unix paths with zero dependencies. Handle UNC paths, mixed separators, and drive letters seamlessly.

- <img src="/icons/link.svg" class="inline-icon" alt=""> Bidirectional conversion
- <img src="/icons/layers.svg" class="inline-icon" alt=""> UNC path support
- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Type safe
- <img src="/icons/beaker.svg" class="inline-icon" alt=""> Dogfooding validated

[Get Started →](/tools/cross-platform-path-normalizer/getting-started)

</div>

## Progress

**Phase 1: Quick Tools** — 3 of 5 complete (60%)

- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Test Flakiness Detector
- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> CLI Progress Reporting
- <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Cross-Platform Path Normalizer
- <img src="/icons/target.svg" class="inline-icon" alt=""> File-Based Semaphore (next)
- <img src="/icons/circle.svg" class="inline-icon" alt=""> Output Diffing Utility

**Overall Progress:** 3 of 33 tools (9%)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt

# Try Test Flakiness Detector
cd test-flakiness-detector
npm install
npx tsx src/index.ts --test "npm test" --runs 10

# Try CLI Progress Reporting
cd ../cli-progress-reporting
npm install
npx tsx src/index.ts init --total 100 --message "Processing files"

# Try Cross-Platform Path Normalizer
cd ../cross-platform-path-normalizer
npm install
npx tsx src/index.ts --format unix "C:\Users\file.txt"
```

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
