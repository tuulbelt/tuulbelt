# Tuulbelt Development Roadmap

**Last Updated:** 2025-12-23
**Total Tools Planned:** 33
**Completed:** 1 (3%)
**Current Phase:** Phase 1 - Infrastructure & Quick Tools

---

## Development Philosophy

Tools are built in priority order based on:
1. **Complexity** (Quick → Medium)
2. **Utility** (High-impact tools first)
3. **Dependencies** (Foundational tools before dependent ones)

See `docs/setup/TUULBELT_TRIAGE.md` for detailed complexity analysis.

---

## Phase 1: Infrastructure & Quick Tools

**Goal:** Establish development infrastructure and build highest-impact quick tools

**Timeline:** Current phase
**Status:** Infrastructure ✅ Complete | Quick Tools 🔄 In Progress

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

#### 🎯 Next Up: CLI Progress/Status Reporting

**Priority:** MEDIUM
**Language:** TypeScript
**Complexity:** Quick (2-3 days)
**Status:** Not started

**Description:** Concurrent-safe progress updates. Proven pattern from existing tools.

**Use Case:**
```bash
# From your script
progress-report --total 100 --message "Processing files"
progress-report --increment 1
```

---

#### Cross-Platform Path Handling

**Priority:** MEDIUM
**Language:** TypeScript or Rust
**Complexity:** Quick (2-3 days)
**Status:** Not started

**Description:** Normalize paths consistently, especially Windows. Standard library base.

**Use Case:**
```bash
path-normalizer /path/to/file    # Unix
path-normalizer C:\path\to\file  # Windows
# Both output: normalized absolute path
```

---

#### File-Based Semaphore

**Priority:** MEDIUM
**Language:** Rust
**Complexity:** Quick (2-3 days)
**Status:** Not started

**Description:** Cross-platform locking utility for shell scripts. Handles stale locks.

**Use Case:**
```bash
# Script 1
semaphore acquire my-lock || exit 1
# ... critical section ...
semaphore release my-lock

# Script 2 (blocks until lock available)
semaphore acquire my-lock --timeout 30
```

---

#### Output Diffing Utility

**Priority:** LOW
**Language:** TypeScript
**Complexity:** Quick (2-3 days)
**Status:** Not started

**Description:** Better than textual diffs for JSON, binary, structured data in assertions.

**Use Case:**
```bash
diff-structured actual.json expected.json
# Output: Semantic diff showing field-level changes
```

---

## Phase 2: High-Impact Medium Tools

**Goal:** Build most valuable medium-complexity tools

**Timeline:** After Phase 1 completion
**Status:** Not started

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
**Status:** 🔄 In Progress (1/5 complete - 20%)

**Tools:**
1. ✅ Test Flakiness Detector (Complete v0.1.0)
2. 🎯 CLI Progress Reporting (Next)
3. Cross-Platform Path Handling
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

**Phase:** Phase 1 - Quick Tools (1/5 complete - 20%)
**Next Tool:** CLI Progress/Status Reporting
**Language:** TypeScript
**Timeline:** 2-3 days

**Recently Completed:**
- ✅ Test Flakiness Detector v0.1.0 (2025-12-23)

**After Next Tool:**
- Continue with remaining Quick tools
- Consider Cross-Platform Path Handling or File-Based Semaphore

---

## References

- See `STATUS.md` for current session state
- See `CHANGELOG.md` for completed work
- See `docs/setup/TUULBELT_TRIAGE.md` for complexity matrix
- See `PRINCIPLES.md` for tool selection criteria

---

*This roadmap is a living document. Update after completing each tool or making significant progress.*
