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

## Quick Commands

```bash
# MANDATORY pre-commit quality checks
/quality-check

# Create new tool from template
/new-tool <tool-name> <typescript|rust>

# Run all tests across templates
/test-all [filter]

# Security analysis
/security-scan

# Semantic git commit
/git-commit <type> <scope> <message>

# Session management
/handoff
/resume-work
```

See @.claude/commands/ for complete command documentation.

---

## Project Overview

Tuulbelt is a meta-repository for curating focused, zero-dependency tools and utilities. Each tool is a standalone GitHub repository, referenced as a git submodule in `tools/`.

This meta repository provides:
- Scaffolding templates for TypeScript and Rust tools
- Standardized development workflows
- Cross-cutting quality assurance (testing, security, linting)
- Claude Code automation for consistent development

See @README.md for detailed architecture and @PRINCIPLES.md for design philosophy.

---

## Essential References

**Core Documentation:**
- @README.md - Project architecture
- @PRINCIPLES.md - Design philosophy
- @ARCHITECTURE.md - Repository structure, tech stack
- @CONTRIBUTING.md - Contribution workflow

**Quality & Standards:**
- @docs/QUALITY_CHECKLIST.md - **Pre-commit quality checks**
- @docs/testing-standards.md - Testing requirements
- @docs/security-guidelines.md - Security checklist
- @.claude/rules/code-style.md - Code examples and patterns

**Session Continuity:**
- @.claude/HANDOFF.md - **Session handoff (session → session continuity)**
- @.claude/NEXT_TASKS.md - **Task backlog and priorities**
- @docs/KNOWN_ISSUES.md - **Tracked bugs and known issues**

**Commands & Automation:**
- @.claude/commands/ - All slash command documentation
- @.claude/skills/ - Cross-cutting expertise (typescript-patterns, rust-idioms, zero-deps-checker)
- @.claude/agents/ - Specialized AI agents

**Templates:**
- @templates/tool-repo-template/ - TypeScript template
- @templates/rust-tool-template/ - Rust template

---

*This CLAUDE.md file is automatically included in Claude Code's context for all development in this repository.*
