# Philosophy

Tuulbelt is a curated collection of focused, zero-dependency tools for modern software development.

## Core Philosophy

Each Tuulbelt tool:
- **Solves one problem** — Narrow, well-defined scope
- **Zero external dependencies** — Uses standard library only
- **Portable interface** — CLI, files, sockets; not proprietary APIs
- **Composable** — Works via pipes, environment variables, file I/O
- **Independently cloneable** — Each tool is a standalone repository
- **Proven implementation** — No moonshots, no "works 80%" solutions

## Why Tuulbelt?

### The Problem

Modern development tools often:
- Require complex dependency management
- Lock you into specific frameworks
- Have broad, unfocused scope ("do everything" tools)
- Break when dependencies update
- Can't be easily composed with other tools

### The Solution

Tuulbelt tools are:
- **Self-contained** — Zero runtime dependencies means no `npm install` headaches
- **Focused** — Each tool does one thing and does it well
- **Portable** — Standard interfaces (CLI, files, env vars) work everywhere
- **Composable** — Chain tools together with Unix pipes
- **Independent** — Clone only what you need
- **Reliable** — Production-tested, no experimental features

## Design Principles

See [Principles](/guide/principles) for detailed design principles that guide every Tuulbelt tool.

## Unix Philosophy Applied

Tuulbelt embraces the Unix philosophy:
1. **Make each program do one thing well**
2. **Expect output to become input to another program**
3. **Don't insist on interactive input when a program can be run automatically**

Applied to modern development:
- Single-purpose tools, not frameworks
- JSON output for programmatic parsing
- CLI-first design for automation

## What Doesn't Belong

❌ **Frameworks** (anything that tries to be your whole app)
❌ **Heavy abstractions** (frameworks disguised as utilities)
❌ **Language-specific hacks** (unless the problem is language-specific)
❌ **Vaporware** (things that work 80% of the time)
❌ **Opinionated workflows** (unless they're optional)

## Quality Standards

Every Tuulbelt tool must meet:
- ✅ 80%+ test coverage (aim for 90%+)
- ✅ Comprehensive edge case testing
- ✅ Dogfooding validation (tested with other Tuulbelt tools)
- ✅ Production-ready documentation
- ✅ Zero runtime dependencies verified

## Tool Selection

Before building a new tool, we ask:
1. What single problem does it solve?
2. How would we test it end-to-end?
3. Can it run without installing dependencies?
4. Can it be used from any programming language?
5. Have we hit this problem in real code?
6. Is there a proven implementation path?

If the answers aren't clear, we wait.

## Long-Term Vision

**Phase 1:** Build foundational tools (testing, CLI utilities, file operations)
**Phase 2:** Expand to data processing and protocol tools
**Phase 3:** Network and observability tools
**Phase 4:** Interoperability and advanced tooling

**Goal:** 33 production-ready tools that work together seamlessly.

---

**Current Progress:** 2 of 33 tools (6%) | Phase 1: 2/5 (40%)

See [ROADMAP](https://github.com/tuulbelt/tuulbelt/blob/main/ROADMAP.md) for detailed planning.
