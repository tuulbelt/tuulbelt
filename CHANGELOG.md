# Tuulbelt Meta Repository Changelog

All notable changes to the Tuulbelt meta repository will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added - Phase 1 Complete! 🎉 (2025-12-26)

**Milestone:** All 5 Phase 1 Quick Tools implemented (15% of 33 total)

### Added - Output Diffing Utility v0.1.0 (2025-12-26)

**Tool Features:**
- Semantic diff for text, JSON, and binary files (Rust)
- Text diff using LCS algorithm with configurable context
- JSON structural diff with field-level comparison
- Binary hex dump comparison
- Multiple output formats (unified, context, side-by-side, JSON)
- File size safety limit (100MB default, --max-size override)
- Optimized performance (vector pre-allocation in 3 locations)

**Testing Excellence:**
- 99 tests passing (76 lib + 18 CLI + 5 doc)
- Zero clippy warnings, zero runtime dependencies
- Comprehensive edge case coverage

**Documentation:**
- Complete SPEC.md defining LCS algorithm and diff formats
- Full VitePress documentation (7 pages + SPEC)
- Two working examples (basic CLI + library integration)

**Impact:**
- Second Rust tool in Tuulbelt
- Demonstrates SPEC.md pattern for algorithm documentation
- Establishes performance optimization patterns
- 5/5 Phase 1 tools complete ✅

### Added - Infrastructure Improvements (2025-12-26)

**Demo Workflow Optimization:**
- Smart detection for demo recording (75-80% CI time savings)
- Path filters for all 5 tools in `create-demos.yml`
- --title flags added to all recording scripts
- Only records demos when tool implementation or script changes

**Template Enhancements:**
- Performance patterns added to both templates (Rust + TypeScript)
- SPEC.md guidance (when to create, what to include, examples)
- Advanced examples: `examples/advanced.rs` and `examples/advanced.ts`
- Rust patterns: Vec::with_capacity, resource limits, streaming
- TypeScript patterns: Array pre-sizing, generators, async concurrency

**Documentation:**
- Updated CI_GUIDE.md with demo workflow documentation
- Updated QUALITY_CHECKLIST.md with demo requirements
- Updated template READMEs with CI/CD integration instructions

**Impact:**
- Future tools inherit optimization patterns
- Systematic CI efficiency improvements
- Knowledge capture from 5 completed tools

### Added - File-Based Semaphore v0.1.0 (2025-12-25)

**Tool Features:**
- Cross-platform process locking via filesystem (Rust)
- First Rust tool in Tuulbelt
- CLI commands: acquire, release, check, info, list
- Stale lock detection and automatic cleanup
- Timeout support for lock acquisition
- Lock metadata (PID, hostname, timestamp)

**Testing Excellence:**
- 85 tests passing (31 unit + 39 CLI + 11 integration + 4 doctests)
- Zero clippy warnings, zero runtime dependencies
- Comprehensive CLI test coverage

**Documentation:**
- Complete SPEC.md defining lock file protocol and semantics
- Full VitePress documentation (7 pages)
- Two working examples (basic + advanced patterns)

**Impact:**
- First Rust tool in Tuulbelt
- Establishes SPEC.md pattern for protocol documentation
- Demonstrates comprehensive CLI testing approach
- 4/5 Phase 1 tools complete

### Added - Cross-Platform Path Normalizer v0.1.0 (2025-12-24)

**Tool Features:**
- Windows/Unix path conversion and normalization
- Multiple output formats (unix, windows, posix, native)
- CLI and library API
- Absolute/relative path handling
- Extension and component extraction

**Testing Excellence:**
- 145 tests across all edge cases
- Dogfooding validation (10 runs, 1,450 executions, 0 flaky)
- Comprehensive Windows/Unix path coverage

**Documentation:**
- Full VitePress documentation with examples
- StackBlitz integration for online demos
- Fuzzy test descriptions (removed hard-coded counts)

**Impact:**
- Third Tuulbelt tool complete (3/33, 9%)
- All tools now use fuzzy test descriptions
- 3/5 Phase 1 tools complete

### Added - CLI Progress Reporting v0.1.0 (2025-12-24)

**Tool Features:**
- File-based atomic writes for concurrent-safe progress tracking
- Multiple independent progress trackers (ID-based isolation)
- CLI interface with commands: init, increment, set, get, finish, clear
- Library API for programmatic usage
- Progress state persistence across process boundaries

**Testing Excellence:**
- 93 tests across 34 test suites
- Unit tests (35), CLI integration tests (28), filesystem tests (21), performance tests (9)
- Comprehensive edge case coverage
- Dogfooding validation with Test Flakiness Detector (100% pass rate, 0 flaky tests)

**Documentation:**
- Complete README with usage examples
- DOGFOODING.md establishing validation practice for all future tools
- Design patterns to prevent test flakiness
- CI integration examples

**Impact:**
- Second Tuulbelt tool complete (2/33, 6%)
- Establishes dogfooding as standard practice
- Demonstrates concurrent-safe file operations pattern
- Progress: Phase 1 Quick Tools 2/5 (40%)

### Added - Documentation Updates (2025-12-24)
- Session handoff documentation system (STATUS.md, CHANGELOG.md, ROADMAP.md)
- Tool-level STATUS.md and CHANGELOG.md templates in scaffolding
- Updated TOOL_DASHBOARD.md with both completed tools

