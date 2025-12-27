# Next Tasks

**Last Updated:** 2025-12-27

This document tracks pending work across the Tuulbelt project. Tasks are organized by type and priority.

---

## 🎯 Short CLI Names Reference

All tools have short CLI names for better DX:

### Implemented (6 tools)

| Tool | Short Name | Long Name |
|------|------------|-----------|
| Test Flakiness Detector | `flaky` | `test-flakiness-detector` |
| CLI Progress Reporting | `prog` | `cli-progress-reporting` |
| Cross-Platform Path Normalizer | `normpath` | `cross-platform-path-normalizer` |
| File-Based Semaphore | `sema` | `file-semaphore` |
| Output Diffing Utility | `odiff` | `output-diff` |
| Structured Error Handler | `serr` | `structured-error-handler` |

### Proposed (27 remaining tools)

| Tool | Short Name | Rationale |
|------|------------|-----------|
| Configuration File Merger | `cfgmerge` | config + merge |
| Snapshot Comparison | `snapcmp` | snapshot + compare |
| Test Port Conflict Resolver | `portres` | port + resolver |
| Component Prop Validator | `propval` | prop + validate |
| Exhaustiveness Checker | `excheck` | exhaustive + check |
| Content-Addressable Blob Store | `blobstore` | self-explanatory |
| Schema Converter (YAML↔JSON) | `schconv` | schema + convert |
| Minimalist Pub-Sub Protocol | `pubsub` | self-explanatory |
| Self-Describing Binary Wire Protocol | `wireproto` | wire + protocol |
| Request/Response Envelope Codec | `envcodec` | envelope + codec |
| API Versioning Helper | `apiver` | api + version |
| JSON Schema Validator | `jsonval` | json + validate |
| Streaming JSON Parser | `jsonstream` | json + stream |
| Stateless Identity Generator | `idgen` | identity + generate |
| Static Site Search Indexer | `searchidx` | search + index |
| Peer Discovery (UDP Multicast) | `peerdisco` | peer + discover |
| One-File Reverse Proxy | `revproxy` | reverse + proxy |
| Universal Log Normalizer | `lognorm` | log + normalize |
| Manifest-First Sync Tool | `mansync` | manifest + sync |
| Universal Health-Check Probe | `healthprobe` | health + probe |
| Secret Injector | `secretinj` | secret + inject |
| Deterministic Task Runner | `taskrun` | task + run |
| Zero-Overhead Timing | `timeinj` | timing + inject |
| Deterministic Build Artifact Generator | `detbuild` | deterministic + build |
| Structured Trace-to-SVG | `tracesvg` | trace + svg |
| Backpressure Proxy | `bpproxy` | backpressure + proxy |
| FFI Binding Generator | `ffigen` | ffi + generate |

---

## 🚀 New Tools (Priority Order)

### Phase 1: Quick Tools ✅ COMPLETE

All 5 Phase 1 tools implemented!

### Completed (Phase 1: 5/5 = 100%) 🎉

✅ **Test Flakiness Detector** (v0.1.0) - TypeScript
✅ **CLI Progress Reporting** (v0.1.0) - TypeScript
✅ **Cross-Platform Path Normalizer** (v0.1.0) - TypeScript
✅ **File-Based Semaphore** (v0.1.0) - Rust
✅ **Output Diffing Utility** (v0.1.0) - Rust

### Completed (Phase 2: 1/28) 🆕

✅ **Structured Error Handler** (v0.1.0) - TypeScript 🆕

### Phase 2: Next Up

See `README.md` for complete roadmap (27 remaining tools).

**Recommended Next Tools:**
- **Configuration File Merger** - ENV + config + CLI merging (TypeScript)
- **Snapshot Comparison** - Binary/structured data snapshots (Rust)
- **Test Port Conflict Resolver** - Concurrent test port allocation (TypeScript)
- **Component Prop Validator** - TypeScript runtime validation (TypeScript)

---

## 🔧 Tool Maintenance

### Test Flakiness Detector

- ✅ v0.1.0 stable
- ✅ Dogfooding: Integrates cli-progress-reporting for progress tracking
- ✅ Dogfooding: 4 composition scripts implemented
  - dogfood-diff.sh: Find ROOT CAUSE of flaky tests via output comparison
  - dogfood-paths.sh: Validate Path Normalizer (145 tests × 10 runs)
  - dogfood-progress.sh: Bidirectional validation (125 tests × 20 runs)
  - dogfood-pipeline.sh: Validate all 5 Phase 1 tools (602 tests × 10 runs)
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- 148 tests passing

### CLI Progress Reporting

- ✅ v0.1.0 stable
- ✅ Dogfooding: Bidirectional validation with test-flakiness-detector
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate concurrent safety (125 tests × 20 runs)
  - dogfood-diff.sh: Prove deterministic outputs
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- 125 tests passing

### Cross-Platform Path Normalizer

- ✅ v0.1.0 stable
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate determinism (145 tests × 10 runs)
  - dogfood-diff.sh: Prove identical outputs
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- 145 tests passing

