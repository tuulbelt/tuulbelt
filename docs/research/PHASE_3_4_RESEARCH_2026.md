# Phase 3/4 Research: Pain Points & Vacuums (2026)

**Date:** 2026-01-12
**Objective:** Identify opportunities for Libraries, Frameworks, Protocols, Systems, Meta, and Research categories
**Methodology:** Apply category-specific validation rubrics from PRINCIPLES.md

---

## Summary

This research applies Tuulbelt's two-tier principle architecture to evaluate opportunities across all non-tool categories. Each opportunity is validated against category-specific requirements.

---

## Category: Libraries

**Requirements:** Zero external deps, API stability, type safety, focused domain, API.md required

### High Priority Opportunities

#### 1. Result Type / Option Type Library

**Problem:** TypeScript lacks native Result/Option types for type-safe error handling.

**Existing Solutions:**
- [fp-ts](https://gcanti.github.io/fp-ts/) - Full FP library, heavy (~40KB)
- [neverthrow](https://github.com/supermacro/neverthrow) - Good API but has dependencies
- [true-myth](https://github.com/true-myth/true-myth) - Similar approach
- [Effect-TS](https://effect.website/) - Full effect system, very large

**Vacuum:** No zero-dependency, minimal Result<T,E> / Option<T> implementation with:
- TypeScript-first design
- Chainable operations (map, flatMap, match)
- Pattern matching support
- < 5KB bundle size

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Zero external deps | ✅ Achievable |
| Focused domain | ✅ Error handling only |
| API stability | ✅ Simple surface area |
| Type safety | ✅ Core purpose |

**Recommendation:** ✅ Strong candidate

---

#### 2. JSON Pointer (RFC 6901) Implementation

**Problem:** Need for spec-compliant JSON path manipulation.

**Existing Solutions:**
- [js-pointer](https://github.com/toboid/js-pointer) - Zero deps, but not TypeScript-first
- [@hyperjump/json-pointer](https://www.npmjs.com/package/@hyperjump/json-pointer) - Has dependencies
- [json-ptr](https://www.npmjs.com/package/json-ptr) - Migrated to TypeScript

**Vacuum:** TypeScript-first RFC 6901 implementation with:
- Type-safe get/set operations
- Immutable update operations
- Zero dependencies

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Zero external deps | ✅ Achievable |
| Focused domain | ✅ RFC 6901 only |
| API stability | ✅ RFC-defined |
| Type safety | ✅ Typed path operations |

**Recommendation:** ✅ Good candidate

---

#### 3. Persistent Immutable Data Structures

**Problem:** Need for efficient immutable collections.

**Existing Solutions:**
- [Immutable.js](https://immutable-js.com/) - Large, Facebook-backed
- [Immer](https://immerjs.github.io/immer/) - Proxy-based, different approach
- [persistent-ts](https://github.com/cronokirby/persistent-ts) - TypeScript-first

**Vacuum:** Minimal persistent collections (List, Map, Set) with:
- Structural sharing
- TypeScript-first
- Zero dependencies
- Performance competitive with Immutable.js

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Zero external deps | ✅ Achievable |
| Focused domain | ✅ Persistent collections |
| API stability | ⚠️ Large API surface |
| Type safety | ✅ Generic types |

**Recommendation:** ⚠️ Medium priority - larger scope than typical library

---

### Lower Priority

- **Streaming JSON Parser** - Existing solutions adequate
- **Binary Search Trees** - Niche use case
- **Functional Lenses** - Part of fp-ts ecosystem

---

## Category: Frameworks

**Requirements:** Max 3 external deps (justified), bounded scope, escape hatches, incremental adoption, ARCHITECTURE.md required

### Assessment of Current Landscape

#### Minimal Web Frameworks (2025-2026)

**Current Leaders:**
- [Hono](https://hono.dev/) - 14KB, zero deps, Web Standards
- [Elysia](https://elysiajs.com/) - Bun-first, excellent TypeScript
- [Fastify](https://fastify.dev/) - Performance-focused, mature

**Vacuum Analysis:**

The minimal web framework space is **well-served** by Hono. Creating another would violate the "Proven Implementation" principle unless it addresses a specific gap Hono doesn't.

**Potential Gaps:**
1. **Type-safe middleware composition** - Hono has some, could be stronger
2. **OpenAPI-first framework** - Define routes from spec
3. **Testing framework** - No dominant zero-dep solution

**Recommendation:** ❌ Avoid web frameworks - Hono covers this space

---

#### Test Runner Framework

**Problem:** Node.js native test runner is minimal; Jest/Vitest are heavy.

**Existing Solutions:**
- Node.js `node:test` - Built-in, minimal
- [uvu](https://github.com/lukeed/uvu) - Fast, minimal (~3KB)
- Jest/Vitest - Feature-rich but heavy

**Vacuum:** A test framework that:
- Extends `node:test` with fixtures and assertions
- Zero external dependencies
- First-class TypeScript support
- Snapshot testing built-in

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Max 3 deps | ✅ Could be zero |
| Bounded scope | ✅ Test running only |
| Escape hatches | ✅ Falls back to node:test |
| Incremental adoption | ✅ Works with existing tests |

**Recommendation:** ⚠️ Medium - Tuulbelt already has snapshot-comparison tool

---

## Category: Protocols

**Requirements:** Zero deps, SPEC.md required, test vectors, cross-language implementable

### High Priority Opportunities

#### 1. Self-Describing Binary Wire Protocol (TLV)

**Problem:** Need for efficient, self-describing binary format.

**Existing Solutions:**
- [MessagePack](https://msgpack.org/) - Schema-optional, IETF-like
- [CBOR (RFC 7049)](https://cbor.io/) - IETF standard, concise
- [Protocol Buffers](https://protobuf.dev/) - Schema-required, Google
- [Cap'n Proto](https://capnproto.org/) - Zero-copy, fast

**Vacuum:** A Tuulbelt-style protocol with:
- Formal SPEC.md documenting wire format
- Test vectors for compliance
- Reference implementations in TypeScript and Rust
- Focus on simplicity over features

**Analysis:** This is a "Systems" category project more than Protocol, as it would require significant architecture. MessagePack/CBOR already solve this well.

**Recommendation:** ❌ Skip - CBOR/MessagePack adequate

---

#### 2. Request/Response Envelope Codec

**Problem:** Standardized API response wrapping for errors, metadata, pagination.

**Existing Solutions:**
- [JSON-RPC 2.0](https://www.jsonrpc.org/specification) - Standard, but minimal
- [JSON-RPC 3.0](https://github.com/KrishnaPG/json-rpc-v3.0) - Streaming support
- Custom implementations per company

**Vacuum:** A lightweight envelope specification for:
- Error structure (code, message, details)
- Pagination metadata
- Request tracing (requestId, timestamp)
- Version negotiation

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Zero deps | ✅ Spec + codec only |
| SPEC.md | ✅ Core deliverable |
| Test vectors | ✅ Required |
| Cross-language | ✅ JSON-based |

**Recommendation:** ✅ Good candidate - fills gap in lightweight API standards

---

#### 3. Pub-Sub Message Protocol

**Problem:** Simple wire format for service-to-service messaging.

**Existing Solutions:**
- MQTT - IoT-focused
- AMQP - Heavy, complex
- Custom JSON formats

**Vacuum:** Minimal pub-sub protocol with:
- Topic addressing
- Message acknowledgment
- Dead letter handling
- Formal specification

**Recommendation:** ⚠️ Medium - niche use case

---

## Category: Systems

**Requirements:** Minimal deps (pragmatic), ARCHITECTURE.md + SPEC.md, benchmarks required, incremental milestones

### Assessment

Systems are complex, integrated projects. These require significant investment and clear architecture before starting.

#### 1. Embeddable Scripting Language

**Problem:** Need for sandboxed scripting in applications.

**Existing Solutions for Rust:**
- [Rhai](https://rhai.rs/) - Embedded scripting, no-std support
- [Dyon](https://github.com/PistonDevelopers/dyon) - Game-focused
- [Gluon](https://github.com/gluon-lang/gluon) - Statically typed

**Existing Solutions for TypeScript:**
- QuickJS bindings - Full ES2023
- Custom DSLs

**Vacuum Analysis:**

The Rust scripting space is **well-served** by Rhai. A Tuulbelt scripting language would need a compelling differentiator:
- Smaller/faster than Rhai
- Better TypeScript integration
- Novel feature (e.g., effect handlers)

**Recommendation:** ❌ Skip unless clear differentiator - Rhai is excellent

---

#### 2. Single-File Embedded Database

**Problem:** SQLite alternatives for specific use cases.

**Existing Solutions:**
- [SQLite](https://sqlite.org/) - Gold standard
- [libSQL/Turso](https://turso.tech/) - SQLite fork with enhancements
- [PoloDB](https://github.com/PoloDB/PoloDB) - MongoDB-like in Rust

**Vacuum:** Very limited. SQLite is hard to compete with.

**Recommendation:** ❌ Skip - SQLite ecosystem is comprehensive

---

## Category: Meta

**Requirements:** Zero deps, deterministic output, output quality matching hand-written, CLI + Library API

### High Priority Opportunities

#### 1. PEG Parser Generator with Typed AST

**Problem:** PEG parsers often produce untyped trees requiring manual conversion.

**Existing Solutions:**
- [pest](https://pest.rs/) - Great PEG, but untyped output
- [Peginator](https://github.com/badicsalex/peginator) - Typed AST for Rust
- [tree-sitter](https://tree-sitter.github.io/) - Incremental parsing

**Vacuum for TypeScript:**
A PEG parser generator that:
- Takes grammar definition
- Generates TypeScript types for AST nodes
- Generates parser code
- Zero runtime dependencies

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Zero deps | ✅ Generated code is self-contained |
| Deterministic | ✅ Grammar → same output |
| Output quality | ✅ Typed, clean code |
| CLI + API | ✅ CLI generator + runtime lib |

**Recommendation:** ✅ Strong candidate

---

#### 2. Schema to Code Compiler

**Problem:** Generate code from schemas (OpenAPI, JSON Schema, etc.)

**Existing Solutions:**
- [GraphQL Code Generator](https://the-guild.dev/graphql/codegen) - GraphQL-focused
- [OpenAPI Generator](https://openapi-generator.tech/) - Many languages
- @rtk-query/codegen-openapi - RTK-specific

**Vacuum:** A unified schema-to-code tool that:
- Supports JSON Schema, OpenAPI, GraphQL
- Generates TypeScript types
- Generates validation code (using propval!)
- Zero dependencies in generated code

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Zero deps | ⚠️ May need schema parser |
| Deterministic | ✅ Schema → same output |
| Output quality | ✅ Critical requirement |
| CLI + API | ✅ Both needed |

**Recommendation:** ⚠️ Medium - scope may be too large

---

## Category: Research

**Requirements:** HYPOTHESIS.md required, failure acceptable, clear lifecycle status

### Exploratory Opportunities

#### 1. Effect System for TypeScript

**Hypothesis:** Can we implement algebraic effects in TypeScript that are:
- More ergonomic than Effect-TS
- Zero-dependency
- Compatible with async/await

**Prior Art:**
- [Effect-TS](https://effect.website/) - Full effect system
- [susisu/effects](https://github.com/susisu/effects) - PoC implementation
- React Suspense - Similar concept

**Research Questions:**
1. Can generator functions provide effect handlers?
2. What's the performance overhead?
3. Can we infer effect types automatically?

**Validation:**
| Criterion | Assessment |
|-----------|------------|
| Hypothesis clear | ✅ Defined above |
| Success criteria | ⚠️ Needs definition |
| Graduation path | Libraries if successful |

**Recommendation:** ✅ Good research candidate

---

#### 2. Zero-Copy Parsing Experiments

**Hypothesis:** Can we achieve zero-copy parsing for structured data in TypeScript/Rust with:
- No intermediate allocations
- Type-safe access
- Competitive performance

**Prior Art:**
- FlatBuffers (C++)
- Cap'n Proto (zero-copy RPC)

**Recommendation:** ✅ Good research candidate for Rust

---

#### 3. Compile-Time Validation via TypeScript Transformers

**Hypothesis:** Can propval schemas be validated at compile time, eliminating runtime overhead for static data?

**Prior Art:**
- [Typia](https://typia.io/) - Uses ts-patch
- [ts-morph](https://ts-morph.com/) - TypeScript manipulation

**Recommendation:** ✅ Good research candidate - extends propval

---

## Priority Matrix

### Phase 3 (Libraries)

| Opportunity | Priority | Effort | Impact |
|-------------|----------|--------|--------|
| Result/Option Type | 🟢 High | Low | High |
| JSON Pointer RFC 6901 | 🟢 High | Low | Medium |
| Persistent Collections | 🟡 Medium | High | Medium |

### Phase 4 (Frameworks/Protocols/Systems/Meta)

| Opportunity | Category | Priority | Effort | Impact |
|-------------|----------|----------|--------|--------|
| Request/Response Envelope | Protocol | 🟢 High | Low | Medium |
| PEG Parser Generator (TS) | Meta | 🟢 High | High | High |
| Test Runner Framework | Framework | 🟡 Medium | Medium | Medium |
| Schema to Code Compiler | Meta | 🟡 Medium | High | High |

### Research

| Opportunity | Priority | Timeframe |
|-------------|----------|-----------|
| Effect System TS | 🟢 High | 3-6 months |
| Zero-Copy Parsing | 🟡 Medium | 6-12 months |
| Compile-Time Validation | 🟡 Medium | 3-6 months |

---

## Recommendations for Next Steps

### Immediate (Phase 3)

1. **Create `libraries/` directory structure**
2. **Implement Result/Option library** - Validates library template
3. **Implement JSON Pointer library** - RFC compliance testing

### Short-term (Phase 4)

4. **Define Request/Response Envelope SPEC.md** - Validates protocol template
5. **Prototype PEG Parser Generator** - Validates meta template

### Research Track

6. **Start Effect System exploration** - Create HYPOTHESIS.md
7. **Document findings regardless of success**

---

## Sources

### Libraries
- [fp4ts - Zero-dependency FP library](https://github.com/fp4ts/fp4ts)
- [fp-ts](https://gcanti.github.io/fp-ts/)
- [persistent-ts](https://github.com/cronokirby/persistent-ts)
- [js-pointer](https://github.com/toboid/js-pointer)

### Frameworks
- [Hono - Web framework on Web Standards](https://github.com/honojs/hono)
- [Node.js Frameworks Comparison 2024](https://dev.to/encore/nodejs-frameworks-roundup-2024-elysia-hono-nest-encore-which-should-you-pick-19oj)

### Protocols
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [CBOR vs Other Formats](https://cborbook.com/introduction/cbor_vs_the_other_guys.html)
- [MessagePack](https://msgpack.org/index.html)

### Systems
- [Rhai - Embedded Scripting for Rust](https://rhai.rs/)
- [libSQL/Turso](https://github.com/tursodatabase/libsql)
- [PoloDB](https://github.com/PoloDB/PoloDB)

### Meta
- [Peginator - PEG parser with typed AST](https://github.com/badicsalex/peginator)
- [pest parser](https://pest.rs/)
- [GraphQL Code Generator](https://the-guild.dev/graphql/codegen)

### Research
- [Effect-TS](https://effect.website/)
- [Algebraic Effects for the Rest of Us](https://overreacted.io/algebraic-effects-for-the-rest-of-us/)
- [Effects Bibliography](https://github.com/yallop/effects-bibliography)

---

**Document Status:** Complete
**Next Review:** After Phase 3 implementation begins
