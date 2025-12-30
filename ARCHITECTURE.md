# Tuulbelt Architecture

## Repo Organization

```
tuulbelt (meta repo)
├── README.md                    # Index of all tools, quick start
├── PRINCIPLES.md               # Design philosophy
├── CONTRIBUTING.md             # How to create/maintain tools
├── ARCHITECTURE.md             # This file
├── tools/                      # Git submodules (standalone tool repos)
│   ├── test-flakiness-detector/     → github.com/tuulbelt/test-flakiness-detector
│   ├── cli-progress-reporting/      → github.com/tuulbelt/cli-progress-reporting
│   ├── cross-platform-path-normalizer/ → github.com/tuulbelt/cross-platform-path-normalizer
│   ├── config-file-merger/          → github.com/tuulbelt/config-file-merger
│   ├── structured-error-handler/    → github.com/tuulbelt/structured-error-handler
│   ├── file-based-semaphore/        → github.com/tuulbelt/file-based-semaphore (Rust)
│   ├── file-based-semaphore-ts/     → github.com/tuulbelt/file-based-semaphore-ts
│   ├── output-diffing-utility/      → github.com/tuulbelt/output-diffing-utility (Rust)
│   ├── snapshot-comparison/         → github.com/tuulbelt/snapshot-comparison (Rust)
│   └── port-resolver/               → github.com/tuulbelt/port-resolver
├── docs/
│   ├── QUALITY_CHECKLIST.md     # Pre-commit quality checks
│   ├── testing-standards.md     # Cross-tool testing patterns
│   ├── security-guidelines.md   # Security considerations
│   └── CLEANUP_PLAN.md          # Repository cleanup strategy
├── templates/
│   ├── tool-repo-template/      # TypeScript/Node.js tool template
│   │   ├── src/index.ts         # Entry point
│   │   ├── test/index.test.ts   # Tests using Node test runner
│   │   ├── examples/basic.ts    # Usage example
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── LICENSE
│   │   └── .github/workflows/test.yml
│   └── rust-tool-template/      # Rust tool template
│       ├── src/lib.rs
│       ├── tests/
│       ├── Cargo.toml
│       └── README.md
├── scripts/
│   ├── setup-github-auth.sh     # Configure GitHub authentication
│   ├── commit.sh                # Commit with correct author
│   └── push.sh                  # Push with pre-push validation
├── .claude/
│   ├── commands/                # Slash commands (/new-tool, /quality-check, etc.)
│   ├── skills/                  # Cross-cutting expertise
│   ├── agents/                  # Specialized AI agents
│   └── hooks/                   # Quality gates
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   ├── feature_request.md
│   │   └── tool_proposal.md
│   └── workflows/
│       ├── meta-validation.yml  # Validates meta repo integrity
│       ├── create-demos.yml     # Auto-generates demo recordings
│       └── deploy-docs.yml      # Deploys VitePress to GitHub Pages
└── LICENSE                      # MIT for all tools
```

## Git Submodules

**All tools are git submodules** in the `tools/` directory. Each submodule:
- Points to a standalone GitHub repository
- Can be cloned and used independently
- Has its own version, CI/CD, and release cycle
- References other tools via git URL dependencies (not path dependencies)

**Working with submodules:**
```bash
# Clone meta repo with all tools
git clone --recursive https://github.com/tuulbelt/tuulbelt.git

# Initialize submodules after clone
git submodule update --init --recursive

# Update all submodules to latest
git submodule update --remote

# Update specific tool
cd tools/test-flakiness-detector
git pull origin main
cd ../..
git add tools/test-flakiness-detector
git commit -m "chore: update test-flakiness-detector submodule"
```

## Tool Repository Structure

Each tool is a standalone repository following this structure:

```
<tool-name>/
├── README.md                  # What it does, why, how to use
├── CLAUDE.md                  # Tool-specific Claude Code context
├── SPEC.md                    # (optional) Formal spec for protocols/formats
├── ARCHITECTURE.md            # (optional) Design decisions for complex tools
├── DOGFOODING_STRATEGY.md     # (optional) How tool uses other Tuulbelt tools
├── src/
│   └── index.ts/js           # Main implementation
├── test/
│   └── *.test.ts/js          # All tests here
├── examples/
│   └── basic.ts/js           # Runnable usage example
├── docs/
│   └── demo.gif              # Demo recording (auto-generated)
├── scripts/
│   └── dogfood-*.sh          # Dogfooding scripts
├── package.json              # (Node/TS only)
├── tsconfig.json             # (Node/TS only)
├── Cargo.toml                # (Rust only)
├── Cargo.lock                # (Rust only)
├── .gitignore
├── LICENSE                    # MIT
└── .github/
    └── workflows/
        └── test.yml          # Runs tests on every push/PR
```

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

