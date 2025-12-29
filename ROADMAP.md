# Tuulbelt Development Roadmap

**Last Updated:** 2025-12-29
**Total Tools Planned:** 33
**Completed:** 10 (30%)
**Current Phase:** Phase 2 - Core Tools (In Progress)

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

## Phase 2: Core Tools

**Goal:** Build essential utilities for testing, configuration, and error handling

**Timeline:** Current phase (started 2025-12-26)
**Status:** 🔄 In Progress (5/28 tools complete) | Next Up: Component Prop Validator

### Completed Phase 2 Tools (5 tools)

#### ✅ Structured Error Handler (COMPLETE)

**Priority:** HIGH
**Language:** TypeScript
**Status:** ✅ Complete v0.1.0 (2025-12-27)

**Description:** Error format + serialization with context preservation through call stacks.

**Location:** `/structured-error-handler/`

---

#### ✅ Configuration File Merger (COMPLETE)

**Priority:** HIGH
**Language:** TypeScript
**Status:** ✅ Complete v0.1.0 (2025-12-27)

**Description:** ENV + config + CLI arg merging with clear precedence (CLI > ENV > File > Defaults).

**Location:** `/config-file-merger/`

---

#### ✅ Snapshot Comparison (COMPLETE)

**Priority:** MEDIUM
**Language:** Rust
**Status:** ✅ Complete v0.1.0 (2025-12-27)

**Description:** Snapshot testing with integrated diffs. First tool using Tuulbelt-to-Tuulbelt library composition.

**Location:** `/snapshot-comparison/`

---

#### ✅ File-Based Semaphore (TypeScript) (COMPLETE)

**Priority:** MEDIUM
**Language:** TypeScript
**Status:** ✅ Complete v0.1.0 (2025-12-28)

**Description:** TypeScript implementation with cross-language compatibility with Rust version.

**Location:** `/file-based-semaphore-ts/`

---

#### ✅ Test Port Resolver (COMPLETE)

**Priority:** HIGH
**Language:** TypeScript
**Status:** ✅ Complete v0.1.0 (2025-12-29)

**Description:** Concurrent test port allocation to avoid EADDRINUSE errors. Uses semats as library dependency.

**Location:** `/test-port-resolver/`

---

### Remaining Phase 2 Tools (23 tools)

#### Configuration & CLI

- [x] Configuration File Merger/Resolver ✅
- [ ] Component Prop Validator - TypeScript runtime validation 🎯 NEXT
- [ ] Exhaustiveness Checker - Union case coverage for TS/JS

#### Testing & Observability

- [x] Snapshot Comparison ✅
- [x] Test Port Resolver ✅
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

### Milestone 2: First 5 Quick Tools ✅ COMPLETE

**Completed:** 2025-12-26
**Status:** ✅ Complete (5/5 - 100%)

**Tools:**
1. ✅ Test Flakiness Detector v0.1.0
2. ✅ CLI Progress Reporting v0.1.0
3. ✅ Cross-Platform Path Normalizer v0.1.0
4. ✅ File-Based Semaphore (Rust) v0.1.0
5. ✅ Output Diffing Utility v0.1.0

**Success Criteria Met:**
- All 5 tools published and documented
- All tools have 80%+ test coverage
- All tools dogfooded and validated
- Infrastructure patterns established

---

### Milestone 3: 10 Tools Total (Phase 1 + Phase 2)

**Target:** Q1 2025
**Status:** ✅ Complete (10/10 - 100%) - Completed 2025-12-29

**Phase 1 Tools (5):** All complete ✅
**Phase 2 Tools (5):** All complete ✅

**Breakdown:**
- Configuration management: Config File Merger ✅
- Testing infrastructure: Test Port Resolver, Snapshot Comparison ✅
- Error handling: Structured Error Handler ✅
- Cross-platform utilities: File-Based Semaphore (TS) ✅

**Success Criteria Met:**
- All 10 tools solve real production problems
- Cross-tool composition verified (snapcmp uses odiff, portres uses semats)
- Dogfooding network established
- Documentation standards proven

---

### Milestone 4: 15 Tools (Next Goal)

**Target:** Q2 2025
**Status:** 🔄 Not started (0/5 complete)

**Next 5 Tools:**
1. Component Prop Validator - TypeScript runtime validation 🎯 NEXT
2. Exhaustiveness Checker - Union case coverage for TS/JS
3. Content-Addressable Blob Store - SHA-256 hash-based storage
4. Streaming JSON Parser - Memory-efficient parsing (Rust)
5. Universal Log Normalizer - Structured log standardization

**Focus Areas:**
- Frontend validation (2 tools)
- Data storage (1 tool)
- Data parsing (1 tool)
- Observability (1 tool)

---

### Milestone 5: 20 Tools

**Target:** Q3 2025
**Status:** Not started (0/20 complete)

**Focus Areas:**
- Complete remaining Phase 2 tools
- Build more integrations between tools
- Establish ecosystem patterns

---

### Milestone 6: All 33 Tools Complete

**Target:** Q4 2025
**Status:** Not started (10/33 complete - 30%)

**Goals:**
- All 33 tools published and maintained
- Comprehensive cross-tool integration examples
- Real-world production usage documented
- Complete documentation and tutorials

---

### Milestone 7: Ecosystem Maturity

**Target:** 2026+
**Status:** Not started

**Goals:**
- Active community contributions
- Tool discovery and search capabilities
- Performance benchmarking suite
- Case studies and real-world usage examples

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

**Phase:** Phase 2 - Core Tools (5/28 complete - 18%)
**Next Tool:** Component Prop Validator 🎯
**Language:** TypeScript
**Timeline:** 3-5 days (Medium complexity)

**Recently Completed:**
- ✅ Test Port Resolver v0.1.0 (2025-12-29)
- ✅ File-Based Semaphore (TS) v0.1.0 (2025-12-28)
- ✅ Snapshot Comparison v0.1.0 (2025-12-27)
- ✅ Configuration File Merger v0.1.0 (2025-12-27)
- ✅ Structured Error Handler v0.1.0 (2025-12-27)

**Total Progress:**
- 10 of 33 tools complete (30%)
- Phase 1: 5/5 complete (100%) ✅
- Phase 2: 5/28 complete (18%)

**After Next Tool:**
- 11/33 tools (33%)
- Exhaustiveness Checker (TypeScript)
- Then Content-Addressable Blob Store

---

## References

- See `STATUS.md` for current session state
- See `CHANGELOG.md` for completed work
- See `docs/setup/TUULBELT_TRIAGE.md` for complexity matrix
- See `PRINCIPLES.md` for tool selection criteria

---

*This roadmap is a living document. Update after completing each tool or making significant progress.*