## [0.2.0] - 2025-12-23

### Added - Claude Code Infrastructure

**Slash Commands:**
- `/test-all [filter]` - Run comprehensive test suite for TypeScript and Rust templates
- `/security-scan` - Comprehensive security analysis (dependencies, secrets, zero-deps validation)
- `/scaffold-tool <name> <lang>` - Create new tool from template with validation
- `/git-commit <type> <scope> <msg>` - Semantic commits following Conventional Commits

**Specialized Agents:**
- `test-runner` - Testing expert for test execution, debugging, and coverage analysis
- `security-reviewer` - Security analysis and vulnerability scanning specialist
- `scaffold-assistant` - Tool creation and template customization expert

**Skills:**
- `typescript-patterns` - TypeScript best practices (strict types, Result pattern, zero deps)
- `rust-idioms` - Rust idioms (ownership, borrowing, Result types, iterators)
- `zero-deps-checker` - Validates zero-dependency principle enforcement

**Quality Gate Hooks:**
- `pre-write-check.sh` - Protects sensitive files (lock files, .env, secrets)
- `post-write-format.sh` - Auto-formats code (prettier for TS, rustfmt for Rust)
- `post-bash-audit.sh` - Logs and audits all bash commands, warns of dangerous patterns
- `on-stop-cleanup.sh` - Cleanup temporary files, archive session logs

**Documentation:**
- `CLAUDE.md` - Main project context for Claude Code (200 lines)
- `docs/claude-code-implementation.md` - Complete infrastructure guide (900+ lines)

### Changed
- Updated `.gitignore` to exclude Claude Code local settings and logs

### Testing
- TypeScript template: All 12 tests passing
- Rust template: All 19 tests passing
- Infrastructure validated with both templates

### Infrastructure Impact
- All 33 planned tools will inherit these workflows
- Automated quality gates for formatting, security, testing
- Zero-dependency validation enforced automatically
- Security scanning before every commit
- Audit trail for all bash commands

## [0.1.0] - 2025-12-22

### Added - Initial Setup

**Meta Repository Structure:**
- Created base repository structure
- Established project organization

**Templates:**
- TypeScript tool template (`templates/tool-repo-template/`)
  - Node.js native test runner configuration
  - TypeScript strict mode setup
  - CI/CD workflow for GitHub Actions
  - Example implementation and tests
- Rust tool template (`templates/rust-tool-template/`)
  - Cargo configuration for lib and bin targets
  - Cross-platform CI/CD workflow
  - Example implementation and tests
  - Integration test setup

**Documentation:**
- `README.md` - Project overview and tool catalog (33 planned tools)
- `PRINCIPLES.md` - 10 design principles for tool development
- `ARCHITECTURE.md` - Repository structure and organization
- `CONTRIBUTING.md` - Contribution workflow and guidelines
- `docs/testing-standards.md` - Testing requirements (80% coverage minimum)
- `docs/security-guidelines.md` - Security checklist and best practices
- `docs/claude-code-workflow.md` - Claude Code development best practices
- Triage documentation (`docs/setup/TUULBELT_TRIAGE.md`) - Complexity matrix for all tools

**Development Standards:**
- Zero-dependency principle established
- Semantic versioning conventions
- Conventional Commits specification
- Cross-platform compatibility requirements

**GitHub Configuration:**
- Issue templates for bug reports, feature requests, tool proposals
- Pull request template
- CODEOWNERS file
- Meta validation CI/CD workflow

**Tools:**
- 0 tools implemented
- 33 tools planned (5 Quick, 28 Medium complexity)

### Infrastructure
- Git repository initialized
- GitHub Actions workflows configured
- Template testing validated
- Development environment documented

---

## Release Notes

### Version 0.2.0 - Claude Code Automation

This release adds comprehensive development automation through Claude Code, providing:

- **Standardized Workflows:** Same commands work across all tools
- **Quality Gates:** Automated formatting, security scanning, testing
- **Developer Productivity:** Reusable slash commands and specialized agents
- **Safety:** Protected files, audit logs, zero-dependency enforcement
- **Documentation:** Complete guides for all workflows

**ROI:** Saves 30-60 minutes per tool × 33 tools = 15-30+ hours total

### Version 0.1.0 - Foundation

Initial meta repository setup with:

- Production-ready templates for TypeScript and Rust
- Comprehensive documentation and guidelines
- CI/CD infrastructure
- Tool triage and planning (33 tools prioritized)

---

## Tool Changelog Links

Individual tool changelogs will be listed here as tools are developed.

*(No tools implemented yet - infrastructure complete, ready to build)*

---

## Migration Notes

### From 0.1.0 to 0.2.0

No breaking changes. New features:

1. **Slash commands available:** `/test-all`, `/security-scan`, `/scaffold-tool`, `/git-commit`
2. **Hooks active:** Auto-formatting on file save, bash command auditing
3. **Protected files:** Cannot modify lock files or .env files via Claude Code

If you have local changes to `.claude/settings.json`, create `.claude/settings.local.json` for overrides.

---

## Contributors

- @koficodedat - Initial setup and Claude Code infrastructure

---

*For detailed commit history, see: https://github.com/tuulbelt/tuulbelt/commits/main*
