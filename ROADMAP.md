# Tuulbelt Development Roadmap

**Last Updated:** 2025-12-26
**Total Tools Planned:** 33
**Completed:** 5 (15%)
**Current Phase:** Phase 2 - High-Impact Medium Tools

---

## Development Philosophy

Tools are built in priority order based on:
1. **Complexity** (Quick → Medium)
2. **Utility** (High-impact tools first)
3. **Dependencies** (Foundational tools before dependent ones)

See `docs/setup/TUULBELT_TRIAGE.md` for detailed complexity analysis.

---

## Phase 1: Infrastructure & Quick Tools ✅ COMPLETE

**Goal:** Establish development infrastructure and build highest-impact quick tools

**Timeline:** Completed 2025-12-26
**Status:** Infrastructure ✅ Complete | Quick Tools ✅ Complete (5/5)

### Infrastructure (Complete ✅)

- [x] Meta repository structure
- [x] TypeScript template with tests
- [x] Rust template with tests
- [x] Claude Code automation (commands, agents, skills, hooks)
- [x] Documentation system
- [x] Session handoff system (STATUS.md, CHANGELOG.md, ROADMAP.md)

### Quick Tools (5 tools, days each)

#### ✅ Test Flakiness Detector (COMPLETE)

**Priority:** HIGH - Critical for testing infrastructure
**Language:** TypeScript
**Complexity:** Quick (2-3 days)
**Status:** ✅ Complete v0.1.0 (2025-12-23)

**Description:** Run tests N times, identify unreliable ones. Proven approach.

**Completed Features:**
- ✅ Runs test command N times (configurable)
- ✅ Tracks pass/fail per execution
- ✅ Calculates failure rate statistics
- ✅ Outputs structured JSON report
- ✅ 107+ tests with 80%+ coverage
- ✅ Full VitePress documentation site
- ✅ GitHub Pages deployment
- ✅ Automated demo examples

**Location:** `/test-flakiness-detector/`
**Docs:** https://tuulbelt.github.io/tuulbelt/

---

#### ✅ CLI Progress/Status Reporting (COMPLETE)

**Priority:** MEDIUM
**Language:** TypeScript
**Complexity:** Quick (2-3 days)
**Status:** ✅ Complete v0.1.0 (2025-12-23)

**Description:** Concurrent-safe progress updates using file-based atomic writes.

**Completed Features:**
- ✅ File-based atomic writes for concurrent safety
- ✅ Multiple independent progress trackers (ID-based)
- ✅ CLI and library API
- ✅ Progress state persistence
- ✅ 93 tests across 34 suites with comprehensive coverage
- ✅ Dogfooding validation with Test Flakiness Detector
- ✅ Comprehensive documentation and examples

**Location:** `/cli-progress-reporting/`
**Dogfooding:** Validated with Test Flakiness Detector (100% pass rate, 0 flaky tests)

---

#### ✅ Cross-Platform Path Normalizer (COMPLETE)

**Priority:** MEDIUM
**Language:** TypeScript
**Complexity:** Quick (2-3 days)
**Status:** ✅ Complete v0.1.0 (2025-12-24)

**Description:** Normalize paths consistently across Windows/Unix. Standard library base.

**Completed Features:**
- ✅ Windows/Unix path conversion and normalization
- ✅ Multiple output formats (unix, windows, posix, native)
- ✅ CLI and library API
- ✅ 145 tests across all edge cases
- ✅ Dogfooding validation (10 runs, 1,450 executions, 0 flaky)
- ✅ Full VitePress documentation

**Location:** `/cross-platform-path-normalizer/`
**Dogfooding:** Validated with Test Flakiness Detector (100% pass rate)

---

#### ✅ File-Based Semaphore (COMPLETE)

**Priority:** MEDIUM
**Language:** Rust
**Complexity:** Quick (2-3 days)
**Status:** ✅ Complete v0.1.0 (2025-12-25)

**Description:** Cross-platform locking utility for shell scripts. First Rust tool in Tuulbelt.

