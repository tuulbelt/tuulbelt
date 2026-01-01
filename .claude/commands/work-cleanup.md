# /work-cleanup Command

Remove workspace after PR merge.

## Usage

```bash
/work-cleanup <feature-name> [--force]
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| feature-name | Yes | Branch name to cleanup (e.g., `feature/my-feature`) |
| --force | No | Skip PR merge verification (dangerous) |

## Description

Safely removes workspace after pull requests have been merged:

**CLI (worktree):**
1. Verify PRs merged (or `--force` flag)
2. Remove worktree directory
3. Delete local branches
4. Delete remote branches (optional)
5. Update tracking file

**CLI (branch):**
1. Verify PRs merged (or `--force` flag)
2. Checkout main
3. Delete local branch
4. Delete remote branch (optional)
5. Update tracking file

**Web:**
1. Verify PRs merged (or `--force` flag)
2. Checkout main
3. Delete local branches (meta + submodules)
4. Delete remote branches (optional)
5. Update tracking file

## Examples

```bash
# Cleanup after PR merge (safe)
/work-cleanup feature/component-prop-validator

# Force cleanup (skip merge check)
/work-cleanup fix/docs-typo --force
```

## Output

### Success:
```
Cleaning up workspace: feature/component-prop-validator

Checking PR status...
  ✓ Meta repo PR #123: merged
  ✓ Submodule test-flakiness-detector PR #45: merged

Removing worktree...
  ✓ Removed: .claude/worktrees/feature-component-prop-validator

Deleting branches...
  ✓ Deleted local branch: feature/component-prop-validator

✓ Workspace cleaned up successfully!
```

### PR Not Merged:
```
❌ ERROR: Pull requests not yet merged

Meta repo: PR #123 (open)
Submodule test-flakiness-detector: PR #45 (open)

Wait for PRs to be merged, or use --force to cleanup anyway:
  /work-cleanup feature/component-prop-validator --force
```

## Warnings

⚠️ **Using `--force` may lose work if:**
- PRs are rejected and you haven't backed up changes
- PRs have requested changes that need addressing
- You have unpushed commits

**Always verify PRs are merged before cleanup!**

## Implementation

**Main Script:** `scripts/workflow/cleanup-workspace.sh`

**CLI Script:** `scripts/cli/cleanup-cli-workspace.sh`

**Web Script:** `scripts/web/cleanup-web-session.sh`

## Related Commands

- `/work-init` - Create new workspace
- `/work-status` - Display workspace state
- `/work-pr` - Create pull requests

## See Also

- [Unified Workflow Plan](../docs/UNIFIED_WORKFLOW_PLAN.md)
