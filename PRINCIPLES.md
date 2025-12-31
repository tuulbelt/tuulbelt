# Tuulbelt Design Principles

These principles guide what gets built, how it's built, and what doesn't belong.

## 1. Single Problem Per Tool

Each tool solves **one problem** and solves it well.

**Good:** "Detect unreliable tests by running them N times"
**Bad:** "Complete test framework with mocking, fixtures, and reporting"

If you're adding features and the scope is growing, it's time to split into a new tool.

## 2. Zero External Dependencies (Standard Library Only)

No `npm install`, no `cargo add`, no runtime dependency management required.

**Exception 1:** Development dependencies (TypeScript compiler, test runners) are okay if they don't ship with the tool.

**Exception 2:** Tuulbelt tools MAY use other Tuulbelt tools via library integration (git URL dependencies). Since all Tuulbelt tools have zero external dependencies, composing them preserves the zero-dep guarantee. This enables richer functionality without dependency chains.

```toml
# Rust: Use git URL dependency for standalone repos
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility.git" }
```

```json
// TypeScript: Use git URL dependency for standalone repos
{
  "dependencies": {
    "@tuulbelt/cli-progress-reporting": "git+https://github.com/tuulbelt/cli-progress-reporting.git"
  }
}
```

**Rationale:** Tools that don't require dependency resolution outlive language trends. They're more portable, more maintainable, and less fragile. Tuulbelt-to-Tuulbelt composition maintains these benefits while enabling powerful tool combinations.

## 3. Portable Interface

Tools communicate via:
- **CLI** with flags and stdin/stdout
- **Files** (plain text, JSON, binary)
- **Environment variables**
- **Sockets** (TCP, UDP)

NOT via:
- Proprietary package APIs
- Internal state management
- Plugin systems
- Configuration frameworks

**Rationale:** Interfaces that work across languages, shells, and systems are reusable.

## 4. Composable (Pipes, Not Plugins)

Tools should chain together via pipes:

```bash
tool-a | tool-b | tool-c
```

NOT via:
- Plugin systems
- Hook APIs
- Event listeners

**Rationale:** Composition over extension keeps tools independent.

## 5. Independently Cloneable

Each tool is its own GitHub repository.

Users should be able to:
```bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
npm test && npm run build
```

Without needing the meta repo or any other tool.

## 6. Proven Implementation, Not Moonshots

Before building:
- Problem must be real (you hit it, or it's documented in production)
- Solution must have a clear implementation path
- No "it would be cool if..." without proof it works
- No "works 80% of the time" solutions

**Red flags:**
- "I think this would be useful"
- "It's like X but better" (without explaining why X is broken)
- Complex state management or learning curve
- Vague scope ("improve developer experience")

## 7. Documentation Over Configuration

If a tool needs explanation, document it. Don't add config complexity.

**Good:**
```
-f, --format <json|text>  Output format (default: json)
```

**Bad:**
```
--config my-config.json   # 50 keys, most optional
```

## 8. Fail Loudly, Recover Gracefully

- Errors should be explicit and helpful
- Stack traces should include context (file, line, operation)
- Degradation should be obvious, not silent

## 9. Test Everything, Ship Nothing Untested

- Unit tests for core logic
- Integration tests for CLI behavior
- Edge case coverage (empty input, malformed input, concurrent access)
- Tests run in CI on every commit

## 10. Version Independently

Each tool has its own version and changelog. The meta repo has no version.

Use semantic versioning:
- `0.1.0` — First release, unstable API
- `1.0.0` — Stable API, production-ready
- `2.0.0` — Breaking change

---

## What Doesn't Belong in Tuulbelt

- **Frameworks** (anything that tries to be your whole app)
- **Heavy abstractions** (frameworks disguised as utilities)
- **Language-specific hacks** (unless the problem is language-specific)
- **Vaporware** (things that work 80% of the time)
- **Opinionated workflows** (unless they're optional)
- **Things that require significant setup** (complexity = fragility)

---

## Examples of Tools That Fit

✅ "Run tests N times, report flaky ones" (single problem, clear output)
✅ "Convert YAML to JSON" (single problem, standard I/O)
✅ "Manage concurrent test port allocation" (single problem, solves real issue)
✅ "Generate FFI boilerplate from C headers" (single problem, code generation)

## Examples of Tools That Don't Fit

❌ "Complete testing framework" (too broad)
❌ "Multi-format data transformer with plugin system" (framework)
❌ "Universal configuration management" (scope creep)
❌ "AI-powered development assistant" (vaporware)
❌ "Framework for building frameworks" (meta-framework)

---

## Before Building a New Tool, Answer:

1. What single problem does it solve?
2. How would I test it end-to-end?
3. Can it run without installing dependencies?
4. Can it be used from any programming language?
5. Have I hit this problem in real code?
6. Is there a proven implementation path, or am I inventing something new?

If you can't answer clearly, wait. Revisit the idea later.
