# Tuulbelt Meta Repository Changelog

All notable changes to the Tuulbelt meta repository will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Session handoff documentation system (STATUS.md, CHANGELOG.md, ROADMAP.md)
- Tool-level STATUS.md and CHANGELOG.md templates in scaffolding

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
- 1 tool implemented: Structured Error Handler
- 32 tools planned (5 Quick, 27 Medium complexity)

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

Individual tool changelogs:

- [Structured Error Handler](https://github.com/tuulbelt/structured-error-handler/blob/main/CHANGELOG.md) - v1.0.0

*(More tool links will be added as tools are developed)*

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
