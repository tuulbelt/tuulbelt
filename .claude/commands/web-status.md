# /web-status Command

Display status of current Claude Code Web session.

Shows tracked submodules, feature branches, commits, and PR status in a table format.

## Usage

```
/web-status
```

## What It Shows

### Session Information
- Meta branch name
- Session ID
- Creation timestamp
- Session status (active/complete)

### Submodules Table
For each tracked submodule:
- **Submodule name**: Path (with `tools/` prefix removed)
- **Changes**: Whether submodule has uncommitted/unpushed changes
- **Commits**: Number of commits on feature branch
- **PR**: Pull request number (if created)
- **Merged**: Whether PR has been merged

## Example Output

```
╔══════════════════════════════════════════════════════════════════╗
║  Web Session Status                                              ║
╚══════════════════════════════════════════════════════════════════╝

Meta Branch: claude/analyze-repo-structure-AtEoX
Session ID: 4f7e70b1-0ab1-4117-94bd-1e51b9078b2f
Created: 2025-12-31T16:53:45Z
Status: active

┌────────────────────────────────────┬─────────┬─────────┬──────────┬───────────┐
│ Submodule                          │ Changes │ Commits │ PR       │ Merged    │
├────────────────────────────────────┼─────────┼─────────┼──────────┼───────────┤
│ file-based-semaphore               │ Yes     │ 5       │ #123     │ No        │
│ cli-progress-reporting             │ Yes     │ 2       │ #124     │ No        │
│ test-flakiness-detector            │ No      │ 0       │ -        │ -         │
└────────────────────────────────────┴─────────┴─────────┴──────────┴───────────┘
```

## When to Use

- Check current session status
- See which submodules have changes
- Track PR status
- Before committing or pushing changes

## Implementation

Runs: `./scripts/web/show-status.sh`

## See Also

- `/web-setup` - Initialize web session
- `./scripts/web/manage-submodule-branch.sh` - Manage submodule branches