**Completed Features:**
- ✅ Cross-platform process locking via filesystem
- ✅ CLI commands: acquire, release, check, info, list
- ✅ Stale lock detection and automatic cleanup
- ✅ 85 tests (31 unit + 39 CLI + 11 integration + 4 doctests)
- ✅ Zero clippy warnings, zero runtime dependencies
- ✅ Complete SPEC.md defining lock protocol
- ✅ Full VitePress documentation

**Location:** `/file-based-semaphore/`

---

#### ✅ Output Diffing Utility (COMPLETE)

**Priority:** LOW
**Language:** Rust
**Complexity:** Quick (2-3 days)
**Status:** ✅ Complete v0.1.0 (2025-12-26)

**Description:** Semantic diff for text, JSON, and binary files. Second Rust tool in Tuulbelt.

**Completed Features:**
- ✅ Text diff using LCS algorithm with context
- ✅ JSON structural diff (field-level comparison)
- ✅ Binary hex dump comparison
- ✅ Multiple output formats (unified, context, side-by-side, JSON)
- ✅ 99 tests (76 lib + 18 CLI + 5 doc)
- ✅ Zero clippy warnings, zero runtime dependencies
- ✅ File size safety (100MB default, --max-size override)
- ✅ Optimized performance (vector pre-allocation)
- ✅ Complete SPEC.md defining diff algorithm and formats
- ✅ Full VitePress documentation (7 pages + SPEC)

**Location:** `/output-diffing-utility/`

---

## Phase 2: High-Impact Medium Tools

**Goal:** Build most valuable medium-complexity tools

**Timeline:** Current phase (started 2025-12-26)
**Status:** 🔄 In Progress | Recommended: Structured Error Handler

### Configuration & CLI (3 tools)

- [ ] Configuration File Merger/Resolver - ENV + config + CLI arg merging
- [ ] Component Prop Validator - TypeScript runtime validation
- [ ] Exhaustiveness Checker - Union case coverage for TS/JS

### Testing & Observability (3 tools)

- [ ] Snapshot Comparison - Binary/structured data snapshots
- [ ] Test Port Conflict Resolver - Concurrent test port allocation
- [ ] Structured Trace-to-SVG - Events → Flame Graph visualization

### Data & Protocols (4 tools)

- [ ] Content-Addressable Blob Store - SHA-256 hash-based storage
- [ ] Schema Converter (YAML ↔ JSON) - Format conversion
- [ ] Minimalist Pub-Sub Protocol - Wire protocol for service messaging
- [ ] Self-Describing Binary Wire Protocol - TLV format for RPC

### Security & Networking (4 tools)

- [ ] Stateless Identity Generator - Ed25519 token generation
- [ ] Static Site Search Indexer - HTML/Markdown → compressed index
- [ ] Peer Discovery (UDP Multicast) - Local service discovery
- [ ] One-File Reverse Proxy - Basic HTTP mapping

### APIs & Integration (4 tools)

- [ ] Request/Response Envelope Codec - Standard RPC response wrapping
- [ ] API Versioning Helper - Multi-version API logic
- [ ] JSON Schema Validator - Compliance checking
- [ ] Streaming JSON Parser - Memory-efficient parsing (Rust)

### Utilities & Infrastructure (6 tools)

- [ ] Universal Log Normalizer - Structured log standardization
- [ ] Manifest-First Sync Tool - Directory sync via manifest diffs
- [ ] Universal Health-Check Probe - Multi-check abstraction
- [ ] Secret Injector - Encrypted secret injection
- [ ] Deterministic Task Runner - DAG executor with file-hash skipping
- [ ] Backpressure Proxy - Cascading failure prevention

### Build & Performance (2 tools)

- [ ] Zero-Overhead Timing - Compile-time optional instrumentation
- [ ] Deterministic Build Artifact Generator - Reproducible builds

### Interoperability (1 tool)

- [ ] FFI Binding Generator - Rust FFI from C headers

---

## Phase 3: Ecosystem & Refinement

**Goal:** Polish, document, and promote the ecosystem

**Timeline:** After Phase 2 completion
**Status:** Not started

- [ ] Cross-tool integration examples
- [ ] Performance benchmarking suite
- [ ] Comprehensive tutorial series
- [ ] Case studies and real-world usage
- [ ] Community contribution guidelines
- [ ] Tool discovery and search
- [ ] Automated compatibility matrix

