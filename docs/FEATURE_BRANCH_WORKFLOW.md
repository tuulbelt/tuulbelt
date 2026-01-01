# Feature Branch Workflow

**Status:** Production Ready
**Applies To:** CLI and Web environments
**Last Updated:** 2025-12-31

---

## Overview

The Tuulbelt feature branch workflow ensures all changes to the main branch flow through pull requests. This provides code review, testing, and audit trails for all modifications.

### Why Feature Branches?

**Safety:**
- Main branch is always stable and deployable
- Pre-commit hooks prevent accidental commits to main
- All changes require review before merging

**Traceability:**
- Every change has a corresponding pull request
- Commit history shows clear feature development
- Easy to revert or cherry-pick changes

**Quality:**
- CI/CD runs on every pull request
- Code review catches issues before merge
- Testing happens in isolation

**Collaboration:**
- Multiple features can be developed in parallel
- Clear ownership of changes
- Easy to see what's being worked on

---

## Core Principles

These principles apply to **both** CLI and Web environments:

### 1. Never Commit to Main

**Rule:** The `main` branch is read-only. All changes must go through feature branches and pull requests.

**Enforcement:**
- Git pre-commit hooks reject commits to main
- Commands detect main branch and error
- Documentation emphasizes PR workflow

**If you try:**
```bash
git checkout main
echo "test" > test.txt
git add test.txt
git commit -m "test"
```

**You'll see:**
```
❌ ERROR: Direct commits to main branch are not allowed

Please create a feature workspace:
  CLI: /work-init <feature-name>
  Web: Automatically handled by session hooks

Example:
  /work-init feature/my-feature
```

### 2. Feature Branch Per Task

**Rule:** One task = one feature branch (named consistently)

**Naming Convention:**
```
<type>/<description>

Types:
  - feature/  - New functionality
  - fix/      - Bug fixes
  - chore/    - Maintenance (docs, cleanup)
  - refactor/ - Code restructuring

Examples:
  - feature/component-prop-validator
  - fix/docs-typo
  - chore/cleanup-audit-files
  - refactor/simplify-error-handling
```

**Why this matters:**
- Clear intent from branch name
- Easy to find related PRs
- Consistent across team members

### 3. Track Workspace State

**Rule:** The system knows what you're working on, what's changed, and what needs a PR.

**Tracking Mechanisms:**
- **CLI:** `.claude/cli-workspace-tracking.json` + worktree directories
- **Web:** `.claude/web-session-tracking.json` + branch refs

**What's tracked:**
- Feature branch name
- Creation timestamp
- Commits count
- Changed files
- PR URL and status
- Submodule branches (if applicable)

### 4. PR-Based Workflow

**Rule:** All merges to main go through GitHub pull requests (reviewable, auditable).

**Standard Process:**
1. Create feature branch via `/work-init`
2. Make changes, commit
3. Push feature branch to GitHub
4. Create PR via `/work-pr` or manually
5. Review, approve, merge
6. Cleanup via `/work-cleanup`

**Benefits:**
- Code review before merge
- CI/CD runs on PR
- Discussion thread for changes
- Audit trail in GitHub

### 5. Environment Detection

**Rule:** Commands adapt behavior based on environment (CLI vs Web).

**How it works:**
```bash
# Environment variable set by Claude Code
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment - use branches
else
  # CLI environment - use worktrees (default)
fi
```

**Why:**
- CLI has persistent filesystem → worktrees work great
- Web has ephemeral filesystem → branches are lightweight
- Same commands, different implementation

---

## Mental Model

### Universal Workflow (Same for CLI and Web)

```
┌─────────────────────────────────────┐
│ 1. Create Feature Branch            │
│    (Isolated from main)              │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│ 2. Make Changes & Commit             │
│    (Changes only affect your branch) │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│ 3. Push to GitHub                    │
│    (Share your work)                 │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│ 4. Create Pull Request               │
│    (Request review)                  │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│ 5. Review, Test, Approve             │
│    (Quality gate)                    │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│ 6. Merge to Main                     │
│    (Changes integrated)              │
└─────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────┐
│ 7. Cleanup Feature Branch            │
│    (Delete branch, worktree)         │
└─────────────────────────────────────┘
```