### File-Based Semaphore

- ✅ v0.1.0 stable (First Rust tool!)
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate concurrent safety (85 tests × 10 runs)
  - dogfood-diff.sh: Prove deterministic outputs
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ Full CLI test coverage added
- 85 tests passing (31 unit + 39 CLI + 11 integration + 4 doctests)
- Zero clippy warnings

### Output Diffing Utility

- ✅ v0.1.0 stable (Second Rust tool!)
- ✅ Dogfooding: 1 composition script implemented
  - dogfood-flaky.sh: Validate determinism (99 tests × 10 runs)
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- 99 tests passing (76 lib + 18 CLI + 5 doc)
- Zero clippy warnings
- Complete documentation (7 VitePress pages + SPEC.md)
- File size safety (100MB default, --max-size override)
- Optimized vector allocations

### Structured Error Handler 🆕

- ✅ v0.1.0 stable (First Phase 2 tool!)
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate determinism (81 tests × 10 runs)
  - dogfood-diff.sh: Prove deterministic serialization
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- 81 tests passing (core, serialization, edge cases, validation, CLI)
- Complete documentation (6 VitePress pages + SPEC.md)
- Context chain preservation through call stacks
- Full JSON serialization/deserialization

---

## 🐛 Bug Fixes

See `docs/KNOWN_ISSUES.md` for tracked issues.

**High Priority:**
- None currently

**Medium Priority (Cosmetic):**
- [ ] Fix VitePress card icon theming on main page (dark mode)
- [ ] Fix StackBlitz badge vertical alignment in READMEs

**Low Priority:**
- None currently

---

## 📚 Documentation

### Completed
- ✅ Test Flakiness Detector - Full VitePress docs
- ✅ CLI Progress Reporting - Full README (no VitePress yet)
- ✅ Cross-Platform Path Normalizer - Full VitePress docs
- ✅ README consistency across tools
- ✅ Footer standardization
- ✅ **Dogfooding documentation** - All 3 tool READMEs updated
- ✅ **Dogfooding documentation** - Root README.md updated
- ✅ **Dogfooding documentation** - VitePress docs for test-flakiness-detector and cross-platform-path-normalizer
- ✅ **Bidirectional validation network** documented across all tools
- ✅ **Dogfooding patterns added to QUALITY_CHECKLIST.md** - Dynamic import pattern documented
- ✅ **Dogfooding guidance added to scaffold templates** - Both TypeScript and Rust templates
- ✅ **detectFlakiness async/await documentation** - All docs show correct async pattern
- ✅ **Fuzzy test descriptions** - Removed test counts from all READMEs
- ✅ **GitHub Pages formatting fixes** - Fixed cross-platform-path-normalizer library-usage.md (50→376 lines) and examples.md (43→484 lines)
- ✅ **Demo section standardization** - All 3 GitHub Pages tool index files have Demo sections with StackBlitz buttons
- ✅ **Local VitePress Demo sections** - Added to test-flakiness-detector and cross-platform-path-normalizer local docs
- ✅ **Asciinema placeholders** - Added placeholder link for cross-platform-path-normalizer (GitHub Actions will populate)
- ✅ **Documentation consistency achieved** - All tools have matching structure, quality, and Demo sections
- ✅ **File-Based Semaphore docs** - README, SPEC.md, 7 VitePress pages, 2 examples
- ✅ **File-Based Semaphore CLI tests** - 39 CLI tests added, docs updated (85 total tests)
- ✅ **Output Diffing Utility docs** - README, SPEC.md, 7 VitePress pages, 2 examples 🆕
- ✅ **Demo workflow optimization** - Smart detection, path filters, proper titles 🆕
- ✅ **Template performance patterns** - Added to both Rust and TypeScript templates 🆕
- ✅ **Template SPEC.md guidance** - When/how to create specifications 🆕
- ✅ **Template advanced examples** - examples/advanced.rs and examples/advanced.ts 🆕
- ✅ **Systematic dogfooding implementation** - All 4 remaining Phase 1 tools
  - Created DOGFOODING_STRATEGY.md for all tools
  - Implemented 10 composition scripts (4+2+2+2)
  - Updated all tool documentation (README + GH Pages)
  - Created templates for future tools (TypeScript + Rust)
  - Updated QUALITY_CHECKLIST.md with dogfooding requirements
- ✅ **Short CLI names documentation** - All 6 tools updated 🆕
  - All READMEs show `# Tool Name / \`short-name\`` format
  - All VitePress docs use short names in examples
  - All dogfooding scripts use short names
  - Proposed names for 27 remaining tools documented
  - /quality-check verifies short name configuration

### Pending
- [ ] Add "Contributing" guide page to VitePress (currently just links to CONTRIBUTING.md)
- [ ] Consider adding troubleshooting sections to tool docs
- [ ] Add more visual examples (screenshots, diagrams)

---

## 🎨 Design / UX

