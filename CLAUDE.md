# Tuulbelt Meta Repository

## CRITICAL: Claude Code Work Standards

**When working in this repository, you MUST:**

1. **DO NOT HALLUCINATE ISSUES OR SUCCESS**
   - Never claim something is complete without verifying it exists
   - Never report test failures or successes without actually running tests
   - Always verify file contents before making claims about them
   - If uncertain, check the actual state - don't assume

2. **DO NOT ADD FLUFF**
   - No unnecessary commentary or verbose explanations
   - No marketing language or superlatives
   - Stick to facts: what exists, what works, what doesn't
   - Code and documentation should be concise and purposeful

3. **BE PRAGMATIC AND SERIOUS ABOUT THE WORK**
   - This is production infrastructure for 33 tools
   - Follow Tuulbelt principles strictly (zero deps, single problem, etc.)
   - Question requirements that violate principles
   - Focus on what actually works, not what would be cool

4. **DOUBLE CHECK YOUR WORK AND TEST THOROUGHLY**
   - Run tests before claiming they pass
   - Verify files exist before referencing them
   - Test commands before documenting them
   - Ensure STATUS.md accurately reflects reality
   - Have the upmost confidence in your work before committing

5. **RUN QUALITY CHECKS BEFORE EVERY COMMIT**
   - Execute `/quality-check` before committing code
   - See @docs/QUALITY_CHECKLIST.md for complete checklist
   - Build must succeed, tests must pass 100%, zero runtime dependencies

6. **ALWAYS USE TodoWrite FOR MULTI-STEP TASKS**
   - Any task with 3+ steps MUST use TodoWrite to track progress
   - Do NOT mark work as complete until ALL todos are checked
   - For new tools, use the checklist from @docs/QUALITY_CHECKLIST.md
   - Update todo status in real-time as work progresses
   - This prevents missing critical steps (like GitHub Pages integration)

**Violation of these standards compromises the entire project. When in doubt, verify.**

---

## MANDATORY WORKFLOW (Checkpoint-Based Enforcement)

**READ THIS SECTION EVERY SESSION. These are not suggestions - they are required checkpoints.**

### ✓ Before Starting ANY Multi-Step Task (3+ steps):
- [ ] **CREATE TodoWrite checklist** from `@docs/QUALITY_CHECKLIST.md`
- [ ] **DO NOT proceed** without checklist in place
- [ ] Verify checklist covers: implementation, testing, documentation, quality checks

### ✓ During Implementation (After Each Major Section):
- [ ] **Mark TodoWrite item as `in_progress`** BEFORE starting work
- [ ] **STOP when section complete** → READ what you created → VERIFY against reference
- [ ] **Compare against existing tools** (use test-flakiness-detector as reference for docs)
- [ ] **Mark `completed` ONLY IF**: tests pass, docs exist, quality standards met
- [ ] **If incomplete** → DO NOT mark done, DO NOT move to next section

### ✓ Before EVERY Commit:
- [ ] **Run `/quality-check`** - build, tests, zero deps
- [ ] **All TodoWrite items marked `completed`** (no `in_progress` or `pending`)
- [ ] **If ANY item incomplete** → DO NOT commit, finish work first
- [ ] **Read `@docs/QUALITY_CHECKLIST.md`** and verify compliance

### ✓ For New Tool Documentation (CRITICAL):
- [ ] **Read existing tool docs FIRST** (test-flakiness-detector, cli-progress-reporting)
- [ ] **Verify line counts comparable** (library-usage ~300-400 lines, examples ~300-500 lines)
- [ ] **Check BOTH locations exist**:
  - GitHub Pages: `docs/tools/{tool-name}/`
  - Local VitePress (if applicable): `{tool-name}/docs/`
- [ ] **Verify Demo sections match structure**:
  - demo.gif image reference
  - asciinema interactive recording link
  - StackBlitz "Try it online" button
  - Description text
  - Automation note
- [ ] **All internal links work** (no dead links in sidebar)

### ✓ Zero Hallucination Enforcement:
- **Never claim tests pass** → Must run tests and show actual output
- **Never claim files exist** → Must read files and verify contents
- **Never claim work complete** → Must verify against TodoWrite checklist
- **If uncertain about ANYTHING** → CHECK the actual state, don't assume

### ✓ Honest Status Reporting:
- **If tests fail** → Report failure, don't mark as complete
- **If docs incomplete** → Report what's missing, don't mark as complete
- **If quality issues exist** → Report them, add to blockers in HANDOFF.md
- **If you discover issues in already-completed work** → Report immediately, fix before proceeding

**REMEMBER: This workflow prevents after-the-fact fixes. Follow it EVERY time.**

---

## Project Overview

Tuulbelt is a meta-repository for curating focused, zero-dependency tools and utilities for modern software development. Each tool solves one specific problem and is maintained as an independent repository.

