# Tuulbelt Architecture

## Repo Organization

```
tuulbelt (meta repo)
├── README.md                    # Index of all tools, quick start
├── PRINCIPLES.md               # Design philosophy
├── CONTRIBUTING.md             # How to create/maintain tools
├── ARCHITECTURE.md             # This file
├── docs/
│   ├── claude-code-workflow.md # Claude Code best practices
│   ├── testing-standards.md    # Cross-tool testing patterns
│   └── security-guidelines.md  # Security considerations
├── templates/
│   ├── tool-repo-template/     # Copy this to start Node/TS tools
│   │   ├── src/index.ts        # Entry point
│   │   ├── test/index.test.ts  # Tests using Node test runner
│   │   ├── examples/basic.ts   # Usage example
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── LICENSE
│   │   └── .github/workflows/test.yml
│   └── rust-tool-template/     # Copy this to start Rust tools
│       ├── src/lib.rs
│       ├── tests/
│       ├── Cargo.toml
│       └── README.md
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   ├── feature_request.md
│   │   └── tool_proposal.md
│   └── workflows/
│       └── meta-validation.yml # Validates all tool repos exist
└── LICENSE                      # MIT for all tools
```

## Tool Repository Structure

Each tool follows this structure (copy from templates/):

```
<tool-name>/
├── README.md                  # What it does, why, how to use
├── SPEC.md                    # (optional) Formal spec for protocols/formats
├── ARCHITECTURE.md            # (optional) Design decisions for complex tools
├── src/
│   └── index.ts/js           # Main implementation
├── test/
│   └── *.test.ts/js          # All tests here
├── examples/
│   └── basic.ts/js           # Runnable usage example
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

## Dependency Model

```
tuulbelt (meta)
├── No code dependencies
├── References templates/
└── Links to individual tool repos (via README, not code)

Tool (e.g., structured-error-handler/)
├── Zero runtime dependencies
├── Dev dependencies: TypeScript, Node test runner only
└── Fully standalone, independently cloneable
```

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

---

## How Tools Are Discovered

1. **Meta repo README** — Index of all tools with status
2. **GitHub org** — Browse all repos
3. **GitHub search** — By name or problem

---

## Migration Path

If a tool becomes too broad (risk of becoming a framework):

1. Split into multiple, focused tools
2. Document the split in both tools' READMEs
3. Update meta repo README

Example: If "Config Merger" becomes "Config Merger + Validator + Transformer", split into three tools.

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