- [ ] Fix card icon dark mode theming (see KNOWN_ISSUES.md)
- [ ] Fix StackBlitz badge alignment (see KNOWN_ISSUES.md)
- [ ] Consider adding dark/light mode toggle preview in docs
- [ ] Evaluate adding syntax highlighting themes

---

## ⚙️ Infrastructure

### CI/CD
- ✅ Auto-discovery workflows functioning
- ✅ Demo generation automated
- ✅ GitHub Pages deployment working
- ✅ **CI Optimization Phase 1** - Path filters, concurrency controls, modern actions, caching
- ✅ **CI Optimization Phase 2** - Artifact-based dashboard (no re-testing)
- ✅ **docs/CI_GUIDE.md** - Comprehensive CI documentation as single source of truth
- ✅ **Demo workflow smart detection** - 75-80% CI time savings on recordings 🆕
- [ ] Consider adding performance benchmarks to CI

### Workflows
- [ ] Consider adding automated PR description generation
- [ ] Evaluate adding changelog automation
- [ ] Consider semantic-release integration

---

## 🧪 Testing

### Test Infrastructure
- [ ] Consider adding mutation testing
- [ ] Evaluate adding visual regression tests for VitePress
- [ ] Consider adding E2E tests for workflows

### Coverage Goals
- ✅ Test Flakiness Detector: 80%+ coverage ✅
- ✅ CLI Progress Reporting: 80%+ coverage ✅
- ✅ Cross-Platform Path Normalizer: 80%+ coverage ✅
- ✅ File-Based Semaphore: 80%+ coverage ✅
- ✅ Output Diffing Utility: 80%+ coverage ✅
- ✅ Structured Error Handler: 80%+ coverage ✅ 🆕
- Target: All tools maintain 80%+ coverage

---

## 📊 Metrics & Monitoring

- [ ] Track test flakiness across all tools (dogfooding opportunity!)
- [ ] Monitor docs page views (if GitHub Pages analytics available)
- [ ] Track StackBlitz usage

---

## 🎯 Quick Actions (No Session Needed)

These can be done quickly without a full implementation session:

- [ ] Update README badges (if desired)
- [ ] Add more emoji to docs (if desired)
- [ ] Improve error messages in existing tools
- [ ] Add more examples to existing docs

---

## Task Types Reference

When updating this document or using `/handoff`, categorize tasks:

**🚀 NEW_TOOL** - Implementing a new tool from scratch
**🔧 UPDATE_TOOL** - Modifying existing tool functionality
**🧪 ADD_TESTS** - Adding or improving tests
**🐛 FIX_BUG** - Fixing bugs or issues
**📚 DOCS** - Documentation work
**⚙️ INFRA** - CI/CD, workflows, infrastructure
**🎨 DESIGN** - UI/UX improvements
**♻️ REFACTOR** - Code cleanup or restructuring
**🔒 SECURITY** - Security fixes or improvements

---

## How to Use This Document

**Adding New Tasks:**
```bash
# Add task to appropriate section
# Include: Description, Priority, Type, Complexity estimate
```

**Claiming a Task:**
```bash
# Update status from [ ] to [🔄 In Progress]
# Update HANDOFF.md with current work
```

**Completing a Task:**
```bash
# Update status from [🔄 In Progress] to [✅ Done]
# Move to "Completed" section or remove
# Update HANDOFF.md
```

**Referencing from HANDOFF.md:**
```markdown
## Next Immediate Tasks

See NEXT_TASKS.md section: "🚀 New Tools"
Priority: Cross-Platform Path Normalizer
```

---

**Last Review:** 2025-12-27
**Next Review:** When starting next session

---

## Session Notes (2025-12-27)

**npm link and Demo Fixes Complete!** Short CLI names now work correctly.

**Latest Session (Demo & npm link Fixes):**
- Fixed npm link support for TypeScript CLIs ✅
  - Added shebang `#!/usr/bin/env -S npx tsx` to all 5 TypeScript entry points
  - Without shebang, `npm link` symlinks fail with "import: command not found"
- Fixed demo workflow race condition ✅
  - Added cleanup before `git pull --rebase` (build artifacts caused conflicts)
- Updated demo recording scripts to use `npm link --force` + actual short names
- Reset all broken demo content (GIFs, asciinema URLs) to placeholders
- Updated scaffold template with bin entry + shebang requirements
- Added shebang requirement to QUALITY_CHECKLIST.md

**Previous Session (Short CLI Names):**
- Implemented short CLI names for all 6 tools ✅
  - `flaky`, `prog`, `normpath`, `sema`, `odiff`, `serr`
- Updated 69 files with documentation changes
- Added bin entries to all TypeScript package.json files
- Added [[bin]] entries to Rust Cargo.toml files
- Updated all dogfooding scripts
- Enhanced `/quality-check` with short name verification
- Proposed short names for all 27 remaining tools

**Previous Session (Structured Error Handler):**
- Implemented Structured Error Handler (first Phase 2 tool) ✅
- 81 tests passing, complete documentation
- Demo recording script created

**Next Priority:** Merge branch, then Configuration File Merger (`cfgmerge`) or Snapshot Comparison (`snapcmp`)
