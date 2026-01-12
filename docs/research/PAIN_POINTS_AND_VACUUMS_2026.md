# Developer Pain Points and Market Vacuums Research (2026)

**Date:** 2026-01-12
**Status:** Research Round 3
**Branch:** claude/expand-meta-repo-structure-Bdx1K

---

## Executive Summary

This research identifies pain points and vacuums in the developer ecosystem that could be filled with effective, focused solutions. The focus is on **external opportunities**, not internal Tuulbelt improvements.

---

## Part 1: Major Developer Pain Points (2025-2026)

### 1.1 Time Lost to Non-Coding Tasks

**Key Statistics:**
- Developers lose **3 hours/week** to tool failures, outages, workflow glitches (~20 workdays/year)
- Only **16%** of time spent on rewarding work (writing code, building features)
- **75%** of time wasted maintaining toolchains rather than coding
- **$8,000/developer/year** lost to tool-related productivity drains

**Specific Frustrations:**
- Networking/connectivity problems (48%)
- Permissions and access issues (35%)
- Tool setup/configuration (34%)
- Frequent context-switching between 10+ tools

**Sources:** [Jellyfish](https://jellyfish.co/library/developer-productivity/pain-points/), [Platform Engineering](https://platformengineering.com/developer-experience/hidden-tech-frustrations-cost-developers-20-workdays-a-year/)

---

### 1.2 AI Tool Frustrations (Emerging 2025-2026)

**Key Findings:**
- **66%** frustrated with "AI solutions that are almost right, but not quite"
- **45%** find debugging AI-generated code more time-consuming
- Trust in AI accuracy **dropped from 40% to 29%**
- Positive favorability decreased from **72% to 60%**
- **40%+** say AI tools slow them down

**The New Challenge:**
Developers now spend time guiding AI, reviewing AI-generated code, and correcting errors they didn't create. They're responsible for code they don't fully understand.

**Sources:** [Stack Overflow 2025 Survey](https://survey.stackoverflow.co/2025/), [MIT Technology Review](https://www.technologyreview.com/2025/12/15/1128352/rise-of-ai-coding-developers-2026/)

---

### 1.3 Documentation Gaps

- Only **16%** received adequate training/documentation
- **33%** received no support at all
- Documentation separated from code becomes unmanaged
- Complex schema validation errors produce unhelpful messages

---

### 1.4 Environment Inconsistencies

- "It works on my machine" still frustrating
- Teams spend **30 minutes/day** getting dev environments working
- New team members take **weeks** to become productive
- Dev, test, production environment mismatches

---

## Part 2: Ecosystem-Specific Gaps

### 2.1 TypeScript Ecosystem

**Toolchain Fragmentation:**
- "The JavaScript ecosystem really needs leadership and unification in tooling"
- Build tools fragmented: Rollup, esbuild, Vite, webpack, tsup, bunup
- Rollup being rewritten in Rust (Rolldown) - signals performance gaps

**TypeScript 7.0 Transition Issues:**
- Corsa API replacing Strada API
- No stable tooling integration for new API
- Linters, formatters, IDE extensions will break

**Missing:**
- Sub-100ms builds still not universal
- Same bundler for dev and production (Rolldown aims to solve)

**Sources:** [TypeScript Blog](https://devblogs.microsoft.com/typescript/progress-on-typescript-7-december-2025/), [This Dot Labs](https://www.thisdot.co/blog/the-2025-guide-to-js-build-tools)

---

### 2.2 Rust Ecosystem

**Backend Framework Gaps:**
- Smaller ecosystem than Node.js (~170K crates vs millions of npm packages)
- No "batteries-included" experience like Express
- Frameworks: Axum, Actix, Rocket - all require significant assembly

**Specific Pain Points:**
- Doctests tooling issues (IDE support, clippy integration)
- Async ecosystem fragmentation (tokio vs async-std)
- Steep learning curve for newcomers
- Self-referential structures remain difficult

**Sources:** [Rust in 2025](https://lucisqr.substack.com/p/rust-in-2025-trends-tools-and-controversies), [corrode Rust Consulting](https://corrode.dev/learn/migration-guides/typescript-to-rust/)

---

### 2.3 Testing Frameworks

**Jest Limitations:**
- CommonJS-oriented, experimental ESM support
- Limited TypeScript support (requires configuration)
- Slower debugging, especially with TypeScript
- Cannot mock shared workers effectively
- Hoisting system confusing

**Vitest Limitations:**
- Newer, fewer advanced features/plugins
- No native browser testing
- Depends on Vite ecosystem
- Documentation still evolving
- 95% Jest compatible (5% requires adjustments)

**Gap:** No testing framework that is:
- Native ESM + TypeScript
- Fast (no transpilation)
- Browser + Node unified
- Zero/minimal configuration

**Sources:** [Vitest Comparisons](https://vitest.dev/guide/comparisons), [Sauce Labs](https://saucelabs.com/resources/blog/vitest-vs-jest-comparison)

---

### 2.4 Observability & Tracing

**Pre-OpenTelemetry Chaos:**
- 5+ dashboards to debug one request
- CloudWatch, Azure Monitor, Stackdriver, Prometheus, ELK fragmentation
- Tool switching required rewriting instrumentation

**OpenTelemetry Problems (2025):**
- Metrics, logs, profiling still evolving/experimental
- Sampling challenges (consistency in distributed systems)
- Browser/frontend observability gaps (no session data model)
- Version compatibility issues between components
- AI agent observability complexity

**Gap:** Simple, unified tracing that works across:
- Backend services
- Frontend/browser
- AI agents
- Without OpenTelemetry complexity

**Sources:** [CNCF](https://www.cncf.io/blog/2025/11/27/from-chaos-to-clarity-how-opentelemetry-unified-observability-across-clouds/), [Site24x7](https://www.site24x7.com/blog/4-common-opentelemetry-challenges)

---

### 2.5 Configuration Management

**JSON Problems:**
- No comments allowed
- Strict syntax (trailing commas forbidden)
- Debugging misplaced commas time-consuming

**YAML Problems:**
- Indentation-based errors (single space breaks everything)
- Auto-conversion surprises (`yes` → boolean, dates auto-parsed)
- Security: `yaml.load()` can execute arbitrary code
- Third-party library inconsistencies

**Schema Validation Issues:**
- Validator fragmentation (different draft implementations)
- Dense specifications, hard to master
- Error messages point to internal paths, not clear explanations

**Observation:**
"Configuration becomes a programming language pretending to be YAML" - the problem isn't the format, it's complexity creep.

**Sources:** [DataFormatHub](https://www.dataformathub.com/blog/json-vs-yaml-vs-json5-the-truth-about-data-formats-in-2025-ewv), [Kula Blog](https://kula.blog/posts/yaml/)

---

### 2.6 Build Systems

**CMake Frustrations:**
- Complex syntax, steep learning curve
- Functions cannot return values conventionally
- Caching problems in CI/CD
- No integration with Rust/Cargo ecosystem
- Backwards compatibility tension

**Bazel Challenges:**
- Cryptic error messages
- Poor documentation beyond basics
- Complex installation process
- Google control over contributions

**Gap:** Build system that is:
- Simple syntax (not its own language)
- Cross-language (Rust + TypeScript + C++)
- Fast incremental builds
- Good error messages
- Easy to get started

**Sources:** [Buildkite](https://buildkite.com/resources/comparison/bazel-vs-cmake/), [Kea Sigma Delta](https://keasigmadelta.com/blog/cmake-vs-bazel-in-practise-real-code-real-results/)

---

## Part 3: Phase 4 Category Opportunities

Based on research, here are validated opportunities for each Phase 4 category:

### 3.1 Systems Category

#### Embeddable Scripting Languages

**Current Options:**
- Lua: Ubiquitous but limited features
- Wren: Smaller than Lua, strict OOP
- Janet: Batteries-included, Lisp syntax
- Squirrel: Lua-inspired, OOP support (~200KB)
- Daslang: High performance, rivals compiled languages
- Rhai/Dyon: Rust-native options

**Vacuum:** No embeddable language that is:
- Modern syntax (not Lisp/Forth)
- Rust-native (not C bindings)
- Small footprint (<100KB)
- Type-safe (optional typing)
- Async-aware
- Good error messages

**Validation:** Real problem (game engines, config, automation), proven approaches exist, clear implementation path.

---

#### Embedded Databases

**Current Options:**
- SQLite: C-based, ubiquitous
- Limbo/Turso: SQLite rewrite in Rust (BETA)
- Stoolap: MVCC, ACID, Rust-native
- PoloDB: MongoDB-like, document-based
- sled: Modern embedded KV store

**Vacuum:**
- Limbo is BETA, not production-ready
- Stoolap is new
- No mature, pure-Rust SQLite alternative
- Document databases (PoloDB) less mature

**Validation:** Real problem (Rust apps need C wrappers for SQLite), active development (Turso/Limbo), but risky (large scope).

---

### 3.2 Frameworks Category

**Web Frameworks - Rust:**
- Axum: Modern async, recommended for beginners
- Actix: Performance leader, millions of requests/day
- Rocket: Simplicity focus
- Tide: Minimal but async-std limited
- Poem: Lightweight, HTTP/3 support
- Cot: Async-first, modular (early development)

**Vacuum:** All require assembly. No "batteries-included" Rust web framework with:
- Auth built-in
- Database integration
- Deployment helpers
- Like Django/Rails but for Rust

**Validation:** Real need, but HIGH RISK - frameworks tend toward bloat. Against Tuulbelt minimal philosophy.

**Recommendation:** SKIP frameworks category or keep extremely minimal.

---

### 3.3 Protocols Category

**Binary Serialization:**
- Protobuf: Schema-required, Google ecosystem
- MessagePack: Schema-less, 50+ languages
- FlatBuffers: Zero-copy, gaming focus
- Cap'n Proto: Complex encoding
- BSON: MongoDB ecosystem

**Vacuum:** No self-describing binary format that is:
- Schema-optional (like MessagePack)
- Type-tagged (self-describing like CBOR)
- Streaming-friendly
- Zero-copy possible
- Simple specification

**Validation:** Specific need exists (RPC without schema management), proven approaches.

---

### 3.4 Meta Category

**Parser Generators:**
- pest: PEG-based, popular, no typed AST output
- nom: Combinator-based, faster, binary-focused
- LALRPOP: Yacc-like, typed output
- rust-peg: Simple, Rust macro-based

**Vacuum:** No parser generator that:
- Produces typed AST automatically
- Has good error recovery
- Generates both parser + serializer
- Simple PEG-like syntax

**Validation:** Real problem (pest users must write AST conversion manually), clear solution space.

---

**Code Generation:**
- GraphQL Codegen: Operations-based, flexible plugins
- OpenAPI Codegen: RTK Query integration
- datamodel-codegen: Python/Pydantic focus

**Vacuum:** No unified schema → code generator for:
- TypeScript types
- Rust structs
- Validation code
- From multiple sources (JSON Schema, OpenAPI, GraphQL)

**Validation:** Real need, but scope creep risk HIGH.

---

### 3.5 Research Category

**Effect Systems:**
- Effect-TS: Full-featured, production-ready, complex
- algebraic-effects-ts: Multi-shot continuations, not production-ready
- susisu/effects: "Poor man's effects", PoC

**Vacuum:** No lightweight effect system that:
- Works with existing TypeScript (no new syntax)
- Minimal learning curve
- Incrementally adoptable
- Doesn't require "Effect" ecosystem buy-in

**Validation:** Academic interest high, production adoption growing (Effect-TS), research opportunity.

---

## Part 4: Prioritized Opportunities

### High Confidence (Proven need, clear path)

| Opportunity | Category | Why |
|-------------|----------|-----|
| **Parser generator with typed AST** | meta | pest users want this, clear gap |
| **Self-describing binary protocol** | protocols | MessagePack + type tags, spec-first |
| **Minimal effect system research** | research | Effect-TS is complex, simpler approach possible |

### Medium Confidence (Real need, execution risk)

| Opportunity | Category | Risk |
|-------------|----------|------|
| **Embeddable language** | systems | Large scope, competition from Wren/Janet |
| **Unified schema codegen** | meta | Scope creep, many partial solutions exist |
| **Simple tracing library** | libraries | OpenTelemetry momentum, ecosystem fragmentation |

### Low Confidence (High risk, uncertain value)

| Opportunity | Category | Risk |
|-------------|----------|------|
| **Web framework (Rust)** | frameworks | Against minimal philosophy, scope explosion |
| **Embedded database** | systems | Massive scope, Turso/Limbo competition |
| **Build system** | systems | CMake/Bazel entrenched, huge investment |

---

## Part 5: Validation Rubric Applied

For each high-confidence opportunity:

### Parser Generator with Typed AST

| Criterion | Assessment |
|-----------|------------|
| **Proven implementation path** | Yes - pest + LALRPOP show both approaches work |
| **Real problem** | Yes - pest users manually write AST conversion |
| **Zero deps possible** | Yes - parser generators are pure computation |
| **Single problem** | Yes - grammar → typed parser |
| **Independently cloneable** | Yes |
| **Not vaporware** | Clear implementation: PEG grammar → Rust/TS types |

**Verdict:** ✅ STRONG CANDIDATE

---

### Self-Describing Binary Protocol

| Criterion | Assessment |
|-----------|------------|
| **Proven implementation path** | Yes - CBOR, MessagePack show approaches |
| **Real problem** | Yes - schema management overhead |
| **Zero deps possible** | Yes - pure encode/decode |
| **Single problem** | Yes - serialization format |
| **Independently cloneable** | Yes |
| **Not vaporware** | Spec-first, reference implementation follows |

**Verdict:** ✅ STRONG CANDIDATE (protocols category)

---

### Minimal Effect System (Research)

| Criterion | Assessment |
|-----------|------------|
| **Proven implementation path** | Partial - Effect-TS works but complex |
| **Real problem** | Yes - async/error composition is messy |
| **Zero deps possible** | Yes (research can be relaxed) |
| **Single problem** | Focused on effects, not full framework |
| **Independently cloneable** | Yes |
| **Not vaporware** | Research category allows exploration |

**Verdict:** ✅ GOOD RESEARCH CANDIDATE

---

## Part 6: Recommended Next Steps

1. **Protocols:** Design spec for self-describing binary format
2. **Meta:** Prototype parser generator with typed output
3. **Research:** Explore minimal effect system for TypeScript
4. **Skip:** Frameworks, large systems (database, build system)

---

## Sources

### Developer Productivity
- [Jellyfish - Developer Pain Points](https://jellyfish.co/library/developer-productivity/pain-points/)
- [Platform Engineering - Hidden Tech Frustrations](https://platformengineering.com/developer-experience/hidden-tech-frustrations-cost-developers-20-workdays-a-year/)
- [Stack Overflow 2025 Survey](https://survey.stackoverflow.co/2025/)

### Ecosystem Analysis
- [TypeScript 7.0 Progress](https://devblogs.microsoft.com/typescript/progress-on-typescript-7-december-2025/)
- [Rust in 2025](https://lucisqr.substack.com/p/rust-in-2025-trends-tools-and-controversies)
- [Vitest Comparisons](https://vitest.dev/guide/comparisons)

### Specific Technologies
- [CNCF - OpenTelemetry](https://www.cncf.io/blog/2025/11/27/from-chaos-to-clarity-how-opentelemetry-unified-observability-across-clouds/)
- [Embedded Scripting Languages](https://github.com/dbohdan/embedded-scripting-languages)
- [Turso/Limbo](https://turso.tech/blog/introducing-limbo-a-complete-rewrite-of-sqlite-in-rust)
- [pest Parser](https://pest.rs/)
- [Effect-TS](https://github.com/Effect-TS/effect)

---

*End of Research Document*
