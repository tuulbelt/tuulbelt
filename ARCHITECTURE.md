# Tuulbelt Architecture

Tuulbelt is a **software ecosystem** organized into categories, each serving a different purpose in the software development stack.

## Ecosystem Organization

```
tuulbelt (meta repo)
├── README.md                    # Ecosystem overview
├── PRINCIPLES.md                # Design philosophy (category-specific)
├── CONTRIBUTING.md              # How to create/maintain projects
├── ARCHITECTURE.md              # This file
│
├── tools/                       # CLI utilities (git submodules)
│   ├── test-flakiness-detector/     → github.com/tuulbelt/test-flakiness-detector
│   ├── cli-progress-reporting/      → github.com/tuulbelt/cli-progress-reporting
│   ├── property-validator/          → github.com/tuulbelt/property-validator
│   └── ... (11 implemented, 33 planned)
│
├── libraries/                   # Programmatic APIs (git submodules)
│   └── ... (planned)
│
├── frameworks/                  # Application structures (git submodules)
│   └── ... (planned)
│
├── protocols/                   # Wire formats & specs (git submodules)
│   └── ... (planned)
│
├── systems/                     # Complex infrastructure (git submodules)
│   └── ... (planned)
│
├── meta/                        # Tools for building tools (git submodules)
│   └── ... (planned)
│
├── research/                    # Experimental projects (git submodules)
│   └── ... (planned)
│
├── templates/                   # Scaffolding templates
│   ├── tool-repo-template/      # TypeScript tool
│   ├── rust-tool-template/      # Rust tool
│   ├── library-template/        # Library (planned)
│   ├── framework-template/      # Framework (planned)
│   ├── protocol-template/       # Protocol (planned)
│   ├── system-template/         # System (planned)
│   ├── meta-template/           # Meta (planned)
│   └── research-template/       # Research (planned)
│
├── docs/                        # VitePress documentation
│   ├── index.md
│   ├── guide/
│   ├── tools/
│   ├── libraries/               # (planned)
│   ├── frameworks/              # (planned)
│   ├── protocols/               # (planned)
│   ├── systems/                 # (planned)
│   ├── meta/                    # (planned)
│   └── research/                # (planned)
│
├── scripts/
│   ├── clone-category.sh        # Clone entire category
│   ├── clone-project.sh         # Clone single project
│   ├── clone-with-deps.sh       # Clone with dependencies
│   ├── commit.sh                # Commit with correct author
│   └── push.sh                  # Push with pre-push validation
│
├── .claude/
│   ├── commands/                # Slash commands
│   ├── skills/                  # Cross-cutting expertise
│   ├── agents/                  # Specialized AI agents
│   └── hooks/                   # Quality gates
│
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── workflows/
│
└── LICENSE                      # MIT for all projects
```

---

## Category Overview

