# Next Tasks

**Last Updated:** 2025-12-28

This document tracks pending work across the Tuulbelt project. Tasks are organized by type and priority.

---

## 🎯 Short CLI Names Reference

All tools have short CLI names for better DX:

### Implemented (8 tools)

| Tool | Short Name | Long Name |
|------|------------|-----------|
| Test Flakiness Detector | `flaky` | `test-flakiness-detector` |
| CLI Progress Reporting | `prog` | `cli-progress-reporting` |
| Cross-Platform Path Normalizer | `normpath` | `cross-platform-path-normalizer` |
| File-Based Semaphore | `sema` | `file-semaphore` |
| Output Diffing Utility | `odiff` | `output-diff` |
| Structured Error Handler | `serr` | `structured-error-handler` |
| Configuration File Merger | `cfgmerge` | `config-file-merger` |
| Snapshot Comparison | `snapcmp` | `snapshot-comparison` |

### Proposed (25 remaining tools)

| Tool | Short Name | Rationale |
|------|------------|-----------|
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

### Completed (Phase 2: 3/28) 🆕

✅ **Structured Error Handler** (v0.1.0) - TypeScript
✅ **Configuration File Merger** (v0.1.0) - TypeScript
✅ **Snapshot Comparison** (v0.1.0) - Rust 🆕

### Phase 2: Next Up

See `README.md` for complete roadmap (25 remaining tools).

**Recommended Next Tools:**
- **Test Port Conflict Resolver** - Concurrent test port allocation (TypeScript)
- **Component Prop Validator** - TypeScript runtime validation (TypeScript)
- **Exhaustiveness Checker** - Union case coverage for TS/JS (TypeScript)

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
- ✅ **Security:** 6 resource limit tests added
- 132 tests passing

### CLI Progress Reporting

- ✅ v0.1.0 stable
- ✅ Dogfooding: Bidirectional validation with test-flakiness-detector
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate concurrent safety (125 tests × 20 runs)
  - dogfood-diff.sh: Prove deterministic outputs
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ **Security:** Path traversal prevention + 10 security tests
- 121 tests passing

### Cross-Platform Path Normalizer

- ✅ v0.1.0 stable
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate determinism (145 tests × 10 runs)
  - dogfood-diff.sh: Prove identical outputs
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ **Security:** 13 malicious input handling tests
- 141 tests passing

### File-Based Semaphore

- ✅ v0.1.0 stable (First Rust tool!)
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate concurrent safety (85 tests × 10 runs)
  - dogfood-diff.sh: Prove deterministic outputs
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ Full CLI test coverage added
- ✅ **Security:** Tag newline injection prevention + 10 security tests
- 95 tests passing (33 unit + 39 CLI + 19 integration + 4 doctests)
- Zero clippy warnings

### Output Diffing Utility

- ✅ v0.1.0 stable (Second Rust tool!)
- ✅ Dogfooding: 1 composition script implemented
  - dogfood-flaky.sh: Validate determinism (99 tests × 10 runs)
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ **Security:** 10 tests (JSON bombs, unicode, binary patterns)
- 108 tests passing (76 lib + 27 CLI + 5 doc)
- Zero clippy warnings
- Complete documentation (7 VitePress pages + SPEC.md)
- File size safety (100MB default, --max-size override)
- Optimized vector allocations

### Structured Error Handler

- ✅ v0.1.0 stable (First Phase 2 tool!)
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate determinism (81 tests × 10 runs)
  - dogfood-diff.sh: Prove deterministic serialization
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ **Security:** Stack trace exclusion + metadata sanitization + 7 tests
- 88 tests passing (core, serialization, edge cases, validation, CLI)
- Complete documentation (6 VitePress pages + SPEC.md)
- Context chain preservation through call stacks
- Full JSON serialization/deserialization

### Configuration File Merger

- ✅ v0.1.0 stable (Second Phase 2 tool!)
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate determinism (135 tests × 10 runs)
  - dogfood-diff.sh: Prove config merging produces identical output
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ **Security:** Prototype pollution prevention + 9 tests
- 144 tests passing (parsing, merging, CLI, type coercion, edge cases, security)
- Complete documentation (7 VitePress pages)
- Clear precedence: CLI > ENV > File > Defaults
- Source tracking for debugging config origins

### Snapshot Comparison

