# CI/CD Workflow Optimization Proposal

**Date:** 2025-12-25
**Status:** Draft for Review

---

## Executive Summary

Current CI workflows will scale **linearly** with tool count. With 4 tools today, we run ~12+ workflow jobs per push. With 33 tools, this becomes 100+ jobs. This proposal outlines optimizations to reduce runtime by 60-70% and prevent linear scaling.

---

## Current State Analysis

### Workflow Inventory (6 Root + Per-Tool)

| Workflow | Trigger | Jobs | Problem |
|----------|---------|------|---------|
| `test-all-tools.yml` | push, PR, nightly | 3+ matrix | ✅ Good - uses dynamic matrices |
| `deploy-docs.yml` | push to docs | 2 | ✅ Good - has path filter |
| `create-demos.yml` | push to scripts | 1 | ⚠️ Installs cargo for agg every time |
| `meta-validation.yml` | push, PR | 1 | ⚠️ Re-runs template tests |
| `update-dashboard.yml` | workflow_run, nightly | 1 | ❌ **RE-RUNS ALL TESTS** |
| `update-demos.yml` | push, weekly | 1 | ⚠️ Only handles one tool |
| Per-tool `test.yml` | push, PR | 3-4 | ❌ **DUPLICATE of test-all-tools** |

### Critical Issues

#### 1. **Triple Testing** ❌ (Most Expensive)

For each push to main affecting a tool:
1. Tool's own `test.yml` runs (3 Node versions = 3 jobs)
2. `test-all-tools.yml` runs (tests all tools)
3. `update-dashboard.yml` runs (tests all tools AGAIN)

**Current cost per push:** 3 + N + N = 3 + 2N jobs (where N = tool count)
**With 33 tools:** 3 + 66 = **69 jobs minimum**

#### 2. Outdated Actions

```yaml
# Current (deprecated)
uses: actions-rs/toolchain@v1
uses: actions/cache@v3

# Should be
uses: dtolnay/rust-toolchain@stable
uses: actions/cache@v4
```

#### 3. No Path Filtering on Per-Tool Workflows

Per-tool workflows run on ANY push to main, not just changes to that tool.

#### 4. Missing Caching

- Dashboard workflow has **zero caching**
- TypeScript jobs don't share npm cache effectively
- Demo workflow installs `agg` from source every time (~2 min)

#### 5. Redundant Demo Workflows

- `update-demos.yml` - Only for test-flakiness-detector
- `create-demos.yml` - For all tools

These should be one workflow.

---

## Proposed Architecture

### Strategy: Monorepo-First with Artifacts

```
                    ┌─────────────────┐
                    │  test-all-tools │ (Primary test workflow)
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
        ┌─────────┐    ┌─────────┐    ┌─────────┐
        │   TS    │    │  Rust   │    │ Summary │
        │ Matrix  │    │ Matrix  │    │   Job   │
        └────┬────┘    └────┬────┘    └─────────┘
             │              │
             ▼              ▼
        ┌─────────────────────────┐
        │   Upload Test Results   │ (JSON artifacts)
        └────────────┬────────────┘
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
  ┌──────────┐ ┌──────────┐ ┌──────────┐
  │Dashboard │ │  Badges  │ │  Notify  │
  │ (read)   │ │ (read)   │ │ (read)   │
  └──────────┘ └──────────┘ └──────────┘
```

### Key Changes

#### 1. Remove Per-Tool Workflows for Monorepo

Since all tools are in one repo, `test-all-tools.yml` is the source of truth.

**Option A: Delete per-tool workflows entirely**
- Pros: Simplest, no duplication
- Cons: Tools aren't independently testable if extracted

**Option B: Keep per-tool workflows with path filters** (Recommended)
- Add `paths: ['<tool>/**']` to each tool's workflow
- Tools only test themselves when their code changes
- Maintains independence if tools are extracted

#### 2. Dashboard Reads Artifacts, No Re-Testing

```yaml
# update-dashboard.yml (NEW)
jobs:
  update-dashboard:
    needs: [] # No test dependency
    steps:
      - name: Download test results artifact
        uses: actions/download-artifact@v4
        with:
          name: test-results
          github-token: ${{ secrets.GITHUB_TOKEN }}
          run-id: ${{ github.event.workflow_run.id }}

      - name: Generate dashboard from results
        run: node scripts/generate-dashboard.js test-results.json
```

#### 3. Reusable Workflows

Create `.github/workflows/reusable-ts-test.yml`:

```yaml
name: Reusable TypeScript Test

on:
  workflow_call:
    inputs:
      tool-path:
        required: true
        type: string
      node-versions:
        required: false
        type: string
        default: '["20"]'  # Only LTS by default

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: ${{ fromJson(inputs.node-versions) }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: 'npm'
          cache-dependency-path: '${{ inputs.tool-path }}/package-lock.json'
      - run: npm ci
        working-directory: ${{ inputs.tool-path }}
      - run: npm test
        working-directory: ${{ inputs.tool-path }}
```

