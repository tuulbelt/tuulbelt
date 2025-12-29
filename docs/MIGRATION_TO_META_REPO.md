# Migration Plan: Monorepo to Meta Repository

**Created:** 2025-12-29
**Status:** Planning
**Priority:** Critical (Architectural Correction)

---

## Executive Summary

Tuulbelt was designed as a **meta repository** — a coordination layer for independent, standalone tools. However, during initial development, it was incorrectly implemented as a **monorepo** where all tool code lives in subdirectories of a single repository.

This document outlines the migration plan to correct this architectural mistake and achieve the intended meta repository structure.

---

## Table of Contents

1. [Context and Rationale](#context-and-rationale)
2. [Current State Analysis](#current-state-analysis)
3. [Target State Architecture](#target-state-architecture)
4. [Pre-Migration Setup](#pre-migration-setup)
5. [Migration Tasks by Tool](#migration-tasks-by-tool)
6. [Dependency Resolution Strategy](#dependency-resolution-strategy)
7. [CI/CD Workflow Migration](#cicd-workflow-migration)
8. [Claude Code Configuration](#claude-code-configuration)
9. [Documentation Updates](#documentation-updates)
10. [Testing Strategy](#testing-strategy)
11. [Rollback Plan](#rollback-plan)
12. [Post-Migration Verification](#post-migration-verification)
13. [Timeline and Phases](#timeline-and-phases)

---

## Context and Rationale

### The Original Vision (PRINCIPLES.md)

From PRINCIPLES.md:
> "Each tool is its own GitHub repository. Users should be able to:
> ```bash
> git clone https://github.com/tuulbelt/<tool-name>.git
> cd <tool-name>
> npm test && npm run build
> ```
> Without needing the meta repo or any other tool."

### What Went Wrong

During initial development, tools were created as subdirectories within `tuulbelt/tuulbelt`:

```
tuulbelt/tuulbelt/
├── test-flakiness-detector/    # Should be tuulbelt/test-flakiness-detector
├── cli-progress-reporting/     # Should be tuulbelt/cli-progress-reporting
├── file-based-semaphore-ts/    # Should be tuulbelt/file-based-semaphore-ts
└── ... (10 tools total)
```

This created several problems:

1. **Violates Independence Principle**: Users must clone the entire repository (~50MB+) to use any single tool
2. **Path Dependencies Don't Work Standalone**: `file:../sibling-tool` fails when tool is cloned independently
3. **Coupled Release Cycles**: Changes to one tool affect the entire repo
4. **Confusing Identity**: Is it `tuulbelt/tuulbelt/test-port-resolver` or `tuulbelt/test-port-resolver`?

### The Correct Architecture: Meta Repository

A **meta repository** is a coordination layer that:
- Contains documentation, templates, and shared workflows
- References independent tool repositories (via git submodules)
- Does NOT contain tool source code directly
- Provides orchestration without coupling

### PRINCIPLES.md Exception 2 — Tool Composition

> "Tuulbelt tools MAY use other Tuulbelt tools via library integration. Since all Tuulbelt tools have zero external dependencies, composing them preserves the zero-dep guarantee."

This exception requires git URL dependencies to work correctly:

```json
// TypeScript - automatically fetches from GitHub
"dependencies": {
  "@tuulbelt/file-based-semaphore-ts": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
}
```

```toml
# Rust - automatically fetches from GitHub
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility" }
```

With path dependencies (`file:../sibling`), Exception 2 only works in the monorepo context. With git URL dependencies, it works everywhere.

---

## Current State Analysis

### Repository Structure (Monorepo - INCORRECT)

```
tuulbelt/tuulbelt/
├── .claude/
│   ├── commands/           # Slash commands
│   ├── agents/             # Agent definitions
│   ├── skills/             # Skill definitions
│   ├── hooks/              # Hook configurations
│   ├── HANDOFF.md          # Session handoff
│   └── NEXT_TASKS.md       # Task backlog
├── .github/
│   └── workflows/
│       ├── test-all-tools.yml
│       ├── deploy-docs.yml
│       ├── create-demos.yml
│       └── update-dashboard.yml
├── docs/                   # VitePress documentation site
│   ├── .vitepress/
│   ├── tools/
│   │   ├── test-flakiness-detector/
│   │   ├── cli-progress-reporting/
│   │   └── ... (per-tool docs)
│   └── index.md
├── templates/
│   ├── tool-repo-template/     # TypeScript template
│   └── rust-tool-template/     # Rust template
├── test-flakiness-detector/    # TOOL CODE (should be separate repo)
├── cli-progress-reporting/     # TOOL CODE (should be separate repo)
├── cross-platform-path-normalizer/
├── config-file-merger/
├── structured-error-handler/
├── file-based-semaphore/       # Rust
├── file-based-semaphore-ts/    # TypeScript
├── output-diffing-utility/     # Rust
├── snapshot-comparison/        # Rust
├── test-port-resolver/
├── CLAUDE.md
├── PRINCIPLES.md
├── README.md
└── package.json
```

### Tool Inventory (10 Tools)

| Tool | Language | Has Dependencies | Depends On |
|------|----------|------------------|------------|
| test-flakiness-detector | TypeScript | No | cli-progress-reporting (optional) |
| cli-progress-reporting | TypeScript | No | - |
| cross-platform-path-normalizer | TypeScript | No | - |
| config-file-merger | TypeScript | No | - |
| structured-error-handler | TypeScript | No | - |
| file-based-semaphore | Rust | No | - |
| file-based-semaphore-ts | TypeScript | No | - |
| output-diffing-utility | Rust | No | - |
| snapshot-comparison | Rust | Yes | output-diffing-utility |
| test-port-resolver | TypeScript | Yes | file-based-semaphore-ts |

### Current Dependency Implementation (BROKEN for standalone use)

**test-port-resolver/package.json:**
```json
"dependencies": {
  "@tuulbelt/file-based-semaphore-ts": "file:../file-based-semaphore-ts"
}
```

**snapshot-comparison/Cargo.toml:**
```toml
[dependencies]
output_diffing_utility = { path = "../output-diffing-utility" }
```

These work ONLY when both tools exist as siblings. They fail for standalone clones.

---

## Target State Architecture

### Repository Structure (Meta Repo - CORRECT)

**Meta Repository: `tuulbelt/tuulbelt`**
```
tuulbelt/tuulbelt/
├── .claude/                    # Claude Code configuration (STAYS)
│   ├── commands/
│   ├── agents/
│   ├── skills/
│   ├── hooks/
│   ├── HANDOFF.md
│   └── NEXT_TASKS.md
├── .github/
│   └── workflows/
│       ├── deploy-docs.yml     # Documentation deployment
│       ├── sync-submodules.yml # Keep submodules updated
│       └── cross-repo-tests.yml # Test tool compositions
├── docs/                       # VitePress documentation (STAYS)
│   ├── .vitepress/
│   ├── tools/                  # Tool docs (copied from submodules or linked)
│   └── index.md
├── templates/                  # Tool templates (STAYS)
│   ├── tool-repo-template/
│   └── rust-tool-template/
├── tools/                      # Git submodules (NEW)
│   ├── test-flakiness-detector/    -> tuulbelt/test-flakiness-detector
│   ├── cli-progress-reporting/     -> tuulbelt/cli-progress-reporting
│   ├── file-based-semaphore-ts/    -> tuulbelt/file-based-semaphore-ts
│   └── ... (all 10 tools)
├── scripts/
│   ├── init-submodules.sh      # Setup script
│   └── update-all-tools.sh     # Update all submodules
├── CLAUDE.md
├── PRINCIPLES.md
├── README.md
└── package.json                # Workspace-level (docs only)
```

**Individual Tool Repositories:**
```
tuulbelt/test-flakiness-detector/
├── .github/
│   └── workflows/
│       └── test.yml            # Tool-specific CI
├── src/
├── test/
├── examples/
├── docs/                       # Tool's own docs (optional)
├── scripts/
│   ├── dogfood-flaky.sh
│   └── record-demo.sh
├── package.json
├── tsconfig.json
├── README.md
├── SPEC.md (if applicable)
└── LICENSE

tuulbelt/file-based-semaphore-ts/
├── ... (similar structure)

tuulbelt/test-port-resolver/
├── package.json                # With git URL dependency
│   "dependencies": {
│     "@tuulbelt/file-based-semaphore-ts": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
│   }
├── ... (similar structure)
```

### Dependency Resolution (CORRECT)

**TypeScript (npm) — Git URL:**
```json
{
  "dependencies": {
    "@tuulbelt/file-based-semaphore-ts": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
  }
}
```

When you run `npm install`:
1. npm detects git URL dependency
2. Clones `tuulbelt/file-based-semaphore-ts` automatically
3. Installs it in `node_modules/@tuulbelt/file-based-semaphore-ts`
4. Works regardless of where you cloned the tool from

**Rust (Cargo) — Git URL:**
```toml
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility" }
```

When you run `cargo build`:
1. Cargo detects git dependency
2. Clones `tuulbelt/output-diffing-utility` to `~/.cargo/git/`
3. Compiles and links it
4. Works regardless of where you cloned the tool from

---

## Pre-Migration Setup

### Phase 0: Preparation (REQUIRED before any migration)

#### Task 0.1: Create GitHub Organization Structure

- [ ] Verify `tuulbelt` GitHub organization exists
- [ ] Ensure admin access to create new repositories
- [ ] Configure default repository settings:
  - Default branch: `main`
  - License: MIT
  - Visibility: Public

#### Task 0.2: Create Empty Repositories for All Tools

Create these repositories on GitHub (empty, no README):

**TypeScript Tools:**
- [ ] `tuulbelt/test-flakiness-detector`
- [ ] `tuulbelt/cli-progress-reporting`
- [ ] `tuulbelt/cross-platform-path-normalizer`
- [ ] `tuulbelt/config-file-merger`
- [ ] `tuulbelt/structured-error-handler`
- [ ] `tuulbelt/file-based-semaphore-ts`
- [ ] `tuulbelt/test-port-resolver`

**Rust Tools:**
- [ ] `tuulbelt/file-based-semaphore`
- [ ] `tuulbelt/output-diffing-utility`
- [ ] `tuulbelt/snapshot-comparison`

#### Task 0.3: Backup Current State

```bash
# Create backup branch
git checkout main
git checkout -b backup/pre-meta-repo-migration
git push origin backup/pre-meta-repo-migration

# Create tagged release
git tag v0.2.0-monorepo-final
git push origin v0.2.0-monorepo-final
```

#### Task 0.4: Document Current Test Counts

Record baseline for verification:

| Tool | Test Count |
|------|------------|
| test-flakiness-detector | 132 |
| cli-progress-reporting | 121 |
| cross-platform-path-normalizer | 141 |
| config-file-merger | 144 |
| structured-error-handler | 88 |
| file-based-semaphore (Rust) | 95 |
| file-based-semaphore-ts | 160 |
| output-diffing-utility | 108 |
| snapshot-comparison | 96 |
| test-port-resolver | 56 |
| **Total** | **1,141** |

---

## Migration Tasks by Tool

### Migration Order (Dependency-Aware)

Tools must be migrated in dependency order:

**Wave 1: Independent Tools (No Dependencies)**
1. cli-progress-reporting
2. cross-platform-path-normalizer
3. config-file-merger
4. structured-error-handler
5. file-based-semaphore (Rust)
6. file-based-semaphore-ts
7. output-diffing-utility

**Wave 2: Tools with Optional Dependencies**
8. test-flakiness-detector (optional: cli-progress-reporting)

**Wave 3: Tools with Required Dependencies**
9. snapshot-comparison (requires: output-diffing-utility)
10. test-port-resolver (requires: file-based-semaphore-ts)

### Per-Tool Migration Template

For each tool, execute these micro-tasks:

#### Task N.1: Extract Tool with Git History

```bash
# From monorepo root
cd /path/to/tuulbelt

# Extract subdirectory with full history
git subtree split --prefix=<tool-name> -b extract/<tool-name>

# Clone the new empty repo
git clone https://github.com/tuulbelt/<tool-name>.git /tmp/<tool-name>
cd /tmp/<tool-name>

# Pull the extracted branch
git pull /path/to/tuulbelt extract/<tool-name>

# Push to GitHub
git push origin main
```

#### Task N.2: Update package.json / Cargo.toml

**TypeScript:**
```json
{
  "name": "@tuulbelt/<tool-name>",
  "version": "0.1.0",
  "repository": {
    "type": "git",
    "url": "https://github.com/tuulbelt/<tool-name>.git"
  },
  "homepage": "https://tuulbelt.github.io/tuulbelt/tools/<tool-name>/",
  "bugs": {
    "url": "https://github.com/tuulbelt/<tool-name>/issues"
  }
}
```

**Rust:**
```toml
[package]
name = "<tool-name>"
version = "0.1.0"
repository = "https://github.com/tuulbelt/<tool-name>"
homepage = "https://tuulbelt.github.io/tuulbelt/tools/<tool-name>/"
```

#### Task N.3: Add Tool-Specific CI Workflow

Create `.github/workflows/test.yml`:

**TypeScript:**
```yaml
name: Test

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [18, 20, 22]

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js ${{ matrix.node-version }}
        uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Type check
        run: npx tsc --noEmit

      - name: Run tests
        run: npm test

      - name: Build
        run: npm run build

      - name: Verify zero external dependencies
        run: |
          if grep -q '"dependencies"' package.json; then
            deps=$(node -e "console.log(Object.keys(require('./package.json').dependencies || {}).filter(d => !d.startsWith('@tuulbelt/')).length)")
            if [ "$deps" != "0" ]; then
              echo "ERROR: Found external dependencies!"
              exit 1
            fi
          fi
```

**Rust:**
```yaml
name: Test

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-action@stable

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Check
        run: cargo check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Format check
        run: cargo fmt --check

      - name: Test
        run: cargo test

      - name: Build release
        run: cargo build --release
```

#### Task N.4: Update README.md

Update installation instructions:

```markdown
## Installation

Clone this repository:

\`\`\`bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
npm install
\`\`\`

Or install directly from git (in another project):

\`\`\`bash
npm install git+https://github.com/tuulbelt/<tool-name>.git
\`\`\`
```

Remove monorepo-specific paths:
- Change `../PRINCIPLES.md` → `https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md`
- Change `../CONTRIBUTING.md` → `https://github.com/tuulbelt/tuulbelt/blob/main/CONTRIBUTING.md`
- Change `../file-based-semaphore-ts/` → `https://github.com/tuulbelt/file-based-semaphore-ts`

#### Task N.5: Update Dependencies (Wave 3 Tools Only)

For tools with dependencies, update to git URLs:

**test-port-resolver/package.json:**
```json
{
  "dependencies": {
    "@tuulbelt/file-based-semaphore-ts": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
  }
}
```

**snapshot-comparison/Cargo.toml:**
```toml
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility" }
```

#### Task N.6: Verify Tool Works Standalone

```bash
# Clone fresh (simulating new user)
cd /tmp
rm -rf <tool-name>
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>

# Install and test
npm install  # or cargo build
npm test     # or cargo test

# Verify test count matches baseline
```

#### Task N.7: Add CLAUDE.md for Tool

Create minimal CLAUDE.md for tool-specific Claude Code context:

```markdown
# <Tool Name>

Part of the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) collection.

## Quick Reference

- **Language:** TypeScript / Rust
- **Short name:** `<short-name>`
- **Tests:** `npm test` / `cargo test`
- **Build:** `npm run build` / `cargo build`

## Development

See main Tuulbelt repo for:
- [PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md)
- [CONTRIBUTING.md](https://github.com/tuulbelt/tuulbelt/blob/main/CONTRIBUTING.md)
- [Testing Standards](https://github.com/tuulbelt/tuulbelt/blob/main/docs/testing-standards.md)

## Zero Dependencies

This tool has zero external runtime dependencies.
Only Tuulbelt sibling tools may be used as dependencies.
```

---

## Dependency Resolution Strategy

### Git URL Dependency Format

**TypeScript (npm):**
```json
"@tuulbelt/<name>": "git+https://github.com/tuulbelt/<name>.git"
```

**With specific version/tag:**
```json
"@tuulbelt/<name>": "git+https://github.com/tuulbelt/<name>.git#v0.1.0"
```

**With specific commit:**
```json
"@tuulbelt/<name>": "git+https://github.com/tuulbelt/<name>.git#abc1234"
```

**Rust (Cargo):**
```toml
<name> = { git = "https://github.com/tuulbelt/<name>" }
```

**With specific version/tag:**
```toml
<name> = { git = "https://github.com/tuulbelt/<name>", tag = "v0.1.0" }
```

**With specific commit:**
```toml
<name> = { git = "https://github.com/tuulbelt/<name>", rev = "abc1234" }
```

### Future: Package Registry Publication

After meta repo migration is complete, tools can be published to registries:

**npm:**
```json
"@tuulbelt/<name>": "^0.1.0"
```

**crates.io:**
```toml
<name> = "0.1"
```

This is a future enhancement, not required for migration.

---

## CI/CD Workflow Migration

### Workflows to Remove from Meta Repo

These workflows test tool code and should move to individual repos:

- [ ] `test-all-tools.yml` → Split into per-repo `test.yml`

### Workflows to Keep in Meta Repo

These workflows handle meta-repo concerns:

- [ ] `deploy-docs.yml` — Deploys VitePress documentation
- [ ] `create-demos.yml` — May need refactoring to work with submodules

### New Workflows for Meta Repo

#### sync-submodules.yml

```yaml
name: Sync Submodules

on:
  schedule:
    - cron: '0 */6 * * *'  # Every 6 hours
  workflow_dispatch:

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive

      - name: Update submodules
        run: |
          git submodule update --remote --merge

      - name: Check for changes
        id: changes
        run: |
          if git diff --quiet; then
            echo "changed=false" >> $GITHUB_OUTPUT
          else
            echo "changed=true" >> $GITHUB_OUTPUT
          fi

      - name: Commit and push
        if: steps.changes.outputs.changed == 'true'
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add .
          git commit -m "chore: sync submodules"
          git push
```

#### cross-repo-tests.yml

```yaml
name: Cross-Repo Integration Tests

on:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM
  workflow_dispatch:

jobs:
  test-compositions:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Setup Rust
        uses: dtolnay/rust-action@stable

      # Test portres + semats composition
      - name: Test portres uses semats
        run: |
          cd tools/test-port-resolver
          npm install
          npm test

      # Test snapcmp + odiff composition
      - name: Test snapcmp uses odiff
        run: |
          cd tools/snapshot-comparison
          cargo test
```

### Demo Recording Workflow

The `create-demos.yml` workflow needs updates to work with submodules:

```yaml
# Updated to reference tools/ submodule directory
- name: Record demo
  run: |
    cd tools/${{ matrix.tool }}
    ./scripts/record-demo.sh
```

---

## Claude Code Configuration

### Meta Repo CLAUDE.md Updates

The main CLAUDE.md needs updates for submodule-based development:

#### Add Submodule Setup Instructions

```markdown
## Repository Setup

This is a **meta repository**. Tool source code lives in git submodules.

### First-Time Setup

\`\`\`bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt

# Initialize and fetch all tool submodules
git submodule update --init --recursive

# Or use the setup script
./scripts/init-submodules.sh
\`\`\`

### Working with Submodules

\`\`\`bash
# Update all submodules to latest
git submodule update --remote --merge

# Enter a specific tool
cd tools/test-flakiness-detector
git checkout main
git pull

# Commit submodule pointer update in meta repo
cd ../..
git add tools/test-flakiness-detector
git commit -m "chore: update test-flakiness-detector submodule"
\`\`\`
```

#### Update Project Structure Section

```markdown
## Project Structure

\`\`\`
/home/user/tuulbelt/
├── .claude/                      # Claude Code workflows & automation
├── docs/                         # VitePress documentation site
├── templates/                    # Tool scaffolding templates
├── tools/                        # Git submodules (tool source code)
│   ├── test-flakiness-detector/  -> github.com/tuulbelt/test-flakiness-detector
│   ├── cli-progress-reporting/   -> github.com/tuulbelt/cli-progress-reporting
│   └── ... (10 tools)
├── scripts/
│   ├── init-submodules.sh
│   └── update-all-tools.sh
├── CLAUDE.md
└── README.md
\`\`\`
```

### Slash Commands Updates

#### /scaffold-tool

Update to create new repositories:

```markdown
When scaffolding a new tool:
1. Create new GitHub repository: tuulbelt/<tool-name>
2. Clone and initialize from template
3. Add as submodule to meta repo: git submodule add https://github.com/tuulbelt/<tool-name>.git tools/<tool-name>
```

#### /test-all

Update to work with submodules:

```bash
# Test all tools via submodules
for tool in tools/*/; do
  echo "Testing $tool..."
  cd "$tool"
  if [ -f "package.json" ]; then
    npm test
  elif [ -f "Cargo.toml" ]; then
    cargo test
  fi
  cd ../..
done
```

### Hooks Updates

The pre-write hook may need updates if it references tool paths.

### Agents Updates

Update agent descriptions to reference `tools/` subdirectory:

```yaml
# scaffold-assistant agent
description: |
  Creates new tool in tools/ submodule directory.
  Also creates the GitHub repository.
```

---

## Documentation Updates

### VitePress Configuration

#### Option A: Copy Docs from Submodules

```typescript
// docs/.vitepress/config.ts
export default {
  // Build script copies docs from tools/*/docs/ to docs/tools/
}
```

Build script:
```bash
#!/bin/bash
# scripts/copy-tool-docs.sh

for tool in tools/*/; do
  tool_name=$(basename "$tool")
  if [ -d "$tool/docs" ]; then
    cp -r "$tool/docs" "docs/tools/$tool_name"
  fi
done
```

#### Option B: Symlinks (Simpler)

```bash
# Create symlinks
for tool in tools/*/; do
  tool_name=$(basename "$tool")
  ln -s "../../$tool/docs" "docs/tools/$tool_name"
done
```

Note: VitePress may have issues with symlinks. Test carefully.

### README.md Updates

Update main README to reflect meta repo structure:

```markdown
## Quick Start

Clone with all tools:

\`\`\`bash
git clone --recursive https://github.com/tuulbelt/tuulbelt.git
\`\`\`

Or clone a single tool:

\`\`\`bash
git clone https://github.com/tuulbelt/test-flakiness-detector.git
\`\`\`
```

### PRINCIPLES.md Updates

Clarify Exception 2:

```markdown
## 2. Zero External Dependencies

**Exception 2:** Tuulbelt tools MAY depend on other Tuulbelt tools via **git URL dependencies**:

\`\`\`json
// TypeScript
"@tuulbelt/semats": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
\`\`\`

\`\`\`toml
# Rust
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility" }
\`\`\`

These dependencies are automatically fetched during install/build. Users do not need to manually clone sibling tools.
```

---

## Testing Strategy

### Pre-Migration Testing

Before starting migration:

```bash
# Record all test counts
for tool in */; do
  if [ -f "$tool/package.json" ]; then
    echo "$tool: $(cd $tool && npm test 2>&1 | grep -E '# tests [0-9]+')"
  elif [ -f "$tool/Cargo.toml" ]; then
    echo "$tool: $(cd $tool && cargo test 2>&1 | grep -E 'test result:')"
  fi
done
```

### Per-Tool Migration Testing

After each tool is migrated:

1. **Fresh clone test:**
   ```bash
   cd /tmp
   git clone https://github.com/tuulbelt/<tool>.git
   cd <tool>
   npm install && npm test  # or cargo build && cargo test
   ```

2. **Verify test count matches baseline**

3. **For dependent tools, verify dependency auto-fetches:**
   ```bash
   # Should see git clone happening during install
   npm install --verbose 2>&1 | grep "git"
   ```

### Post-Migration Integration Testing

After all tools migrated:

1. **Clone meta repo with submodules:**
   ```bash
   git clone --recursive https://github.com/tuulbelt/tuulbelt.git
   cd tuulbelt
   ```

2. **Run all tests via submodules:**
   ```bash
   ./scripts/test-all-tools.sh
   ```

3. **Verify docs build:**
   ```bash
   npm run docs:build
   ```

4. **Verify total test count: 1,141**

---

## Rollback Plan

### If Migration Fails Mid-Way

1. **Restore from backup branch:**
   ```bash
   git checkout backup/pre-meta-repo-migration
   git branch -D main
   git checkout -b main
   git push -f origin main
   ```

2. **Delete created repositories:**
   - Go to GitHub organization settings
   - Delete any tool repos created during failed migration

3. **Document what went wrong** in KNOWN_ISSUES.md

### If Tool Migration Fails

Individual tool migrations are isolated. If one fails:

1. Delete the created tool repository
2. Remove from submodules if added
3. Keep tool in monorepo temporarily
4. Debug and retry

---

## Post-Migration Verification

### Verification Checklist

- [ ] All 10 tool repositories exist and are public
- [ ] Each tool can be cloned and tested independently
- [ ] Tools with dependencies auto-fetch via git URLs
- [ ] Meta repo submodules point to correct commits
- [ ] `git clone --recursive` works for meta repo
- [ ] All 1,141 tests pass across all tools
- [ ] VitePress documentation builds and deploys
- [ ] Demo recording workflow functions
- [ ] CI passes on all tool repos
- [ ] CI passes on meta repo

### Smoke Tests

```bash
# 1. Fresh user experience - single tool
cd /tmp && rm -rf test-port-resolver
git clone https://github.com/tuulbelt/test-port-resolver.git
cd test-port-resolver
npm install  # Should auto-fetch semats
npm test     # Should pass (56 tests)

# 2. Fresh user experience - meta repo
cd /tmp && rm -rf tuulbelt
git clone --recursive https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt
./scripts/test-all-tools.sh  # Should pass (1,141 tests)

# 3. Docs build
npm install
npm run docs:build  # Should succeed
```

---

## Timeline and Phases

### Phase 1: Preparation (1 session)
- [ ] Task 0.1: Verify GitHub organization
- [ ] Task 0.2: Create 10 empty repositories
- [ ] Task 0.3: Create backup branch and tag
- [ ] Task 0.4: Document test counts

### Phase 2: Wave 1 Migration (2-3 sessions)
Migrate 7 independent tools:
- [ ] cli-progress-reporting
- [ ] cross-platform-path-normalizer
- [ ] config-file-merger
- [ ] structured-error-handler
- [ ] file-based-semaphore (Rust)
- [ ] file-based-semaphore-ts
- [ ] output-diffing-utility

### Phase 3: Wave 2 Migration (1 session)
Migrate tools with optional dependencies:
- [ ] test-flakiness-detector

### Phase 4: Wave 3 Migration (1 session)
Migrate tools with required dependencies:
- [ ] snapshot-comparison (after output-diffing-utility is migrated)
- [ ] test-port-resolver (after file-based-semaphore-ts is migrated)

### Phase 5: Meta Repo Restructure (1 session)
- [ ] Remove tool directories from meta repo
- [ ] Add git submodules for all tools
- [ ] Update CLAUDE.md
- [ ] Update workflows
- [ ] Update documentation

### Phase 6: Verification & Cleanup (1 session)
- [ ] Run full verification checklist
- [ ] Clean up old branches
- [ ] Update all cross-references
- [ ] Final documentation review

---

## Appendix: Script Templates

### scripts/init-submodules.sh

```bash
#!/bin/bash
# Initialize all tool submodules

set -e

echo "Initializing Tuulbelt tool submodules..."

git submodule update --init --recursive

echo ""
echo "All submodules initialized. Tools available in tools/ directory:"
ls -1 tools/

echo ""
echo "To update all tools to latest: ./scripts/update-all-tools.sh"
```

### scripts/update-all-tools.sh

```bash
#!/bin/bash
# Update all tool submodules to latest main branch

set -e

echo "Updating all tool submodules..."

git submodule update --remote --merge

echo ""
echo "All tools updated. Don't forget to commit the submodule pointer changes:"
echo "  git add tools/"
echo "  git commit -m 'chore: update tool submodules'"
```

### scripts/test-all-tools.sh

```bash
#!/bin/bash
# Run tests for all tools

set -e

TOTAL_TESTS=0
FAILED_TOOLS=()

for tool_dir in tools/*/; do
  tool_name=$(basename "$tool_dir")
  echo ""
  echo "=========================================="
  echo "Testing: $tool_name"
  echo "=========================================="

  cd "$tool_dir"

  if [ -f "package.json" ]; then
    npm install
    if npm test; then
      echo "✅ $tool_name passed"
    else
      FAILED_TOOLS+=("$tool_name")
      echo "❌ $tool_name failed"
    fi
  elif [ -f "Cargo.toml" ]; then
    if cargo test; then
      echo "✅ $tool_name passed"
    else
      FAILED_TOOLS+=("$tool_name")
      echo "❌ $tool_name failed"
    fi
  fi

  cd ../..
done

echo ""
echo "=========================================="
echo "Summary"
echo "=========================================="

if [ ${#FAILED_TOOLS[@]} -eq 0 ]; then
  echo "✅ All tools passed!"
else
  echo "❌ Failed tools:"
  for tool in "${FAILED_TOOLS[@]}"; do
    echo "  - $tool"
  done
  exit 1
fi
```

---

## Document History

| Date | Author | Changes |
|------|--------|---------|
| 2025-12-29 | Claude | Initial migration plan created |

---

**End of Migration Plan**
