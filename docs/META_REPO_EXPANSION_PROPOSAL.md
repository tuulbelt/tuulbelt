# Tuulbelt Meta Repository Expansion Proposal

**Status:** Draft
**Author:** Claude Code Session
**Date:** 2026-01-11
**Branch:** claude/expand-meta-repo-structure-Bdx1K

---

## Executive Summary

This proposal outlines the expansion of Tuulbelt from a focused **tools-only** meta repository to a comprehensive **software ecosystem** encompassing tools, libraries, frameworks, protocols, systems, meta-frameworks, and research projects.

The key insight: **the git submodule architecture already supports this**. Each category is simply a new directory containing submodules, with category-specific principles and templates.

---

## Part 1: Current State Analysis

### What Exists Today

```
tuulbelt/
├── tools/                    # 11 implemented, 33 planned
│   ├── test-flakiness-detector/
│   ├── cli-progress-reporting/
│   ├── property-validator/
│   └── ... (11 total submodules)
├── templates/
│   ├── tool-repo-template/   # TypeScript
│   └── rust-tool-template/   # Rust
├── docs/                     # VitePress documentation
├── scripts/                  # Automation
└── .claude/                  # Claude Code configuration
```

### Current Principles (Tools)

| Principle | Description |
|-----------|-------------|
| Single Problem | One tool, one problem |
| Zero External Deps | Standard library only (Exception: Tuulbelt-to-Tuulbelt composition) |
| Portable Interface | CLI, files, env vars, sockets |
| Composable | Pipes, not plugins |
| Independently Cloneable | Each tool works standalone |
| Proven Implementation | No moonshots |

### Current Constraints

1. **PRINCIPLES.md explicitly rejects frameworks** (line 136-142)
2. **ARCHITECTURE.md says "Tools vs Frameworks"** - tools only (line 269-283)
3. **Templates are tool-specific** (CLI-first, single entry point)
4. **Documentation is tool-centric** (VitePress structure assumes tools/)

---

## Part 2: Proposed Category Taxonomy

### Category Overview

