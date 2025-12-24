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
  - icon: 🎯
    title: Single Problem Per Tool
    details: Each tool solves one specific problem and solves it well. No framework bloat, no feature creep.

  - icon: ⚡
    title: Zero External Dependencies
    details: Tools use only standard library features. No dependency management, no supply chain vulnerabilities.

  - icon: 🔌
    title: Portable Interface
    details: CLI, files, sockets—not proprietary APIs. Works across languages, shells, and systems.

  - icon: 🔗
    title: Composable
    details: Tools chain together via pipes, environment variables, and file I/O. Unix philosophy applied.

  - icon: 📦
    title: Independently Cloneable
    details: Each tool is a standalone repository. Clone what you need, nothing more.

  - icon: ✅
    title: Proven Implementation
    details: No moonshots, no "works 80%" solutions. Every tool is production-tested and reliable.
---

## Available Tools

### Testing & Observability

<div class="tool-card">

#### [Test Flakiness Detector](/tools/test-flakiness-detector/)

Identify unreliable tests by running them multiple times and tracking failure rates.

- 🎯 Framework agnostic
- 📊 Comprehensive JSON reports
- 🚀 107+ tests, production-ready
- 📖 Full documentation with examples

[Get Started →](/tools/test-flakiness-detector/getting-started)

</div>

<div class="tool-card">

#### [CLI Progress Reporting](/tools/cli-progress-reporting/)

Concurrent-safe progress tracking with file-based atomic writes.

- 🔒 Concurrent-safe operations
- 🆔 Multiple independent trackers
- 💾 State persistence
- 🧪 Dogfooding validated

[Get Started →](/tools/cli-progress-reporting/getting-started)

</div>

## Progress

**Phase 1: Quick Tools** — 2 of 5 complete (40%)

- ✅ Test Flakiness Detector
- ✅ CLI Progress Reporting
- 🎯 Cross-Platform Path Handling (next)
- ⚪ File-Based Semaphore
- ⚪ Output Diffing Utility

**Overall Progress:** 2 of 33 tools (6%)

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
