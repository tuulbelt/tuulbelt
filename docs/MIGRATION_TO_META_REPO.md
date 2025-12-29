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

**AUTOMATION AVAILABLE:** Use `/migrate-tool <tool-name>` command to automate the entire migration process.

The command handles all steps below automatically:
- Git history extraction
- GitHub repository creation
- Metadata updates (package.json/Cargo.toml, CI, README, CLAUDE.md)
- Commit and tag v0.1.0 with correct author
- Git submodule addition to meta repo
- Tracking document updates

See `.claude/commands/migrate-tool.md` for complete details.

**Manual steps below are for reference only. Use `/migrate-tool` instead.**

---

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

## Addendum: Detailed Component Updates

After cross-checking against the current codebase, the following detailed updates are required.

### A.1 Workflow Updates (Complete List)

**Current Workflows in `.github/workflows/`:**

| Workflow | Current Purpose | Migration Action |
|----------|-----------------|------------------|
| `test-all-tools.yml` | Auto-discovers and tests all tools | **SPLIT**: Move to individual tool repos; keep simplified cross-repo version in meta |
| `deploy-docs.yml` | Deploys VitePress to GitHub Pages | **UPDATE**: Change paths to reference `tools/` submodules |
| `create-demos.yml` | Records and embeds demo GIFs | **UPDATE**: Change hardcoded paths; may need per-tool approach |
| `update-demos.yml` | (Appears to be duplicate) | **REVIEW**: Consolidate with create-demos.yml |
| `meta-validation.yml` | Validates meta repo structure | **UPDATE**: Validate submodules instead of subdirectories |
| `update-dashboard.yml` | Updates status dashboard | **UPDATE**: Aggregate from individual tool repos |

**Per-Tool Workflow (Standard Template):**

Each individual tool repo should have `.github/workflows/test.yml`:

```yaml
name: Test

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [18, 20, 22]  # Or rust versions for Rust tools
    steps:
      - uses: actions/checkout@v4

      # For tools with Tuulbelt dependencies, they auto-fetch via git URL
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: 'npm'

      - run: npm ci
      - run: npx tsc --noEmit
      - run: npm test
      - run: npm run build
```

### A.2 Slash Commands Updates

**Commands Requiring Complete Rewrite:**

#### `/scaffold-tool` - MAJOR REWRITE

Current behavior: Creates tool as subdirectory in monorepo.
New behavior: Creates GitHub repo + adds as submodule.

> ⚠️ **CRITICAL: Claude Code Web Limitation**
>
> The `gh` CLI has known bugs in Claude Code Web that cause "Permission denied" errors.
> Repository creation via `gh repo create` only works reliably in **Claude Code CLI**.
>
> **Workaround:** The scaffold command should detect environment and provide fallback.

```markdown
## New Scaffolding Process

1. **Create GitHub Repository:**

   **Option A: Via gh CLI (CLI version only)**
   ```bash
   # Check if gh is available and authenticated
   if gh auth status &>/dev/null; then
     gh repo create tuulbelt/$TOOL_NAME --public --clone
   else
     echo "⚠️  gh CLI not authenticated. Create repository manually:"
     echo "   https://github.com/organizations/tuulbelt/repositories/new"
     echo "   Repository name: $TOOL_NAME"
     echo "   Visibility: Public"
     echo ""
     echo "Then clone it:"
     echo "   git clone https://github.com/tuulbelt/$TOOL_NAME.git"
     exit 1
   fi
   ```

   **Option B: Manual creation (required for Web version)**
   - Go to https://github.com/organizations/tuulbelt/repositories/new
   - Create public repository named `$TOOL_NAME`
   - Clone locally: `git clone https://github.com/tuulbelt/$TOOL_NAME.git`

2. **Initialize from Template:**
   ```bash
   cd $TOOL_NAME
   cp -r ../templates/tool-repo-template/* .  # or rust template
   # Customize files...
   git add . && git commit -m "feat: initialize from template"
   git push origin main
   ```

3. **Add as Submodule to Meta Repo:**
   ```bash
   cd /path/to/tuulbelt
   git submodule add https://github.com/tuulbelt/$TOOL_NAME.git tools/$TOOL_NAME
   git commit -m "chore: add $TOOL_NAME submodule"
   ```

4. **Update VitePress Config:**
   - Add to sidebar in `docs/.vitepress/config.ts`
   - Add to tools index
```

**Environment Detection for Scaffold Command:**

```bash
#!/bin/bash
# Detect if running in Claude Code Web vs CLI

detect_environment() {
  # Check for web-specific environment markers
  if [ -n "$CLAUDE_CODE_WEB" ] || [ ! -t 0 ]; then
    echo "web"
  else
    echo "cli"
  fi
}

ENV=$(detect_environment)

if [ "$ENV" = "web" ]; then
  echo "⚠️  Running in Claude Code Web - manual repo creation required"
  echo "   gh CLI may not work due to known permission issues"
  echo ""
  echo "Please create the repository manually at:"
  echo "   https://github.com/organizations/tuulbelt/repositories/new"
else
  # CLI version - can use gh
  if gh auth status &>/dev/null; then
    gh repo create tuulbelt/$TOOL_NAME --public --clone
  else
    echo "Run: gh auth login"
    exit 1
  fi
fi
```

#### `/test-all` - UPDATE

Current behavior: Tests only templates.
New behavior: Tests all submodules.

```markdown
## Updated Test Execution

\`\`\`bash
# Initialize submodules if needed
git submodule update --init --recursive

# Test all tools
for tool in tools/*/; do
  echo "Testing $(basename $tool)..."
  cd "$tool"
  if [ -f "package.json" ]; then
    npm ci && npm test
  elif [ -f "Cargo.toml" ]; then
    cargo test
  fi
  cd ../..
done
\`\`\`
```

#### `/quality-check` - UPDATE

Current behavior: References monorepo structure, runs dogfood scripts.
New behavior: Works in both meta repo and individual tool repo contexts.

**Changes needed:**
- Detect if running in meta repo (`tools/` exists) or individual tool repo
- In meta repo: iterate over submodules
- In tool repo: run checks directly
- Update dogfood script paths

#### `/handoff` and `/resume-work` - MINOR UPDATE

- Update references from tool subdirectories to `tools/` submodule paths
- Update HANDOFF.md location references

### A.3 Agent Updates

#### `scaffold-assistant` - MAJOR REWRITE

The entire workflow changes:

```markdown
## New Responsibilities

1. **GitHub Repository Creation**: Use `gh` CLI to create new repo
2. **Template Initialization**: Clone template into new repo
3. **Submodule Addition**: Add to meta repo as submodule
4. **VitePress Integration**: Update config.ts for new tool
5. **CI Setup**: Ensure tool has standalone CI workflow
```