This meta repository provides:
- Scaffolding templates for TypeScript and Rust tools
- Standardized development workflows
- Cross-cutting quality assurance (testing, security, linting)
- Claude Code automation for consistent development

See @README.md for detailed architecture and @PRINCIPLES.md for design philosophy.

## Tech Stack

**TypeScript Tools:**
- Runtime: Node.js 18+
- Language: TypeScript 5.3+ (strict mode)
- Test Framework: Node.js native test runner (`node:test`)
- Assertions: Node.js native (`node:assert/strict`)
- Dev Dependencies: TypeScript, tsx, @types/node
- **Zero Runtime Dependencies**

**Rust Tools:**
- Rust Edition: 2021+
- Test Framework: Built-in cargo test
- Linter: clippy (all warnings denied)
- Formatter: rustfmt
- **Zero Runtime Dependencies**

**Development Tools:**
- Git with conventional commits
- GitHub Actions for CI/CD
- npm for TypeScript package management
- cargo for Rust package management

## Project Structure

```
/home/user/tuulbelt/
├── .claude/                      # Claude Code workflows & automation
│   ├── commands/                 # Reusable slash commands
│   │   ├── test-all.md          # Run all tests (TS + Rust)
│   │   ├── quality-check.md     # Pre-commit quality checks
│   │   ├── security-scan.md     # Security analysis
│   │   ├── scaffold-tool.md     # Create new tool from template
│   │   └── git-commit.md        # Semantic commits
│   ├── rules/                   # Code style and conventions
│   │   └── code-style.md        # TS/Rust code examples
│   ├── agents/                  # Specialized AI agents
│   ├── skills/                  # Cross-cutting expertise
│   └── hooks/                   # Quality gates & automation
├── templates/                   # Tool scaffolding templates
│   ├── tool-repo-template/     # TypeScript/Node.js
│   └── rust-tool-template/     # Rust
├── docs/                       # Development documentation
│   ├── QUALITY_CHECKLIST.md    # Pre-commit quality checks
│   ├── testing-standards.md    # Test requirements
│   └── security-guidelines.md  # Security checklist
├── CLAUDE.md                   # This file
├── README.md                   # Project architecture
├── PRINCIPLES.md               # Design philosophy
└── CONTRIBUTING.md             # Contribution workflow
```

## Quick Commands

### Development Workflows

```bash
# Pre-commit quality checks (MANDATORY before commit)
/quality-check

# Create new tool from template
/scaffold-tool <tool-name> <typescript|rust>

# Migrate existing tool to standalone repo (Phase 2 meta repo migration)
/migrate-tool <tool-name>

# Run all tests across templates
/test-all [filter]

# Security analysis
/security-scan

# Semantic git commit
/git-commit <type> <scope> <message>
```

### TypeScript Development

```bash
# In any tool repository using TypeScript template
npm install          # Install dev dependencies
npm test             # Run tests
npm run build        # TypeScript compilation
npx tsc --noEmit     # Type check only
```

### Rust Development

```bash
# In any tool repository using Rust template
cargo test                   # Run tests
cargo check                  # Check without building
cargo fmt                    # Format code
cargo clippy -- -D warnings  # Lint (zero warnings)
cargo build --release        # Build release
```

## Code Conventions

See @.claude/rules/code-style.md for detailed code examples and patterns.

**TypeScript:**
- ES modules only (`import`, not `require()`)
- Use `node:` prefix for built-ins: `import { readFileSync } from 'node:fs'`
- Explicit return types on all exported functions
- Result pattern for error handling: `Result<T> = { ok: true, value: T } | { ok: false, error: Error }`
- Include `@types/node` in devDependencies
- Zero `any` types (use `unknown` and type guards)

**Rust:**
- Use `?` operator for error propagation (avoid `unwrap()`)
- Explicit error types with `Result<T, E>`
- Run `cargo fmt` and `cargo clippy -- -D warnings` before committing
- All public items documented with `///`

## Testing

**Coverage Requirements:**
- Minimum 80% line coverage for all tools
- 90% coverage for critical paths (data processing, error handling)
- Edge cases required (empty input, malformed data, large inputs)

**Test Organization:**
- TypeScript: Colocated tests (`src/index.ts` + `test/index.test.ts`)
- Rust: Unit tests in `#[cfg(test)]` modules, integration tests in `tests/`

See @docs/testing-standards.md for complete requirements.

## Security

**Protected Files (enforced by hooks):**
- `package-lock.json`, `Cargo.lock` (use package managers)
- `.env*`, `*.secret.*`, `*.private.*` (sensitive data)

**Security Checklist:**
- No hardcoded credentials
- All external input validated
- Path traversal prevention
- Error messages contain no sensitive data

See @docs/security-guidelines.md and run `/security-scan` before releases.

## Zero Dependencies Principle

**Every Tuulbelt tool must have ZERO runtime dependencies.**