---

## Milestone Tracking

### Milestone 1: Infrastructure Complete ✅

**Completed:** 2025-12-23

- Meta repository structure
- Templates for TypeScript and Rust
- Claude Code automation
- Documentation system
- Session handoff system

**Outcome:** Ready to build tools efficiently

---

### Milestone 2: First 5 Quick Tools

**Target:** Q1 2026
**Status:** 🔄 In Progress (2/5 complete - 40%)

**Tools:**
1. ✅ Test Flakiness Detector (Complete v0.1.0)
2. ✅ CLI Progress Reporting (Complete v0.1.0)
3. 🎯 Cross-Platform Path Handling (Next)
4. File-Based Semaphore
5. Output Diffing Utility

**Success Criteria:**
- All tools published
- All tools have 80%+ test coverage
- All tools validated in real projects
- Infrastructure refinements documented

---

### Milestone 3: 10 Medium Tools

**Target:** Q2 2026
**Status:** Not started (0/10 complete)

**Focus Areas:**
- Configuration management (2 tools)
- Testing infrastructure (2 tools)
- Data protocols (2 tools)
- Security (2 tools)
- APIs (2 tools)

**Success Criteria:**
- Tools solve real production problems
- Cross-tool compatibility verified
- Performance benchmarks established

---

### Milestone 4: 20 Medium Tools

**Target:** Q3-Q4 2026
**Status:** Not started (0/20 complete)

**Focus Areas:**
- Complete all remaining medium tools
- Build integrations between tools
- Establish ecosystem patterns

---

### Milestone 5: Ecosystem Maturity

**Target:** 2027
**Status:** Not started

**Goals:**
- All 33 tools published and maintained
- Active community contributions
- Real-world production usage
- Documentation and tutorials complete

---

## Tool Selection Criteria

Before building a new tool, verify:

1. **Single Problem** - Solves one thing well
2. **Proven Implementation** - Not experimental
3. **Zero Dependencies** - Standard library only
4. **Portable Interface** - CLI, files, sockets
5. **Composable** - Works via pipes
6. **Real Need** - Documented production use case

See `PRINCIPLES.md` for detailed criteria.

---

## Dependencies Between Tools

Some tools depend on others:

**No Dependencies (Can build anytime):**
- Test Flakiness Detector
- CLI Progress Reporting
- Path Normalizer
- File-Based Semaphore
- Output Diffing

**Soft Dependencies (Better with other tools):**
- Structured Trace-to-SVG → Output Diffing (for comparing traces)
- Health-Check Probe → Config Merger (for configuration)
- Task Runner → Manifest Sync (for build artifacts)

**Build order optimizes for minimal dependencies first.**

---

## Prioritization Changes

Priorities may change based on:

1. **User Feedback** - Real-world demand for specific tools
2. **Infrastructure Needs** - Tools that improve development workflow
3. **Dependency Discovery** - Finding that tool X needs tool Y first
4. **Complexity Reassessment** - Tools proving harder/easier than estimated

**Process for changing priorities:**
1. Document rationale in this file
2. Update STATUS.md with decision
3. Get consensus if significant change

---

## Current Focus

**Phase:** Phase 1 - Quick Tools (2/5 complete - 40%)
**Next Tool:** Cross-Platform Path Handling
**Language:** TypeScript or Rust
**Timeline:** 2-3 days

**Recently Completed:**
- ✅ CLI Progress Reporting v0.1.0 (2025-12-23)
- ✅ Test Flakiness Detector v0.1.0 (2025-12-23)

**After Next Tool:**
- Continue with remaining Quick tools (File-Based Semaphore, Output Diffing Utility)
- Phase 1 will be 60% complete (3/5 tools)

---

## References

- See `STATUS.md` for current session state
- See `CHANGELOG.md` for completed work
- See `docs/setup/TUULBELT_TRIAGE.md` for complexity matrix
- See `PRINCIPLES.md` for tool selection criteria

---

*This roadmap is a living document. Update after completing each tool or making significant progress.*