### Implementation Differences

**CLI (Worktree-Based):**
```
Workspace = Worktree directory at .claude/worktrees/<feature>/
Isolation = Separate working directory per feature
Parallel = Multiple worktrees active simultaneously
Resume = Worktrees persist on disk
Storage = Tracking file + filesystem state
```

**Web (Branch-Based):**
```
Workspace = Checked out feature branch
Isolation = Different branch, same working directory
Serial = Checkout different branch to switch features
Resume = Branches persist in git (checkout on session start)
Storage = Tracking file committed to git
```

**Key Insight:** Same mental model, different mechanics. You always work in isolation, commit to a feature branch, and merge via PR.

---

## Environment Comparison

| Aspect | CLI | Web |
|--------|-----|-----|
| **Filesystem** | Persistent across sessions | Ephemeral (destroyed each session) |
| **Worktrees** | ✅ Available and persistent | ❌ Not practical (would need recreation) |
| **Best Approach** | Worktree-based (parallel work) | Branch-based (lightweight) |
| **State Storage** | Worktree paths on disk + tracking file | Branches in git + tracking file |
| **Resume Cost** | Free (worktrees exist) | Free (checkout branch from git) |
| **Parallel Features** | ✅ Multiple worktrees simultaneously | ❌ Serial (checkout different branches) |
| **Session Hooks** | Show workspace status on start | Resume session from tracking file |
| **Cleanup** | Remove worktree directory | Delete local branch |

**When to Use CLI:**
- Working on multiple features simultaneously
- Long-running feature development
- Frequent context switching between features

**When to Use Web:**
- Short-lived sessions
- Single feature at a time
- Collaboration across machines

---

## Common Commands

All commands work in both CLI and Web (environment-aware):

### Create Workspace

```bash
/work-init <feature-name>
```

**CLI behavior:** Creates worktree at `.claude/worktrees/<sanitized-name>/`
**Web behavior:** Creates and checks out feature branch

**Example:**
```bash
/work-init feature/add-error-handling
```

### Show Workspace Status

```bash
/work-status
```

**Shows:**
- Current environment (CLI or Web)
- Active workspace/session
- Branch names
- Changed files count
- Commits count
- PR status

### Create Pull Requests

```bash
/work-pr [--meta] [--submodules]
```

**Behavior:**
1. Detects changed repos (meta + submodules)
2. Pushes branches to GitHub
3. Creates PRs via `gh` CLI
4. Updates tracking file with PR URLs

**Flags:**
- `--meta`: Create PR for meta repo only
- `--submodules`: Create PRs for submodules only
- (no flags): Create PRs for both

### Cleanup Workspace

```bash
/work-cleanup <feature-name>
```

**CLI behavior:**
1. Verify PRs merged (or --force flag)
2. Remove worktree directory
3. Delete local branches
4. Delete remote branches
5. Update tracking file

**Web behavior:**
1. Verify PRs merged (or --force flag)
2. Checkout main
3. Delete local branches
4. Delete remote branches
5. Update tracking file

---

## Best Practices

### 1. Start Every Session with `/work-init`

**Why:** Ensures you're never on main branch accidentally.

**CLI:**
```bash
# Session starts
/work-init feature/my-feature
cd .claude/worktrees/feature-my-feature
# Start coding
```

**Web:**
```bash
# Session starts, auto-resumes if existing session
# Or create new:
/work-init feature/my-feature
# Start coding
```

### 2. Commit Often, Push Regularly

**Why:** Preserves work, enables collaboration, triggers CI.

```bash
# Make changes
git add .
git commit -m "feat: add validation logic"

# Push to GitHub (for backup and CI)
git push origin feature/my-feature
```

