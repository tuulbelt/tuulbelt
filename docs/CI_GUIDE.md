# CI/CD Guide

**Last Updated:** 2025-12-30

This document is the single source of truth for all CI/CD workflows in Tuulbelt. Reference this when adding new tools, debugging CI issues, or optimizing workflows.

---

## Quick Reference

| Workflow | Trigger | Purpose | Runtime |
|----------|---------|---------|---------|
| `test-all-tools.yml` | push, PR, nightly, manual | Test all tools | ~4 min |
| `update-dashboard.yml` | after test-all-tools, nightly, manual | Generate quality dashboard | ~30 sec |
| `deploy-docs.yml` | push to docs/*, manual | Build & deploy VitePress | ~2 min |
| `sync-demos-to-vitepress.yml` | push to tools/*/docs/demo.gif | Sync demos to VitePress | ~1 min |
| `meta-validation.yml` | push to templates/docs, PR | Validate repo structure | ~1 min |
| Per-tool `test.yml` | push/PR to tool's files | Test individual tool | ~1 min |
| Per-tool `create-demo.yml` | push to tool/*, manual | Smart demo recording for specific tool | 0-2 min* |

\* 0 min if no changes, ~2 min per tool that changed (75%+ time savings)

---

## Workflow Architecture

```
                         ┌──────────────────────────┐
                         │     Push to main         │
                         └───────────┬──────────────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         ▼                           ▼                           ▼
┌─────────────────┐        ┌─────────────────┐        ┌─────────────────┐
│ test-all-tools  │        │  deploy-docs    │        │ meta-validation │
│ (if any tool)   │        │ (if docs/*)     │        │ (if templates/) │
└────────┬────────┘        └─────────────────┘        └─────────────────┘
         │
         │ workflow_run (on success)
         │
         ▼
┌─────────────────┐
│update-dashboard │
│ (reads artifact)│
└─────────────────┘
```

> **Note:** Dogfood scripts (`scripts/dogfood-*.sh`) are for local development only. They verify cross-tool composition in the monorepo but are not run in CI. Tests are validated by `test-all-tools.yml`.

---

## Root Workflows (`.github/workflows/`)

### test-all-tools.yml

**Purpose:** Run tests for ALL tools in the monorepo.

**Triggers:**
- `push` to `main` branch
- `pull_request` to `main` branch
- `schedule`: Nightly at 2 AM UTC
- `workflow_dispatch`: Manual trigger

**Features:**
- ✅ Concurrency controls (cancels superseded runs)
- ✅ Dynamic matrix (auto-discovers tools)
- ✅ npm/cargo caching
- ✅ Fail-fast ordering (lint before tests)
- ✅ Uploads test results as artifact

**Outputs:**
- `test-results` artifact (JSON) - retained 30 days
- Used by `update-dashboard.yml`

**Jobs:**
1. `discover-tools` - Find all TypeScript/Rust tools
2. `test-typescript-tools` - Matrix job for each TS tool
3. `test-rust-tools` - Matrix job for each Rust tool
4. `summary` - Combine results, upload artifact

---

### update-dashboard.yml

**Purpose:** Generate quality dashboard from test results (NO re-testing).

**Triggers:**
- `workflow_run`: After `test-all-tools` completes
- `schedule`: Daily at 3 AM UTC
- `workflow_dispatch`: Manual trigger

**Features:**
- ✅ Concurrency controls
- ✅ Downloads artifact instead of re-running tests
- ✅ Generates markdown dashboard

**Key Behavior:**
- Does NOT run any tests
- Downloads `test-results` artifact from test-all-tools
- Parses JSON to generate `docs/TOOL_DASHBOARD.md`

---

### deploy-docs.yml

**Purpose:** Build and deploy VitePress documentation to GitHub Pages.

**Triggers:**
- `push` to `main` with paths:
  - `docs/**`
  - `*/docs/**` (any tool's docs)
  - `*/README.md`
  - `*/SPEC.md`
- `workflow_dispatch`: Manual trigger

**Features:**
- ✅ Path filters (only runs when docs change)
- ✅ Concurrency controls (for pages deployment)
- ✅ npm caching

---

### sync-demos-to-vitepress.yml

**Purpose:** Sync demo recordings from standalone tool repositories to VitePress documentation.

**Triggers:**
- `push` to `main` with paths: `tools/*/docs/demo.gif`
- `workflow_dispatch`: Manual trigger

**Features:**
- ✅ Concurrency controls
- ✅ Auto-detects changed demos from git submodules
- ✅ Copies demo.gif files to VitePress public directory
- ✅ Syncs demo-url.txt for asciinema links

**Architecture:**
- Each standalone tool repo has its own `.github/workflows/create-demo.yml`
- Demos are generated in the tool repo and committed there
- Meta repo's sync workflow pulls the demos when submodules update
- VitePress serves demos from `docs/public/{tool-name}/demo.gif`

**Note:** This replaced the centralized `create-demos.yml` approach. Each tool now manages its own demo recording.

---

### meta-validation.yml

**Purpose:** Validate repository structure and templates.

**Triggers:**
- `push`/`pull_request` to `main` with paths:
  - `templates/**`
  - `docs/*.md`
  - `README.md`, `PRINCIPLES.md`, `CONTRIBUTING.md`, `ARCHITECTURE.md`

**Features:**
- ✅ Path filters (only runs when relevant files change)
- ✅ Concurrency controls

**Checks:**
- Required files exist
- TypeScript template compiles and tests pass
- Rust template compiles and tests pass
- No broken internal links in README.md

---

## Per-Tool Workflows

Each standalone tool repository has its own CI workflows for testing and demo generation.

### TypeScript Tools

**Location:** `{tool}/.github/workflows/test.yml`

**Triggers:**
- `push`/`pull_request` to `main` with paths: `{tool}/**`

**Features:**
- ✅ Path filters (only runs when tool changes)
- ✅ Concurrency controls
- ✅ npm caching
- ✅ Single Node version (20 LTS) for speed

**Steps:**
1. Checkout
2. Setup Node.js with cache
3. `npm ci`
4. `npx tsc --noEmit` (fail fast)
5. `npm test`
6. `npm run build`

### Rust Tools

**Location:** `{tool}/.github/workflows/test.yml`

**Triggers:**
- `push`/`pull_request` to `main` with paths: `{tool}/**`

**Features:**
- ✅ Path filters
- ✅ Concurrency controls
- ✅ Swatinem/rust-cache

**Steps:**
1. Checkout
2. Setup Rust (dtolnay/rust-toolchain)
3. Cache cargo
4. `cargo fmt -- --check` (fail fast)
5. `cargo clippy -- -D warnings` (fail fast)
6. `cargo test`
7. `cargo build --release`

### Demo Recording Workflows (Per-Tool)

**Location:** `{tool}/.github/workflows/create-demo.yml` (in standalone tool repo)

**Triggers:**
- `workflow_dispatch`: Manual trigger
- `push` to `main` - auto-triggers if implementation changed

**Features:**
- ✅ **Smart detection** - Only records when needed (4 checks):
  1. Did implementation change? (src/** or lib/**)
  2. Is demo.gif missing or a placeholder?
  3. Is demo-url.txt missing or empty?
  4. Does README have placeholder demo link (#)?
- ✅ Concurrency controls (cancel in-progress runs)
- ✅ Installs asciinema and agg for recording
- ✅ Updates README with asciinema URL
- ✅ Commits demo files with `[skip ci]`

**Organization Secret Required:** `ASCIINEMA_INSTALL_ID` (shared across all tool repos)

**Steps:**
1. Check if demo needs generation (4 conditions)
2. If needed: Install dependencies, install asciinema, install agg
3. Record demo using tool's `scripts/record-demo.sh`
4. Update README with asciinema URL
5. Commit demo files (demo.cast, docs/demo.gif, demo-url.txt, README.md)

**Key Behavior:**
- Commits include `[skip ci]` to prevent infinite loops
- Only runs when changes warrant new demo
- Standalone - each tool controls its own demo recording
- Meta repo syncs demos via sync-demos-to-vitepress.yml

**Adding to New Tools:**

When creating a new tool, the `/new-tool` command automatically:
1. Creates `.github/workflows/create-demo.yml` in tool repo
2. Creates `scripts/record-demo.sh` for demo commands
3. Creates placeholder `docs/demo.gif` (1x1 transparent)
4. Adds tool to meta repo's sync workflow

See [QUALITY_CHECKLIST.md](QUALITY_CHECKLIST.md#new-tool-completion-checklist) for complete requirements.

---

## Standards & Patterns

### All Workflows Must Have

```yaml
# Cancel in-progress runs when new commits are pushed
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

### Path Filters (When Applicable)

```yaml
on:
  push:
    branches: [main]
    paths:
      - 'relevant-directory/**'
      - '.github/workflows/this-workflow.yml'
  pull_request:
    branches: [main]
    paths:
      - 'relevant-directory/**'
```

### Caching

**TypeScript:**
```yaml
- uses: actions/setup-node@v4
  with:
    node-version: '20'
    cache: 'npm'
    cache-dependency-path: '{tool}/package-lock.json'
```

**Rust:**
```yaml
- uses: Swatinem/rust-cache@v2
  with:
    workspaces: "{tool} -> target"
```

### Fail-Fast Ordering

Run quick checks before slow tests:
1. Formatting (`cargo fmt`, implicit for TS)
2. Linting (`cargo clippy`, `tsc --noEmit`)
3. Tests (`cargo test`, `npm test`)
4. Build (`cargo build`, `npm run build`)

---

## Adding a New Tool

When adding a new tool, ensure:

1. **Create per-tool workflow** with path filters:
   ```yaml
   paths:
     - '{tool-name}/**'
   ```

2. **No changes needed to root workflows** - they auto-discover tools

3. **For demos**, create `scripts/record-{tool-name}-demo.sh`

4. **For docs**, add pages to `docs/tools/{tool-name}/`

---

## Troubleshooting

### Tests Pass Locally But Fail in CI

1. Check Node/Rust version matches CI
2. Ensure `package-lock.json` or `Cargo.lock` is committed
3. Check for platform-specific code (CI runs Ubuntu)

### Workflow Not Triggering

1. Check path filters match changed files
2. Check branch name matches trigger
3. For `workflow_run`, ensure parent workflow completed

### Dashboard Shows Stale Data

1. Check `test-all-tools` completed successfully
2. Check `test-results` artifact exists
3. Manually trigger `update-dashboard`

### Cache Not Working

1. Check cache key includes relevant lock files
2. Cache is per-branch; try main branch
3. GitHub cache limit is 10GB per repo

---

## Scheduled Workflows

| Workflow | Schedule | Purpose |
|----------|----------|---------|
| test-all-tools | 2 AM UTC daily | Catch regressions |
| update-dashboard | 3 AM UTC daily | Update after nightly tests |
| update-demos | 4 AM UTC Sundays | Refresh demo outputs |

---

## Secrets Required

| Secret | Used By | Purpose |
|--------|---------|---------|
| `ASCIINEMA_INSTALL_ID` | create-demos.yml | Upload recordings to asciinema.org |
| `GITHUB_TOKEN` | All workflows | Auto-provided by GitHub |

---

## Performance Optimizations

### Implemented (Phase 1 & 2)

- ✅ Path filters on per-tool workflows
- ✅ Concurrency controls on all workflows
- ✅ Modern actions (`dtolnay/rust-toolchain`, `actions/cache@v4`)
- ✅ Proper caching (npm, cargo, agg binary)
- ✅ Fail-fast step ordering
- ✅ Artifact-based dashboard (no re-testing)
- ✅ Single Node version matrix (20 LTS only)

### Future Improvements (Phase 3)

- [ ] Reusable workflows for TypeScript/Rust testing
- [ ] Consolidate `update-demos.yml` into `create-demos.yml`
- [ ] Add test result badges to README

---

## Related Documentation

- [CI Optimization Proposal](./archive/ci-optimization-proposal-2025-12-25.md) - Detailed analysis (archived - implemented)
- [Quality Checklist](./QUALITY_CHECKLIST.md) - Pre-commit checks
- [Testing Standards](./testing-standards.md) - Test requirements

---

## Changelog

| Date | Change |
|------|--------|
| 2025-12-28 | Removed dogfood-validation.yml (dogfood is local-only) |
| 2025-12-25 | Phase 2: Artifact-based dashboard |
| 2025-12-25 | Phase 1: Path filters, concurrency, caching |
| 2025-12-25 | Initial documentation |

---

*This document should be updated whenever workflows are modified.*