Key changes to agent:
- Remove references to `cp -r templates/... "$TOOL_NAME"`
- Add `gh repo create` commands
- Add `git submodule add` commands
- Update directory references from `$TOOL_NAME/` to `tools/$TOOL_NAME/`

#### `test-runner` - MINOR UPDATE

- Update working directory references
- Add submodule initialization step

#### `security-reviewer` - MINOR UPDATE

- Update paths for submodule structure

#### `session-manager` - MINOR UPDATE

- Update HANDOFF.md references

### A.4 Skills Updates

**Current Skills in `.claude/skills/`:**

| Skill | Purpose | Migration Action |
|-------|---------|------------------|
| `typescript-patterns` | TS best practices | **COPY**: Include in each TS tool repo's CLAUDE.md |
| `rust-idioms` | Rust best practices | **COPY**: Include in each Rust tool repo's CLAUDE.md |
| `zero-deps-checker` | Validates zero deps | **UPDATE**: Check for git URL deps (allowed) vs external deps (forbidden) |

**Zero-Deps Checker Update:**

```markdown
## Updated Validation Logic

**Allowed dependencies:**
- Git URLs to other Tuulbelt tools: `git+https://github.com/tuulbelt/*`
- Path dependencies in development: `file:../sibling` (for local dev only)

**Forbidden dependencies:**
- npm packages: Any package without `@tuulbelt/` scope
- crates.io packages: Any external crate

**Validation script:**
\`\`\`bash
# TypeScript
deps=$(jq -r '.dependencies // {} | keys[]' package.json 2>/dev/null)
for dep in $deps; do
  if [[ ! "$dep" =~ ^@tuulbelt/ ]]; then
    echo "ERROR: External dependency found: $dep"
    exit 1
  fi
done

# Rust
if grep -A20 '^\[dependencies\]' Cargo.toml | grep -v '^#' | grep -v 'git = "https://github.com/tuulbelt' | grep -qE '^[a-z]'; then
  echo "ERROR: External dependency found"
  exit 1
fi
\`\`\`
```

### A.5 Template Updates

**Both templates need these updates:**

1. **Remove monorepo path comments:**
   ```yaml
   # OLD (monorepo)
   # paths: ['<tool-name>/**']

   # NEW (standalone) - remove these comments entirely
   ```

2. **Update package.json repository field:**
   ```json
   {
     "repository": {
       "type": "git",
       "url": "https://github.com/tuulbelt/TOOL_NAME.git"
     }
   }
   ```

3. **Update Cargo.toml repository field:**
   ```toml
   repository = "https://github.com/tuulbelt/TOOL_NAME"
   ```

4. **Add standalone CLAUDE.md template:**
   ```markdown
   # Tool Name

   Part of [Tuulbelt](https://github.com/tuulbelt/tuulbelt).

   ## Development

   - Tests: `npm test` / `cargo test`
   - Build: `npm run build` / `cargo build`

   ## Principles

   - Zero external dependencies
   - Single problem focus
   - See [PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md)
   ```

5. **Update dogfood scripts for standalone:**

   Since tools are now independent, dogfood scripts that require sibling tools should use git URLs:

   ```bash
   # OLD (monorepo - path access)
   DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"

   # NEW (standalone - clone if needed)
   DETECTOR_DIR="/tmp/tuulbelt-flaky"
   if [ ! -d "$DETECTOR_DIR" ]; then
     git clone --depth 1 https://github.com/tuulbelt/test-flakiness-detector.git "$DETECTOR_DIR"
     cd "$DETECTOR_DIR" && npm install && cd -
   fi
   ```

### A.6 VitePress Strategy (Decision Required)

**Recommended Approach: Hybrid with Build-Time Copy**

1. **Tool repos contain their own docs** in `docs/` directory
2. **Meta repo build script copies** docs from submodules before VitePress build
3. **VitePress config stays in meta repo** - single source of navigation

**Implementation:**

```bash
#!/bin/bash
# scripts/prepare-docs.sh - Run before npm run docs:build

# Ensure submodules are updated
git submodule update --init --recursive

# Copy tool docs to VitePress structure
for tool in tools/*/; do
  tool_name=$(basename "$tool")

  # Copy tool's VitePress docs if they exist
  if [ -d "$tool/docs" ]; then
    mkdir -p "docs/tools/$tool_name"
    cp -r "$tool/docs/"* "docs/tools/$tool_name/"
  fi

  # Copy demo.gif if it exists
  if [ -f "$tool/docs/demo.gif" ]; then
    mkdir -p "docs/public/$tool_name"
    cp "$tool/docs/demo.gif" "docs/public/$tool_name/"
  fi
done
```

**Update package.json:**
```json
{
  "scripts": {
    "docs:prepare": "./scripts/prepare-docs.sh",
    "docs:build": "npm run docs:prepare && vitepress build docs",
    "docs:dev": "npm run docs:prepare && vitepress dev docs"
  }
}
```

### A.7 Demo Workflow Strategy

**Option A: Per-Tool Demo Workflows (Recommended)**

Each tool repo has its own demo workflow:

```yaml
# In each tool repo: .github/workflows/demo.yml
name: Record Demo

on:
  push:
    branches: [main]
    paths:
      - 'src/**'
      - 'scripts/record-demo.sh'
  workflow_dispatch:

jobs:
  record:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # ... record demo ...
      - name: Commit demo
        run: |
          git add docs/demo.gif demo.cast
          git commit -m "docs: update demo recording" || true
          git push
```

**Option B: Centralized Demo Workflow**

Meta repo pulls latest from all submodules and records demos centrally. More complex but keeps all demos in sync.

**Recommendation:** Option A - simpler, each tool owns its demo.

### A.8 Root package.json

The root `package.json` is used for VitePress docs. After migration:

```json
{
  "name": "@tuulbelt/meta",
  "version": "1.0.0",
  "private": true,
  "description": "Tuulbelt meta repository - documentation and orchestration",
  "scripts": {
    "docs:prepare": "./scripts/prepare-docs.sh",
    "docs:dev": "npm run docs:prepare && vitepress dev docs",
    "docs:build": "npm run docs:prepare && vitepress build docs",
    "docs:preview": "vitepress preview docs",
    "submodules:init": "./scripts/init-submodules.sh",
    "submodules:update": "./scripts/update-all-tools.sh",
    "test:all": "./scripts/test-all-tools.sh"
  },
  "devDependencies": {
    "vitepress": "^1.6.4"
  }
}
```

### A.9 Individual Tool CLAUDE.md

Each tool repo should have a CLAUDE.md that provides context without requiring the meta repo:

```markdown
# [Tool Name] / `short-name`

Part of the [Tuulbelt](https://github.com/tuulbelt/tuulbelt) collection.

## Quick Reference

- **Language:** TypeScript / Rust
- **CLI Short Name:** `short-name`
- **Tests:** `npm test` / `cargo test`
- **Build:** `npm run build` / `cargo build`

## Development Commands

\`\`\`bash
npm install      # Install dependencies
npm test         # Run tests
npm run build    # Build for distribution
npx tsc --noEmit # Type check only
\`\`\`

## Code Conventions

- Zero external dependencies (Tuulbelt tools allowed via git URL)
- Result pattern for error handling
- 80%+ test coverage
- See main repo for full [PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md)

## Dependencies

This tool [uses/does not use] other Tuulbelt tools:
- `@tuulbelt/sibling-tool` - Purpose (if applicable)

Dependencies are fetched automatically via git URL during `npm install`.

## Testing

\`\`\`bash
npm test                    # Run all tests
npm test -- --watch         # Watch mode
./scripts/dogfood-flaky.sh  # Validate test reliability (optional)
\`\`\`

## Security

- No hardcoded secrets
- Input validation on all public APIs
- Run security scan: See main repo `/security-scan` command

## Related

- [Main Tuulbelt Repository](https://github.com/tuulbelt/tuulbelt)
- [Documentation](https://tuulbelt.github.io/tuulbelt/tools/[tool-name]/)
- [Contributing Guide](https://github.com/tuulbelt/tuulbelt/blob/main/CONTRIBUTING.md)
```

### A.10 Migration Task Updates

Add these tasks to Phase 5 (Meta Repo Restructure):

```markdown
### Phase 5: Meta Repo Restructure (EXPANDED)

- [ ] Remove tool directories from meta repo root
- [ ] Create `tools/` directory for submodules
- [ ] Add git submodules for all 10 tools
- [ ] Create `scripts/prepare-docs.sh` for VitePress
- [ ] Update `package.json` with new scripts
- [ ] **UPDATE: `.claude/commands/scaffold-tool.md`** - Complete rewrite
- [ ] **UPDATE: `.claude/commands/test-all.md`** - Submodule iteration
- [ ] **UPDATE: `.claude/commands/quality-check.md`** - Dual context support
- [ ] **UPDATE: `.claude/agents/scaffold-assistant.md`** - Complete rewrite
- [ ] **UPDATE: `.claude/skills/zero-deps-checker/SKILL.md`** - Git URL validation
- [ ] **UPDATE: `templates/tool-repo-template/`** - Standalone CI, CLAUDE.md
- [ ] **UPDATE: `templates/rust-tool-template/`** - Standalone CI, CLAUDE.md
- [ ] Update main CLAUDE.md for submodule structure
- [ ] Update all workflows for submodule paths
- [ ] Test `npm run docs:build` with submodule structure
- [ ] Verify all 10 tool repos have working CI
```

---

## Addendum B: Second Review - Additional Gaps (2025-12-29)

After a second thorough review of the codebase, the following additional gaps were identified.

### B.1 Issue Templates Need Updating

**Location:** `.github/ISSUE_TEMPLATE/tool_proposal.md`

**Problem:** The tool proposal template contains a relative link that will break:
```markdown
- [ ] I've read [PRINCIPLES.md](../../PRINCIPLES.md)
```

**Fix Required:**

```markdown
# Option A: Absolute GitHub URL
- [ ] I've read [PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md)

# Option B: Copy PRINCIPLES.md to each tool repo (not recommended - duplication)
```

**Additional Issue Templates:**
- `bug_report.md` - No relative links, OK as-is
- `feature_request.md` - No relative links, OK as-is

**Task:** Update tool_proposal.md to use absolute GitHub URL before migration.

### B.2 VitePress Guide Pages Need Content Updates

**Location:** `docs/guide/*.md`

Seven guide pages exist with outdated content:

| File | Issues |
|------|--------|
| `getting-started.md` | Progress shows "3 of 33 (9%)" - should be 10/33 (30%) |
| `getting-started.md` | Tool table incomplete (missing 7 tools) |
| `getting-started.md` | Clone instructions assume monorepo |
| `philosophy.md` | Progress shows "2 of 33 (6%)" - outdated |
| `philosophy.md` | References non-existent ROADMAP.md |

**Required Updates:**

```markdown
# getting-started.md - Update installation section
## Quick Start

### Clone Individual Tool (Recommended)
git clone https://github.com/tuulbelt/test-flakiness-detector.git
cd test-flakiness-detector
npm install && npm test

### Clone Meta Repository (All Tools)
git clone --recurse-submodules https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt
```

**Task:** Add to Phase 5 checklist:
- [ ] Update `docs/guide/getting-started.md` - tool counts, clone instructions
- [ ] Update `docs/guide/philosophy.md` - progress, remove ROADMAP.md reference
- [ ] Audit all guide pages for outdated content

### B.3 No Git Tags Exist

**Problem:** No version tags exist in the repository. This will cause issues when:
- Creating GitHub releases for individual tool repos
- Tracking version history
- npm/crates.io publishing

**Current State:**
```bash
git tag -l  # Returns empty
```

**Required for Migration:**

Before splitting tools into separate repos, each tool should have a tag:
```bash
# For each tool
git tag test-flakiness-detector-v0.1.0 <commit-hash>
git tag cli-progress-reporting-v0.1.0 <commit-hash>
# etc.
```

**Alternative:** Create tags in new repos after migration (simpler, loses history).

**Task:** Add to Phase 1 (Pre-Migration):
- [ ] Decide on tagging strategy (pre-migration vs post-migration)
- [ ] If pre-migration: Create tool-specific tags for each v0.1.0 release

### B.4 Versioning Strategy Not Documented

**Problem:** Individual tool repos need clear versioning guidance.

**Add to Each Tool's CONTRIBUTING.md or README:**

```markdown
## Versioning

This tool uses [Semantic Versioning](https://semver.org/):
- `0.x.y` - Initial development (API may change)
- `1.0.0` - First stable release
- Breaking changes increment major version

### Release Process
1. Update `package.json`/`Cargo.toml` version
2. Update CHANGELOG.md
3. Commit: `git commit -m "chore: release v0.2.0"`
4. Tag: `git tag v0.2.0`
5. Push: `git push && git push --tags`
6. GitHub Release created automatically (if workflow exists)
```

**Task:** Add to template files:
- [ ] Add versioning section to `templates/tool-repo-template/CONTRIBUTING.md`
- [ ] Add versioning section to `templates/rust-tool-template/CONTRIBUTING.md`

### B.5 Issue Tracking Strategy Not Documented

**Problem:** After migration, unclear where issues should be filed.

**Options:**

1. **Centralized (Recommended):** All issues in meta repo `tuulbelt/tuulbelt`
   - Pros: Single place to search, cross-tool issues easy
   - Cons: May get cluttered, requires labels

2. **Per-Tool:** Issues in each tool's repo
   - Pros: Cleaner separation, tool-specific context
   - Cons: Cross-tool issues harder, users must find correct repo