## Dependency Model

```
tuulbelt (meta)
├── No code dependencies
├── Contains templates for scaffolding
├── References tools via git submodules (tools/)
└── Links to tools in README, not code

Tool (e.g., test-flakiness-detector/)
├── Zero external runtime dependencies
├── MAY depend on other Tuulbelt tools via git URL
│   Example: "cli-progress-reporting": "git+https://github.com/tuulbelt/cli-progress-reporting.git"
├── Dev dependencies: TypeScript, test runners only
└── Fully standalone, independently cloneable
```

## Tool Creation Workflow

**Use `/new-tool` command:**
```bash
# In Claude Code
/new-tool <tool-name> <typescript|rust>
```

This command:
1. Creates standalone GitHub repository
2. Scaffolds from template (tool-repo-template or rust-tool-template)
3. Configures CI/CD, metadata, documentation
4. Tags v0.1.0 and pushes to GitHub
5. Adds as git submodule to meta repo

**Manual workflow (not recommended):**
1. Create GitHub repo manually
2. Copy appropriate template
3. Customize package.json/Cargo.toml
4. Set up CI/CD manually
5. Add as submodule: `git submodule add <url> tools/<tool-name>`

## Tool Lifecycle

```
Proposal (issue) → Implementation → Testing → Stable (1.0.0) → Maintenance

0.1.0 - Initial release (unstable)
0.2.0 - Features added
1.0.0 - API stable, production-ready
1.1.0 - Bug fixes
2.0.0 - Breaking changes
```

## Language Support

| Language | Use Case | Template |
|----------|----------|----------|
| TypeScript/Node.js | CLI tools, utilities, web frontends | `templates/tool-repo-template/` |
| Rust | Systems utilities, protocols, performance-critical | `templates/rust-tool-template/` |
| (others) | Only if problem requires it | Create template first |

## Testing Strategy

All tools must have:

1. **Unit tests** — Core logic, edge cases
2. **Integration tests** — CLI behavior
3. **CI/CD** — Tests run on every push/PR
4. **Coverage target** — 80%+ for core logic

Use language-native test runners:
- Node.js: `node --test`
- Rust: `cargo test`

See @docs/testing-standards.md for complete requirements.

## Maintenance Model

Each tool has a primary maintainer:
- Responds to issues
- Reviews PRs
- Makes releases
- Updates docs

Falls back to meta repo maintainer if unmaintained for 3+ months.

## Versioning

- **Meta repo:** No version (always latest)
- **Each tool:** Semantic versioning (0.1.0, 1.0.0, etc.)
- **Release cadence:** As needed, no fixed schedule

## Security Model

- **No external deps** = fewer supply chain attacks
- **Code review required** for all changes
- **Secrets never in code** — tools that handle secrets use encryption
- **Open by default** — all repos public, no private dependencies

See @docs/security-guidelines.md for complete checklist.

---

## How Tools Are Discovered

1. **Meta repo README** — Index of all tools with status
2. **GitHub Pages** — https://tuulbelt.github.io/tuulbelt/
3. **GitHub org** — Browse all repos at https://github.com/tuulbelt
4. **GitHub search** — By name or problem

---

## Migration Path

If a tool becomes too broad (risk of becoming a framework):

1. Split into multiple, focused tools
2. Document the split in both tools' READMEs
3. Update meta repo README
4. Use git URL dependencies for composition

Example: If "Config Merger" becomes "Config Merger + Validator + Transformer", split into three tools and use git URLs to compose.

---

## Tools vs Frameworks

```
Tool = solves one problem, user controls composition
Framework = controls entire application lifecycle

Examples:

TOOL (good):           FRAMEWORK (bad):
- Config merger        - Full config + validation + secrets + env management system
- Test detector        - Complete test framework with fixtures, mocks, reporters
- Diffing utility      - Assertion library with custom syntax
```

If you're building something that dictates *how* the user structures their app, it's probably too big for Tuulbelt.
