# /work-status Command

Display current workspace state.

## Usage

```bash
/work-status
```

## Description

Shows the current state of all active workspaces/sessions, including:
- Current environment (CLI or Web)
- Active workspaces/sessions
- Branch names (meta + submodules)
- Changed files count
- Commits count
- PR status (if PRs exist)

## Output Format

### CLI Output (Worktree-based):

```
┌──────────────────────────────────────────────────────────────┐
│ CLI Workspace Status                                         │
└──────────────────────────────────────────────────────────────┘

Worktree: .claude/worktrees/feature-component-prop-validator
Meta Branch: feature/component-prop-validator (3 commits)
Status: active
Created: 2025-12-31T18:00:00Z
Updated: 2025-12-31T19:30:00Z

┌────────────────────────────────┬─────────┬─────────┬──────┐
│ Submodule                      │ Changes │ Commits │ PR   │
├────────────────────────────────┼─────────┼─────────┼──────┤
│ test-flakiness-detector        │ Yes     │ 2       │ #123 │
│ cli-progress-reporting         │ No      │ 0       │ -    │
└────────────────────────────────┴─────────┴─────────┴──────┘
```

### CLI Output (No Worktrees):

```
┌──────────────────────────────────────────────────────────────┐
│ CLI Workspace Status                                         │
└──────────────────────────────────────────────────────────────┘

No active worktrees found.

Use /work-init to create a new workspace.
```

### Web Output:

```
┌──────────────────────────────────────────────────────────────┐
│ Web Session Status                                           │
└──────────────────────────────────────────────────────────────┘

Session: feature-component-prop-validator
Feature Branch: feature/component-prop-validator
Status: active
Created: 2025-12-31T18:00:00Z
Updated: 2025-12-31T19:30:00Z

┌────────────────────────────────┬─────────┬─────────┬──────┐
│ Repository                     │ Changes │ Commits │ PR   │
├────────────────────────────────┼─────────┼─────────┼──────┤
│ meta                           │ Yes     │ 1       │ #456 │
│ test-flakiness-detector        │ Yes     │ 2       │ #123 │
│ cli-progress-reporting         │ No      │ 0       │ -    │
└────────────────────────────────┴─────────┴─────────┴──────┘
```

## Examples

```bash
# Display workspace status
/work-status

# Get status after making changes
git add .
git commit -m "feat: add feature"
/work-status  # Shows 1 commit
```

## Implementation

**Main Script:** `scripts/workflow/show-status.sh`

**CLI Script:** `scripts/cli/show-cli-status.sh`

**Web Script:** `scripts/web/show-web-status.sh`

## Related Commands

- `/work-init` - Create new workspace
- `/work-pr` - Create pull requests
- `/work-cleanup` - Remove workspace

## See Also

- [Unified Workflow Plan](../docs/UNIFIED_WORKFLOW_PLAN.md)
- [CLI Workspace Tracking Schema](../schemas/cli-workspace-tracking.json)
- [Web Session Tracking Schema](../schemas/web-session-tracking.json)