| Category | Purpose | Scope | External Deps | Tuulbelt Deps |
|----------|---------|-------|---------------|---------------|
| **tools/** | CLI utilities | Single problem | Zero | Allowed |
| **libraries/** | Programmatic APIs | Focused domain | Zero | Allowed |
| **frameworks/** | Application structures | Broad, opinionated | Minimal | Allowed |
| **protocols/** | Wire formats & specs | Specification + ref impl | Zero | Allowed |
| **systems/** | Complex infrastructure | Large, integrated | Minimal | Allowed |
| **meta/** | Tools for building tools | Generative | Zero | Allowed |
| **research/** | Experimental projects | Exploratory | Relaxed | Allowed |

---

### Category 1: Tools (Existing)

**Definition:** Single-purpose CLI utilities that solve one problem well.

**Current Status:** 11 implemented, 33 planned

**Principles:**
- Zero external dependencies (standard library only)
- Single problem per tool
- CLI-first interface (stdin/stdout/stderr)
- Composable via pipes
- Independently cloneable

**Examples (Current):**
- `test-flakiness-detector` — Run tests N times, identify unreliable ones
- `cli-progress-reporting` — Concurrent-safe progress updates
- `property-validator` — Runtime schema validation

**Interface Pattern:**
```bash
tool-name [options] <input>
echo "data" | tool-name --format json
```

**Template:** `templates/tool-repo-template/`, `templates/rust-tool-template/`

---

### Category 2: Libraries (New)

**Definition:** Programmatic APIs that provide reusable functionality without CLI overhead.

**Key Difference from Tools:**
- Tools are CLI-first; libraries are API-first
- Libraries may have CLI wrappers, but core value is programmatic
- Libraries emphasize type safety, ergonomics, and composability

**Principles:**
- Zero external dependencies (standard library only)
- Focused domain (validation, parsing, algorithms)
- Type-safe, well-documented API
- Can compose with other Tuulbelt libraries
- No CLI required (optional wrapper)

**Potential Projects:**
| Project | Description | Language |
|---------|-------------|----------|
| `result-type` | Rust-style Result<T,E> for TypeScript | TypeScript |
| `option-type` | Rust-style Option<T> for TypeScript | TypeScript |
| `immutable-collections` | Persistent data structures | TypeScript/Rust |
| `json-pointer` | RFC 6901 JSON Pointer implementation | TypeScript |
| `uri-template` | RFC 6570 URI Template expansion | TypeScript |
| `base-encoding` | Base16/32/58/64 encoding suite | Rust |
| `crypto-primitives` | Hash, HMAC, KDF implementations | Rust |

**Interface Pattern:**
```typescript
import { validate, schema } from '@tuulbelt/validation-lib';

const UserSchema = schema.object({
  name: schema.string(),
  age: schema.number().min(0)
});

const result = validate(data, UserSchema);
if (result.ok) {
  console.log(result.value.name);
}
```

**Template:** `templates/library-template/` (NEW)

---

### Category 3: Frameworks (New)

**Definition:** Opinionated structures that guide how applications are built.

**Key Difference from Tools/Libraries:**
- Frameworks provide structure and conventions
- Frameworks call your code (inversion of control)
- Broader scope, more opinionated

**Principles:**
- Minimal external dependencies (prefer Tuulbelt composition)
- Clear conventions over configuration
- Escape hatches for customization
- Well-documented architecture
- Incremental adoption path

**Potential Projects:**
| Project | Description | Language |
|---------|-------------|----------|
| `web-framework` | Minimal HTTP framework (think Hono/Express) | TypeScript |
| `test-framework` | Test runner with fixtures and assertions | TypeScript |
| `cli-framework` | CLI application framework | TypeScript/Rust |
| `plugin-framework` | Extensibility pattern for applications | TypeScript |
| `event-sourcing` | Event sourcing application framework | TypeScript |

**Interface Pattern:**
```typescript
import { createApp, router } from '@tuulbelt/web-framework';

const app = createApp();

app.use(router()
  .get('/users', getUsers)
  .post('/users', createUser)
);

app.listen(3000);
```

**Template:** `templates/framework-template/` (NEW)

**Note:** This category requires careful governance. Frameworks can easily become bloated. Each framework must:
1. Solve a real, documented problem
2. Have a clear scope boundary
3. Be incrementally adoptable
4. Not require "all-or-nothing" commitment

---

### Category 4: Protocols (New)

**Definition:** Wire formats, communication standards, and protocol specifications with reference implementations.

**Key Difference:**
- Protocols are **specifications first**, implementations second
- Must have a formal SPEC.md document
- Reference implementations must be minimal and correct

**Principles:**
- Zero external dependencies in reference implementation
- Specification completeness (edge cases, error handling)
- Cross-language compatibility (spec is language-agnostic)
- Test vectors for compliance
- Versioning and evolution strategy

**Potential Projects:**
| Project | Description | Language |
|---------|-------------|----------|
| `wire-protocol` | Self-describing binary TLV format | Rust |
| `message-envelope` | Standard request/response envelope | TypeScript |
| `pubsub-protocol` | Minimalist pub-sub wire format | Rust |
| `rpc-protocol` | Simple RPC over TCP/Unix sockets | Rust |
| `event-log-format` | Structured event log format | Rust |

**Interface Pattern:**
```
SPEC.md              # Formal specification (MUST exist)
src/lib.rs           # Reference implementation
tests/vectors/       # Compliance test vectors
examples/            # Usage examples
```

**Template:** `templates/protocol-template/` (NEW)

---

### Category 5: Systems (New)

**Definition:** Complex, integrated infrastructure projects like programming languages, databases, and runtimes.

**Key Difference:**
- Large scope, long development timeline
- Multiple components working together
- May have own build system, tooling
- Often written in Rust for performance

**Principles:**
- Prefer minimal dependencies (but pragmatic)
- Well-documented architecture
- Incremental development milestones
- Clear module boundaries
- Performance-conscious

**Potential Projects:**
| Project | Description | Language |
|---------|-------------|----------|
| `tuu-lang` | Small, embeddable scripting language | Rust |
| `embedded-db` | Single-file embedded database | Rust |
| `key-value-store` | Persistent key-value storage | Rust |
| `virtual-machine` | Stack-based bytecode VM | Rust |
| `build-system` | Declarative build system | Rust |

**Interface Pattern:**
```
src/
├── lexer/           # Tokenization
├── parser/          # AST construction
├── compiler/        # Bytecode generation
├── vm/              # Execution
└── stdlib/          # Standard library

ARCHITECTURE.md      # Required - detailed design
SPEC.md              # Language/format specification
```

**Template:** `templates/system-template/` (NEW)

---

### Category 6: Meta (New)

**Definition:** Tools and frameworks for building other tools, frameworks, and systems.

**Key Difference:**
- Generative in nature (produces code, scaffolding)
- Higher abstraction level
- Enables creation of other Tuulbelt projects

**Principles:**
- Zero external dependencies (dogfood principles)
- Well-documented generation patterns
- Customizable output
- Version-aware (handles evolution)

**Potential Projects:**
| Project | Description | Language |
|---------|-------------|----------|
| `parser-generator` | PEG/CFG parser generator | Rust |
| `codegen-toolkit` | Code generation utilities | TypeScript |
| `dsl-builder` | Domain-specific language toolkit | Rust |
| `schema-compiler` | Schema → code generator | TypeScript |
| `api-generator` | OpenAPI → client/server code | TypeScript |

**Interface Pattern:**
```bash
# CLI for generation
meta-tool generate --input schema.json --output src/

# Library for programmatic use
import { generate } from '@tuulbelt/codegen-toolkit';
const code = generate(schema, { language: 'typescript' });
```

**Template:** `templates/meta-template/` (NEW)

---

### Category 7: Research (New)

**Definition:** Experimental projects pushing boundaries, exploring novel approaches.

**Key Difference:**
- Exploratory, not production-ready
- May fail (that's okay)
- Relaxed principles for experimentation
- Clear "experimental" labeling

**Principles:**
- Relaxed dependency rules (pragmatism over purity)
- Document hypotheses and findings
- Clear experimental status labeling
- May graduate to other categories when proven

**Potential Projects:**
| Project | Description | Language |
|---------|-------------|----------|
| `zero-copy-parsing` | Zero-copy parser experiments | Rust |
| `wasm-runtime` | WebAssembly runtime experiments | Rust |
| `effect-system` | Effect system for TypeScript | TypeScript |
| `incremental-compute` | Incremental computation framework | Rust |
| `formal-verification` | Proof-carrying code experiments | Rust |

**Interface Pattern:**
```
HYPOTHESIS.md        # What we're exploring
FINDINGS.md          # What we've learned
STATUS.md            # Current state (active/paused/concluded)
src/                 # Implementation
experiments/         # Experiment scripts
```

**Template:** `templates/research-template/` (NEW)

---

## Part 3: Proposed Directory Structure

### New Meta Repository Layout

```
tuulbelt/
├── README.md                     # Ecosystem overview
├── PRINCIPLES.md                 # Updated with category-specific principles
├── ARCHITECTURE.md               # Updated with full ecosystem architecture
├── CONTRIBUTING.md               # Updated contribution guide
│
├── tools/                        # CLI utilities (existing)
│   ├── test-flakiness-detector/
│   ├── cli-progress-reporting/
│   ├── property-validator/
│   └── ... (33 planned)
│
├── libraries/                    # Programmatic APIs (NEW)
│   ├── result-type/
│   ├── option-type/
│   └── ...
│
├── frameworks/                   # Application structures (NEW)
│   ├── web-framework/
│   ├── test-framework/
│   └── ...
│
├── protocols/                    # Wire formats & specs (NEW)
│   ├── wire-protocol/
│   ├── message-envelope/
│   └── ...
│
├── systems/                      # Complex infrastructure (NEW)
│   ├── tuu-lang/
│   ├── embedded-db/
│   └── ...
│
├── meta/                         # Meta tools (NEW)
│   ├── parser-generator/
│   ├── codegen-toolkit/
│   └── ...
│
├── research/                     # Experimental projects (NEW)
│   ├── zero-copy-parsing/
│   ├── effect-system/
│   └── ...
│
├── templates/                    # Scaffolding templates
│   ├── tool-repo-template/       # Existing
│   ├── rust-tool-template/       # Existing
│   ├── library-template/         # NEW
│   ├── framework-template/       # NEW
│   ├── protocol-template/        # NEW
│   ├── system-template/          # NEW
│   ├── meta-template/            # NEW
│   └── research-template/        # NEW
│
├── docs/                         # VitePress documentation
│   ├── index.md                  # Home page
│   ├── guide/                    # Getting started, principles
│   ├── tools/                    # Tool documentation
│   ├── libraries/                # Library documentation (NEW)
│   ├── frameworks/               # Framework documentation (NEW)
│   ├── protocols/                # Protocol documentation (NEW)
│   ├── systems/                  # System documentation (NEW)
│   ├── meta/                     # Meta tool documentation (NEW)
│   └── research/                 # Research documentation (NEW)
│
├── scripts/
│   ├── clone-category.sh         # Clone entire category (NEW)
│   ├── clone-project.sh          # Clone single project with deps (NEW)
│   ├── clone-deps.sh             # Clone project dependencies (NEW)
│   ├── new-project.sh            # Scaffolding script (NEW)
│   ├── commit.sh                 # Existing
│   └── push.sh                   # Existing
│
└── .claude/
    ├── commands/
    │   ├── new-tool.md           # Existing
    │   ├── new-library.md        # NEW
    │   ├── new-framework.md      # NEW
    │   ├── new-protocol.md       # NEW
    │   ├── new-system.md         # NEW
    │   └── new-research.md       # NEW
    └── ...
```

---

## Part 4: Selective Cloning Mechanism

### Problem Statement

Users shouldn't need to clone the entire ecosystem. They should be able to:
1. Clone a single project
2. Clone an entire category
3. Clone a project with its Tuulbelt dependencies
4. Shallow clone for CI/CD

### Solution: Cloning Scripts

#### 1. Clone Single Project

```bash
# Clone just test-flakiness-detector
./scripts/clone-project.sh tools/test-flakiness-detector

# What it does:
# 1. git submodule update --init tools/test-flakiness-detector
# 2. Done (single project, ready to use)
```

#### 2. Clone Entire Category

```bash
# Clone all tools
./scripts/clone-category.sh tools

# What it does:
# 1. git submodule update --init tools/*
# 2. All 33 tools available locally
```

#### 3. Clone Project with Dependencies

```bash
# Clone snapshot-comparison with its dependency (output-diffing-utility)
./scripts/clone-with-deps.sh tools/snapshot-comparison

# What it does:
# 1. Read project's package.json/Cargo.toml for Tuulbelt deps
# 2. git submodule update --init tools/snapshot-comparison
# 3. git submodule update --init tools/output-diffing-utility
# 4. Both projects available, composition works
```

#### 4. Shallow Clone for CI

```bash
# Fast clone for CI (depth=1, single project)
./scripts/clone-shallow.sh tools/test-flakiness-detector

# What it does:
# 1. git submodule update --init --depth 1 tools/test-flakiness-detector
# 2. Minimal fetch, fast CI
```

### Dependency Discovery

To enable `clone-with-deps.sh`, we need a way to discover Tuulbelt dependencies.

**Option A: Parse package.json/Cargo.toml**
```bash
# TypeScript - look for git+https://github.com/tuulbelt/
grep -o 'git+https://github.com/tuulbelt/[^"]*' package.json

# Rust - look for git = "https://github.com/tuulbelt/
grep -oP 'git = "https://github.com/tuulbelt/\K[^"]+' Cargo.toml
```

**Option B: Manifest file (NEW)**
```yaml
# DEPS.yaml in each project
dependencies:
  - tools/output-diffing-utility
  - libraries/result-type
```

**Recommendation:** Option A (parse existing files) for now, Option B if complexity grows.

---

## Part 5: Principles Evolution

### Current PRINCIPLES.md Issues

Line 136-142 currently says:
> **What Doesn't Belong in Tuulbelt**
> - **Frameworks** (anything that tries to be your whole app)
> - **Heavy abstractions** (frameworks disguised as utilities)

This directly contradicts the expansion. We need to evolve the principles.

### Proposed: Category-Specific Principles

**Universal Principles (All Categories):**
1. **Proven Implementation** — No moonshots (except research/)
2. **Documentation Over Configuration** — Clear docs, minimal config
3. **Test Everything** — Comprehensive testing required
4. **Version Independently** — Each project has its own version
5. **Independently Cloneable** — Works without meta repo

**Category-Specific Principles:**

| Category | Zero External Deps | Single Problem | CLI Interface | Scope |
|----------|-------------------|----------------|---------------|-------|
| tools | Required | Required | Required | Narrow |
| libraries | Required | Focused Domain | Optional | Focused |
| frameworks | Minimal | No | Optional | Broad |
| protocols | Required (ref impl) | Focused | Optional | Focused |
| systems | Minimal | No | Varies | Large |
| meta | Required | Generative | Required | Focused |
| research | Relaxed | Exploratory | Optional | Varies |

### Updated PRINCIPLES.md Structure

```markdown
# Tuulbelt Design Principles

## Universal Principles
(Apply to all categories)

## Category: Tools
(Current strict principles)

## Category: Libraries
(API-focused principles)

## Category: Frameworks
(Structure-focused principles)

## Category: Protocols
(Specification-focused principles)

## Category: Systems
(Infrastructure-focused principles)

## Category: Meta
(Generative-focused principles)

## Category: Research
(Exploration-focused principles)
```

---

## Part 6: Template Expansion

### New Templates Required

| Template | Purpose | Base |
|----------|---------|------|
| `library-template/` | Programmatic APIs | Extend tool-repo-template |
| `framework-template/` | Application structures | New structure |
| `protocol-template/` | Wire formats | Extend rust-tool-template |
| `system-template/` | Complex systems | New structure |
| `meta-template/` | Meta tools | Extend tool-repo-template |
| `research-template/` | Experiments | Minimal structure |

### Template Differences

**Library Template:**
```
library-name/
├── src/
│   ├── index.ts           # Main exports
│   ├── types.ts           # Type definitions
│   └── internal/          # Internal modules
├── test/
├── examples/
├── package.json           # No bin entry (not CLI)
├── README.md
└── API.md                 # Detailed API documentation (NEW)
```

**Framework Template:**
```
framework-name/
├── src/
│   ├── core/              # Core framework
│   ├── plugins/           # Plugin system (if any)
│   └── index.ts
├── test/
├── examples/
│   ├── basic/             # Basic example app
│   └── advanced/          # Advanced example app
├── package.json
├── README.md
├── ARCHITECTURE.md        # Required for frameworks
└── MIGRATION.md           # Version migration guide
```

**Protocol Template:**
```
protocol-name/
├── SPEC.md                # Formal specification (REQUIRED)
├── src/
│   ├── lib.rs             # Reference implementation
│   └── ...
├── tests/
│   └── vectors/           # Compliance test vectors
├── examples/
├── Cargo.toml
└── README.md
```

**System Template:**
```
system-name/
├── ARCHITECTURE.md        # Required detailed architecture
├── SPEC.md                # Format/language specification
├── src/
│   ├── module-a/
│   ├── module-b/
│   └── main.rs
├── tests/
├── benches/               # Performance benchmarks required
├── examples/
├── Cargo.toml
└── README.md
```

**Research Template:**
```
research-name/
├── HYPOTHESIS.md          # What we're exploring
├── FINDINGS.md            # What we've learned
├── STATUS.md              # Current state
├── src/
├── experiments/           # Experiment scripts
└── README.md
```

---

## Part 7: Documentation Expansion

### VitePress Structure Update

```
docs/
├── index.md               # Updated home page
├── guide/
│   ├── getting-started.md # Updated for ecosystem
│   ├── principles.md      # Updated with categories
│   ├── selective-cloning.md  # NEW
│   └── contributing.md
│
├── tools/                 # Existing
│   ├── index.md           # Tool category overview
│   └── [tool-name]/       # Per-tool docs
│
├── libraries/             # NEW
│   ├── index.md           # Library category overview
│   └── [lib-name]/        # Per-library docs
│
├── frameworks/            # NEW
│   ├── index.md
│   └── [framework-name]/
│
├── protocols/             # NEW
│   ├── index.md
│   └── [protocol-name]/
│       ├── index.md
│       └── specification.md  # SPEC.md rendered
│
├── systems/               # NEW
│   ├── index.md
│   └── [system-name]/
│
├── meta/                  # NEW
│   ├── index.md
│   └── [meta-name]/
│
└── research/              # NEW
    ├── index.md
    └── [research-name]/
```

### Updated VitePress Config

```typescript
// docs/.vitepress/config.ts
export default defineConfig({
  themeConfig: {
    sidebar: {
      '/tools/': toolsSidebar,
      '/libraries/': librariesSidebar,
      '/frameworks/': frameworksSidebar,
      '/protocols/': protocolsSidebar,
      '/systems/': systemsSidebar,
      '/meta/': metaSidebar,
      '/research/': researchSidebar,
    },
    nav: [
      { text: 'Tools', link: '/tools/' },
      { text: 'Libraries', link: '/libraries/' },
      { text: 'Frameworks', link: '/frameworks/' },
      { text: 'Protocols', link: '/protocols/' },
      { text: 'Systems', link: '/systems/' },
      { text: 'Meta', link: '/meta/' },
      { text: 'Research', link: '/research/' },
    ]
  }
});
```

---

## Part 8: Command/Workflow Expansion

### New Claude Code Commands

| Command | Purpose |
|---------|---------|
| `/new-library <name> <ts\|rust>` | Create new library project |
| `/new-framework <name> <ts\|rust>` | Create new framework project |
| `/new-protocol <name>` | Create new protocol project |
| `/new-system <name>` | Create new system project |
| `/new-research <name>` | Create new research project |
| `/clone-category <category>` | Clone entire category |
| `/clone-with-deps <path>` | Clone project with dependencies |

### Updated Quality Checklist

Each category has specific quality gates:

**Tools:** (Existing)
- Build succeeds
- Tests pass (80%+)
- Zero runtime deps
- CLI works

**Libraries:**
- Build succeeds
- Tests pass (80%+)
- Zero runtime deps
- API.md exists
- Type exports correct

**Frameworks:**
- Build succeeds
- Tests pass
- ARCHITECTURE.md exists
- Example app works
- Migration guide exists

**Protocols:**
- Build succeeds
- Tests pass
- SPEC.md exists and complete
- Test vectors pass
- Cross-language compatibility verified

**Systems:**
- Build succeeds
- Tests pass
- ARCHITECTURE.md exists
- Benchmarks exist
- Documentation complete

**Research:**
- Build succeeds (can have warnings)
- HYPOTHESIS.md exists
- STATUS.md current
- Findings documented

---

## Part 9: Implementation Roadmap

### Phase 1: Foundation (Week 1-2)

**Goal:** Establish category structure without breaking existing tools.

1. **Update PRINCIPLES.md** with category-specific principles
2. **Update ARCHITECTURE.md** with full ecosystem structure
3. **Create directory stubs:** `libraries/`, `frameworks/`, `protocols/`, `systems/`, `meta/`, `research/`
4. **Create selective cloning scripts:** `clone-category.sh`, `clone-project.sh`, `clone-with-deps.sh`
5. **Update VitePress config** with new navigation

**Deliverables:**
- Updated documentation
- Empty category directories
- Working cloning scripts
- No changes to existing tools

### Phase 2: Templates (Week 3-4)

**Goal:** Create templates for new categories.

1. **Create `library-template/`** based on tool-repo-template
2. **Create `protocol-template/`** with SPEC.md requirement
3. **Create `research-template/`** minimal structure
4. **Create Claude Code commands** for new project types
5. **Test template scaffolding**

**Deliverables:**
- 3 new templates working
- `/new-library`, `/new-protocol`, `/new-research` commands
- Template documentation

### Phase 3: First Projects (Week 5-8)

**Goal:** Validate structure with real projects.

1. **Library:** Implement `result-type` (Rust-style Result for TypeScript)
2. **Protocol:** Implement `message-envelope` (request/response wrapper)
3. **Research:** Start `effect-system` experiment

**Deliverables:**
- 1 library published
- 1 protocol specified and implemented
- 1 research project started
- Validation that structure works

### Phase 4: Frameworks & Systems (Week 9-16)

**Goal:** Add complex categories.

1. **Create `framework-template/`**
2. **Create `system-template/`**
3. **Create `meta-template/`**
4. **Start first framework:** `test-framework`
5. **Start first system:** `tuu-lang` (small language)

**Deliverables:**
- All 6 templates complete
- 1 framework started
- 1 system started
- Full ecosystem structure operational

### Phase 5: Ecosystem Maturity (Ongoing)

**Goal:** Grow the ecosystem.

1. Continue building tools (complete 33)
2. Build libraries as needed
3. Develop frameworks based on real needs
4. Research novel approaches
5. Community contributions

---

## Part 10: Governance & Quality

### Category Governance

| Category | Approval Required | Quality Bar |
|----------|------------------|-------------|
| tools | Low (proven patterns) | High (strict principles) |
| libraries | Low | High |
| frameworks | Medium (scope review) | High |
| protocols | High (spec review) | Very High |
| systems | High (architecture review) | High |
| meta | Medium | High |
| research | Low (exploratory) | Low (experimental) |

### Promotion Path

```
research/ → Proven → Graduates to appropriate category

Example:
research/effect-system → libraries/effect-system
(when hypothesis validated and implementation stable)
```

### Rejection Criteria (What Doesn't Belong)

- **Vaporware** — Ideas without implementation path
- **Scope creep** — Projects that should be split
- **External dependency heavy** — Violates zero-dep philosophy
- **Unmaintained** — No clear maintainer
- **Duplicates** — Already solved by existing project

---

## Part 11: Risk Analysis

### Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Scope creep (especially frameworks) | High | Strict governance, clear scope boundaries |
| Maintenance burden | Medium | Clear ownership, deprecation policy |
| Quality dilution | High | Category-specific quality gates |
| Complexity explosion | Medium | Selective cloning, minimal coupling |
| Community confusion | Medium | Clear documentation, category definitions |

### Success Metrics

1. **Adoption:** Projects cloned/used in production
2. **Quality:** Test coverage, zero critical bugs
3. **Ecosystem health:** Inter-project composition works
4. **Community:** External contributions
5. **Documentation:** Complete, accurate, helpful

---

## Part 12: Decision Summary

### Key Decisions Required

1. **Approve category taxonomy** (7 categories as proposed)
2. **Approve selective cloning approach** (scripts vs manifest)
3. **Approve principles evolution** (category-specific)
4. **Approve template expansion** (6 new templates)
5. **Approve implementation roadmap** (16-week initial rollout)

### Recommended First Steps

1. Create this proposal as permanent document
2. Update PRINCIPLES.md with category framework
3. Create category directories (empty stubs)
4. Create cloning scripts
5. Build first library to validate approach

---

## Appendix A: Full Category Comparison

| Aspect | tools | libraries | frameworks | protocols | systems | meta | research |
|--------|-------|-----------|------------|-----------|---------|------|----------|
| **Primary Interface** | CLI | API | API + Conventions | Spec + Impl | Varies | CLI + API | Varies |
| **Scope** | Single problem | Focused domain | Broad, opinionated | Specification | Large, integrated | Generative | Exploratory |
| **External Deps** | Zero | Zero | Minimal | Zero (ref impl) | Minimal | Zero | Relaxed |
| **Tuulbelt Deps** | Allowed | Allowed | Allowed | Allowed | Allowed | Allowed | Allowed |
| **SPEC.md Required** | No | No | No | Yes | Often | No | No |
| **ARCHITECTURE.md** | Optional | Optional | Required | No | Required | Optional | No |
| **Governance** | Low | Low | Medium | High | High | Medium | Low |
| **Quality Bar** | High | High | High | Very High | High | High | Low |

---

## Appendix B: Example Project Ideas per Category

### Tools (33 planned, 11 complete)
*See ROADMAP.md for full list*

### Libraries
- `result-type` — Rust-style Result<T,E>
- `option-type` — Rust-style Option<T>
- `immutable-collections` — Persistent data structures
- `json-pointer` — RFC 6901 implementation
- `uri-template` — RFC 6570 implementation
- `base-encoding` — Base16/32/58/64
- `crypto-primitives` — Hash, HMAC, KDF
- `datetime` — Date/time utilities
- `color` — Color manipulation
- `money` — Currency and money handling

### Frameworks
- `web-framework` — Minimal HTTP framework
- `test-framework` — Test runner with fixtures
- `cli-framework` — CLI application framework
- `plugin-framework` — Extensibility patterns
- `event-sourcing` — Event sourcing framework
- `state-machine` — State machine framework

### Protocols
- `wire-protocol` — Self-describing binary TLV
- `message-envelope` — Request/response wrapper
- `pubsub-protocol` — Pub-sub wire format
- `rpc-protocol` — Simple RPC protocol
- `event-log-format` — Structured event format
- `config-format` — Configuration file format

### Systems
- `tuu-lang` — Small scripting language
- `embedded-db` — Single-file database
- `key-value-store` — Persistent KV storage
- `virtual-machine` — Stack-based VM
- `build-system` — Declarative build system
- `package-manager` — Minimal package manager

### Meta
- `parser-generator` — PEG/CFG parser gen
- `codegen-toolkit` — Code generation
- `dsl-builder` — DSL toolkit
- `schema-compiler` — Schema → code
- `api-generator` — OpenAPI → code

### Research
- `zero-copy-parsing` — Zero-copy experiments
- `wasm-runtime` — WASM runtime experiments
- `effect-system` — Effect system for TS
- `incremental-compute` — Incremental computation
- `formal-verification` — Proof-carrying code

---

*End of Proposal*