3. **Hybrid:** General issues in meta, tool-specific in tool repos
   - Most flexible but most complex

**Recommendation:** Start with Centralized, migrate to Per-Tool if volume increases.

**Implementation for Centralized:**

```markdown
# In each tool's README.md
## Issues & Support

Found a bug? Have a suggestion?
- [Open an issue](https://github.com/tuulbelt/tuulbelt/issues/new)
- Use label: `tool:test-flakiness-detector`
```

**Task:** Add to Phase 3:
- [ ] Decide on issue tracking strategy
- [ ] Create GitHub labels for each tool (if centralized)
- [ ] Update tool READMEs with issue instructions
- [ ] Update issue templates if needed

### B.6 URL Backward Compatibility

**Problem:** After migration, existing URLs may break.

**Affected URLs:**

| Old URL Pattern | Status |
|-----------------|--------|
| `github.com/tuulbelt/tuulbelt/tree/main/test-flakiness-detector` | Will redirect to 404 |
| `tuulbelt.github.io/tuulbelt/tools/test-flakiness-detector/` | Should still work (VitePress) |
| npm package URLs (future) | N/A - new registry entries |

**Mitigation Options:**

1. **GitHub Redirects:** Not available for repository structure changes
2. **README Notice:** Add migration notice to old locations (temporary)
3. **VitePress Redirects:** Configure redirects in VitePress for any changed paths

**VitePress Redirect Configuration:**

```typescript
// docs/.vitepress/config.ts
export default defineConfig({
  head: [
    // Meta redirect for any moved pages
    ['script', {}, `
      if (window.location.pathname === '/old/path/') {
        window.location.replace('/new/path/');
      }
    `]
  ]
})
```

**Task:** Add to Phase 5:
- [ ] Audit current VitePress URLs for any that will change
- [ ] Configure VitePress redirects if needed
- [ ] Update any external documentation with new URLs

### B.7 CHANGELOG Strategy Clarification

**Current State:**
- Root `CHANGELOG.md` tracks meta repo changes
- Each tool has its own `CHANGELOG.md`

**Post-Migration:**

```
tuulbelt/tuulbelt (meta repo)
├── CHANGELOG.md              # Meta repo infrastructure changes only
└── tools/
    └── test-flakiness-detector (submodule)
        └── CHANGELOG.md      # Tool-specific changes

tuulbelt/test-flakiness-detector (tool repo)
└── CHANGELOG.md              # Same as submodule - tool-specific changes
```

**Clarification Needed:**
- Root CHANGELOG: Infrastructure, workflows, VitePress, templates
- Tool CHANGELOGs: Feature changes, bug fixes, API changes

**Task:** Add to documentation:
- [ ] Add section to CONTRIBUTING.md explaining CHANGELOG scopes
- [ ] Ensure tool CHANGELOGs are self-contained (no references to monorepo)

### B.8 VitePress Internal Links Audit

**Problem:** Guide pages have internal links that assume certain URL structure.

**Current Links Found:**

```markdown
# In docs/guide/getting-started.md
[Test Flakiness Detector](/tools/test-flakiness-detector/)
[Philosophy](/guide/philosophy)
[Contributing Guide](/guide/contributing)

# In docs/guide/philosophy.md
[Principles](/guide/principles)
```

**These should continue working** if VitePress structure is preserved. However:

**Potential Issue:** If tool docs are copied from submodules, they may have different internal link structures.

**Mitigation:**
1. Tool docs in submodules should use relative links within their section
2. `scripts/prepare-docs.sh` must preserve link structure
3. Test all links after build: `npm run docs:build`

**Task:** Add to Phase 5:
- [ ] Audit all VitePress internal links
- [ ] Ensure `prepare-docs.sh` preserves link structure
- [ ] Add link validation to CI: `npx vitepress build docs 2>&1 | grep -i "dead link"`

### B.9 Missing ROADMAP.md

**Problem:** `docs/guide/philosophy.md` references ROADMAP.md which doesn't exist:
```markdown
See [ROADMAP](https://github.com/tuulbelt/tuulbelt/blob/main/ROADMAP.md) for detailed planning.
```

**Options:**
1. Create ROADMAP.md with tool priorities and phases
2. Remove the reference from philosophy.md
3. Point to NEXT_TASKS.md instead

**Recommendation:** Create a simple ROADMAP.md:

```markdown
# Tuulbelt Roadmap

## Current Status
- 10 of 33 tools implemented (30%)

## Phases

### Phase 1: Foundation (Complete)
- [x] Test Flakiness Detector
- [x] CLI Progress Reporting
- [x] Cross-Platform Path Normalizer
- [x] File-Based Semaphore (Rust)
- [x] Output Diffing Utility

### Phase 2: Core Tools (In Progress)
- [x] Structured Error Handler
- [x] Configuration File Merger
- [x] Snapshot Comparison
- [x] File-Based Semaphore (TS)
- [x] Test Port Resolver
- [ ] Component Prop Validator (Next)
- [ ] Exhaustiveness Checker
- ...

### Phase 3-4: Advanced Tools
See README.md for complete list.
```

**Task:** Add to Phase 1 (Pre-Migration):
- [ ] Create ROADMAP.md at root level
- [ ] Or update philosophy.md to remove broken reference

### B.10 Tool Repo GitHub Settings Template

**Problem:** Each new tool repo needs consistent GitHub settings.

**Settings to Configure:**

```yaml
# Suggested repository settings for each tool repo

# General
- Default branch: main
- Enable Issues: Yes
- Enable Discussions: No (use meta repo)
- Enable Projects: No (use meta repo)

# Branches
- Branch protection on main:
  - Require pull request reviews: Optional
  - Require status checks: Yes (test workflow)
  - Require linear history: Optional

# Actions
- Allow all actions: Yes
- Allow GitHub Actions to create PRs: Yes

# Pages (if tool has its own docs)
- Source: GitHub Actions
```

**Task:** Create setup documentation:
- [ ] Add `docs/setup/TOOL_REPO_SETTINGS.md` with GitHub settings guide
- [ ] Include screenshots or gh CLI commands for automation

### B.11 Dependabot/Renovate Strategy

**Problem:** How to handle dependency updates across 10+ repos.

**Options:**

1. **Per-Repo Dependabot:** Each repo has own `.github/dependabot.yml`
2. **No Automation:** Tools have zero deps, only update devDependencies manually
3. **Hybrid:** Dependabot for security updates only

**Recommendation:** Option 2 (manual) for now since:
- Zero runtime dependencies
- devDependencies rarely need urgent updates
- Reduces PR noise across repos

**If Automation Needed Later:**

```yaml
# .github/dependabot.yml (template for each tool)
version: 2
updates:
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "monthly"
    open-pull-requests-limit: 5
    # Only devDependencies exist
```

