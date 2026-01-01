# /work-init Command

Create a new workspace for feature development.

## Usage

```bash
/work-init <feature-name> [--no-worktree]
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| feature-name | Yes | Branch name in format `<type>/<description>` |
| --no-worktree | No | (CLI only) Use branch instead of worktree |

## Branch Naming Convention

**Format:** `<type>/<description>`

**Valid Types:**
- `feature/` - New features
- `fix/` - Bug fixes
- `chore/` - Maintenance tasks
- `refactor/` - Code refactoring

**Examples:**
- `feature/component-prop-validator`
- `fix/docs-typo`
- `chore/update-dependencies`
- `refactor/simplify-error-handling`

## Environment Behavior

### CLI (default)
Creates worktree at `.claude/worktrees/<sanitized-name>/`:
- Parallel working directory for feature branch
- Submodules initialized automatically
- Hooks installed automatically
- Tracking file entry created

### CLI (--no-worktree)
Creates feature branch in main working directory:
- Checks out branch directly
- Simpler but no parallel work capability
- Tracking file entry created

### Web
Creates feature branch automatically (no worktrees):
- Checks out branch
- Session tracking entry created
- Ephemeral filesystem considerations handled

## Examples

```bash
# CLI: Create worktree (recommended)
/work-init feature/component-prop-validator

# CLI: Create branch without worktree
/work-init fix/docs-typo --no-worktree

# Web: Always uses branch-based workflow
/work-init feature/new-tool
```

## Output

### Success (CLI worktree):
```
💻 CLI environment - creating worktree...
Checking branch status...
Creating new branch in worktree...
Initializing submodules...
Installing hooks...
Updating tracking file...

✓ Worktree created successfully!

📁 Worktree path: .claude/worktrees/feature-component-prop-validator
🌿 Branch: feature/component-prop-validator

Next steps:
  cd .claude/worktrees/feature-component-prop-validator
  # Make your changes
  git add .
  git commit -m 'feat: implement feature'
  /work-pr  # Create PRs
```

### Success (Web):
```
🌐 Web environment detected - creating feature branch...
✓ Feature branch created: feature/component-prop-validator
✓ Session tracking updated

Next steps:
  # Make your changes
  git add .
  git commit -m 'feat: implement feature'
  /work-pr  # Create PRs
```

## Error Cases

**Invalid branch name:**
```
❌ Invalid branch name format
Expected: <type>/<description>
Types: feature, fix, chore, refactor
Example: feature/component-prop-validator
```

**Worktree already exists:**
```
❌ Worktree already exists: .claude/worktrees/feature-component-prop-validator
Use /work-switch to resume or /work-cleanup to remove
```

## Implementation

**Main Script:** `scripts/workflow/init-workspace.sh`

**CLI Scripts:**
- `scripts/cli/create-worktree.sh` - Worktree creation
- `scripts/cli/create-branch.sh` - Branch creation (--no-worktree)
- `scripts/cli/update-cli-tracking.sh` - Tracking file updates

**Web Scripts:**
- `scripts/web/create-session-branches.sh` - Branch creation
- `scripts/web/update-web-tracking.sh` - Session tracking updates

## Related Commands

- `/work-status` - Display workspace state
- `/work-pr` - Create pull requests
- `/work-cleanup` - Remove workspace after merge

## See Also

- [Unified Workflow Plan](../docs/UNIFIED_WORKFLOW_PLAN.md)
- [CLI Workspace Tracking Schema](../schemas/cli-workspace-tracking.json)
- [Web Session Tracking Schema](../schemas/web-session-tracking.json)
