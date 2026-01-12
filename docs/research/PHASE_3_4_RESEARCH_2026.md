# Phase 3/4 Research: Pain Points & Vacuums (2026)

**Date:** 2026-01-12
**Objective:** Identify opportunities across all categories with deep analysis
**Methodology:** Surgical improvement analysis + novel approach exploration

---

## Research Rubric (Revised)

For each opportunity, we ask:

### Improvement Analysis (Surgical)
1. **What's broken?** - Inefficient, slow, buggy, insecure, over-complicated
2. **What's missing?** - Features no one has implemented well
3. **What's the DX pain?** - Developer experience friction

### Innovation Analysis (Generalist)
1. **Fresh perspective:** What would we do differently starting from scratch?
2. **Novel composition:** How can Tuulbelt tools enhance this?
3. **Horizontal expansion:** Does this enable new use cases?

### Decision Criteria
- ❌ **Skip:** Existing solutions are genuinely excellent AND no novel angle
- ⚠️ **Research first:** Promising but needs hypothesis validation
- ✅ **Build:** Clear improvements possible with proven approach

---

## Deep Dive: JSON Pointer (RFC 6901)

### What Exists

| Library | Deps | TypeScript | Performance | Features |
|---------|------|------------|-------------|----------|
| [json-ptr](https://npmjs.com/package/json-ptr) | 0 | Migrated | ❓ Unknown | Basic get/set |
| [jsonpointer](https://npmjs.com/package/jsonpointer) | 0 | Weak types | ❓ Unknown | Get only |
| [@hyperjump/json-pointer](https://npmjs.com/package/@hyperjump/json-pointer) | 2 | Good | ❓ Unknown | RFC compliant |

### What's Broken

1. **Performance: Parse on every access**
   ```typescript
   // Current libs do this EVERY TIME:
   get(obj, '/deeply/nested/path/to/value')
   // → split('/') → iterate → traverse → return

   // Called 1000x = parse 1000x
   ```

2. **Type Safety: Returns `any` or `unknown`**
   ```typescript
   const value = get(obj, '/address/city');
   // value: any ← No type information!
   // TypeScript can't help you
   ```

3. **Read-only RFC:** JSON Pointer is get/has only. Everyone needs updates:
   ```typescript
   // You want:
   const updated = set(obj, '/address/city', 'NYC');

   // But RFC doesn't define this, so everyone implements differently
   ```

4. **Error Handling: Throws or returns undefined**
   ```typescript
   get(obj, '/nonexistent/path')
   // → undefined? throw? depends on library
   // No Result<T, E> pattern
   ```

5. **No Schema Awareness:**
   ```typescript
   // I have a propval schema for User
   // I want: pointer that knows User.address.city is string
   // Current libs: "what schema?"
   ```

### Novel Approach: Typed Compiled Pointers

**Core Insight:** A JSON Pointer is like a property path. TypeScript can type property paths.

```typescript
// API Vision
import { pointer, compile } from '@tuulbelt/json-pointer';

// 1. Compiled pointers (parse once, use many)
const getCity = compile<User>('/address/city');
// getCity is typed as (obj: User) => string

const city = getCity(user); // city: string, not any!

// 2. Schema-integrated (with propval)
import { pointerFor } from '@tuulbelt/json-pointer';
import { UserSchema } from './schemas';

const getCity = pointerFor(UserSchema, '/address/city');
// Type inferred from schema!

// 3. Immutable updates with type safety
import { set } from '@tuulbelt/json-pointer';

const updated = set(user, '/address/city', 'NYC');
// updated: User (new object, original unchanged)
// Type error if you try: set(user, '/address/city', 123)

// 4. Result-based error handling
const result = get(user, '/nonexistent/path');
// result: Result<unknown, PointerError>
// No throws, explicit error handling

// 5. Batch operations (single traversal)
const values = getMany(user, ['/name', '/address/city', '/address/zip']);
// Traverses once, returns all values

// 6. JSON Patch (RFC 6902) natural extension
import { patch } from '@tuulbelt/json-pointer';

const updated = patch(user, [
  { op: 'replace', path: '/name', value: 'Jane' },
  { op: 'add', path: '/tags/-', value: 'new' }
]);
```

### Performance Innovation

```typescript
// Current: O(p) per access where p = path depth
get(obj, '/a/b/c/d/e'); // 5 operations per call

// Compiled: O(1) after compilation
const getter = compile('/a/b/c/d/e'); // Parse once
getter(obj); // Direct property access chain
getter(obj); // Reuse compiled form
getter(obj); // Reuse compiled form
```

**Benchmark potential:** 5-10x faster for repeated access patterns.

### Tuulbelt Composition

- **propval:** Schema → typed pointer derivation
- **structured-error-handler:** PointerError extends StructuredError
- **output-diffing-utility:** Diff at specific paths

### Recommendation

✅ **Strong candidate for Libraries category**

| Improvement | Impact |
|-------------|--------|
| Compiled pointers | 5-10x performance |
| Type-safe paths | Eliminates runtime type errors |
| Immutable updates | Missing from RFC, everyone needs |
| Result-based errors | Explicit error handling |
| propval integration | Unique to Tuulbelt ecosystem |

---

## Deep Dive: Request/Response Envelope

### What Exists

| Standard | Errors | Pagination | Metadata | Streaming | Typing |
|----------|--------|------------|----------|-----------|--------|
| JSON-RPC 2.0 | Basic (code/message) | ❌ | ❌ | ❌ | Weak |
| JSON-RPC 3.0 | Extended | ❌ | ❌ | ✅ | Weak |
| RFC 7807 (Problem Details) | Detailed | ❌ | ❌ | ❌ | Weak |
| gRPC Status | Detailed | Via metadata | Via metadata | ✅ | Strong |
| GraphQL | Errors array | Built-in | Extensions | ✅ | Strong |

### What's Broken

1. **JSON-RPC is Minimal:**
   ```json
   {
     "jsonrpc": "2.0",
     "result": {...},
     "id": 1
   }
   ```
   - Where's the request timing?
   - Where's the pagination?
   - Where's the request trace ID?
   - Error codes are just integers with no taxonomy

2. **Everyone Reinvents Differently:**
   ```json
   // Company A
   { "data": {...}, "meta": {...}, "errors": [...] }

   // Company B
   { "success": true, "result": {...}, "error": null }

   // Company C
   { "status": "ok", "payload": {...} }
   ```
   No interoperability, no shared tooling.

3. **Pagination is Ad-hoc:**
   ```json
   // Offset-based
   { "page": 2, "per_page": 20, "total": 100 }

   // Cursor-based
   { "cursor": "abc123", "has_more": true }

   // Keyset
   { "after_id": 500, "limit": 20 }
   ```
   Every API does it differently.

4. **Error Taxonomy Missing:**
   ```json
   { "code": -32600, "message": "Invalid Request" }
   // Is this:
   // - Validation error? (fix your input)
   // - Auth error? (get a new token)
   // - Server error? (retry later)
   // - Rate limit? (back off)
   ```

5. **No Correlation/Tracing:**
   - Request comes in, response goes out
   - No standard way to trace through systems
   - No built-in timing information

### Novel Approach: Composable Typed Envelope

**Core Insight:** The envelope is a protocol. Protocols need SPEC.md + test vectors + reference implementations.

**SPEC.md Preview:**

```markdown
# Tuulbelt Envelope Protocol v1.0

## Success Response
{
  "ok": true,
  "data": <T>,
  "meta": {
    "requestId": "<uuid>",
    "timestamp": "<iso8601>",
    "duration_ms": <number>,
    "version": "<semver>"
  },
  "pagination?": {
    "type": "cursor" | "offset" | "keyset",
    // type-specific fields
  }
}

## Error Response
{
  "ok": false,
  "error": {
    "code": "<category>.<specific>",  // e.g., "validation.required_field"
    "message": "<human readable>",
    "details?": {...},
    "path?": "<json-pointer>",        // Where the error occurred
    "requestId": "<uuid>",
    "timestamp": "<iso8601>"
  }
}
```

**TypeScript Codec:**

```typescript
import { Envelope, success, error } from '@tuulbelt/envelope';

// Type-safe envelope creation
const response = success({
  data: users,
  pagination: { type: 'cursor', cursor: 'abc', hasMore: true }
});
// response: Envelope<User[], CursorPagination>

// Type-safe error creation
const errorResp = error({
  code: 'validation.required_field',
  message: 'Email is required',
  path: '/email'
});

// Integration with structured-error-handler
import { StructuredError } from '@tuulbelt/structured-error-handler';

const errorResp = fromStructuredError(structuredError);
// Converts Tuulbelt errors to envelope format

// Integration with propval
import { validate } from '@tuulbelt/property-validator';

const result = validate(UserSchema, input);
if (!result.ok) {
  return validationError(result.errors);
  // Automatically formats validation errors
}
```

**Error Code Taxonomy:**

```typescript
// Hierarchical error codes
type ErrorCode =
  | `validation.${string}`    // 400: Fix your input
  | `auth.${string}`          // 401/403: Authenticate/authorize
  | `not_found.${string}`     // 404: Resource doesn't exist
  | `conflict.${string}`      // 409: State conflict
  | `rate_limit.${string}`    // 429: Slow down
  | `server.${string}`        // 500: Our fault, retry later
  | `unavailable.${string}`;  // 503: Temporarily down

// Machine-readable, human-understandable
// Clients can switch on category: code.split('.')[0]
```

**Streaming Extension:**

```typescript
// Stream delimiter: newline-delimited JSON
{"ok":true,"data":{"chunk":1},"meta":{...}}
{"ok":true,"data":{"chunk":2},"meta":{...}}
{"ok":true,"data":null,"meta":{"complete":true}}

// Or Server-Sent Events format
event: data
data: {"ok":true,"data":{"chunk":1}}

event: complete
data: {"ok":true,"meta":{"complete":true}}
```

### Tuulbelt Composition

- **structured-error-handler:** Error ↔ Envelope conversion
- **property-validator:** Validation errors → Envelope format
- **json-pointer:** Error paths reference exact locations

### Recommendation

✅ **Strong candidate for Protocols category**

| Improvement | Impact |
|-------------|--------|
| Formal SPEC.md | Interoperability across implementations |
| Error taxonomy | Machine-actionable error handling |
| Pagination patterns | All three types standardized |
| Tracing built-in | Observability by default |
| propval integration | Validation → error format seamless |
| Streaming support | Modern API patterns covered |

---

## Deep Dive: PEG Parser Generator

### What Exists

| Tool | Language | Output Types | Runtime Deps | Error Recovery |
|------|----------|--------------|--------------|----------------|
| peggy | JavaScript | ❌ any | ✅ Yes (peggy) | ❌ Poor |
| tree-sitter | C/WASM | ❌ Generic nodes | ✅ Yes (WASM) | ✅ Good |
| chevrotain | TypeScript | ❌ Manual | ✅ Yes | ⚠️ Basic |
| pest | Rust | ❌ Pairs iterator | ❌ Zero | ⚠️ Basic |
| Peginator | Rust | ✅ Typed AST | ❌ Zero | ❌ None |

### What's Broken

1. **peggy/pegjs: Runtime Heavy + Untyped**
   ```typescript
   // Grammar
   Start = Integer / String
   Integer = [0-9]+
   String = '"' [^"]* '"'

   // Generated parser returns:
   parse(input): any  // ← No types!

   // You get back nested arrays and strings
   // Have to manually construct AST
   ```

2. **tree-sitter: Overkill + Complex**
   - Requires C toolchain to build grammars
   - WASM bundle overhead (~200KB+)
   - Generic node types, not grammar-specific

3. **chevrotain: Verbose + Manual**
   ```typescript
   // You write the parser AND the types manually
   class MyParser extends CstParser {
     constructor() {
       super(allTokens);
       this.performSelfAnalysis();
     }

     // Hundreds of lines of parser rules...
   }
   ```

4. **pest (Rust): Untyped Output**
   ```rust
   // You get Pairs<Rule> iterator
   // Have to manually match and extract
   for pair in pairs {
     match pair.as_rule() {
       Rule::integer => { /* extract manually */ }
       Rule::string => { /* extract manually */ }
     }
   }
   ```

5. **Error Messages: Generally Poor**
   ```
   Parse error at line 5, column 12
   Expected one of: INTEGER, STRING, LPAREN
   ```
   - No context about what was being parsed
   - No suggestions for fixes
   - No recovery to continue parsing

### Novel Approach: Grammar → Types → Zero-Runtime Parser

**Core Insight:** The grammar IS the type definition. Generate both from one source.

**Grammar DSL:**

```
// math.peg

@name MathExpr
@description "Simple arithmetic expression parser"

// Tokens (terminals)
Number = /[0-9]+(\.[0-9]+)?/
Op = '+' | '-' | '*' | '/'

// Rules (non-terminals)
Expr = Term (('+' | '-') Term)*
Term = Factor (('*' | '/') Factor)*
Factor = Number | '(' Expr ')'
```

**Generated TypeScript Types:**

```typescript
// math.types.ts (GENERATED - DO NOT EDIT)

export type Expr = {
  type: 'Expr';
  terms: Term[];
  ops: ('+' | '-')[];
  loc: SourceLocation;
};

export type Term = {
  type: 'Term';
  factors: Factor[];
  ops: ('*' | '/')[];
  loc: SourceLocation;
};

export type Factor =
  | { type: 'Number'; value: number; loc: SourceLocation }
  | { type: 'Grouped'; expr: Expr; loc: SourceLocation };

export type MathExprAST = Expr;
```

**Generated Parser (Zero Runtime):**

```typescript
// math.parser.ts (GENERATED - DO NOT EDIT)

export function parse(input: string): Result<MathExprAST, ParseError> {
  // Pure TypeScript, no imports needed!
  // All parser logic is inlined

  let pos = 0;

  function parseExpr(): Result<Expr, ParseError> {
    // Generated parsing logic...
  }

  // ...rest of generated parser

  return parseExpr();
}
```

**CLI Tool:**

```bash
# Generate parser from grammar
$ peg-gen math.peg --out src/parser/

Generated:
  src/parser/math.types.ts    (TypeScript types)
  src/parser/math.parser.ts   (Zero-dep parser)
  src/parser/math.visitor.ts  (AST visitor pattern)

# Validate grammar
$ peg-gen validate math.peg

# Show type preview without generating
$ peg-gen preview math.peg
```

**Library API (for programmatic use):**

```typescript
import { compileGrammar } from '@tuulbelt/peg-gen';

const grammar = `
  Expr = Number ('+' Number)*
  Number = /[0-9]+/
`;

const { types, parser, visitor } = compileGrammar(grammar, {
  name: 'SimpleExpr',
  target: 'typescript'
});

// types: string (TypeScript source)
// parser: string (Parser source)
// visitor: string (Visitor source)
```

**Error Recovery:**

```typescript
// Grammar with recovery points
Expr = Term (('+' | '-') Term)* @recover(';')

// Parser will:
// 1. Try to parse normally
// 2. On error, skip to next ';' and continue
// 3. Collect all errors, return partial AST

const result = parse('1 + + 3; 4 + 5');
// result.ok = false
// result.errors = [{ message: "Unexpected '+'" at position 4 }]
// result.partial = { /* AST for "4 + 5" at least */ }
```

**Source Location Tracking:**

```typescript
// Every AST node includes location
type SourceLocation = {
  start: { line: number; column: number; offset: number };
  end: { line: number; column: number; offset: number };
};

// Useful for:
// - Error messages pointing to exact source
// - IDE features (go to definition, highlight)
// - Source maps
```

### Tuulbelt Composition

- **property-validator:** Validate parsed AST against schemas
- **structured-error-handler:** ParseError extends StructuredError
- **output-diffing-utility:** Diff ASTs for regression testing

### Recommendation

✅ **Strong candidate for Meta category**

| Improvement | Impact |
|-------------|--------|
| Grammar → Types | Eliminates manual type definitions |
| Zero runtime deps | Generated code is self-contained |
| Error recovery | Better DX for DSL authors |
| Source locations | IDE-ready ASTs |
| Visitor generation | Common pattern built-in |

---

## Reconsidered: Binary Protocol

I initially dismissed this as "CBOR/MessagePack adequate." Let me reconsider.

### What Could Be Better?

1. **Schema-Optional but Type-Inferred:**
   - MessagePack has no types (like JSON)
   - Protobuf requires schema (friction)
   - What if: Schema optional, but when present, full type inference?

2. **propval Schema → Serializer:**
   ```typescript
   import { toBinary, fromBinary } from '@tuulbelt/binary-codec';
   import { UserSchema } from './schemas';

   // Schema-aware serialization
   const bytes = toBinary(UserSchema, user);
   const decoded = fromBinary(UserSchema, bytes);
   // decoded: User (fully typed!)
   ```

3. **Zero-Copy in Rust, Streaming in TS:**
   - Rust: Memory-mapped, zero-copy access
   - TypeScript: Streaming decode for large payloads

### Assessment

⚠️ **Research first** - Interesting but needs hypothesis validation:
- Is propval-integrated binary encoding genuinely useful?
- What's the performance vs MessagePack?
- Is the complexity justified?

---

## Reconsidered: Embeddable Scripting

I initially dismissed as "Rhai is excellent." Let me reconsider.

### What Could Be Different?

1. **Validation DSL:**
   ```
   // Instead of propval TypeScript:
   const schema = v.object({ name: v.string(), age: v.number() });

   // A DSL for non-developers:
   schema User {
     name: string
     age: number range(0, 150)
     email: string email
   }
   ```

   This could compile to propval schemas.

2. **Config DSL with Logic:**
   ```
   // Beyond JSON/YAML - config with conditionals
   config {
     database {
       host = env("DB_HOST") ?? "localhost"
       pool_size = if (env("NODE_ENV") == "production") 20 else 5
     }
   }
   ```

3. **Workflow DSL:**
   ```
   workflow deploy {
     step build { run "npm build" }
     step test {
       run "npm test"
       on_failure { notify "slack" }
     }
     step deploy {
       needs [build, test]
       run "deploy.sh"
     }
   }
   ```

### Assessment

⚠️ **Research first** - Multiple potential angles:
- Validation DSL → propval compiler (most focused)
- Config DSL → config-file-merger enhancement
- General scripting → competing with Rhai (harder to differentiate)

---

## Priority Matrix (Revised)

### Clear Builds (Surgical Improvements)

| Opportunity | Category | Key Innovation | Effort | Impact |
|-------------|----------|----------------|--------|--------|
| JSON Pointer | Library | Compiled + typed + updates | Medium | High |
| Request/Response Envelope | Protocol | SPEC.md + error taxonomy + propval | Medium | High |
| PEG Parser Generator | Meta | Grammar → types + zero-runtime | High | High |
| Result/Option Type | Library | Minimal + typed + chain ops | Low | Medium |

### Research First (Novel Exploration)

| Opportunity | Category → Target | Hypothesis | Effort |
|-------------|-------------------|------------|--------|
| Validation DSL | Research → Meta | Can non-devs define schemas? | Medium |
| Binary Codec | Research → Protocol | Is propval-aware binary useful? | High |
| Effect System | Research → Library | Can we beat Effect-TS ergonomics? | High |
| Config DSL | Research → Meta | Is logic in config useful? | Medium |

### Skip (Genuinely Well-Served)

| Space | Why Skip |
|-------|----------|
| Web Frameworks | Hono is excellent AND minimal |
| Embedded DB | SQLite is unbeatable |
| General Scripting | Rhai is excellent AND extensive |

---

## Next Steps

### Immediate Actions

1. **Deep-dive JSON Pointer:** Create detailed API design doc
2. **Draft Envelope SPEC.md:** Wire format, error codes, pagination
3. **PEG Generator PoC:** Grammar → types proof of concept

### Research Track

4. **Validation DSL hypothesis:** HYPOTHESIS.md for non-dev schema authoring
5. **Binary codec prototype:** propval → MessagePack-like format

---

## Sources

- [JSON Pointer RFC 6901](https://datatracker.ietf.org/doc/html/rfc6901)
- [JSON Patch RFC 6902](https://datatracker.ietf.org/doc/html/rfc6902)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [RFC 7807 Problem Details](https://datatracker.ietf.org/doc/html/rfc7807)
- [peggy (JavaScript PEG)](https://peggyjs.org/)
- [pest (Rust PEG)](https://pest.rs/)
- [Peginator (Rust typed AST)](https://github.com/badicsalex/peginator)
- [tree-sitter](https://tree-sitter.github.io/)
- [MessagePack](https://msgpack.org/)
- [CBOR RFC 7049](https://cbor.io/)

---

**Document Status:** Revised with deep analysis
**Next Review:** After API design docs created