**Task:** Add to templates:
- [ ] Add optional dependabot.yml to tool templates (commented out)
- [ ] Document rationale in template README

---

## Updated Phase 5 Checklist (Incorporating B.x)

```markdown
### Phase 5: Meta Repo Restructure (FINAL)

**Pre-Restructure (from B.x):**
- [ ] Create ROADMAP.md at root level (B.9)
- [ ] Update `.github/ISSUE_TEMPLATE/tool_proposal.md` - absolute URLs (B.1)
- [ ] Decide on issue tracking strategy (B.5)
- [ ] Decide on git tagging strategy (B.3)

**Documentation Updates (from B.x):**
- [ ] Update `docs/guide/getting-started.md` - tool counts, clone instructions (B.2)
- [ ] Update `docs/guide/philosophy.md` - progress, ROADMAP link (B.2)
- [ ] Add versioning section to template CONTRIBUTING.md files (B.4)
- [ ] Add CHANGELOG scope documentation (B.7)
- [ ] Create `docs/setup/TOOL_REPO_SETTINGS.md` (B.10)

**Structure Changes:**
- [ ] Remove tool directories from meta repo root
- [ ] Create `tools/` directory for submodules
- [ ] Add git submodules for all 10 tools
- [ ] Create `scripts/prepare-docs.sh` for VitePress
- [ ] Update `package.json` with new scripts

**Claude Code Updates (from A.x):**
- [ ] UPDATE: `.claude/commands/scaffold-tool.md` - Complete rewrite
- [ ] UPDATE: `.claude/commands/test-all.md` - Submodule iteration
- [ ] UPDATE: `.claude/commands/quality-check.md` - Dual context support
- [ ] UPDATE: `.claude/agents/scaffold-assistant.md` - Complete rewrite
- [ ] UPDATE: `.claude/skills/zero-deps-checker/SKILL.md` - Git URL validation

**Template Updates:**
- [ ] UPDATE: `templates/tool-repo-template/` - Standalone CI, CLAUDE.md
- [ ] UPDATE: `templates/rust-tool-template/` - Standalone CI, CLAUDE.md
- [ ] Add optional dependabot.yml to templates (B.11)

**Finalization:**
- [ ] Update main CLAUDE.md for submodule structure
- [ ] Update all workflows for submodule paths
- [ ] Configure VitePress redirects if needed (B.6)
- [ ] Audit VitePress internal links (B.8)
- [ ] Test `npm run docs:build` with submodule structure
- [ ] Verify all 10 tool repos have working CI
- [ ] Create GitHub labels for each tool if using centralized issues (B.5)
```

---

## Addendum C: Tool Creation Automation System (2025-12-29)

After migration, creating new tools should be highly automated. This section defines the automation architecture.

### C.1 Overview: What Gets Automated

When creating a new Tuulbelt tool, there are **30+ manual steps** that can be automated:

| Category | Manual Steps | Automation |
|----------|--------------|------------|
| GitHub repo creation | 3 | MCP server (CLI) / Instructions (Web) |
| Template scaffolding | 8 | `/new-tool` command |
| Meta repo integration | 4 | `tool-creator` agent |
| Tracking doc updates | 9 | Automatic edits |
| CI/CD setup | 5 | Template generation |
| Verification | 3 | Quality check |

### C.2 `/new-tool` Command Specification

**Location:** `.claude/commands/new-tool.md`

**Purpose:** Single command to create a fully-integrated Tuulbelt tool.

```markdown
# /new-tool Command

Create a new Tuulbelt tool with full automation.

## Usage

/new-tool <tool-name> <language> [short-name] [description]

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| tool-name | Yes | Full kebab-case name | `component-prop-validator` |
| language | Yes | `typescript` or `rust` | `typescript` |
| short-name | No | CLI short name (auto-generated if omitted) | `propval` |
| description | No | One-line description | `"Runtime prop validation"` |

## Examples

\`\`\`bash
# Full specification
/new-tool component-prop-validator typescript propval "Runtime prop validation for components"

# Minimal (auto-generates short name and description)
/new-tool json-schema-validator typescript
\`\`\`

## What This Command Does

### Phase 1: Repository Creation
1. Check if repo already exists (MCP: `check_repo_exists`)
2. Create GitHub repository (MCP: `create_tool_repo` or manual fallback)
3. Configure repo settings (MCP: `configure_repo_settings`)
4. Clone to local workspace

### Phase 2: Scaffolding
5. Copy appropriate template (TypeScript or Rust)
6. Customize package.json / Cargo.toml with tool info
7. Generate README.md with all sections
8. Create CHANGELOG.md with initial entry
9. Generate DOGFOODING_STRATEGY.md
10. Create dogfood-*.sh scripts
11. Set up test boilerplate

### Phase 3: Documentation
12. Create docs/ directory structure
13. Generate 6 VitePress pages from templates
14. Create placeholder demo.gif
15. Create demo recording script

### Phase 4: Meta Repo Integration
16. Add as git submodule to tools/
17. Update docs/.vitepress/config.ts (sidebar)
18. Update docs/tools/index.md (count, card)
19. Update docs/index.md (home page)
20. Update README.md (tool list)
21. Update .claude/NEXT_TASKS.md
22. Update docs/guide/getting-started.md

### Phase 5: CI/CD Setup
23. Add path filter to create-demos.yml
24. Verify tool's test.yml workflow exists
25. Create GitHub labels (if centralized issues)

### Phase 6: Verification
26. Run `npm install` / `cargo build`
27. Run `npm test` / `cargo test`
28. Run `npm run docs:build` (VitePress)
29. Run `/quality-check`

### Phase 7: Commit
30. Commit tool repo: "feat: initialize {tool-name} from template"
31. Push tool repo to origin
32. Commit meta repo: "chore: add {tool-name} submodule"

## Environment Detection

The command automatically detects CLI vs Web environment:
- **CLI:** Uses MCP for GitHub operations
- **Web:** Provides manual instructions, continues with scaffolding after user confirms repo creation
```

### C.3 `tool-creator` Agent Specification

**Location:** `.claude/agents/tool-creator.md`

**Purpose:** Specialized agent with full Tuulbelt context for tool creation.

