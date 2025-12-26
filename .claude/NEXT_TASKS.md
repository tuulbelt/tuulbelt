# Next Tasks

**Last Updated:** 2025-12-25

This document tracks pending work across the Tuulbelt project. Tasks are organized by type and priority.

---

## 🚀 New Tools (Priority Order)

### Phase 1: Quick Tools ✅ COMPLETE

All 5 Phase 1 tools implemented!

### Completed (Phase 1: 5/5 = 100%) 🎉

✅ **Test Flakiness Detector** (v0.1.0) - TypeScript
✅ **CLI Progress Reporting** (v0.1.0) - TypeScript
✅ **Cross-Platform Path Normalizer** (v0.1.0) - TypeScript
✅ **File-Based Semaphore** (v0.1.0) - Rust
✅ **Output Diffing Utility** (v0.1.0) - Rust 🆕

### Phase 2: Next Up

See `README.md` for complete roadmap (28 remaining tools).

**Recommended Next Tools:**
- **Structured Error Handler** - Error format + serialization (TypeScript)
- **Configuration File Merger** - ENV + config + CLI merging (TypeScript)
- **Snapshot Comparison** - Binary/structured data snapshots (Rust)
- **Test Port Conflict Resolver** - Concurrent test port allocation (TypeScript)

---

## 🔧 Tool Maintenance

### Test Flakiness Detector

- ✅ v0.1.0 stable
- ✅ Dogfooding: Integrates cli-progress-reporting for progress tracking
- ✅ Dogfooding: Validates cli-progress-reporting and cross-platform-path-normalizer test suites
- 148 tests passing

### CLI Progress Reporting

- ✅ v0.1.0 stable
- ✅ Dogfooding: Validated by test-flakiness-detector (125 tests × 20 runs = 2,500 executions)
- ✅ Dogfooding: Used by test-flakiness-detector for real-time progress
- 125 tests passing

### Cross-Platform Path Normalizer

- ✅ v0.1.0 stable
- ✅ Dogfooding: Validated by test-flakiness-detector (145 tests × 10 runs = 1,450 executions)
- 145 tests passing

### File-Based Semaphore

- ✅ v0.1.0 stable (First Rust tool!)
- ✅ Dogfooding: Can be validated by test-flakiness-detector
- ✅ Full CLI test coverage added
- 85 tests passing (31 unit + 39 CLI + 11 integration + 4 doctests)
- Zero clippy warnings

### Output Diffing Utility 🆕

- ✅ v0.1.0 stable (Second Rust tool!)
- ✅ Dogfooding: Can be validated by test-flakiness-detector
- 99 tests passing (76 lib + 18 CLI + 5 doc)
- Zero clippy warnings
- Complete documentation (7 VitePress pages + SPEC.md)
- File size safety (100MB default, --max-size override)
- Optimized vector allocations

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
- ✅ Output Diffing Utility: 80%+ coverage ✅ 🆕
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

**Last Review:** 2025-12-26
**Next Review:** When starting next session

---

## Session Notes (2025-12-26)

**Phase 1 Complete!** All 5 Quick Tools implemented (15% of 33 total).

**Recent Accomplishment:**
- Output Diffing Utility: 2,874 lines Rust code, 99 tests, complete documentation
- Demo workflow: 75-80% CI time savings with smart detection
- Quality infrastructure: Templates and checklist updated for systematic integration

**Ready for Phase 2!**