| Category | Purpose | External Deps | Governance |
|----------|---------|---------------|------------|
| **tools/** | CLI utilities | Zero | Low |
| **libraries/** | Programmatic APIs | Zero | Low |
| **frameworks/** | Application structures | Minimal | Medium |
| **protocols/** | Wire formats & specs | Zero | High |
| **systems/** | Complex infrastructure | Minimal | High |
| **meta/** | Tools for building tools | Zero | Medium |
| **research/** | Experimental projects | Relaxed | Low |

See [PRINCIPLES.md](PRINCIPLES.md) for detailed category-specific principles.

---

## Git Submodules

**All projects are git submodules** in their respective category directories. Each submodule:
- Points to a standalone GitHub repository
- Can be cloned and used independently
- Has its own version, CI/CD, and release cycle
- References other projects via git URL dependencies (not path dependencies)

**Working with submodules:**

```bash
# Clone meta repo with everything
git clone --recursive https://github.com/tuulbelt/tuulbelt.git

# Initialize submodules after clone
git submodule update --init --recursive

# Update all submodules to latest
git submodule update --remote

# Update specific project
cd tools/test-flakiness-detector
git pull origin main
cd ../..
git add tools/test-flakiness-detector
git commit -m "chore: update test-flakiness-detector submodule"
```

---

## Selective Cloning

Most users don't need the entire ecosystem. Use selective cloning scripts:

### Clone Single Project

```bash
./scripts/clone-project.sh tools/test-flakiness-detector
```

What it does:
1. `git submodule update --init tools/test-flakiness-detector`
2. Project ready to use

### Clone Entire Category

```bash
./scripts/clone-category.sh tools
```

What it does:
1. `git submodule update --init tools/*`
2. All tools available locally

### Clone with Dependencies

```bash
./scripts/clone-with-deps.sh tools/snapshot-comparison
```

What it does:
1. Parse project's dependencies for Tuulbelt references
2. Clone the project
3. Clone all Tuulbelt dependencies
4. Both projects available, composition works

### Shallow Clone (for CI)

```bash
./scripts/clone-project.sh tools/test-flakiness-detector --shallow
```

What it does:
1. `git submodule update --init --depth 1 tools/test-flakiness-detector`
2. Minimal fetch, fast CI

---

## Project Repository Structure

Each project follows a category-specific structure:

### Tools

```
tool-name/
├── README.md                  # What it does, why, how to use
├── CLAUDE.md                  # Claude Code context
├── src/
│   └── index.ts               # Main implementation
├── test/
│   └── *.test.ts              # Tests
├── examples/
│   └── basic.ts               # Usage examples
├── package.json               # With bin entry (CLI)
├── tsconfig.json
├── LICENSE
└── .github/workflows/test.yml
```

### Libraries

```
library-name/
├── README.md
├── API.md                     # Detailed API reference
├── src/
│   ├── index.ts               # Main exports
│   ├── types.ts               # Type definitions
│   └── internal/              # Internal modules
├── test/
├── examples/
├── package.json               # No bin entry
└── .github/workflows/test.yml
```

### Frameworks

```
framework-name/
├── README.md
├── ARCHITECTURE.md            # Required - design decisions
├── MIGRATION.md               # Version migration guide
├── src/
│   ├── core/                  # Core framework
│   ├── plugins/               # Plugin system (if any)
│   └── index.ts
├── test/
├── examples/
│   ├── basic/                 # Basic example app
│   └── advanced/              # Advanced example app
└── .github/workflows/test.yml
```

### Protocols

```
protocol-name/
├── README.md
├── SPEC.md                    # Required - formal specification
├── src/
│   └── lib.rs                 # Reference implementation
├── tests/
│   └── vectors/               # Compliance test vectors
├── examples/
├── Cargo.toml
└── .github/workflows/test.yml
```

### Systems

```
system-name/
├── README.md
├── ARCHITECTURE.md            # Required - detailed design
├── SPEC.md                    # Format/language specification
├── src/
│   ├── module-a/
│   ├── module-b/
│   └── main.rs
├── tests/
├── benches/                   # Performance benchmarks
├── examples/
├── Cargo.toml
└── .github/workflows/test.yml
```

### Meta

```
meta-name/
├── README.md
├── src/
│   ├── generator/             # Core generation logic
│   ├── templates/             # Output templates
│   └── index.ts
├── test/
├── examples/
│   ├── input/                 # Example inputs
│   └── output/                # Expected outputs
└── .github/workflows/test.yml
```

### Research

```
research-name/
├── README.md
├── HYPOTHESIS.md              # Required - what we're exploring
├── FINDINGS.md                # What we've learned
├── STATUS.md                  # Current state
├── src/
├── experiments/               # Experiment scripts
└── .github/workflows/test.yml
```

---

## Tech Stack

**TypeScript Projects:**
- Runtime: Node.js 18+
- Language: TypeScript 5.3+ (strict mode)
- Test Framework: Node.js native test runner (`node:test`)
- Assertions: Node.js native (`node:assert/strict`)
- Dev Dependencies: TypeScript, tsx, @types/node
- **Zero/Minimal Runtime Dependencies** (category-dependent)

**Rust Projects:**
- Rust Edition: 2021+
- Test Framework: Built-in cargo test
- Linter: clippy (all warnings denied)
- Formatter: rustfmt
- **Zero/Minimal Runtime Dependencies** (category-dependent)

**Development Tools:**
- Git with conventional commits
- GitHub Actions for CI/CD
- npm for TypeScript package management
- cargo for Rust package management

---

## Dependency Model

```
tuulbelt (meta)
├── No code dependencies
├── Contains templates for scaffolding
├── References projects via git submodules
└── Links to projects in README, not code

Project (any category)
├── External deps: Zero to Minimal (category-dependent)
├── MAY depend on other Tuulbelt projects via git URL
│   Example: "cli-progress-reporting": "git+https://github.com/tuulbelt/cli-progress-reporting.git"
├── Dev dependencies: TypeScript, test runners only
└── Fully standalone, independently cloneable
```

---

## Project Creation Workflow

**Use category-specific commands:**

```bash
# Tools
/new-tool <name> <typescript|rust>

# Libraries
/new-library <name> <typescript|rust>

# Frameworks
/new-framework <name> <typescript|rust>

# Protocols
/new-protocol <name>

# Systems
/new-system <name>

# Meta
/new-meta <name>

# Research
/new-research <name>
```

Each command:
1. Creates standalone GitHub repository
2. Scaffolds from appropriate template
3. Configures CI/CD, metadata, documentation
4. Tags v0.1.0 and pushes to GitHub
5. Adds as git submodule to meta repo

---

## Project Lifecycle

```
Proposal (issue) → Implementation → Testing → Stable (1.0.0) → Maintenance

0.1.0 - Initial release (unstable)
0.2.0 - Features added
1.0.0 - API stable, production-ready
1.1.0 - Bug fixes
2.0.0 - Breaking changes
```

**Special lifecycle for research:**
```
Proposal → Exploration → Findings → Graduation OR Archive

Status labels:
- active    - Currently being explored
- paused    - Temporarily on hold
- concluded - Exploration complete
- graduated - Moved to another category
```

---

## Language Support

| Language | Best For | Templates |
|----------|----------|-----------|
| TypeScript/Node.js | CLI tools, libraries, web | tool-repo-template, library-template |
| Rust | Systems, protocols, performance | rust-tool-template, protocol-template, system-template |

---

## Testing Strategy

All projects must have:

1. **Unit tests** — Core logic, edge cases
2. **Integration tests** — Interface behavior
3. **CI/CD** — Tests run on every push/PR
4. **Coverage target** — 80%+ for core logic

Additional requirements by category:
- **Protocols:** Compliance test vectors
- **Systems:** Performance benchmarks
- **Research:** Experiment reproducibility

Use language-native test runners:
- Node.js: `node --test`
- Rust: `cargo test`

---

## Maintenance Model

Each project has a primary maintainer:
- Responds to issues
- Reviews PRs
- Makes releases
- Updates docs

Falls back to meta repo maintainer if unmaintained for 3+ months.

---

## Versioning

- **Meta repo:** No version (always latest)
- **Each project:** Semantic versioning (0.1.0, 1.0.0, etc.)
- **Release cadence:** As needed, no fixed schedule

---

## Security Model

- **Minimal external deps** = fewer supply chain attacks
- **Code review required** for all changes
- **Secrets never in code** — projects that handle secrets use encryption
- **Open by default** — all repos public, no private dependencies

See [docs/security-guidelines.md](docs/security-guidelines.md) for complete checklist.

---

## Discovery

How users find projects:

1. **Meta repo README** — Index of all projects with status
2. **GitHub Pages** — https://tuulbelt.github.io/tuulbelt/
3. **GitHub org** — Browse all repos at https://github.com/tuulbelt
4. **GitHub search** — By name or problem
5. **Category READMEs** — Each category directory has a README

---

## Migration Paths

### Tool → Library

When a tool's programmatic API becomes more valuable than its CLI:
1. Create new library project
2. Move core logic to library
3. Tool becomes thin CLI wrapper importing library
4. Document the split

### Research → Production Category

When a research project proves successful:
1. Verify hypothesis validated
2. Ensure implementation meets target category's principles
3. Create new project in appropriate category
4. Archive research project or keep as reference

### Project Splitting

When a project becomes too broad:
1. Identify natural boundaries
2. Split into focused projects
3. Use git URL dependencies for composition
4. Update documentation

---

## Category Comparison

| Aspect | tools | libraries | frameworks | protocols | systems | meta | research |
|--------|-------|-----------|------------|-----------|---------|------|----------|
| **Interface** | CLI | API | API+Conventions | Spec+Impl | Varies | CLI+API | Any |
| **Scope** | Single | Focused | Broad | Focused | Large | Generative | Exploratory |
| **External Deps** | Zero | Zero | Minimal | Zero | Minimal | Zero | Relaxed |
| **SPEC.md** | No | No | No | **Yes** | Often | No | No |
| **ARCHITECTURE.md** | Optional | Optional | **Yes** | No | **Yes** | Optional | No |
| **HYPOTHESIS.md** | No | No | No | No | No | No | **Yes** |
| **Governance** | Low | Low | Medium | High | High | Medium | Low |