```markdown
# tool-creator Agent

Specialized agent for creating new Tuulbelt tools with full automation.

## Agent Description

Use this agent when creating a new Tuulbelt tool. It has full context of:
- Tuulbelt architecture and principles
- All templates and their customization points
- Tracking document locations and formats
- Quality checklist requirements
- VitePress documentation structure

## Available Tools

- Read, Write, Edit, Glob, Grep (file operations)
- Bash (git, npm, cargo commands)
- MCP tools: tuulbelt-github (if available)

## Context Files (Auto-Loaded)

The agent automatically reads:
- PRINCIPLES.md (design philosophy)
- CLAUDE.md (development standards)
- docs/QUALITY_CHECKLIST.md (requirements)
- .claude/NEXT_TASKS.md (current status)
- templates/tool-repo-template/ (TypeScript template)
- templates/rust-tool-template/ (Rust template)

## Capabilities

### Repository Management
- Create GitHub repos via MCP or manual instructions
- Configure branch protection and settings
- Add submodules to meta repo

### Template Customization
- Replace placeholders in all template files
- Generate appropriate boilerplate for language
- Create tool-specific configurations

### Tracking Document Updates
- Update tool counts across all documents
- Add tool cards to index pages
- Update status tables and lists
- Maintain consistent formatting

### Documentation Generation
- Create VitePress page structure
- Generate API reference from source
- Create demo recording scripts
- Set up dogfooding strategy

### Quality Assurance
- Verify against QUALITY_CHECKLIST.md
- Run build and test commands
- Validate VitePress builds
- Check for broken links

## Workflow

1. **Validate** - Check tool name, conflicts, language support
2. **Create** - GitHub repo (MCP) or manual instructions
3. **Scaffold** - Copy and customize template
4. **Document** - Generate all documentation
5. **Integrate** - Add to meta repo, update tracking
6. **Verify** - Run quality checks
7. **Commit** - Create commits in both repos

## Error Handling

- If GitHub creation fails: Provide manual instructions, continue scaffolding
- If tests fail: Report errors, don't commit, suggest fixes
- If VitePress fails: Check for broken links, fix before commit
- If quality check fails: List failures, require fixes before commit
```

### C.4 `tuulbelt-github` MCP Server Specification

**Location:** `.claude/mcp/tuulbelt-github/`

**Purpose:** GitHub API operations for CLI users.

```
.claude/mcp/tuulbelt-github/
├── package.json
├── index.js          # MCP server implementation
└── README.md         # Setup instructions
```

**MCP Configuration (`.mcp.json` in project root):**

```json
{
  "mcpServers": {
    "tuulbelt-github": {
      "type": "stdio",
      "command": "node",
      "args": ["./.claude/mcp/tuulbelt-github/index.js"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}",
        "GITHUB_ORG": "tuulbelt"
      }
    }
  }
}
```

**Available Tools:**

| Tool | Description | Parameters |
|------|-------------|------------|
| `check_repo_exists` | Check if repository exists | `name: string` |
| `create_tool_repo` | Create new tool repository | `name, description, language, is_private` |
| `configure_repo_settings` | Set up branch protection, labels | `repo: string` |
| `create_github_labels` | Create issue labels for tool | `repo, labels[]` |
| `get_repo_info` | Get repository metadata | `repo: string` |
| `delete_repo` | Delete repository (careful!) | `repo: string, confirm: boolean` |

**Implementation Outline:**

```javascript
// .claude/mcp/tuulbelt-github/index.js
import { Server } from "@modelcontextprotocol/sdk/server/stdio.js";
import { Octokit } from "octokit";

const server = new Server({ name: "tuulbelt-github", version: "1.0.0" });
const octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });
const org = process.env.GITHUB_ORG || "tuulbelt";

// Tool: check_repo_exists
async function checkRepoExists(name) {
  try {
    await octokit.rest.repos.get({ owner: org, repo: name });
    return { exists: true };
  } catch (e) {
    if (e.status === 404) return { exists: false };
    throw e;
  }
}

// Tool: create_tool_repo
async function createToolRepo({ name, description, language, is_private = false }) {
  const repo = await octokit.rest.repos.createInOrg({
    org,
    name,
    description: description || `Tuulbelt tool: ${name}`,
    private: is_private,
    auto_init: false,  // We'll push our own content
    has_issues: true,
    has_projects: false,
    has_wiki: false,
  });

  return {
    url: repo.data.html_url,
    clone_url: repo.data.clone_url,
    ssh_url: repo.data.ssh_url
  };
}

// Tool: configure_repo_settings
async function configureRepoSettings(repo) {
  // Set default branch protection
  await octokit.rest.repos.updateBranchProtection({
    owner: org,
    repo,
    branch: "main",
    required_status_checks: {
      strict: true,
      contexts: ["test"]
    },
    enforce_admins: false,
    required_pull_request_reviews: null,
    restrictions: null
  });

  // Create standard labels
  const labels = [
    { name: "bug", color: "d73a4a", description: "Something isn't working" },
    { name: "enhancement", color: "a2eeef", description: "New feature or request" },
    { name: "documentation", color: "0075ca", description: "Documentation improvements" },
    { name: "good first issue", color: "7057ff", description: "Good for newcomers" }
  ];

  for (const label of labels) {
    try {
      await octokit.rest.issues.createLabel({ owner: org, repo, ...label });
    } catch (e) {
      // Label may already exist
    }
  }

  return { configured: true };
}

// Register tools with MCP server
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    { name: "check_repo_exists", description: "...", inputSchema: {...} },
    { name: "create_tool_repo", description: "...", inputSchema: {...} },
    { name: "configure_repo_settings", description: "...", inputSchema: {...} },
    // ... more tools
  ]
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  switch (request.params.name) {
    case "check_repo_exists":
      return checkRepoExists(request.params.arguments.name);
    case "create_tool_repo":
      return createToolRepo(request.params.arguments);
    case "configure_repo_settings":
      return configureRepoSettings(request.params.arguments.repo);
    // ... more handlers
  }
});

server.connect(process.stdin, process.stdout);
```

**Security Notes:**
- Token passed via environment variable, never committed
- Each developer uses their own token
- Minimum required scopes: `repo`, `admin:org` (for org repos)

### C.5 Additional Commands

#### `/release-tool` Command

**Location:** `.claude/commands/release-tool.md`

```markdown
# /release-tool Command

Release a new version of a Tuulbelt tool.

## Usage

/release-tool <tool-name> <version> [changelog-entry]

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| tool-name | Yes | Tool to release | `test-flakiness-detector` |
| version | Yes | Semver version | `0.2.0` or `1.0.0` |
| changelog-entry | No | What changed | `"Add timeout support"` |

## What This Command Does

1. **Validate** version format and increment logic
2. **Update** package.json / Cargo.toml version
3. **Update** CHANGELOG.md with new entry
4. **Update** README.md badge if needed
5. **Commit** with message: `chore(tool): release v{version}`
6. **Create** git tag: `v{version}`
7. **Push** commits and tags
8. **Update** meta repo docs with new version badge

## Example

\`\`\`bash
/release-tool test-flakiness-detector 0.2.0 "Add --timeout flag for long-running tests"
\`\`\`
```

#### `/add-tool-dependency` Command

**Location:** `.claude/commands/add-tool-dependency.md`

