# Tuulbelt Meta Repo: Complete Setup & Scaffolding Guide

This guide is designed for Claude Code at `https://claude.ai/code` to initialize the entire tuulbelt meta repository structure.

---

## Part 1: GitHub Organization Setup (Manual Prerequisites)

Before using Claude Code, complete these steps manually:

### 1.1 Create GitHub Organization
- Go to https://github.com/organizations/new
- Organization name: `tuulbelt` (or your preference)
- Owner email: your account email
- Visibility: Public
- Accept defaults for other settings

### 1.2 Create Meta Repository
- Navigate to your new org: `https://github.com/tuulbelt`
- Click "New" to create repository
- Repository name: `tuulbelt` (meta repo)
- Description: "Unified index and standards for tuulbelt utilities"
- Visibility: Public
- Initialize with README.md (unchecked - we'll create it)
- Click "Create repository"

### 1.3 Get Repository Details
- Copy the HTTPS clone URL: `https://github.com/tuulbelt/tuulbelt.git`
- You'll paste this into Claude Code

---

## Part 2: Claude Code Integration

### 2.1 Connect Claude Code to Meta Repo

In Claude Code at `https://claude.ai/code`:
1. Click "Open folder"
2. Select "Clone from GitHub"
3. Paste: `https://github.com/tuulbelt/tuulbelt.git`
4. Claude Code will prompt for authentication
5. Authorize GitHub access
6. Select location to clone (~/code/tuulbelt recommended)

---

## Part 3: Meta Repo Structure (To Be Created by Claude Code)

Claude Code should create this exact structure:

```
tuulbelt/
├── README.md
├── PRINCIPLES.md
├── CONTRIBUTING.md
├── ARCHITECTURE.md
├── docs/
│   ├── claude-code-workflow.md
│   ├── testing-standards.md
│   ├── tool-template.md
│   └── security-guidelines.md
├── templates/
│   ├── tool-repo-template/
│   │   ├── README.md
│   │   ├── SPEC.md (optional)
│   │   ├── src/
│   │   │   └── index.ts
│   │   ├── test/
│   │   │   └── index.test.ts
│   │   ├── examples/
│   │   │   └── basic.ts
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── .gitignore
│   │   ├── LICENSE
│   │   └── .github/
│   │       └── workflows/
│   │           └── test.yml
│   └── rust-tool-template/
│       ├── Cargo.toml
│       ├── Cargo.lock
│       ├── src/
│       │   └── lib.rs
│       ├── tests/
│       │   └── integration.rs
│       ├── examples/
│       └── README.md
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   ├── feature_request.md
│   │   └── tool_proposal.md
│   └── workflows/
│       └── meta-validation.yml
├── .gitignore
└── LICENSE
```

---

## Part 4: File Contents (Claude Code Instructions)

### 4.1 README.md

```markdown
# Tuulbelt

A curated collection of focused, zero-dependency tools and utilities for modern software development.

## Philosophy

Each tool in Tuulbelt:
- **Solves one problem** — Narrow, well-defined scope
- **Zero external dependencies** — Uses standard library only
- **Portable interface** — CLI, files, sockets; not proprietary APIs
- **Composable** — Works via pipes, environment variables, file I/O
- **Independently cloneable** — Each tool is a standalone repository
- **Proven implementation** — No moonshots, no "works 80%" solutions

## Current Tools

### CLI/DevTools
- **[Structured Error Handler](https://github.com/tuulbelt/structured-error-handler)** — Error format + serialization with context preservation
- **[CLI Progress Reporting](https://github.com/tuulbelt/cli-progress-reporting)** — Concurrent-safe progress updates (TBD)
- **[Configuration File Merger](https://github.com/tuulbelt/config-file-merger)** — ENV + config + CLI arg merging (TBD)
- **[Cross-Platform Path Normalizer](https://github.com/tuulbelt/path-normalizer)** — Windows/Unix path consistency (TBD)

### Testing & Observability
- **[Test Flakiness Detector](https://github.com/tuulbelt/test-flakiness-detector)** — Identify unreliable tests (TBD)
- **[Output Diffing Utility](https://github.com/tuulbelt/output-diffing)** — Semantic diffs for JSON/binary in assertions (TBD)
- **[Snapshot Comparison](https://github.com/tuulbelt/snapshot-comparison)** — Binary/structured data snapshots (TBD)
- **[Test Port Conflict Resolver](https://github.com/tuulbelt/test-port-resolver)** — Concurrent test port allocation (TBD)

### Frontend
- **[Component Prop Validator](https://github.com/tuulbelt/component-prop-validator)** — TypeScript runtime validation (TBD)
- **[Exhaustiveness Checker](https://github.com/tuulbelt/exhaustiveness-checker)** — Union case coverage for TS/JS (TBD)

### Data & Protocol
- **[Content-Addressable Blob Store](https://github.com/tuulbelt/content-addressable-blob-store)** — SHA-256 hash-based storage (TBD)
- **[Schema Converter (YAML ↔ JSON)](https://github.com/tuulbelt/schema-converter-yaml-json)** — Format conversion, no deps (TBD)
- **[Minimalist Pub-Sub Protocol](https://github.com/tuulbelt/pub-sub-protocol)** — Wire format for service messaging (TBD)
- **[Self-Describing Binary Wire Protocol](https://github.com/tuulbelt/wire-protocol)** — TLV format for RPC (TBD)

### APIs & Integration
- **[Request/Response Envelope Codec](https://github.com/tuulbelt/envelope-codec)** — Standard RPC response wrapping (TBD)
- **[API Versioning Helper](https://github.com/tuulbelt/api-versioning)** — Multi-version API logic (TBD)
- **[JSON Schema Validator](https://github.com/tuulbelt/json-schema-validator)** — Schema compliance checking (TBD)
- **[Streaming JSON Parser](https://github.com/tuulbelt/streaming-json-parser)** — Memory-efficient JSON parsing (TBD)

### Security & Networking
- **[Stateless Identity Generator](https://github.com/tuulbelt/identity-generator)** — Ed25519 token generation (TBD)
- **[Static Site Search Indexer](https://github.com/tuulbelt/static-search-indexer)** — HTML/Markdown → compressed index (TBD)
- **[Peer Discovery (UDP Multicast)](https://github.com/tuulbelt/peer-discovery)** — Local service discovery (TBD)
- **[One-File Reverse Proxy](https://github.com/tuulbelt/reverse-proxy)** — Minimal HTTP mapping (TBD)

### Utilities & Infrastructure
- **[Universal Log Normalizer](https://github.com/tuulbelt/log-normalizer)** — Structured log standardization (TBD)
- **[File-Based Semaphore](https://github.com/tuulbelt/file-semaphore)** — Cross-platform process locking (TBD)
- **[Manifest-First Sync Tool](https://github.com/tuulbelt/manifest-sync)** — Directory sync via manifest diffs (TBD)
- **[Universal Health-Check Probe](https://github.com/tuulbelt/health-check-probe)** — Multi-check abstraction (TBD)
- **[Secret Injector](https://github.com/tuulbelt/secret-injector)** — Encrypted secret injection (TBD)
- **[Deterministic Task Runner](https://github.com/tuulbelt/task-runner)** — DAG executor with file-hash skipping (TBD)
- **[Zero-Overhead Timing](https://github.com/tuulbelt/timing-injector)** — Compile-time optional instrumentation (TBD)
- **[Deterministic Build Artifact Generator](https://github.com/tuulbelt/deterministic-builds)** — Reproducible builds (TBD)

### Observability
- **[Structured Trace-to-SVG](https://github.com/tuulbelt/trace-to-svg)** — Event → Flame Graph visualization (TBD)
- **[Backpressure Proxy](https://github.com/tuulbelt/backpressure-proxy)** — Cascading failure prevention (TBD)

### Interoperability
- **[FFI Binding Generator](https://github.com/tuulbelt/ffi-binding-generator)** — Rust FFI from C headers (TBD)

## Quick Start

Clone any tool independently:

```bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
npm test  # or cargo test for Rust tools
```

## Development

- Read [PRINCIPLES.md](PRINCIPLES.md) for design philosophy
- Read [ARCHITECTURE.md](ARCHITECTURE.md) for repo structure
- Read [CONTRIBUTING.md](CONTRIBUTING.md) for contribution workflow
- See [docs/](docs/) for detailed guides

## Status

🟢 = Implemented  
🟡 = In progress  
🔴 = TBD

Current: Planning phase. Starting with Tier 1 tools.

## License

All tools are MIT licensed unless otherwise specified.

## Support

Found a bug? Have an idea? Open an issue in the specific tool's repository.
```

### 4.2 PRINCIPLES.md

```markdown
# Tuulbelt Design Principles

These principles guide what gets built, how it's built, and what doesn't belong.

## 1. Single Problem Per Tool

Each tool solves **one problem** and solves it well.

**Good:** "Detect unreliable tests by running them N times"  
**Bad:** "Complete test framework with mocking, fixtures, and reporting"

If you're adding features and the scope is growing, it's time to split into a new tool.

## 2. Zero External Dependencies (Standard Library Only)

No `npm install`, no `cargo add`, no runtime dependency management required.

**Exception:** Development dependencies (TypeScript compiler, test runners) are okay if they don't ship with the tool.

**Rationale:** Tools that don't require dependency resolution outlive language trends. They're more portable, more maintainable, and less fragile.

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
```

### 4.3 CONTRIBUTING.md

```markdown
# Contributing to Tuulbelt

Thanks for contributing! This guide explains how to create and maintain tools.

## Before You Start

Read [PRINCIPLES.md](PRINCIPLES.md). If your tool doesn't fit those principles, it doesn't belong in Tuulbelt.

## Creating a New Tool

### Step 1: Proposal

Open an issue on the meta repo (tuulbelt/tuulbelt):
- Title: "Proposal: [Tool Name]"
- Include:
  - Problem statement (1-2 sentences)
  - Use case(s)
  - Proposed interface (CLI flags, input/output format)
  - Implementation approach (proven?)
  - Why existing solutions don't work

Example:
```markdown
## Proposal: Test Flakiness Detector

**Problem:** Tests that pass sometimes, fail sometimes are hard to identify.

**Use case:** Catch intermittent failures in CI.

**Interface:**
$ flakiness-detector --runs 10 --test "npm test"
Output: JSON list of unreliable tests

**Why it's needed:** Jest/Vitest don't have built-in flakiness detection.
```

Get feedback before building.

### Step 2: Set Up Tool Repository

Use the template in `templates/tool-repo-template/` as your starting point.

Create a new repo under https://github.com/tuulbelt:
- Repo name: `<category>-<tool-name>` (e.g., `testing-flakiness-detector`)
- Visibility: Public
- Initialize with: README, LICENSE (MIT)

Clone locally:
```bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
```

### Step 3: Copy Template Files

For Node.js/TypeScript tools:

```bash
cp -r ../tuulbelt/templates/tool-repo-template/* .
```

For Rust tools:

```bash
cp -r ../tuulbelt/templates/rust-tool-template/* .
```

Customize:
- Update `package.json` (or `Cargo.toml`) name, description
- Update README.md with your tool's purpose
- Remove example code, keep structure

### Step 4: Implement in Claude Code

Open the tool repo in Claude Code:
1. In Claude Code, click "Open folder"
2. Clone: `https://github.com/tuulbelt/<tool-name>.git`
3. Start implementing following the template structure

Implementation checklist:
- [ ] Core logic in `src/index.ts` (or `src/lib.rs`)
- [ ] At least 3-5 test cases in `test/index.test.ts`
- [ ] Example usage in `examples/`
- [ ] README with clear usage instructions
- [ ] Run `npm test` (or `cargo test`) successfully

### Step 5: Testing Standards

All tools must have:

**Unit tests** — Core logic validation
```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';

test('core functionality', () => {
  const result = myFunction('input');
  assert.strictEqual(result, 'expected');
});
```

**Integration tests** — CLI behavior validation
```typescript
import { execSync } from 'child_process';

test('CLI works end-to-end', () => {
  const output = execSync('node src/index.js --flag value').toString();
  assert(output.includes('expected-output'));
});
```

**Edge cases** — Malformed input, empty input, boundary conditions
```typescript
test('handles empty input', () => {
  const result = myFunction('');
  assert(result !== null); // Doesn't crash
});
```

Run tests in CI — add `.github/workflows/test.yml`:

```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '20'
      - run: npm test
```

### Step 6: Documentation

**README.md** — One page, includes:
- 1-sentence description
- Why it exists (what problem it solves)
- Installation (just: clone the repo)
- Usage (with examples)
- Exit codes / error handling (if applicable)

**SPEC.md** (optional, for protocols/formats) — Formal wire format, algorithm, or data structure definition.

**ARCHITECTURE.md** (optional, for complex tools) — Design decisions, key functions, interaction flow.

### Step 7: Code Quality

- TypeScript: `strict: true` in tsconfig.json
- No `any` types
- No external dependencies (runtime)
- Clear variable names
- Comments for non-obvious logic

Example:

```typescript
// Detect if a test is flaky by running it multiple times
export function detectFlakiness(
  testFn: () => Promise<void>,
  runs: number = 10
): Promise<FlakeReport> {
  // ...
}
```

### Step 8: Versioning

Start at `0.1.0`. Increment:
- `0.1.x` for bug fixes (internal only)
- `0.2.0` for minor features
- `1.0.0` when API is stable and production-ready
- `2.0.0` for breaking changes

Tag releases on GitHub:
```bash
git tag v0.1.0
git push origin v0.1.0
```

### Step 9: Update Meta Repo

Once your tool is ready for initial release:

1. Update meta `README.md` — change `(TBD)` to `(✓ v0.1.0)` and link to repo
2. Add a one-line entry to the appropriate category

Example:
```markdown
- **[Test Flakiness Detector](https://github.com/tuulbelt/test-flakiness-detector)** — Identify unreliable tests ✓ v0.1.0
```

### Step 10: Maintenance

- Respond to issues within a week
- Keep README updated with new features
- Run tests before every merge
- Consider backwards compatibility; avoid breaking changes in patch versions

---

## Pull Request Process

1. Create feature branch: `git checkout -b feature/description`
2. Commit with clear messages: `git commit -m "Add feature: description"`
3. Push: `git push origin feature/description`
4. Open PR on GitHub with:
   - What changed
   - Why it changed
   - Tests added
   - Any breaking changes

---

## Code Review Checklist

Reviewers check:
- [ ] Solves the stated problem
- [ ] No new runtime dependencies
- [ ] Tests cover core logic + edge cases
- [ ] README is clear and complete
- [ ] No console logs or debug code
- [ ] Follows the style of the existing codebase

---

## Issues & Feedback

- Bug reports: Include steps to reproduce
- Feature requests: Explain the use case
- Questions: Check README and existing issues first

---

## Questions?

Check existing issues first. If unsure, ask in the meta repo discussion.
```

### 4.4 ARCHITECTURE.md

```markdown
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
```

### 4.5 docs/claude-code-workflow.md

```markdown
# Claude Code Workflow for Tuulbelt

This guide explains how to use Claude Code (https://claude.ai/code) effectively for Tuulbelt development.

## Setup

### Initial Setup (First Time)

1. Go to https://claude.ai/code
2. Click "Clone Repository"
3. Paste: `https://github.com/tuulbelt/tuulbelt.git`
4. Authenticate with GitHub when prompted
5. Select folder location: `~/code/tuulbelt` (or your preference)
6. Claude Code clones the meta repo

### For Each New Tool

1. In Claude Code: "New Project"
2. Clone: `https://github.com/tuulbelt/<tool-name>.git`
3. Or: File → Open → select tool directory

## Workflow

### Phase 1: Scaffolding (Start Here)

When creating a new tool, Claude Code should:

```bash
# Copy template
cp -r ../tuulbelt/templates/tool-repo-template/* .

# Structure becomes:
src/index.ts          # Main implementation (empty)
test/index.test.ts    # Test file (empty)
examples/basic.ts     # Example (empty)
package.json          # Pre-configured
tsconfig.json         # Pre-configured
README.md             # Template with placeholders
SPEC.md               # Optional, for protocols
```

Typical Claude Code prompt:
```
Copy the template structure from ../tuulbelt/templates/tool-repo-template/ into this repo. 
Update package.json name to @tuulbelt/<tool-name>. 
Leave src/index.ts and test/index.test.ts empty for now.
Create directory structure.
```

### Phase 2: Specification (Before Coding)

Before implementing, write SPEC.md (if applicable):

```markdown
# [Tool Name] Specification

## Problem
[1-2 sentence description]

## Interface
[How users interact: CLI flags, file format, input/output examples]

## Behavior
[Precise description of what happens in each case]

## Wire Format (if applicable)
[Exact bytes/JSON structure]

## Examples
[Real usage examples]

## Error Cases
[What happens on invalid input]
```

Typical Claude Code prompt:
```
Write SPEC.md for [tool name] based on these requirements:
- Problem: [description]
- Input: [format]
- Output: [format]
- Examples: [2-3 examples]

Use the specification template from docs/tool-template.md.
```

### Phase 3: Core Implementation

Implement in `src/index.ts`:

```typescript
// src/index.ts

// Exports the main API
export interface Config {
  // ...
}

export function process(input: string, config: Config): string {
  // Implementation
}

// CLI entry point (if this is a CLI tool)
if (import.meta.main) {
  // CLI logic
}
```

Typical Claude Code prompt:
```
Implement src/index.ts for [tool name]:
- Input: [description]
- Output: [description]
- Key logic: [description]
- Reference SPEC.md for exact behavior
- Use TypeScript strict mode
- Add JSDoc comments for all exports
```

### Phase 4: Testing

Write tests in `test/index.test.ts`:

```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { process } from '../src/index.ts';

test('basic functionality', () => {
  const result = process('input', { /* config */ });
  assert.strictEqual(result, 'expected');
});

