# Tuulbelt Meta Repository Changelog

All notable changes to the Tuulbelt meta repository will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added - Migration Automation: /migrate-tool Command ✅ (2025-12-29)

**Created comprehensive automation for Phase 2 migrations:**
- New slash command: `/migrate-tool <tool-name>`
- Command automates entire migration workflow (12 steps)
- Documentation: `.claude/commands/migrate-tool.md` (289 lines)

**Automation capabilities:**
- Git history extraction via `git subtree split`
- GitHub repository creation via gh CLI
- Metadata updates: package.json/Cargo.toml, CI workflow, README.md, CLAUDE.md
- Commit and tag v0.1.0 with correct author credentials (scripts/commit.sh)
- Git submodule addition to meta repo
- Tracking document updates: HANDOFF.md, STATUS.md, CHANGELOG.md, NEXT_TASKS.md
- Standalone verification: Clone, test, build

**Documentation updates:**
- CLAUDE.md: Added /migrate-tool to Quick Commands and Slash Commands sections
- CLAUDE.md: Created "Migrating a Tool to Standalone Repository" workflow
- docs/MIGRATION_TO_META_REPO.md: Added automation reference to migration template

**Impact:**
- Eliminates ~30-45 minutes of manual work per tool
- Ensures consistency across all migrations
- Prevents errors from manual steps
- Ready for remaining 6 Wave 1 tools

**Established pattern:**
- cli-progress-reporting migrated manually (proved workflow)
- Remaining tools will use automated /migrate-tool command

### Added - Phase 2 Wave 1: cli-progress-reporting Migration Complete ✅ (2025-12-29)

**First Tool Migrated to Standalone Repository:**
- Created GitHub repository: https://github.com/tuulbelt/cli-progress-reporting
- Extracted 58 commits with full git history using `git subtree split`
- Updated for standalone use:
  - package.json: Added homepage, bugs, repository URLs
  - CI workflow: Standalone test.yml (Node 18, 20, 22, zero-dep check)
  - README.md: Updated badges, converted relative links to absolute GitHub URLs
  - Created CLAUDE.md for tool-specific development context
- Tagged v0.1.0 in new repository
- Verified standalone functionality: 121/121 tests passing

**Authentication Infrastructure:**
- Created scripts/setup-github-auth.sh - Configure authentication from .env
- Created scripts/commit.sh - Commit with correct author, no Claude attribution
- Created scripts/push.sh - Push with correct credentials
- All scripts source GITHUB_TOKEN, GITHUB_USERNAME, GITHUB_EMAIL from meta repo .env

**Git Submodule Integration:**
- Added as git submodule: tools/cli-progress-reporting
- References: https://github.com/tuulbelt/cli-progress-reporting (v0.1.0)
- Tracking documents updated: HANDOFF.md, NEXT_TASKS.md, CHANGELOG.md

**Impact:**
- First tool FULLY migrated (1/7 Wave 1 tools complete)
- Established migration workflow for remaining tools
- Authentication pattern established for all future migrations
- Ready to migrate remaining 6 tools using /migrate-tool

### Added - Phase 2 Wave 1: cross-platform-path-normalizer Migration Complete ✅ (2025-12-29)

**Second Tool Migrated to Standalone Repository:**
- Created GitHub repository: https://github.com/tuulbelt/cross-platform-path-normalizer
- Extracted 457 commits with full git history using `git subtree split`
- Updated for standalone use:
  - package.json: Added homepage, bugs, repository URLs
  - CI workflow: Standalone test.yml (Node 18, 20, 22, zero-dep check)
  - README.md: Updated badges to point to standalone repo
  - Created CLAUDE.md for tool-specific development context
- Tagged v0.1.0 in new repository
- Verified standalone functionality: 141/141 tests passing

**GitHub Repository Configuration:**
- Disabled issues (all issues go to meta repo)
- Disabled wiki and projects
- Added topics: tuulbelt, path, cross-platform, windows, unix, normalize, typescript, zero-dependencies
- Set description: "Cross-platform path normalization utility for Windows/Unix path consistency"

**Git Submodule Integration:**
- Added as git submodule: tools/cross-platform-path-normalizer
- References: https://github.com/tuulbelt/cross-platform-path-normalizer (v0.1.0)
- Tracking documents updated: HANDOFF.md, NEXT_TASKS.md, CHANGELOG.md

**Impact:**
- Second tool FULLY migrated (2/7 Wave 1 tools complete - 28%)
- Migration automation validated and working
- Ready for remaining 5 Wave 1 tools

### Added - Phase 1: Meta Repo Migration Automation Complete ✅ (2025-12-29)

**MCP Server for GitHub Operations:**
- Created `.claude/mcp/tuulbelt-github/` MCP server (420 lines)
- 6 GitHub API tools: check_repo_exists, create_tool_repo, configure_repo_settings, create_github_labels, get_repo_info, delete_repo
- Integration with Octokit for GitHub API access
- Environment variable configuration via `.mcp.json`

**Slash Commands:**
- `/new-tool` - Complete tool creation automation (30+ steps, 350 lines)
- `/release-tool` - Semver version management (280 lines)
- `/add-tool-dependency` - Git URL dependency management (320 lines)
- `/sync-tool-docs` - Documentation synchronization (240 lines)
- `/update-all-counts` - Tool count updates across docs (310 lines)

**Specialized Agent:**
- `tool-creator` - Full Tuulbelt context for tool creation (600 lines)
- Phase-by-phase workflow execution
- Error handling and rollback procedures
- Template customization for TypeScript and Rust

**Environment Setup:**
- `.env.example` template for developers
- `.mcp.json` configuration for MCP server
- GitHub token and org configuration

