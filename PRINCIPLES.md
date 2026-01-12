# Tuulbelt Design Principles

These principles guide what gets built, how it's built, and what doesn't belong.

Tuulbelt is a **software ecosystem** organized into categories, each with shared universal principles and category-specific guidelines.

---

## Universal Principles

These apply to **all categories** (tools, libraries, frameworks, protocols, systems, meta, research).

### 1. Craftsmanship Over Speed

Ship when it's ready, not when it's due. Every project should reflect care in design, implementation, and documentation. The ecosystem's reputation depends on consistent quality.

**Signals of Craftsmanship:**
- Code is readable and well-organized
- Edge cases are handled thoughtfully
- Error messages are helpful
- Documentation is complete before "done"

### 2. Proven Implementation, Not Moonshots

Before building:
- Problem must be real (you hit it, or it's documented in production)
- Solution must have a clear implementation path
- No "it would be cool if..." without proof it works
- No "works 80% of the time" solutions

**Exception:** Research category allows exploration of unproven ideas.

**Red flags:**
- "I think this would be useful"
- "It's like X but better" (without explaining why X is broken)
- Complex state management or learning curve
- Vague scope ("improve developer experience")

### 3. Documentation Over Configuration

If something needs explanation, document it. Don't add config complexity.

**Good:**
```
-f, --format <json|text>  Output format (default: json)
```

**Bad:**
```
--config my-config.json   # 50 keys, most optional
```

### 4. Fail Loudly, Recover Gracefully

- Errors should be explicit and helpful
- Stack traces should include context (file, line, operation)
- Degradation should be obvious, not silent

### 5. Test Everything, Ship Nothing Untested

- Unit tests for core logic
- Integration tests for interfaces
- Edge case coverage (empty input, malformed input, concurrent access)
- Tests run in CI on every commit

### 6. Version Independently

Each project has its own version and changelog. The meta repo has no version.

Use semantic versioning:
- `0.1.0` — First release, unstable API
- `1.0.0` — Stable API, production-ready
- `2.0.0` — Breaking change

### 7. Independently Cloneable

Each project is its own GitHub repository.

Users should be able to:
```bash
git clone https://github.com/tuulbelt/<project-name>.git
cd <project-name>
npm test && npm run build  # or cargo test && cargo build
```

Without needing the meta repo or any other project (unless explicitly dependent).

### 8. Tuulbelt-to-Tuulbelt Composition

Projects MAY use other Tuulbelt projects via git URL dependencies:

```toml
# Rust
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility.git" }
```

```json
// TypeScript
{
  "dependencies": {
    "@tuulbelt/cli-progress-reporting": "git+https://github.com/tuulbelt/cli-progress-reporting.git"
  }
}
```

Since all Tuulbelt projects minimize external dependencies, composition preserves the lightweight guarantee.

---

## Category-Specific Principles

### Category: Tools

**Definition:** Single-purpose CLI utilities that solve one problem well.

**Directory:** `tools/`

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (standard library only) |
| Tuulbelt Dependencies | Allowed |
| Scope | Single problem |
| Interface | CLI-first (stdin/stdout/stderr) |
| Composability | Pipes, not plugins |

**Additional Guidelines:**

1. **Single Problem Per Tool** — Each tool solves one problem and solves it well. If scope is growing, split into multiple tools.

2. **Zero External Dependencies** — No `npm install`, no `cargo add`. Development dependencies (TypeScript compiler, test runners) are okay if they don't ship.

3. **Portable Interface** — Communicate via CLI, files, environment variables, sockets. NOT via proprietary APIs, plugin systems, or configuration frameworks.

4. **Composable** — Tools chain via pipes: `tool-a | tool-b | tool-c`

**Examples:**
- ✅ "Run tests N times, report flaky ones"
- ✅ "Convert YAML to JSON"
- ✅ "Manage concurrent test port allocation"

---

### Category: Libraries

**Definition:** Programmatic APIs that provide reusable functionality.

**Directory:** `libraries/`

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (standard library only) |
| Tuulbelt Dependencies | Allowed |
| Scope | Focused domain |
| Interface | Programmatic API |
| CLI | Optional (wrapper) |

**Additional Guidelines:**

1. **API-First** — Libraries are meant to be imported, not executed. CLI wrappers are optional extras.

2. **Type Safety** — Provide strong type definitions. TypeScript libraries must export types. Rust libraries must have clear public API.

3. **API Stability** — Breaking changes require major version bump. Document migration paths.

4. **API Documentation** — Include API.md with comprehensive API reference.

**Examples:**
- ✅ "Rust-style Result<T,E> for TypeScript"
- ✅ "RFC 6901 JSON Pointer implementation"
- ✅ "Persistent immutable data structures"

---

### Category: Frameworks

**Definition:** Opinionated structures that guide how applications are built.

**Directory:** `frameworks/`

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Minimal (max 3, each justified in docs) |
| Tuulbelt Dependencies | Allowed (encouraged) |
| Scope | Broad but bounded |
| Interface | API + Conventions |
| Documentation | ARCHITECTURE.md required |
| Migration | Version migration guide required |

**Additional Guidelines:**

1. **Clear Scope Boundary** — Define upfront what the framework does and doesn't do. Scope creep is the enemy.

2. **Incremental Adoption** — Frameworks must be adoptable incrementally. No "all-or-nothing" commitment.

3. **Escape Hatches** — Provide ways to override conventions when needed.

4. **Architecture Documentation** — ARCHITECTURE.md is mandatory. Explain design decisions.

5. **Dependency Justification** — Each external dependency must be justified in documentation. Maximum of 3 external dependencies.

**Governance:** Medium — Requires scope review before acceptance.

**Examples:**
- ✅ "Minimal HTTP framework for TypeScript"
- ✅ "Test runner with fixtures and assertions"
- ❌ "Kitchen sink web framework with everything"

---

### Category: Protocols

**Definition:** Wire formats, communication standards, and specifications.

**Directory:** `protocols/`

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (reference implementation) |
| Tuulbelt Dependencies | Allowed |
| Scope | Specification-focused |
| Interface | Spec + Reference Implementation |
| Documentation | **SPEC.md required** |

**Additional Guidelines:**

1. **Specification First** — The spec is the product. Implementation is secondary.

2. **SPEC.md Required** — Every protocol must have a formal specification document covering wire format, semantics, error handling, and versioning.

3. **Test Vectors** — Provide compliance test vectors for cross-implementation testing.

4. **Cross-Language** — Specs should be implementable in any language.

**Governance:** High — Specification completeness review required.

**Examples:**
- ✅ "Self-describing binary TLV format"
- ✅ "Standard request/response envelope"
- ❌ "Protocol without formal spec"

---

### Category: Systems

**Definition:** Complex, integrated infrastructure projects.

**Directory:** `systems/`

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Minimal (pragmatic) |
| Tuulbelt Dependencies | Allowed |
| Scope | Large, integrated |
| Interface | Varies (CLI, REPL, server, client) |
| Documentation | ARCHITECTURE.md + SPEC.md required |

**Additional Guidelines:**

1. **Architecture First** — Design before implementation. ARCHITECTURE.md is mandatory.

2. **Module Boundaries** — Clear separation of concerns. Each module should be understandable independently.

3. **Performance Conscious** — Benchmarks are required. Performance is a feature.

4. **Incremental Milestones** — Ship working versions frequently. No big-bang releases.

**Governance:** High — Architecture review required.

**Examples:**
- ✅ "Small, embeddable scripting language"
- ✅ "Single-file embedded database"
- ❌ "Vaporware without implementation plan"

---

### Category: Meta

**Definition:** Tools and frameworks for building other tools/frameworks.

**Directory:** `meta/`

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Zero (dogfood principles) |
| Tuulbelt Dependencies | Allowed |
| Scope | Generative |
| Interface | CLI + Library API |
| Output | Generated code/artifacts |

**Additional Guidelines:**

1. **Deterministic Output** — Same input must produce same output.

2. **Output Quality** — Generated code must match hand-written quality. If you wouldn't be proud to ship the output, don't ship the generator.

3. **Customizable** — Provide templates and options for different needs.

4. **Dogfood** — Meta projects should follow the same principles they enable.

**Governance:** Medium — Generation pattern review.

**Examples:**
- ✅ "PEG parser generator"
- ✅ "Schema to code compiler"
- ❌ "Magic code generator with unpredictable output"

---

### Category: Research

**Definition:** Experimental projects exploring novel approaches.

**Directory:** `research/`

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Relaxed (pragmatism) |
| Tuulbelt Dependencies | Allowed |
| Scope | Exploratory |
| Interface | Whatever works |
| Documentation | HYPOTHESIS.md required |

**Additional Guidelines:**

1. **Hypothesis Required** — HYPOTHESIS.md must state what we're exploring and how we'll know if it works.

2. **Failure Is Acceptable** — Research projects may fail. Document findings either way.

3. **Clear Status** — Label projects with lifecycle stage in STATUS.md.

4. **Graduation Path** — Successful research may graduate to other categories.

**Lifecycle Stages:**
- `active` — Currently being explored
- `paused` — On hold (document why in STATUS.md)
- `concluded` — Exploration complete (success or failure documented in FINDINGS.md)
- `graduated` — Moved to production category (link to new location)

**Governance:** Low — Exploration is the goal.

**Examples:**
- ✅ "Zero-copy parsing experiments"
- ✅ "Effect system for TypeScript"
- ✅ "Novel compression algorithm prototype"

---

## Principles Comparison Matrix

| Principle | tools | libraries | frameworks | protocols | systems | meta | research |
|-----------|-------|-----------|------------|-----------|---------|------|----------|
| External Deps | Zero | Zero | Minimal (≤3) | Zero | Pragmatic | Zero | Relaxed |
| Scope | Single problem | Focused domain | Broad, bounded | Specification | Large, integrated | Generative | Exploratory |
| Primary Interface | CLI | API | API + Conventions | Spec + Impl | Varies | CLI + API | Any |
| CLI Required | Yes | No | No | No | Varies | Yes | No |
| API Stability | N/A | **Required** | Required | Required | Varies | Required | No |
| SPEC.md | No | No | No | **Yes** | Often | No | No |
| ARCHITECTURE.md | Optional | Optional | **Yes** | No | **Yes** | Optional | No |
| HYPOTHESIS.md | No | No | No | No | No | No | **Yes** |
| Test Vectors | No | No | No | **Yes** | No | No | No |
| Benchmarks | Optional | Optional | No | No | **Yes** | No | No |
| Output Quality | N/A | N/A | N/A | N/A | N/A | **Required** | No |
| Governance | Low | Low | Medium | High | High | Medium | Low |

---

## What Doesn't Belong in Tuulbelt

### Universal Rejections (All Categories)

- **Vaporware** — Ideas without implementation path
- **Unmaintained** — No clear maintainer or ownership
- **Duplicates** — Already solved by existing Tuulbelt project

### Category-Specific Rejections

**Tools:**
- Complete testing/web/build frameworks (too broad)
- Multi-format transformers with plugin systems (framework)
- Universal configuration management (scope creep)

**Libraries:**
- Libraries requiring significant setup
- Libraries with heavy transitive dependencies

**Frameworks:**
- Kitchen-sink frameworks trying to do everything
- Frameworks without escape hatches
- Frameworks requiring total buy-in

**Protocols:**
- Protocols without formal specification
- Proprietary or closed formats

**Systems:**
- Systems without clear architecture
- Vaporware without implementation plan

**Research:**
- Nothing is rejected if it has a clear hypothesis
- But must be labeled as experimental

---

## Before Building, Answer:

### For Tools
1. What single problem does it solve?
2. How would I test it end-to-end?
3. Can it run without installing dependencies?
4. Can it be used from any programming language?

### For Libraries
1. What domain does it focus on?
2. What's the API surface?
3. Are types well-defined?

### For Frameworks
1. What's the scope boundary?
2. How do users adopt incrementally?
3. What are the escape hatches?

### For Protocols
1. Is the specification complete?
2. Are error cases covered?
3. How does it version/evolve?

### For Systems
1. What's the architecture?
2. What are the module boundaries?
3. What are the performance characteristics?

### For Meta
1. What does it generate?
2. Is output deterministic?
3. Is it customizable?

### For Research
1. What's the hypothesis?
2. How will we know if it works?
3. What's the success criteria?

---

If you can't answer clearly, wait. Revisit the idea later.