test('edge case: empty input', () => {
  const result = process('', { /* config */ });
  assert(result !== null);
});

// More tests...
```

Run tests in Claude Code terminal:
```bash
npm test
```

Typical Claude Code prompt:
```
Write comprehensive tests for src/index.ts in test/index.test.ts:
- Test 1: Basic functionality with example input
- Test 2: Edge case — empty input
- Test 3: Edge case — malformed input
- Test 4: [specific behavior]
- Test 5: [error case]

Use Node's native test runner (node:test).
All tests should pass after implementation.
```

### Phase 5: Examples

Create runnable example in `examples/basic.ts`:

```typescript
import { process } from '../src/index.ts';

// Example usage
const result = process('example input', { /* config */ });
console.log(result);
```

Run in Claude Code:
```bash
npx ts-node examples/basic.ts
```

### Phase 6: Documentation

Update README.md with:
- What problem it solves
- Why it exists (existing solutions and their gaps)
- Installation (clone the repo)
- Usage (CLI flags or API)
- Examples (copy from examples/)
- Error handling (exit codes, error messages)

Typical Claude Code prompt:
```
Update README.md with complete usage documentation:
- Problem statement: [one sentence]
- Why it exists: [why existing tools aren't sufficient]
- Installation: [git clone instructions]
- Usage: [CLI example or API example]
- Examples: [2-3 realistic use cases]
- Error handling: [exit codes and messages]
```

## Claude Code Tips

### Running Commands

In Claude Code terminal:
```bash
npm test                    # Run tests
npm run build              # Compile TypeScript
npx ts-node src/index.ts   # Run directly
node src/index.js          # Run compiled JS
```

### File Navigation

- `Ctrl+P` (Cmd+P) — Quick file open
- `Ctrl+Shift+P` (Cmd+Shift+P) — Command palette
- `Ctrl+`` (Cmd+`) — Toggle terminal

### Debugging

In Claude Code:
1. Add `debugger;` statement where you want to pause
2. Run: `node --inspect src/index.ts`
3. Open `chrome://inspect` in browser
4. Click "Inspect" next to the process

## Best Practices

### Keep It Simple

```typescript
// Good: Clear, simple logic
export function parse(input: string): Data {
  const lines = input.split('\n');
  return lines.map(line => JSON.parse(line));
}

// Bad: Over-engineered
export class Parser {
  private cache = new Map();
  async parse(input: string, config?: ParserConfig): Promise<Data> {
    // 50 lines of logic
  }
}
```

### Test First, Code Second

Write test cases before implementation:
```typescript
test('should detect flaky test', () => {
  // Define expected behavior
});

// Then implement to pass test
```

### Use TypeScript Strict Mode

In tsconfig.json:
```json
{
  "compilerOptions": {
    "strict": true  // Catches most bugs at compile time
  }
}
```

### Avoid Console.log for Logic

```typescript
// For debugging only:
console.error('[DEBUG] Value:', value);

// For output, use structured returns:
export function process(input: string): { success: boolean; data?: string } {
  // ...
}
```

### Commit Often

In Claude Code terminal:
```bash
git add .
git commit -m "Add core parsing logic"
git push origin main
```

## Troubleshooting

### "Module not found" error

```bash
npm install  # Should be empty for tuulbelt tools
npm test     # Should pass without install
```

If npm install is needed, you've added an external dependency (don't).

### Tests failing

1. Read error message carefully
2. Add `console.log()` to debug
3. Check SPEC.md — are you matching spec?
4. Ask Claude Code: "Why is this test failing?"

### TypeScript compilation errors

Run: `npx tsc --noEmit` to see all errors
Ask Claude Code: "Fix TypeScript errors"

## Final Checklist Before Release

- [ ] `npm test` passes
- [ ] `npx tsc --noEmit` has no errors
- [ ] Examples run without errors
- [ ] README is complete
- [ ] SPEC.md (if applicable) matches implementation
- [ ] No external dependencies in package.json
- [ ] GitHub Actions test.yml configured
- [ ] Git history is clean (logical commits)
- [ ] Version bumped in package.json
- [ ] Git tag created: `git tag v0.1.0`
```

### 4.6 docs/testing-standards.md

```markdown
# Tuulbelt Testing Standards

All tools must have automated tests. This guide ensures consistency across the tuulbelt.

## Test Framework

Use the language's native test runner:
- **Node.js/TypeScript:** `node:test`
- **Rust:** `cargo test`

No external test libraries (except TypeScript compiler).

## Test Structure

```typescript
// test/index.test.ts
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { myFunction } from '../src/index.ts';

// Group related tests
test('myFunction', async (t) => {
  await t.test('basic case', () => {
    const result = myFunction('input');
    assert.strictEqual(result, 'expected');
  });

  await t.test('edge case: empty input', () => {
    const result = myFunction('');
    assert(result !== null);
  });

  await t.test('error case: invalid input', () => {
    assert.throws(() => myFunction('bad'), TypeError);
  });
});
```

## Test Categories

### 1. Unit Tests (Core Logic)

Test individual functions in isolation:

```typescript
test('parseConfig', () => {
  const config = parseConfig('{ "key": "value" }');
  assert.strictEqual(config.key, 'value');
});
```

Minimum: 1 unit test per exported function

### 2. Integration Tests (CLI/API Behavior)

Test end-to-end behavior:

```typescript
import { execSync } from 'child_process';

test('CLI produces correct output', () => {
  const output = execSync('node src/index.js --format json').toString();
  const parsed = JSON.parse(output);
  assert(parsed.success);
});
```

Minimum: 1 integration test per major feature

### 3. Edge Case Tests

Test boundary conditions, empty input, malformed data:

```typescript
test('edge cases', () => {
  assert.doesNotThrow(() => myFunction(''));       // Empty
  assert.doesNotThrow(() => myFunction('   '));    // Whitespace
  assert.throws(() => myFunction(null), TypeError); // Invalid type
});
```

Minimum: 1 edge case test per major feature

### 4. Error Handling Tests

Ensure errors are clear and recoverable:

```typescript
test('error handling', () => {
  const error = assert.throws(
    () => myFunction('invalid'),
    Error
  );
  assert(error.message.includes('expected'));
});
```

Minimum: 1 error test per error case

## Test Naming

Be specific and descriptive:

```typescript
// Good
test('parseConfig parses valid JSON', () => {});
test('parseConfig throws on invalid JSON', () => {});
test('parseConfig handles empty string', () => {});

// Bad
test('it works', () => {});
test('test 1', () => {});
test('parseConfig', () => {}); // Vague
```

## Assertion Patterns

Use `node:assert/strict`:

```typescript
import assert from 'node:assert/strict';

// Equality
assert.strictEqual(a, b);           // ===
assert.deepEqual(obj1, obj2);       // Deep comparison

// Existence
assert(value);                       // Truthy
assert.ok(value);                    // Same as above

// Type checking
assert.throws(() => fn(), TypeError);
assert.doesNotThrow(() => fn());

// Inclusion
assert(array.includes(item));
assert(string.includes('substring'));
```

## Test Coverage Target

- **Core logic:** 80%+ coverage
- **CLI:** 70%+ coverage
- **Examples:** Don't need to be tested

Check coverage:
```bash
npm test -- --coverage  # If your runner supports it
```

For Node test runner, count tests manually or use a coverage tool as dev dependency.

## Running Tests

```bash
npm test                 # Run all tests
npm test -- --grep "pattern"  # Run specific test
npm test -- --watch     # Rerun on file change
```

## CI/CD (GitHub Actions)

Every tool repo has `.github/workflows/test.yml`:

```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [18, 20]
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: ${{ matrix.node-version }}
      - run: npm test
```

This runs tests on every commit and pull request.

## Test Data

Keep test data small and self-contained:

```typescript
// Good: Data in test file
test('parseCSV', () => {
  const data = 'a,b,c\n1,2,3';
  const result = parseCSV(data);
  assert.deepEqual(result, [['a', 'b', 'c'], ['1', '2', '3']]);
});

// Okay: Data in separate file if large
import testData from './fixtures/large-data.json';

test('parseLargeFile', () => {
  const result = parse(testData);
  // ...
});
```

## Performance Tests (Optional)

If your tool is performance-critical:

```typescript
test('performance: handles 10MB file', () => {
  const start = performance.now();
  const result = process(largeData);
  const elapsed = performance.now() - start;
  assert(elapsed < 1000, `Took ${elapsed}ms, should be < 1s`);
});
```

But keep them optional; don't slow down test suite.

## Debugging Tests

Add logging temporarily:

```typescript
test('debug example', () => {
  const input = 'data';
  console.log('[DEBUG] Input:', input);
  const result = process(input);
  console.log('[DEBUG] Output:', result);
  assert(result);
});
```

Then remove before committing.

## Test Review Checklist

Before pushing:
- [ ] All tests pass locally
- [ ] No `console.log()` or `debugger;`
- [ ] Tests cover happy path + edge cases + errors
- [ ] Test names are descriptive
- [ ] Test data is minimal and clear
- [ ] No hardcoded paths or system-specific values
- [ ] CI passes on GitHub
```

### 4.7 Additional Documentation Files

Create these stub files for future expansion:

**docs/tool-template.md**
```markdown
# Tool Template

[To be filled in with step-by-step guide for creating a tool]
```

**docs/security-guidelines.md**
```markdown
# Security Guidelines

[To be filled in with guidelines for secure tool development]
```

**LICENSE**
```
MIT License

Copyright (c) 2024 Tuulbelt Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy...
[Standard MIT license text]
```

**.gitignore**
```
node_modules/
dist/
build/
*.o
*.a
*.so
.DS_Store
.vscode
.idea
*.swp
*.swo
*~
.env
.env.local
```

---

## Part 5: Instructions for Claude Code

When you open this meta repo in Claude Code, execute this task:

```
You are setting up the Tuulbelt meta repository in Claude Code.

Task: Create the complete directory structure and all files specified in "Part 3: Meta Repo Structure" with the exact file contents provided in "Part 4: File Contents".

Specifically:
1. Create all directories as shown in the structure
2. Create README.md with the provided content
3. Create PRINCIPLES.md with the provided content
4. Create CONTRIBUTING.md with the provided content
5. Create ARCHITECTURE.md with the provided content
6. Create docs/claude-code-workflow.md with the provided content
7. Create docs/testing-standards.md with the provided content
8. Create docs/tool-template.md (stub for now)
9. Create docs/security-guidelines.md (stub for now)
10. Create LICENSE (MIT license)
11. Create .gitignore with standard Node/TypeScript/Rust ignores
12. Create templates/tool-repo-template/ with complete Node/TS template structure:
    - package.json
    - tsconfig.json
    - src/index.ts (stub)
    - test/index.test.ts (stub)
    - examples/basic.ts (stub)
    - README.md (template)
    - SPEC.md (template)
    - LICENSE
    - .github/workflows/test.yml
13. Create templates/rust-tool-template/ with complete Rust template structure:
    - Cargo.toml
    - Cargo.lock
    - src/lib.rs
    - tests/integration.rs
    - examples/basic.rs
    - README.md
14. Create .github/ISSUE_TEMPLATE/:
    - bug_report.md
    - feature_request.md
    - tool_proposal.md
15. Create .github/workflows/meta-validation.yml (placeholder)

After creating all files:
- Run: git add .
- Run: git commit -m "Initialize tuulbelt meta repo with templates and documentation"
- Run: git push origin main

Verify all files are created correctly before pushing.
```

---

## Summary

After Claude Code executes the above instructions:

✅ Meta repo is fully scaffolded  
✅ All documentation is in place  
✅ Tool templates are ready to copy  
✅ CI/CD templates are ready  
✅ Repo is pushed to GitHub  

Next step: Create first tool repo, copy template, begin implementation.