**Testing Completed:**
- ✅ Verified check_repo_exists with tuulbelt/tuulbelt
- ✅ Created test repository (tuulbelt/test-dummy-tool)
- ✅ Verified repository settings and labels
- ✅ Successfully deleted test repository
- Full /new-tool workflow deferred to actual tool creation

**Impact:**
- Complete automation infrastructure for meta repo migration
- 11 files created (~4,200 lines total)
- Ready for Phase 2: Wave 1 tool migration
- Testing validated all GitHub API operations

### Added - Phase 0: Meta Repo Migration Preparation Complete ✅ (2025-12-29)

**Documentation Updates:**
- Created ROADMAP.md with accurate 10/33 tool status
- Updated .github/ISSUE_TEMPLATE/tool_proposal.md (fixed PRINCIPLES.md link)
- Updated docs/guide/getting-started.md (tool count 10/33, added 7 missing tools)
- Updated docs/guide/philosophy.md (progress 10/33)

**Strategic Decisions Documented:**
- Created docs/MIGRATION_DECISIONS.md
- Centralized issue tracking strategy (all issues in meta repo)
- Post-migration git tagging strategy
- Created docs/setup/TOOL_REPO_SETTINGS.md template

**Template Updates:**
- Added CONTRIBUTING.md to templates/tool-repo-template/
- Added CONTRIBUTING.md to templates/rust-tool-template/
- Versioning section added to both templates

**Impact:**
- 4 files modified, 4 files created
- All preparation tasks complete for migration
- Strategic decisions documented for consistency

### Changed - Dogfood Strategy Rethink (2025-12-28)

**Dogfood is now local-only:**
- Removed `dogfood-validation.yml` workflow (CI artifacts can't preserve dev environment)
- Tests are validated by `test-all-tools.yml` in CI
- `/quality-check` command now runs dogfood scripts during local verification
- Dogfood scripts remain for local development in monorepo context

**Root README:**
- 🐕 badges remain (now indicate local dogfood scripts exist)

**Documentation Updates:**
- `CI_GUIDE.md`: Removed dogfood-validation section, updated architecture diagram
- `QUALITY_CHECKLIST.md`: Changed "CI Integration" → "Local Development Only"
- Template READMEs: Clarified dogfood is local-only
- `scaffold-tool.md`: Removed CI automation references

### Added - File-Based Semaphore (TypeScript) v0.1.0 (2025-12-28)

**Tool Features:**
- Cross-platform process locking (TypeScript port of Rust `sema`)
- Atomic locking via temp file + rename pattern
- Compatible with Rust `sema` lock file format (cross-language interop)
- CLI commands: acquire, release, status, clean-stale
- Timeout support with configurable wait intervals
- Stale lock detection and automatic cleanup

**Security Hardening:**
- Path traversal prevention (check `..` before normalization)
- Symlink resolution (including dangling symlinks)
- Tag sanitization (remove all control characters)
- Cryptographic randomness for temp file names
- Orphaned temp file cleanup

**Testing Excellence:**
- 160 tests passing (52 unit + 26 security + 31 CLI + 36 edge + 15 stress)
- Comprehensive security test coverage
- Cross-language dogfood validation with Rust `odiff`

**Documentation:**
- Complete README with CLI and library usage
- SPEC.md (same lock file format as Rust sema)
- 7 VitePress pages + demo recording script

**Impact:**
- 9th Tuulbelt tool complete (27% of 33 total)
- First TypeScript tool with cross-language interop
- Demonstrates security hardening patterns

### Fixed - npm link and Demo Recording (2025-12-27)

**npm link Support for Short CLI Names:**
- Added shebang `#!/usr/bin/env -S npx tsx` to all 5 TypeScript entry points
- Without shebang, `npm link` symlinks fail with "import: command not found"
- Short names (`flaky`, `prog`, `normpath`, `serr`) now work after `npm link`
- Added bin entry and shebang to scaffold template for future tools
- Fixed entry point detection to resolve symlinks with `realpathSync()` before comparing paths

**Demo Recording Workflow Fixes:**
- Fixed race condition in create-demos.yml (rebase conflicts from build artifacts)
- Added `git checkout -- .` and `git clean -fd` before `git pull --rebase`
- Updated all 4 TypeScript demo scripts to use `npm link --force` + actual short names
- Reset all demo content (GIFs, asciinema URLs, cast files) to placeholders
- Workflow will regenerate all 6 demos on merge with correct short CLI names

**Template and Documentation Updates:**
- Updated TypeScript and Rust template READMEs to document both CLI forms
- Both short and long CLI names work; short form recommended
- Added "CLI names documented" check to `docs/QUALITY_CHECKLIST.md`
- Updated `.claude/commands/scaffold-tool.md` with instructions for both languages
- Added shebang requirement and pitfall entry to quality checklist
- Updated tracking documents (STATUS.md, HANDOFF.md, CHANGELOG.md)

### Fixed - Demo and Documentation (2025-12-26)

**Demo Color Implementation:**
- Fixed output-diffing-utility demo script to use `--color always` (was missing value argument)
- Updated README color examples to show correct syntax (`--color always` and `--color auto`)
- Verified ANSI color codes work for text, JSON, and binary diffs

**VitePress Demo Links:**
- Updated all 5 tool VitePress docs with correct asciinema URLs
- Fixed create-demos.yml workflow sed pattern to match ANY asciinema URL (not just `#` placeholder)
- Pattern now: `s|asciinema\.org/a/[^)]*|asciinema.org/a/$NEW_URL|` (prevents silent failures)

**Workflow Improvements:**
- Demo links will now update correctly on future regenerations
- No more stale placeholder URLs in documentation

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