```markdown
# /add-tool-dependency Command

Add a Tuulbelt tool as a dependency of another tool.

## Usage

/add-tool-dependency <tool-name> <dependency-tool> [required|optional]

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| tool-name | Yes | Tool that needs the dependency | `test-port-resolver` |
| dependency-tool | Yes | Tool to depend on | `file-based-semaphore-ts` |
| type | No | `required` (default) or `optional` | `required` |

## What This Command Does

### For TypeScript:
1. Add git URL to package.json dependencies
2. Update README.md with Library Composition section
3. Update VitePress docs with tip callout
4. Update DOGFOODING_STRATEGY.md

### For Rust:
1. Add git dependency to Cargo.toml
2. Update README.md with Library Composition section
3. Update VitePress docs with tip callout

## Example

\`\`\`bash
/add-tool-dependency test-port-resolver file-based-semaphore-ts required
\`\`\`

## Generated package.json Entry

\`\`\`json
{
  "dependencies": {
    "@tuulbelt/file-based-semaphore-ts": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
  }
}
\`\`\`
```

#### `/sync-tool-docs` Command

**Location:** `.claude/commands/sync-tool-docs.md`

```markdown
# /sync-tool-docs Command

Synchronize tool documentation across README and VitePress.

## Usage

/sync-tool-docs <tool-name|all>

## What This Command Does

1. **Read** tool's README.md
2. **Update** VitePress pages to match:
   - CLI usage examples
   - API reference
   - Feature list
3. **Update** tool counts across all docs
4. **Validate** internal links
5. **Report** any inconsistencies

## Example

\`\`\`bash
/sync-tool-docs test-flakiness-detector
/sync-tool-docs all  # Sync all tools
\`\`\`
```

#### `/update-all-counts` Command

**Location:** `.claude/commands/update-all-counts.md`

```markdown
# /update-all-counts Command

Update tool counts across all documentation.

## Usage

/update-all-counts

## Files Updated

- README.md (tool count in header)
- docs/index.md (home page counts)
- docs/tools/index.md (tools page count)
- docs/guide/getting-started.md (tool table)
- docs/guide/philosophy.md (progress)

## What It Does

1. Count actual tool directories in tools/
2. Update all "X of 33 tools" references
3. Update phase progress percentages
4. Commit changes if any updates made
```

### C.6 Tracking Document Auto-Update Specifications

The `tool-creator` agent must update these documents with precise edits:

#### README.md Updates

```markdown
## Location: README.md

### Tool List Entry (add to appropriate category)
Pattern: `- **[{Tool Name}]({tool-dir}/)** — {description} 🟢 v0.1.0 🐕 | [📖 Docs]({tool-dir}/) | [🚀 Examples]({tool-dir}/examples/)`

### Status Section
Pattern: `**Progress:** {X} of 33 tools implemented ({Y}%)`

### Dogfooding Section (if tool uses another)
Add example showing the integration
```

#### docs/index.md Updates

```markdown
## Location: docs/index.md

### Hero Section
Update: "View All {X} Tools" button text

### Features/Cards Section
Add new tool card with icon

### Progress Section
Update counts and percentages
```

#### docs/tools/index.md Updates

```markdown
## Location: docs/tools/index.md

### Header
Update: "{X} tools available" count

### Tool Grid
Add card:
\`\`\`html
<div class="tool-card">
  <h3><a href="/tools/{tool-name}/">{Tool Name}</a></h3>
  <p>{description}</p>
  <span class="badge">v0.1.0</span>
</div>
\`\`\`
```

#### .claude/NEXT_TASKS.md Updates

```markdown
## Location: .claude/NEXT_TASKS.md

### Implemented Section
Move tool from "Proposed" to "Implemented" table

### Short CLI Names Reference
Add entry: `| {Tool Name} | \`{short}\` | \`{long}\` |`

### Test Counts
Add row: `| {Tool Name} | {X} tests | ✅ 🐕 |`
```

#### docs/.vitepress/config.ts Updates

```typescript
// Location: docs/.vitepress/config.ts

// Add to sidebar items array:
{
  text: '{Tool Name}',
  collapsed: true,
  items: [
    { text: 'Overview', link: '/tools/{tool-name}/' },
    { text: 'Getting Started', link: '/tools/{tool-name}/getting-started' },
    { text: 'CLI Usage', link: '/tools/{tool-name}/cli-usage' },
    { text: 'Library Usage', link: '/tools/{tool-name}/library-usage' },
    { text: 'API Reference', link: '/tools/{tool-name}/api-reference' },
    { text: 'Examples', link: '/tools/{tool-name}/examples' }
  ]
}
```

#### .github/workflows/create-demos.yml Updates

```yaml
# Location: .github/workflows/create-demos.yml

# Add to paths: section under on.push and on.pull_request:
- '{tool-name}/**'
```

### C.7 Execution Breakdown: Web vs CLI

This section defines what can be done in each environment and the order of operations.

#### Tasks by Environment

| Task | Web | CLI | Notes |
|------|-----|-----|-------|
| **GitHub repo creation** | ❌ Manual | ✅ MCP | Web users create via GitHub UI |
| **Template scaffolding** | ✅ | ✅ | File operations work in both |
| **package.json customization** | ✅ | ✅ | File edits work in both |
| **README generation** | ✅ | ✅ | File writes work in both |
| **Git submodule add** | ⚠️ Limited | ✅ | Web may have git limitations |
| **VitePress config update** | ✅ | ✅ | File edits work in both |
| **Tracking doc updates** | ✅ | ✅ | File edits work in both |
| **npm install / cargo build** | ⚠️ Limited | ✅ | Web may timeout |
| **Run tests** | ⚠️ Limited | ✅ | Web may timeout |
| **Git push** | ⚠️ Limited | ✅ | Web has auth issues |
| **Quality check** | ⚠️ Partial | ✅ | Some checks may fail in Web |

#### Recommended Workflow by Environment

**If Using Claude Code CLI (Recommended for Tool Creation):**

```
1. Run /new-tool command
2. MCP creates GitHub repo automatically
3. Agent scaffolds, documents, integrates
4. Agent runs verification
5. Agent commits and pushes
6. Done - fully automated
```

**If Using Claude Code Web:**

```
1. MANUAL: Create GitHub repo via UI
   - Go to github.com/organizations/tuulbelt/repositories/new
   - Create public repo with tool name
   - Don't initialize with README

2. Tell Claude the repo is created

3. Claude scaffolds locally:
   - Copies template
   - Customizes files
   - Creates documentation
   - Updates tracking docs

4. MANUAL: Clone repo locally and push
   - git clone https://github.com/tuulbelt/{tool}.git
   - Copy generated files
   - git add . && git commit && git push

5. MANUAL: Add submodule to meta repo
   - cd tuulbelt
   - git submodule add https://github.com/tuulbelt/{tool}.git tools/{tool}

