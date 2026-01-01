# CLI Workflow Guide

**Environment:** Claude Code CLI
**Approach:** Worktree-based development
**Last Updated:** 2025-12-31

---

## Overview

The CLI workflow uses **git worktrees** to enable parallel feature development. Each feature gets its own working directory, allowing you to work on multiple features simultaneously without switching branches.

### Why CLI Worktrees?

**Parallel Development:**
- Multiple features in progress at once
- No context switching between branches
- Each worktree is independent

**Persistent State:**
- Worktrees survive across sessions
- No need to recreate on each session start
- Files and changes persist on disk

**Isolated Environments:**
- Each feature has its own directory
- No file conflicts between features
- Can run tests independently

**Fast Context Switching:**
- `cd` between worktrees instantly
- No git checkout delays
- See all features at once: `ls .claude/worktrees/`

### When to Use CLI

**Best for:**
- Long-running feature development
- Working on multiple features simultaneously
- Frequent context switching
- Large codebases where git checkout is slow

**Not ideal for:**
- Quick one-off fixes (branch-based is faster)
- Limited disk space (worktrees duplicate working tree)

---

## Worktree Architecture

### Directory Structure

```
tuulbelt/                                    # Meta repository
├── .git/                                    # Git metadata
│   └── worktrees/                           # Git manages worktrees
│       ├── feature-add-validation/
│       └── feature-refactor-errors/
│
├── .claude/
│   ├── cli-workspace-tracking.json          # Tracking file
│   └── worktrees/                           # Actual worktree directories
│       ├── feature-add-validation/          # First feature
│       │   ├── .git                         # Points to main .git
│       │   ├── src/
│       │   ├── test/
│       │   └── tools/                       # Submodules initialized
│       │       ├── test-flakiness-detector/
│       │       └── ...
│       │
│       └── feature-refactor-errors/         # Second feature
│           ├── .git
│           ├── src/
│           └── ...
│
├── src/                                     # Main working directory (on main)
├── test/
└── tools/                                   # Submodules
```

### How Worktrees Work

**Git worktree basics:**
```bash
# Create worktree for feature branch
git worktree add .claude/worktrees/feature-my-feature feature/my-feature

# What happens:
# 1. Git creates directory at .claude/worktrees/feature-my-feature/
# 2. Checks out feature/my-feature branch in that directory
# 3. Working tree is independent from main working tree
# 4. .git/ is shared (saves disk space)
```

