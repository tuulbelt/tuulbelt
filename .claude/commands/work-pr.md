# /work-pr Command

Create pull requests for changed repositories.

## Usage

```bash
/work-pr [--meta] [--submodules]
```

## Flags

| Flag | Description |
|------|-------------|
| `--meta` | Create PR for meta repo only |
| `--submodules` | Create PRs for submodules only |
| (no flags) | Create PRs for both meta and submodules |

## Description

Automates pull request creation for all repositories with changes:

1. Detects current workspace (worktree or branch)
2. Finds repos with uncommitted/unpushed changes
3. Pushes branches to remote
4. Creates PRs via `gh` CLI
5. Updates tracking file with PR URLs

## Requirements

- `gh` CLI installed and authenticated
- Changes committed to feature branch
- Remote repository exists
- GitHub token configured (via `.env` or `gh auth login`)

## Examples

```bash
# Create PRs for all changed repos
/work-pr

# Meta repo only
/work-pr --meta

# Submodules only
/work-pr --submodules
```

## Output

### Success:
```
Creating pull requests...

Meta repo:
  ✓ Pushed branch: feature/component-prop-validator
  ✓ Created PR: https://github.com/tuulbelt/tuulbelt/pull/123

Submodules:
  tools/test-flakiness-detector
    ✓ Pushed branch: feature/component-prop-validator
    ✓ Created PR: https://github.com/tuulbelt/test-flakiness-detector/pull/45

  tools/cli-progress-reporting
    → No changes, skipping

✓ Pull requests created successfully!
```

### No Changes:
```
No repositories with changes found.
Commit your changes first:
  git add .
  git commit -m "feat: implement feature"
```

### Missing Requirements:
```
❌ ERROR: gh CLI not found
Install: brew install gh
Then authenticate: gh auth login
```

## Workflow

1. **Before /work-pr:**
   ```bash
   # Make changes
   git add .
   git commit -m "feat: add feature"

   # Verify status
   /work-status
   ```

2. **Create PRs:**
   ```bash
   /work-pr
   ```

3. **After PR creation:**
   ```bash
   # Check status (now shows PR numbers)
   /work-status

   # Wait for PR review/merge
   # Then cleanup workspace
   /work-cleanup feature/component-prop-validator
   ```

## PR Title and Body

**Auto-generated from commits:**
- Title: First commit message
- Body: All commit messages + link to workflow plan

**Example PR Body:**
```markdown
## Changes

- feat: add component prop validator
- test: add validation tests
- docs: update README

## Related

Part of Phase 2: Component validation tools

See [Unified Workflow Plan](../docs/UNIFIED_WORKFLOW_PLAN.md)
```

## Implementation

**Main Script:** `scripts/workflow/create-prs.sh`

**CLI Script:** `scripts/cli/create-cli-prs.sh`

**Web Script:** `scripts/web/create-web-prs.sh`

## Related Commands

- `/work-init` - Create new workspace
- `/work-status` - Display workspace state
- `/work-cleanup` - Remove workspace after merge

## See Also

- [Unified Workflow Plan](../docs/UNIFIED_WORKFLOW_PLAN.md)
- [gh CLI documentation](https://cli.github.com/manual/)