6. Claude commits meta repo changes
```

### C.8 Migration Execution Order (Precedence)

The migration must happen in this specific order:

```
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 0: PREPARATION                          │
│                    (Can be done in Web)                          │
├─────────────────────────────────────────────────────────────────┤
│ 0.1 Create ROADMAP.md                                           │
│ 0.2 Update issue templates (absolute URLs)                      │
│ 0.3 Update guide pages (tool counts)                            │
│ 0.4 Decide on issue tracking strategy                           │
│ 0.5 Decide on tagging strategy                                  │
│ 0.6 Create docs/setup/TOOL_REPO_SETTINGS.md                     │
│ 0.7 Add versioning section to template CONTRIBUTING.md          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 1: AUTOMATION SETUP                     │
│                    (Must be done in CLI)                         │
├─────────────────────────────────────────────────────────────────┤
│ 1.1 Create MCP server: .claude/mcp/tuulbelt-github/             │
│ 1.2 Create .mcp.json configuration                              │
│ 1.3 Test MCP server locally (CLI only)                          │
│ 1.4 Create /new-tool command                                    │
│ 1.5 Create tool-creator agent                                   │
│ 1.6 Create additional commands (/release-tool, etc.)            │
│ 1.7 Test full /new-tool workflow with a test repo               │
│ 1.8 Delete test repo after verification                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 2: CREATE TOOL REPOS                    │
│                    (Must be done in CLI)                         │
├─────────────────────────────────────────────────────────────────┤
│ Wave 1: Independent tools (no dependencies)                     │
│   2.1 Create repo: tuulbelt/cli-progress-reporting              │
│   2.2 Create repo: tuulbelt/cross-platform-path-normalizer      │
│   2.3 Create repo: tuulbelt/structured-error-handler            │
│   2.4 Create repo: tuulbelt/config-file-merger                  │
│   2.5 Create repo: tuulbelt/file-based-semaphore (Rust)         │
│   2.6 Create repo: tuulbelt/output-diffing-utility (Rust)       │
│   2.7 Create repo: tuulbelt/file-based-semaphore-ts             │
│                                                                  │
│ Wave 2: Optional dependencies                                    │
│   2.8 Create repo: tuulbelt/test-flakiness-detector             │
│        - Update: cli-progress-reporting git URL (optional)       │
│                                                                  │
│ Wave 3: Required dependencies                                    │
│   2.9 Create repo: tuulbelt/snapshot-comparison                 │
│        - Update: output-diffing-utility git URL (required)       │
│   2.10 Create repo: tuulbelt/test-port-resolver                 │
│        - Update: file-based-semaphore-ts git URL (required)      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 3: MIGRATE CONTENT                      │
│                    (Can be done in CLI or Web)                   │
├─────────────────────────────────────────────────────────────────┤
│ For each tool repo (in order from Phase 2):                     │
│   3.x.1 Copy source code from monorepo                          │
│   3.x.2 Copy tests                                              │
│   3.x.3 Copy documentation                                      │
│   3.x.4 Update internal paths (../sibling → git URLs)           │
│   3.x.5 Add standalone CI workflow                              │
│   3.x.6 Add CLAUDE.md for tool repo                             │
│   3.x.7 Verify tests pass                                       │
│   3.x.8 Push to tool repo                                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 4: RESTRUCTURE META REPO                │
│                    (Must be done in CLI for git operations)      │
├─────────────────────────────────────────────────────────────────┤
│ 4.1 Create tools/ directory                                     │
│ 4.2 Add all 10 tool repos as submodules                         │
│ 4.3 Remove old tool directories from root                       │
│ 4.4 Update .gitignore for submodules                            │
│ 4.5 Create scripts/prepare-docs.sh                              │
│ 4.6 Update package.json with new scripts                        │
│ 4.7 Test npm run docs:build with new structure                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 5: UPDATE CLAUDE CODE COMPONENTS        │
│                    (Can be done in Web after structure change)   │
├─────────────────────────────────────────────────────────────────┤
│ 5.1 UPDATE: .claude/commands/scaffold-tool.md → /new-tool       │
│ 5.2 UPDATE: .claude/commands/test-all.md                        │
│ 5.3 UPDATE: .claude/commands/quality-check.md                   │
│ 5.4 UPDATE: .claude/agents/scaffold-assistant.md                │
│ 5.5 UPDATE: .claude/skills/zero-deps-checker/                   │
│ 5.6 UPDATE: templates/tool-repo-template/                       │
│ 5.7 UPDATE: templates/rust-tool-template/                       │
│ 5.8 UPDATE: Main CLAUDE.md                                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 6: VERIFICATION                         │
│                    (CLI recommended for full testing)            │
├─────────────────────────────────────────────────────────────────┤
│ 6.1 Verify all 10 tool repos have passing CI                    │
│ 6.2 Verify VitePress builds successfully                        │
│ 6.3 Verify all internal links work                              │
│ 6.4 Verify /new-tool creates working tool repo                  │
│ 6.5 Verify git submodule update --init works                    │
│ 6.6 Test cloning single tool repo standalone                    │
│ 6.7 Test tool with git URL dependency works standalone          │
│ 6.8 Run /quality-check on meta repo                             │
│ 6.9 Update HANDOFF.md with migration completion                 │
└─────────────────────────────────────────────────────────────────┘
```

### C.9 Quick Reference: What to Do Where

#### Do in Claude Code Web (Preparation & Documentation)

- ✅ Update existing markdown files
- ✅ Create new documentation
- ✅ Update CLAUDE.md, commands, agents
- ✅ Write MCP server code (but can't test)
- ✅ Create templates
- ✅ Update VitePress config
- ✅ Code review and planning

#### Do in Claude Code CLI (GitHub & Git Operations)

- ✅ Create GitHub repositories (MCP)
- ✅ Configure repo settings (MCP)
- ✅ Git submodule operations
- ✅ Git push to new repos
- ✅ Run full test suites
- ✅ Test MCP server
- ✅ Full /quality-check
- ✅ Run /new-tool command

#### Hybrid Workflow (Efficient Use of Both)

```
WEB SESSION:
  1. Plan and design
  2. Update documentation
  3. Write command/agent/MCP code
  4. Commit to branch
  5. Create handoff

CLI SESSION:
  1. Resume from handoff
  2. Create GitHub repos (MCP)
  3. Push code to repos
  4. Run verification
  5. Complete migration
```

---

## Document History

| Date | Author | Changes |
|------|--------|---------|
| 2025-12-29 | Claude | Initial migration plan created |
| 2025-12-29 | Claude | Added Addendum A with detailed component updates |
| 2025-12-29 | Claude | Added Addendum B with second review findings |
| 2025-12-29 | Claude | Added Claude Code Web limitation warning (gh CLI bug) |
| 2025-12-29 | Claude | Added Addendum C with tool creation automation system |

---

**End of Migration Plan**