**Why `.claude/worktrees/`?**
- Consistent location for all feature work
- Easy to find and manage
- Not in project root (cleaner workspace)
- Gitignored (worktrees shouldn't be committed)

### Tracking File Schema

`.claude/cli-workspace-tracking.json`:
```json
{
  "version": "1.0",
  "environment": "cli",
  "worktrees": {
    ".claude/worktrees/feature-add-validation": {
      "meta_branch": "feature/add-validation",
      "created_at": "2025-12-31T18:00:00Z",
      "updated_at": "2025-12-31T19:30:00Z",
      "status": "active",
      "submodules": {
        "tools/test-flakiness-detector": {
          "branch": "feature/add-validation",
          "created_at": "2025-12-31T18:05:00Z",
          "has_changes": true,
          "commits_count": 3,
          "last_commit_sha": "abc123...",
          "pr_url": "https://github.com/tuulbelt/test-flakiness-detector/pull/42",
          "pr_number": 42,
          "pr_state": "OPEN",
          "pr_merged": false
        }
      }
    }
  }
}
```

**Why track this:**
- Know which worktrees exist
- See status at a glance
- Track PRs per repository
- Prevent duplicate worktrees

---

## Command Reference

### `/work-init` - Initialize Workspace

**Usage:**
```bash
/work-init <feature-name> [--no-worktree]
```

**What it does:**
1. Validates branch name format
2. Creates worktree at `.claude/worktrees/<sanitized-name>/`
3. Creates feature branch (or checks out existing)
4. Initializes submodules in worktree
5. Installs hooks (meta + submodules)
6. Updates tracking file

**Examples:**
```bash
# Create worktree for new feature
/work-init feature/add-validation

# Output:
# ✓ Worktree created successfully!
#
# 📁 Worktree path: .claude/worktrees/feature-add-validation
# 🌿 Branch: feature/add-validation
#
# Next steps:
#   cd .claude/worktrees/feature-add-validation
#   # Make your changes
#   git add .
#   git commit -m 'feat: implement feature'
#   /work-pr  # Create PRs

# Create branch instead of worktree (lighter weight)
/work-init feature/quick-fix --no-worktree
```

**Branch name validation:**
```
Format: <type>/<description>
Types: feature, fix, chore, refactor

✅ feature/add-validation
✅ fix/docs-typo
✅ chore/cleanup-tests
❌ my-feature (missing type/)
❌ feature/Add Validation (uppercase not allowed)
```

**Resume existing branch:**
```bash
# If branch already exists locally or remotely:
/work-init feature/existing-feature
# Checks out existing branch in new worktree
```

### `/work-status` - Show Workspace Status

**Usage:**
```bash
/work-status
```

**What it shows:**
```
┌──────────────────────────────────────────────────────────────┐
│ CLI Workspace Status                                         │
└──────────────────────────────────────────────────────────────┘

Worktree: .claude/worktrees/feature-add-validation
Meta Branch: feature/add-validation (3 commits)
Status: active

┌────────────────────────────────┬─────────┬─────────┬──────┐
│ Submodule                      │ Changes │ Commits │ PR   │
├────────────────────────────────┼─────────┼─────────┼──────┤
│ test-flakiness-detector        │ Yes     │ 2       │ #123 │
│ cli-progress-reporting         │ No      │ 0       │ -    │
└────────────────────────────────┴─────────┴─────────┴──────┘
```

**Multiple worktrees:**
```
Worktree: .claude/worktrees/feature-add-validation
Meta Branch: feature/add-validation (3 commits)
Status: active

Worktree: .claude/worktrees/fix-docs-typo
Meta Branch: fix/docs-typo (1 commit)
Status: active
```

### `/work-pr` - Create Pull Requests

**Usage:**
```bash
/work-pr [--meta] [--submodules]
```

**What it does:**
1. Detects current worktree
2. Finds repos with uncommitted/unpushed changes
3. Pushes branches to remote
4. Creates PRs via `gh` CLI
5. Updates tracking file with PR URLs

**Examples:**
```bash
# Create PRs for all changed repos
/work-pr

# Create PR for meta repo only
/work-pr --meta

# Create PRs for submodules only
/work-pr --submodules
```

**Output:**
```
📊 Analyzing changes in worktree...

Meta repo:
  Branch: feature/add-validation
  Commits: 3
  Changed files: 5
  🔄 Pushing to origin...
  ✓ Pushed
  🔗 Creating PR...
  ✓ PR created: https://github.com/tuulbelt/tuulbelt/pull/42

Submodules:
  tools/test-flakiness-detector:
    Branch: feature/add-validation
    Commits: 2
    Changed files: 3
    🔄 Pushing to origin...
    ✓ Pushed
    🔗 Creating PR...
    ✓ PR created: https://github.com/tuulbelt/test-flakiness-detector/pull/123

Summary:
  Total PRs created: 2
  Meta: #42
  Submodules: test-flakiness-detector #123
```

**Requirements:**
- `gh` CLI installed and authenticated
- Changes committed to feature branch
- Remote repository exists

### `/work-cleanup` - Remove Workspace

**Usage:**
```bash
/work-cleanup <feature-name> [--force]
```

**What it does:**
1. Verifies PRs merged (unless --force)
2. Removes worktree directory
3. Deletes local feature branch
4. Deletes remote feature branch
5. Updates tracking file

**Examples:**
```bash
# Cleanup after PRs merged
/work-cleanup feature/add-validation

# Force cleanup (use with caution!)
/work-cleanup feature/abandoned-work --force
```

**Output:**
```
🔍 Verifying PRs merged...
  Meta PR #42: MERGED ✓
  Submodule PR #123: MERGED ✓

🗑️  Removing worktree...
  ✓ Removed .claude/worktrees/feature-add-validation

🌿 Deleting local branches...
  ✓ Deleted feature/add-validation (meta)
  ✓ Deleted feature/add-validation (test-flakiness-detector)

☁️  Deleting remote branches...
  ✓ Deleted origin/feature/add-validation (meta)
  ✓ Deleted origin/feature/add-validation (test-flakiness-detector)

📝 Updating tracking file...
  ✓ Removed worktree entry

✓ Cleanup complete!
```

**Safety checks:**
- Won't delete if PRs are still open (unless --force)
- Provides fallback if worktree removal fails
- Confirms before deleting remote branches

---

## Session Lifecycle

### Session Start

**What happens automatically:**
```bash
# .claude/hooks/on-session-start.sh runs
# 1. Detects CLI environment
# 2. Installs hooks in meta repo + all submodules
# 3. Shows active workspaces (if any)
```

**Terminal output:**
```
💻 Claude Code CLI detected - checking workspace status...

Active workspaces:

┌──────────────────────────────────────────────────────────────┐
│ CLI Workspace Status                                         │
└──────────────────────────────────────────────────────────────┘

Worktree: .claude/worktrees/feature-add-validation
Meta Branch: feature/add-validation (3 commits)
Status: active

Tip: Run /work-status anytime to see current state

Session ready! 🚀
```

**No active workspaces:**
```
💻 Claude Code CLI detected - checking workspace status...

Active workspaces:

┌──────────────────────────────────────────────────────────────┐
│ CLI Workspace Status                                         │
└──────────────────────────────────────────────────────────────┘

No active worktrees found.

Use /work-init to create a new workspace.

Session ready! 🚀
```

### Working in Worktree

**Typical workflow:**
```bash
# Navigate to worktree
cd .claude/worktrees/feature-add-validation

# Check current branch
git branch
# * feature/add-validation

# Make changes
vim src/validator.ts

# Check status
git status

# Commit
git add .
git commit -m "feat: add email validation"

# Push (optional, for backup/CI)
git push origin feature/add-validation
```

### Session End

**What happens automatically:**
```bash
# .claude/hooks/on-session-end.sh runs
# 1. Commits tracking file if changed
# 2. Exits (worktrees persist on disk)
```

**Terminal output:**
```
✓ Session ended (worktrees preserved)
```

**Why worktrees persist:**
- CLI filesystem is not ephemeral
- Worktrees are just directories on disk
- Git manages them independently
- No need to recreate on next session

---

## Parallel Development

### Working on Multiple Features

**Scenario:** You're adding a new feature but need to fix a bug.

**Without worktrees:**
```bash
# Working on feature
git checkout feature/add-validation
# Make changes...
# Uh oh, critical bug reported!

# Stash changes
git stash

# Fix bug
git checkout -b fix/critical-bug
# Make changes...
git commit -m "fix: critical bug"
git push

# Back to feature
git checkout feature/add-validation
git stash pop
# Continue work...
```

**With worktrees:**
```bash
# Working on feature in first worktree
cd .claude/worktrees/feature-add-validation
# Make changes...
# Uh oh, critical bug reported!

# Create second worktree for fix
cd ../..
/work-init fix/critical-bug
cd .claude/worktrees/fix-critical-bug

# Fix bug (first worktree untouched)
# Make changes...
git commit -m "fix: critical bug"
git push
/work-pr

# Back to feature (no stash/unstash needed)
cd ../feature-add-validation
# Continue work exactly where you left off
```

**Benefits:**
- No stashing required
- No branch switching
- Both features visible at once
- Can test them independently
- Can run builds in parallel

### Worktree Limits

**How many worktrees can you have?**
- Technical limit: Git supports many (100+)
- Practical limit: Disk space and mental overhead
- Recommendation: 2-3 active worktrees at most

**Disk space considerations:**
```bash
# Each worktree is ~full working tree size
du -sh .claude/worktrees/*
# 150M  .claude/worktrees/feature-add-validation
# 150M  .claude/worktrees/fix-critical-bug
# 150M  .claude/worktrees/refactor-errors

# .git/ is shared (not duplicated)
du -sh .git
# 50M   .git
```

**Clean up inactive worktrees:**
```bash
# List all worktrees
git worktree list

# Remove finished worktrees
/work-cleanup feature/merged-feature
```

---

## Complete Workflow Examples

### Example 1: Simple Feature

**Goal:** Add email validation to configuration file merger

```bash
# 1. Start session (hooks run automatically)
# Shows workspace status

# 2. Create workspace
/work-init feature/add-email-validation

# 3. Navigate to worktree
cd .claude/worktrees/feature-add-email-validation

# 4. Make changes
vim src/validators.ts
# Add email validation logic

# 5. Write tests
vim test/validators.test.ts
# Add test cases

# 6. Run tests locally
npm test

# 7. Commit changes
git add .
git commit -m "feat: add email validation to config merger"

# 8. Push and create PR
/work-pr

# 9. Wait for review, merge on GitHub

# 10. Cleanup
/work-cleanup feature/add-email-validation

# Done! Back on main branch in root directory
```

### Example 2: Multi-Repo Feature (Meta + Submodule)

**Goal:** Add new progress bar style to CLI progress reporting tool

```bash
# 1. Create workspace
/work-init feature/add-spinner-style

# 2. Navigate to worktree
cd .claude/worktrees/feature-add-spinner-style

# 3. Make changes to submodule
cd tools/cli-progress-reporting
vim src/styles.ts
# Add spinner style

# 4. Test submodule changes
npm test

# 5. Commit submodule changes
git add .
git commit -m "feat: add spinner progress style"

# 6. Update meta repo (example usage)
cd ../..
vim examples/spinner-example.ts
# Add example using spinner

# 7. Commit meta repo changes
git add .
git commit -m "docs: add spinner style example"

# 8. Check status
/work-status
# Shows:
# Meta: 1 commit
# cli-progress-reporting: 1 commit

# 9. Create PRs for both repos
/work-pr

# 10. After merge, cleanup
/work-cleanup feature/add-spinner-style
```

### Example 3: Parallel Development

**Goal:** Work on two features simultaneously

```bash
# 1. Start first feature
/work-init feature/add-validation
cd .claude/worktrees/feature-add-validation
# Make changes...
git commit -m "feat: add validation (WIP)"

# 2. Critical bug reported, need to fix immediately
cd ../..
/work-init fix/crash-on-empty-input
cd .claude/worktrees/fix-crash-on-empty-input

# 3. Fix bug
vim src/parser.ts
# Add null check
git add .
git commit -m "fix: prevent crash on empty input"

# 4. Create PR for fix (urgent)
/work-pr

# 5. Back to feature work (no context switch cost)
cd ../feature-add-validation
# Continue exactly where you left off
# No stash/unstash needed
# Bug fix doesn't affect this worktree

# 6. Finish feature
# Make changes...
git commit -m "feat: complete validation logic"
/work-pr

# 7. After both merged, cleanup
/work-cleanup fix/crash-on-empty-input
/work-cleanup feature/add-validation
```

---

## Tips & Tricks

### Quick Navigation

**Alias for worktree directory:**
```bash
# Add to ~/.bashrc or ~/.zshrc
alias cdwork='cd .claude/worktrees'

# Usage
cdwork
ls  # Shows all feature worktrees
cd feature-add-validation
```

**Find worktree by name:**
```bash
# If you forget the exact name
ls .claude/worktrees/ | grep validation
# feature-add-validation
```

### Branch Management

**See all branches across worktrees:**
```bash
git worktree list
# /path/to/tuulbelt                  abc1234 [main]
# .claude/worktrees/feature-add-validation  def5678 [feature/add-validation]
# .claude/worktrees/fix-critical-bug        ghi9012 [fix/critical-bug]
```

**Delete worktree without /work-cleanup:**
```bash
# Manual cleanup (if you need to)
git worktree remove .claude/worktrees/feature-old
git branch -D feature/old  # Delete local branch
git push origin --delete feature/old  # Delete remote
```

### Backup Before Risky Changes

**Snapshot worktree state:**
```bash
# In worktree directory
git stash push -m "snapshot before refactor"

# Make risky changes...

# If something breaks:
git stash pop  # Restore snapshot
```

---

## Troubleshooting

### Common Issues

**"Worktree already exists"**

**Cause:** You already created a worktree for this feature.

**Solution:**
```bash
# Option 1: Resume existing worktree
cd .claude/worktrees/feature-my-feature

# Option 2: Remove and recreate
/work-cleanup feature/my-feature --force
/work-init feature/my-feature
```

**"Fatal: invalid reference"**

**Cause:** Branch doesn't exist locally or remotely.

**Solution:**
```bash
# Create new branch
/work-init feature/new-branch  # Will create branch

# Or fetch from remote
git fetch origin feature/existing-branch
/work-init feature/existing-branch  # Will use remote branch
```

**"Submodules not initialized"**

**Cause:** Worktree was created manually or submodule init failed.

**Solution:**
```bash
cd .claude/worktrees/feature-my-feature
git submodule update --init --recursive
```

**"Tracking file out of sync"**

**Cause:** Worktree was deleted manually outside the workflow.

**Solution:**
```bash
# Regenerate tracking file from current git state
./scripts/workflow/regenerate-tracking.sh

# Or manually edit .claude/cli-workspace-tracking.json
vim .claude/cli-workspace-tracking.json
# Remove stale entries
```

For more troubleshooting, see [WORKFLOW_TROUBLESHOOTING.md](WORKFLOW_TROUBLESHOOTING.md).

---

## Related Documentation

- **Core Principles:** [FEATURE_BRANCH_WORKFLOW.md](FEATURE_BRANCH_WORKFLOW.md)
- **Web Workflow:** [WEB_WORKFLOW.md](WEB_WORKFLOW.md)
- **Troubleshooting:** [WORKFLOW_TROUBLESHOOTING.md](WORKFLOW_TROUBLESHOOTING.md)

---

**Last Updated:** 2025-12-31
**Maintained By:** Tuulbelt Core Team
