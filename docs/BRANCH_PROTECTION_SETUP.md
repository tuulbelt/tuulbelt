# Branch Protection Setup

**Last Updated:** 2025-12-31

This document explains how to configure GitHub branch protection rules to enforce correct PR merge order.

---

## Submodule PR Guard

The `submodule-pr-guard.yml` workflow prevents merging meta repo PRs when submodule PRs are still open, enforcing the correct merge order documented in [CI_GUIDE.md](CI_GUIDE.md#pr-merge-coordination).

### How It Works

1. **Automatic Check**: Runs on every PR to `main` branch
2. **Submodule Scan**: Checks each submodule for open PRs on the same branch
3. **Blocks Merge**: Fails the status check if any submodule has open PRs
4. **Admin Override**: Can be bypassed with `override-submodule-check` label

### Setup Instructions

#### Step 1: Enable the Workflow

The workflow file is already in `.github/workflows/submodule-pr-guard.yml`. It will run automatically on all PRs to `main`.

#### Step 2: Configure Branch Protection Rules

Go to GitHub repository settings and configure branch protection for `main`:

**Settings → Branches → Branch protection rules → Add rule (or edit existing)**

```yaml
Branch name pattern: main

Required status checks:
  ✅ Require status checks to pass before merging
  ✅ Require branches to be up to date before merging

  Status checks that are required:
    ✅ check-submodule-prs (from submodule-pr-guard workflow)

Additional settings (recommended):
  ✅ Require a pull request before merging
  ✅ Require approvals: 1
  ✅ Dismiss stale pull request approvals when new commits are pushed
  ✅ Do not allow bypassing the above settings
     (Unchecked - allows admin override with label)

  Allow specified actors to bypass required pull requests:
    ✅ github-actions[bot] (allows automated dashboard/demo updates)
```

**IMPORTANT:**
- Do NOT check "Do not allow bypassing" - this prevents admin override via label
- Add `github-actions[bot]` to bypass actors - this allows automated workflows (update-dashboard, create-demo) to push directly to main without PRs

#### Step 3: Configure Admin Override Access

Only repository admins should be able to add the `override-submodule-check` label.

**Issues → Labels → New label** (from repository main navigation)

```
Name: override-submodule-check
Description: Bypass submodule PR check (admin only - use with caution)
Color: #d73a4a (red)
```

**Note:** GitHub doesn't have granular access control for labels. The "admin only" enforcement relies on:
- Team trust and PR review process
- Only admins should apply this label to PRs
- Misuse is visible in PR history and audit logs

---

## How to Use

### Normal Workflow (Most Cases)

1. Create feature branch in meta repo
2. Create feature branches in submodules
3. Make changes, commit, push
4. Create PRs in submodules first
5. Create PR in meta repo
6. Merge submodule PRs → wait for demos to generate
7. Meta repo PR status check passes automatically
8. Merge meta repo PR ✅

### Admin Override (Rare Cases)

If you need to bypass the check (e.g., meta-only changes with unrelated submodule PRs):

1. Add `override-submodule-check` label to meta repo PR
2. Workflow detects label and skips check
3. Status check passes with override warning
4. Admin can merge PR

**When to use override:**
- Meta repo has documentation-only changes
- Submodule PRs are unrelated to meta changes
- Emergency fix needed in meta repo
- You understand the merge order implications

**When NOT to use override:**
- Submodule PRs contain changes related to meta PR
- Unsure about merge order implications
- Normal development workflow

---

## Workflow Behavior

### PR Status Check Output

**When submodule PRs are open (blocks merge):**

```
❌ MERGE BLOCKED: Submodule PRs must be merged first

The following submodules have open PRs:
- config-file-merger: #1: Fix configuration handling
- structured-error-handler: #1: Add context preservation

📋 Correct merge order (see docs/CI_GUIDE.md):
1. Merge all submodule PRs first
2. Wait for create-demo.yml to complete in each submodule
3. Then merge this meta repo PR

🔓 Admin override:
Add the 'override-submodule-check' label to this PR to bypass this check
```

**When all submodule PRs are merged (allows merge):**

```
✅ All submodules ready - no blocking PRs
This meta repo PR is safe to merge
```

**When override is enabled:**

```
✅ Check bypassed via override label
⚠️  Admin override enabled - merge order check skipped
```

---

## Troubleshooting

### Status check fails but no submodule PRs visible

**Cause:** Submodule is on a feature branch with an open PR

**Solution:**
1. Run `/web-status` to see all submodule branches and PRs
2. Merge the submodule PR
3. Re-run the workflow (push empty commit or re-trigger)

### Can't add override label

**Cause:** Not a repository admin

**Solution:**
1. Ask a repository admin to add the label
2. Or merge the submodule PRs (correct workflow)

### Workflow doesn't run

**Cause:** No changes in PR or workflow disabled

**Solution:**
1. Check `.github/workflows/submodule-pr-guard.yml` exists
2. Check PR is targeting `main` branch
3. Check workflow is enabled in Actions settings

### False positive - submodule on main but check fails

**Cause:** Submodule might be on detached HEAD

**Solution:**
```bash
cd tools/{submodule}
git checkout main
git pull
cd ../..
git add tools/{submodule}
git commit -m "chore: update submodule to main"
git push
```

---

## Architecture

### Workflow Triggers

```yaml
on:
  pull_request:
    branches: [main]
    types:
      - opened        # New PR created
      - synchronize   # New commits pushed
      - reopened      # PR reopened
      - labeled       # Label added (for override)
      - unlabeled     # Label removed
```

### Detection Logic

```bash
For each submodule:
  1. Get current branch (git rev-parse --abbrev-ref HEAD)
  2. If branch is 'main', skip (no PR expected)
  3. If branch is feature branch:
     - Query GitHub API for open PRs from this branch
     - If PR exists, add to list of blocking PRs
  4. If any blocking PRs found:
     - Fail status check (blocks merge)
  5. If override label present:
     - Skip all checks, pass immediately
```

### Override Mechanism

The workflow checks for the `override-submodule-check` label before running any checks:

1. If label exists → skip checks, pass immediately
2. If label doesn't exist → run checks normally
3. Label can be added/removed at any time (re-triggers workflow)

---

## Related Documentation

- [CI_GUIDE.md](CI_GUIDE.md#pr-merge-coordination) - Complete merge order documentation
- [CONTRIBUTING.md](https://github.com/tuulbelt/tuulbelt/blob/main/CONTRIBUTING.md) - Contribution workflow

---

## Changelog

| Date | Change |
|------|--------|
| 2025-12-31 | Initial branch protection setup documentation |

---

*This document should be updated when branch protection rules or workflows change.*