Per-tool workflows become:

```yaml
# test-flakiness-detector/.github/workflows/test.yml
name: Test
on:
  push:
    paths: ['test-flakiness-detector/**']
  pull_request:
    paths: ['test-flakiness-detector/**']

jobs:
  test:
    uses: ./.github/workflows/reusable-ts-test.yml
    with:
      tool-path: test-flakiness-detector
```

#### 4. Optimized Caching

```yaml
# Rust - use Swatinem/rust-cache (already fast)
- uses: Swatinem/rust-cache@v2
  with:
    workspaces: "${{ matrix.tool }} -> target"
    shared-key: "rust-${{ runner.os }}"

# TypeScript - shared cache key
- uses: actions/setup-node@v4
  with:
    cache: 'npm'
    cache-dependency-path: '**/package-lock.json'  # Shared across tools

# Demo workflow - cache agg binary
- uses: actions/cache@v4
  with:
    path: ~/.cargo/bin/agg
    key: agg-${{ runner.os }}-v1.4.3
```

#### 5. Concurrency Controls (All Workflows)

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

#### 6. Fail-Fast Job Ordering

```yaml
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - run: cargo fmt -- --check  # Fast, catches style issues
      - run: cargo clippy -- -D warnings  # Fast, catches bugs

  test:
    needs: lint  # Only run tests if lint passes
    runs-on: ubuntu-latest
    steps:
      - run: cargo test
```

#### 7. Consolidated Demo Workflow

Merge `update-demos.yml` into `create-demos.yml`:

```yaml
# create-demos.yml handles ALL demo generation
on:
  push:
    paths:
      - 'scripts/record-*-demo.sh'
      - '*/src/**'  # Re-record when source changes
  schedule:
    - cron: '0 4 * * 0'  # Weekly
```

---

## Implementation Plan

### Phase 1: Quick Wins (No Breaking Changes)

1. **Update outdated actions** - `actions-rs/toolchain` → `dtolnay/rust-toolchain`
2. **Add concurrency controls** to all workflows
3. **Cache agg binary** in create-demos.yml
4. **Add path filters** to per-tool workflows

**Estimated savings:** 30-40% reduction in redundant runs

### Phase 2: Remove Duplicate Testing

1. **Refactor update-dashboard.yml** to read artifacts instead of re-testing
2. **test-all-tools.yml uploads results** as JSON artifact
3. **Delete update-demos.yml** (merge into create-demos.yml)

**Estimated savings:** 50% reduction (eliminates double testing)

### Phase 3: Reusable Workflows

1. Create `reusable-ts-test.yml` and `reusable-rust-test.yml`
2. Migrate per-tool workflows to use reusable workflows
3. Standardize caching strategy across all workflows

**Estimated savings:** Maintenance reduction, consistent behavior

---

## Projected Savings

### Current State (4 tools)

| Event | Jobs Run | Est. Time |
|-------|----------|-----------|
| Push to tool | 3 + 4 + 4 = 11 | ~8 min |
| PR | 3 + 4 = 7 | ~6 min |
| Nightly | 4 + 4 = 8 | ~7 min |

### After Optimization (4 tools)

| Event | Jobs Run | Est. Time | Savings |
|-------|----------|-----------|---------|
| Push to tool | 1 + 4 = 5 | ~4 min | 50% |
| PR | 1 + 4 = 5 | ~4 min | 33% |
| Nightly | 4 = 4 | ~4 min | 43% |

### Projected at 33 Tools

| Event | Current | Optimized | Savings |
|-------|---------|-----------|---------|
| Push to tool | 69+ jobs | 34 jobs | **51%** |
| PR | 36 jobs | 34 jobs | 6% |
| Nightly | 66 jobs | 33 jobs | **50%** |

---

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| Breaking existing workflows | Phase 1 is non-breaking, test in feature branch |
| Artifact storage limits | GitHub provides 500MB free, test results are tiny (~KB) |
| Reusable workflow complexity | Start with simple cases, document patterns |
| Path filter false negatives | Include workflow files in paths, test thoroughly |

---

## References

- [GitHub Actions Matrix Strategy](https://codefresh.io/learn/github-actions/github-actions-matrix/)
- [Monorepo with GitHub Actions](https://graphite.com/guides/monorepo-with-github-actions)
- [dorny/paths-filter](https://github.com/dorny/paths-filter)
- [Optimizing GitHub Actions Workflows](https://marcusfelling.com/blog/2025/optimizing-github-actions-workflows-for-speed)
- [Create Reusable Workflows](https://resources.github.com/learn/pathways/automation/intermediate/create-reusable-workflows-in-github-actions/)

---

## Next Steps

1. [ ] Review this proposal
2. [ ] Implement Phase 1 (quick wins)
3. [ ] Test in feature branch
4. [ ] Implement Phase 2 (artifact-based dashboard)
5. [ ] Implement Phase 3 (reusable workflows)

---

**Author:** Claude Code Session
**Reviewers:** TBD