### 3. Use Descriptive Commit Messages

**Format:** `<type>: <description>`

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `chore:` Maintenance
- `refactor:` Code restructuring
- `test:` Test changes

**Examples:**
```bash
git commit -m "feat: add user authentication"
git commit -m "fix: handle empty input in validator"
git commit -m "docs: update README with examples"
```

### 4. Keep Branches Small and Focused

**Why:** Easier to review, faster to merge, lower risk.

**Good:**
- Feature adds one new command (100-200 lines)
- Fix addresses one specific bug
- Refactor touches one module

**Bad:**
- Feature adds 5 new commands + refactors existing code
- Fix changes unrelated components
- Refactor touches half the codebase

**Rule of thumb:** If you can't describe the change in one sentence, it's too big.

### 5. Sync with Main Before Creating PR

**Why:** Avoids merge conflicts, ensures tests pass against latest code.

```bash
# Before creating PR
git fetch origin main
git merge origin/main

# Resolve any conflicts
git add .
git commit -m "chore: merge main"

# Now create PR
/work-pr
```

### 6. Always Use `/work-cleanup` After Merge

**Why:** Keeps workspace clean, deletes obsolete branches.

```bash
# After PR merged on GitHub
/work-cleanup feature/my-feature
```

**What happens:**
- Verifies PR is merged (prevents accidental deletion)
- Removes worktree (CLI) or switches to main (Web)
- Deletes local feature branch
- Deletes remote feature branch
- Updates tracking file

### 7. Use `--force` Cleanup Sparingly

**When to use:**
- PR was rejected/closed without merge
- Need to abandon work
- Emergency cleanup

**Warning:** Using `--force` can lose work if PR isn't merged!

```bash
# Only if you're sure
/work-cleanup feature/abandoned-work --force
```

---

## Troubleshooting

For detailed troubleshooting, see [WORKFLOW_TROUBLESHOOTING.md](WORKFLOW_TROUBLESHOOTING.md).

**Quick fixes:**

**"Direct commits to main not allowed"**
→ Create feature branch: `/work-init feature/my-fix`

**"Worktree already exists"**
→ Resume: `cd .claude/worktrees/feature-my-feature`
→ Or cleanup: `/work-cleanup feature/my-feature --force`

**"PR creation failed"**
→ Check GitHub authentication: `gh auth status`
→ Create PR manually via GitHub UI

**"Session not resumed in Web"**
→ Check tracking file exists in git
→ Verify feature branch pushed to remote

---

## Session Lifecycle

### CLI Session

```
Session Start
├─ Hook: on-session-start.sh
│  ├─ Install hooks (meta + submodules)
│  └─ Show active workspaces
│
Work in Worktree
├─ Make changes in .claude/worktrees/<feature>/
├─ Commit to feature branch
└─ Tracking file updated automatically
│
Session End
└─ Hook: on-session-end.sh
   └─ Worktrees persist on disk (nothing to do)
```

### Web Session

```
Session Start
├─ Hook: on-session-start.sh
│  ├─ Install hooks (meta + submodules)
│  ├─ Load tracking file from git
│  ├─ Checkout feature branch (if session exists)
│  └─ Show session status
│
Work in Branch
├─ Make changes in main working directory
├─ Commit to feature branch
└─ Tracking file updated automatically
│
Session End
└─ Hook: on-session-end.sh
   ├─ Commit tracking file to git
   └─ State preserved for next session
```

---

## Next Steps

- **CLI workflow details:** [CLI_WORKFLOW.md](CLI_WORKFLOW.md)
- **Web workflow details:** [WEB_WORKFLOW.md](WEB_WORKFLOW.md)
- **Troubleshooting:** [WORKFLOW_TROUBLESHOOTING.md](WORKFLOW_TROUBLESHOOTING.md)

---

**Last Updated:** 2025-12-31
**Maintained By:** Tuulbelt Core Team