**Allowed:**
- Standard library (Node.js built-ins, Rust std)
- Dev dependencies (TypeScript, test runners, linters)

**Not Allowed:**
- npm packages in `dependencies` (devDependencies OK)
- External crates in `[dependencies]` (dev-dependencies OK)

**Validation:**
```bash
# Verify zero runtime deps
/quality-check  # Automated check
```

The `zero-deps-checker` skill validates this automatically.

## Git Workflow

**Conventional Commits:**
```
<type>(<scope>): <description>
```

**Types:** `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `ci`, `perf`

**Examples:**
```bash
feat(cli): add verbose logging option
fix(core): handle edge case in path normalization
test(core): add coverage for error conditions
```

Use `/git-commit <type> <scope> <message>` for validation.

See @CONTRIBUTING.md for full contribution workflow.

## Available Workflows

### Slash Commands

- `/quality-check` - **MANDATORY** pre-commit checks (build, tests, zero deps)
- `/test-all [filter]` - Run TypeScript and Rust tests
- `/security-scan` - Comprehensive security analysis
- `/scaffold-tool <name> <lang>` - Create new tool repository
- `/migrate-tool <tool-name>` - **Migrate monorepo tool to standalone repo** (Phase 2)
- `/git-commit <type> <scope> <msg>` - Semantic commit with validation
- `/handoff` - Create session handoff for next session
- `/resume-work` - Resume from previous session handoff

### Specialized Agents

- `test-runner` - Test execution, coverage, debugging
- `security-reviewer` - Security analysis, vulnerability scanning
- `scaffold-assistant` - Tool creation, template customization
- `session-manager` - Session transitions, task planning, complex handoffs

### Skills

- `typescript-patterns` - TypeScript best practices enforcement
- `rust-idioms` - Rust best practices enforcement
- `zero-deps-checker` - Validate zero-dependency principle

## Quality Gates (Hooks)

**Pre-Write:**
- Prevent modification of protected files

**Post-Write:**
- Auto-format with prettier (TypeScript)
- Auto-format with rustfmt (Rust)

**Post-Bash:**
- Audit dangerous commands (rm -rf, etc.)
- Log all command executions

## Common Workflows

### Migrating a Tool to Standalone Repository (Phase 2)

**CRITICAL: Use `/migrate-tool` for all Phase 2 migrations**

1. Configure authentication: `source scripts/setup-github-auth.sh`
2. Verify tool is complete: `cd {tool-name} && npm test`
3. Run migration: `/migrate-tool {tool-name}`
4. Verify standalone: Fresh clone from GitHub, run tests
5. Start fresh session for next tool

**The `/migrate-tool` command automates:**
- Git history extraction (git subtree split)
- GitHub repository creation
- Metadata updates (package.json, CI, README, CLAUDE.md)
- Commit and tag v0.1.0 with correct author
- Git submodule addition to meta repo
- Tracking document updates (HANDOFF.md, STATUS.md, CHANGELOG.md, NEXT_TASKS.md)

**See:** @.claude/commands/migrate-tool.md for complete details

### Creating a New Tool

1. Scaffold: `/scaffold-tool my-tool typescript`
2. Customize implementation
3. Validate: `/quality-check` and `/security-scan`
4. Commit: `/git-commit feat core "implement main functionality"`

### Before Every Commit

1. Run `/quality-check` (build, tests, zero deps)
2. Review checklist: @docs/QUALITY_CHECKLIST.md
3. Fix any issues discovered
4. Commit with `/git-commit`

### Security Review Before Release

1. Run `/security-scan`
2. Address findings
3. Re-run until clean
4. Proceed with release

## References

- @README.md - Project architecture
- @PRINCIPLES.md - Design philosophy
- @CONTRIBUTING.md - Contribution workflow
- @docs/QUALITY_CHECKLIST.md - **Pre-commit quality checks**
- @docs/testing-standards.md - Testing requirements
- @docs/security-guidelines.md - Security checklist
- @docs/KNOWN_ISSUES.md - **Tracked bugs and known issues**
- @docs/MIGRATION_TO_META_REPO.md - **Meta repository migration plan**
- @.claude/HANDOFF.md - **Session handoff (session → session continuity)**
- @.claude/NEXT_TASKS.md - **Task backlog and priorities**
- @.claude/commands/migrate-tool.md - **Tool migration automation**
- @.claude/rules/code-style.md - Code examples and patterns
- @templates/tool-repo-template/ - TypeScript template
- @templates/rust-tool-template/ - Rust template

## Known Limitations

- Templates assume Unix-like environment (Linux, macOS)
- Windows support requires WSL or Git Bash
- Cross-platform path handling must be explicit in tools
- CI/CD assumes GitHub Actions (adaptable to other platforms)

---

*This CLAUDE.md file is automatically included in Claude Code's context for all development in this repository.*