- ✅ v0.1.0 stable (Third Phase 2 tool!)
- ✅ **First tool using Tuulbelt-to-Tuulbelt library composition** (PRINCIPLES.md Exception 2)
- ✅ Integrates output-diffing-utility as path dependency
- ✅ Dogfooding: 2 composition scripts implemented
  - dogfood-flaky.sh: Validate determinism (96 tests × 10 runs)
  - dogfood-sema.sh: Concurrent snapshot safety with file-based-semaphore
- ✅ DOGFOODING_STRATEGY.md: Complete strategy document
- ✅ **Security:** Path traversal, Unicode homoglyph, resource exhaustion prevention + 13 security tests
- 96 tests passing (33 unit + 59 integration + 4 doc tests)
- Complete documentation (7 VitePress pages + SPEC.md)
- Hash-based fast comparison with detailed diff on mismatch
- Semantic diffing for text, JSON, and binary via odiff

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
- ✅ **Short CLI names documentation** - All 6 tools updated
  - All READMEs show `# Tool Name / \`short-name\`` format
  - All VitePress docs use short names in examples
  - All dogfooding scripts use short names
  - Proposed names for 27 remaining tools documented
  - /quality-check verifies short name configuration
- ✅ **VitePress demo workflow fix** - create-demos.yml handles all placeholder patterns 🆕
- ✅ **Library badge styling** - SVG icons instead of emojis, theme-adaptive CSS 🆕

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
- ✅ **Demo workflow smart detection** - 75-80% CI time savings on recordings
- ✅ **VitePress demo integration** - Workflow handles 3 placeholder patterns correctly 🆕
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
- ✅ Structured Error Handler: 80%+ coverage ✅
- ✅ Configuration File Merger: 80%+ coverage ✅
- ✅ Snapshot Comparison: 80%+ coverage ✅ 🆕
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

**Last Review:** 2025-12-28
**Next Review:** When starting next session

---

## Session Notes (2025-12-28)

**Documentation & Infrastructure Fixes** after snapcmp merge:

- ✅ **Icon Cleanup** - Replaced emoji badges (📦) with SVG icons (package.svg)
  - Added `.library-badge` CSS class for consistent styling
  - Icons work in both light and dark mode
- ✅ **VitePress Demo Integration Fixed** - Fixed create-demos.yml workflow
  - Now handles 3 patterns: existing URLs, placeholder `(#)` links, placeholder text
  - Updated QUALITY_CHECKLIST.md with correct VitePress Demo template
- ✅ **Test Count Fixed** - Updated from 42 → 96 in docs/tools/index.md
- ✅ **snapcmp Demo Section** - Added proper demo.gif reference and asciinema link

**Commits:**
- `b7a679b` - fix(docs): clean up snapcmp documentation and demo integration
- `98a2d63` - fix(ci): improve VitePress demo integration in create-demos workflow

---

## Session Notes (2025-12-27)

**Snapshot Comparison Complete!** Tool #8 implemented.

**Latest Session (Snapshot Comparison):**
- Implemented Snapshot Comparison (`snapcmp`) ✅
  - 96 tests passing (33 unit + 59 integration + 4 doc tests)
  - First tool using Tuulbelt-to-Tuulbelt library composition
  - Integrates output-diffing-utility as path dependency
  - Hash-based fast comparison with detailed diff on mismatch
- Complete dogfooding setup ✅
  - DOGFOODING_STRATEGY.md customized
  - dogfood-flaky.sh: Validate 96 tests × 10 runs
  - dogfood-sema.sh: Concurrent snapshot safety with sema
- GitHub Pages documentation ✅
  - 7 VitePress pages + SPEC.md
  - Demo recording script created
- Security hardening ✅
  - Resource exhaustion prevention (stdin, context lines, header limits)
  - Unicode homoglyph attack prevention (ASCII-only names)
  - 13 security tests added
- Updated PRINCIPLES.md with Exception 2 for tool composition

**Previous Session (Configuration File Merger):**
- Implemented Configuration File Merger (`cfgmerge`) ✅
  - 144 tests passing
  - Clear precedence: CLI > ENV > File > Defaults
  - Source tracking for debugging

**Previous Session (Demo & npm link Fixes):**
- Fixed npm link support for TypeScript CLIs ✅
- Fixed demo workflow race condition ✅
- Updated scaffold template with bin entry + shebang requirements

**Next Priority:** Test Port Conflict Resolver (`portres`) or Component Prop Validator (`propval`)
