# Web Workflow Guide

**Environment:** Claude Code Web
**Approach:** Branch-based development with session tracking
**Last Updated:** 2025-12-31

---

## Overview

The Web workflow uses **feature branches** with session tracking to enable feature development in an ephemeral filesystem environment. Sessions automatically save and resume state across browser restarts.

### Why Web Branches (Not Worktrees)?

**Ephemeral Filesystem:**
- Web environment destroys filesystem on session end
- Worktrees would need to be recreated every session
- Branches persist in git (no recreation needed)

**Lightweight:**
- Checking out a branch is fast
- No disk space overhead
- Simple mental model

**Session Tracking:**
- State saved to git commits
- Automatically resumes on session start
- No manual state management

### When to Use Web

**Best for:**
- Short-lived sessions
- Single feature at a time
- Collaboration across machines
- Quick fixes and experiments

**Not ideal for:**
- Working on multiple features simultaneously (use CLI for parallel work)
- Long-running feature development with frequent interruptions

---

## Session Architecture

### Filesystem Lifecycle

```
Session Start (Browser opens)
├─ Clone repository from GitHub
├─ Ephemeral filesystem created
└─ Working directory initialized

Session Active
├─ Files exist on disk
├─ Changes can be made
└─ Commits stored in git

Session End (Browser closes)
├─ Filesystem destroyed
├─ Uncommitted changes LOST
└─ Committed changes safe in git
```

**Critical Rule:** ALWAYS commit before session ends!

### State Persistence Strategy

**What's saved:**
- Committed changes (in git history)
- Tracking file (committed to git)
- Branch names (in git refs)
- PR URLs (in tracking file)

**What's NOT saved:**
- Uncommitted changes
- Environment variables (except those in `.env`)
- Running processes
- Terminal history

**How state persists:**
```
Session 1:
├─ Create feature branch
├─ Make changes
├─ Commit to feature branch
├─ Update tracking file
└─ on-session-end.sh commits tracking file to git

Session 2 (hours/days later):
├─ on-session-start.sh loads tracking file from git
├─ Checks out feature branch
├─ Initializes submodules
└─ Resumes exactly where you left off
```

### Tracking File Schema

`.claude/web-session-tracking.json`:
```json
{
  "version": "1.0",
  "sessions": {
    "feature/add-validation": {
      "created_at": "2025-12-31T18:00:00Z",
      "updated_at": "2025-12-31T19:30:00Z",
      "session_id": "550e8400-e29b-41d4-a716-446655440000",
      "environment": "web",
      "status": "active",
      "submodules": {
        "tools/test-flakiness-detector": {
          "branch": "feature/add-validation",
          "created_at": "2025-12-31T18:05:00Z",
          "has_changes": true,
          "commits_count": 3,
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

**Why this works:**
- Tracking file committed to git on session end
- Loaded from git on session start
- Contains all metadata to resume work
- Session ID prevents conflicts

---

## Command Reference

### `/work-init` - Initialize Session

**Usage:**
```bash
/work-init <feature-name>
```

**What it does:**
1. Creates feature branch in meta repo
2. Checks out feature branch
3. Initializes session tracking entry
4. Updates `.claude/web-session-tracking.json`

**Examples:**
```bash
# Create new feature session
/work-init feature/add-validation

# Output:
# 🌐 Web environment detected - creating feature branch...
# ✓ Created branch: feature/add-validation
# ✓ Checked out feature/add-validation
# ✓ Initialized session tracking
#
# 📝 Session: feature/add-validation
# 🆔 Session ID: 550e8400-e29b-41d4-a716-446655440000
#
# Next steps:
#   # Make your changes
#   git add .
#   git commit -m 'feat: implement feature'
#   /work-pr  # Create PRs
```

**Automatic resume:**
```bash
# If tracking file shows existing session, it's auto-resumed
# on session start (no manual /work-init needed)
```

### `/work-status` - Show Session Status

**Usage:**
```bash
/work-status
```

**What it shows:**
```
┌──────────────────────────────────────────────────────────────┐
│ Web Session Status                                           │
└──────────────────────────────────────────────────────────────┘

Session: feature/add-validation
Session ID: 550e8400-e29b-41d4-a716-446655440000
Meta Branch: feature/add-validation (3 commits)
Status: active

┌────────────────────────────────┬─────────┬─────────┬──────┐
│ Submodule                      │ Changes │ Commits │ PR   │
├────────────────────────────────┼─────────┼─────────┼──────┤
│ test-flakiness-detector        │ Yes     │ 2       │ #123 │
│ cli-progress-reporting         │ No      │ 0       │ -    │
└────────────────────────────────┴─────────┴─────────┴──────┘

⚠️  Remember to commit before ending session!
```

### `/work-pr` - Create Pull Requests

**Usage:**
```bash
/work-pr [--meta] [--submodules]
```

**What it does:**
1. Detects current session branch
2. Finds repos with uncommitted/unpushed changes
3. Pushes branches to remote
4. Creates PRs via `gh` CLI
5. Updates tracking file with PR URLs
6. Commits tracking file to git

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
📊 Analyzing session changes...

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

📝 Updating session tracking...
  ✓ Tracking file updated
  ✓ Changes committed to git

Summary:
  Total PRs created: 2
  Meta: #42
  Submodules: test-flakiness-detector #123
```

### `/work-cleanup` - Remove Session

**Usage:**
```bash
/work-cleanup <feature-name> [--force]
```

**What it does:**
1. Verifies PRs merged (unless --force)
2. Checks out main branch
3. Deletes local feature branch
4. Deletes remote feature branch
5. Removes session from tracking file
6. Commits tracking file update

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

🌿 Switching to main branch...
  ✓ Checked out main

🗑️  Deleting local branches...
  ✓ Deleted feature/add-validation (meta)
  ✓ Deleted feature/add-validation (test-flakiness-detector)

☁️  Deleting remote branches...
  ✓ Deleted origin/feature/add-validation (meta)
  ✓ Deleted origin/feature/add-validation (test-flakiness-detector)

📝 Updating session tracking...
  ✓ Removed session entry
  ✓ Changes committed to git

✓ Cleanup complete!
```

---

## Session Lifecycle

### Session Start (Automatic)

**What happens:**
```bash
# .claude/hooks/on-session-start.sh runs automatically
# 1. Detects Web environment (CLAUDE_CODE_REMOTE=true)
# 2. Installs hooks in meta repo + all submodules
# 3. Runs scripts/web/resume-session.sh
#    ├─ Loads tracking file from git
#    ├─ Finds active session (if any)
#    ├─ Checks out feature branch
#    ├─ Initializes submodules
#    └─ Shows session status
```

**Terminal output (with active session):**
```
🌐 Claude Code Web detected - resuming session...

📂 Loading session state from git...
  ✓ Tracking file loaded

🔍 Found active session: feature/add-validation
  Session ID: 550e8400-e29b-41d4-a716-446655440000
  Created: 2025-12-31T18:00:00Z
  Last updated: 2025-12-31T19:30:00Z

🌿 Checking out feature branch...
  ✓ Switched to feature/add-validation

📦 Initializing submodules...
  ✓ Submodules initialized

┌──────────────────────────────────────────────────────────────┐
│ Session Resumed Successfully                                 │
└──────────────────────────────────────────────────────────────┘

Session: feature/add-validation
Meta: 3 commits
Submodules: 1 with changes

⚠️  Remember to commit before ending session!

Session ready! 🚀
```

**Terminal output (no active session):**
```
🌐 Claude Code Web detected - checking for active sessions...

No active session found.

Use /work-init to create a new feature branch.

Session ready! 🚀
```

### Working in Session

**Typical workflow:**
```bash
# Already on feature branch (auto-resumed or just created)
git branch
# * feature/add-validation

# Make changes
vim src/validator.ts

# Check status
git status

# Stage and commit (CRITICAL in Web!)
git add .
git commit -m "feat: add email validation"

# Push to GitHub (for backup and CI)
git push origin feature/add-validation
```

**IMPORTANT:** Always commit changes! Uncommitted changes will be lost when session ends.

### Session End (Automatic)

**What happens:**
```bash
# .claude/hooks/on-session-end.sh runs automatically
# 1. Commits tracking file if changed
#    ├─ git add .claude/web-session-tracking.json
#    ├─ git commit -m "chore: update Web session tracking"
#    └─ (Tracking file now in git history)
# 2. Runs scripts/web/save-session.sh
#    └─ Additional state saving (if needed)
# 3. Exits (filesystem will be destroyed)
```

**Terminal output:**
```
💾 Saving session state...
  ✓ Tracking file committed to git

✓ Session saved

Browser can now be closed safely.
```

**What's preserved:**
- All commits on feature branch → ✅ Safe in git
- Tracking file → ✅ Committed to git
- Branch refs → ✅ In git
- PR URLs → ✅ In tracking file

**What's lost:**
- Uncommitted changes → ❌ GONE
- Environment variables (not in .env) → ❌ GONE
- Running processes → ❌ GONE

---

## Serial Development (Switching Features)

### Switching to Different Feature

**Scenario:** You're working on feature A, but need to switch to feature B.

**Approach:**
```bash
# Working on feature A
git branch
# * feature/add-validation

# Commit current work (CRITICAL!)
git add .
git commit -m "feat: add validation (WIP)"
git push origin feature/add-validation

# End session normally (tracking file auto-commits)
# Close browser or let session timeout

# Start new session (hours/days later)
# on-session-start.sh auto-resumes feature/add-validation

# Switch to feature B
git checkout -b feature/refactor-errors
# or if branch exists:
git checkout feature/refactor-errors

# Update tracking file to reflect switch
/work-init feature/refactor-errors  # Will update session tracking

# Work on feature B
# Make changes...
git commit -m "refactor: simplify error handling"

# Switch back to feature A
git checkout feature/add-validation
/work-init feature/add-validation  # Update session
```

**Key difference from CLI:**
- CLI: Multiple worktrees = multiple features simultaneously
- Web: One checked out branch = one feature at a time

### Managing Multiple Features

**Option 1: One feature per session (recommended)**
```bash
# Session 1: Work on feature A
/work-init feature/add-validation
# Make changes, commit, push
# End session

# Session 2: Work on feature B
/work-init feature/refactor-errors
# Make changes, commit, push
# End session
```

**Option 2: Manual branch switching (advanced)**
```bash
# Start session (auto-resumes feature A)
git checkout feature/refactor-errors
# Work on feature B
git commit -m "refactor: update"

# Switch back
git checkout feature/add-validation
# Work on feature A
git commit -m "feat: add validation"
```

**Warning:** Manual switching doesn't update session tracking automatically. Use `/work-init` after switching to keep tracking in sync.

---

## Complete Workflow Examples

### Example 1: Simple Feature

**Goal:** Add email validation to configuration file merger

```bash
# 1. Start session (hooks run automatically)
# Shows: "No active session found."

# 2. Create session
/work-init feature/add-email-validation

# 3. Make changes (already on feature branch)
vim src/validators.ts
# Add email validation logic

# 4. Write tests
vim test/validators.test.ts
# Add test cases

# 5. Run tests locally
npm test

# 6. Commit changes (CRITICAL in Web!)
git add .
git commit -m "feat: add email validation to config merger"

# 7. Push to GitHub (backup + CI)
git push origin feature/add-email-validation

# 8. Create PR
/work-pr

# 9. End session (tracking file auto-commits)
# Close browser

# 10. Later: Start new session
# Hooks auto-resume feature/add-email-validation

# 11. After PR merged on GitHub, cleanup
/work-cleanup feature/add-email-validation

# Done! On main branch, session cleared
```

### Example 2: Multi-Repo Feature (Meta + Submodule)

**Goal:** Add new progress bar style to CLI progress reporting tool

```bash
# 1. Create session
/work-init feature/add-spinner-style

# 2. Make changes to submodule
cd tools/cli-progress-reporting
vim src/styles.ts
# Add spinner style

# 3. Test submodule changes
npm test

# 4. Commit submodule changes
git add .
git commit -m "feat: add spinner progress style"

# 5. Update meta repo (example usage)
cd ../..
vim examples/spinner-example.ts
# Add example using spinner

# 6. Commit meta repo changes
git add .
git commit -m "docs: add spinner style example"

# 7. Push both repos
git push origin feature/add-spinner-style  # Meta
cd tools/cli-progress-reporting
git push origin feature/add-spinner-style  # Submodule
cd ../..

# 8. Check status
/work-status
# Shows:
# Meta: 1 commit
# cli-progress-reporting: 1 commit

# 9. Create PRs for both repos
/work-pr

# 10. End session (tracking file auto-commits)
# 11. After merge, cleanup
/work-cleanup feature/add-spinner-style
```

### Example 3: Session Resume After Days

**Goal:** Continue feature work after weekend

```bash
# Friday:
/work-init feature/complex-refactor
# Make changes...
git commit -m "refactor: update module structure (WIP)"
git push origin feature/complex-refactor
# End session

# Monday (new session, different machine):
# Hooks run automatically:
# ✓ Tracking file loaded from git
# ✓ Switched to feature/complex-refactor
# ✓ Submodules initialized
# ✓ Session resumed

# Check what you were doing
git log --oneline -5
# Shows Friday's commits

/work-status
# Shows session state

# Continue work exactly where you left off
vim src/module.ts
# Make more changes...
git commit -m "refactor: complete module restructuring"
/work-pr
```

---

## Best Practices for Web

### 1. Commit Early, Commit Often

**Why:** Uncommitted changes are LOST when session ends!

```bash
# Good: Commit every meaningful change
git add src/validator.ts
git commit -m "feat: add email regex pattern"

git add src/validator.ts
git commit -m "feat: add validation error messages"

git add test/validator.test.ts
git commit -m "test: add email validation tests"

# Bad: Long sessions with no commits
# Make changes for 2 hours...
# Session timeout! All work lost!
```

**Rule of thumb:** Commit every 15-30 minutes of work.

### 2. Push After Every Commit

**Why:** Backup to GitHub, enables CI, allows resumption from any machine.

```bash
# After every commit:
git commit -m "feat: add feature"
git push origin feature/my-feature

# Not just at end of session!
```

### 3. Use Session Status Frequently

**Why:** Know what's committed, what's not.

```bash
# Before making risky changes
/work-status

# Before ending session
/work-status

# To verify PR status
/work-status
```

### 4. Set Session Timeout Reminders

**Why:** Avoid losing work to automatic timeouts.

**Browser extensions:**
- Auto-save/commit extensions
- Session timeout warnings

**Manual reminders:**
```bash
# Set a timer for 30 minutes
# When it goes off: commit and push
```

### 5. Never Leave Session with Uncommitted Changes

**Before ending session:**
```bash
# Check for uncommitted changes
git status

# If anything shows:
git add .
git commit -m "feat: work in progress"
git push origin feature/my-feature
```

### 6. Use Meaningful WIP Commits

**Good WIP commits:**
```bash
git commit -m "feat: add validation logic (WIP - needs tests)"
git commit -m "refactor: extract helper (WIP - needs docs)"
```

**Why:** When you resume, you'll know what to do next.

---

## Troubleshooting

### Common Issues

**"Session not resumed on startup"**

**Cause:** Tracking file not committed or branch not pushed.

**Solution:**
```bash
# Check tracking file
cat .claude/web-session-tracking.json
# If empty or missing:

# Manually create session
/work-init feature/my-feature
git checkout feature/my-feature
```

**"Uncommitted changes lost"**

**Cause:** Session ended without committing.

**Prevention:** Commit every 15-30 minutes!

**Recovery:** If you pushed recently, changes are on GitHub:
```bash
git checkout feature/my-feature
git log
# See what was committed
# Redo uncommitted work
```

**"Submodules not initialized"**

**Cause:** Submodule initialization failed during session resume.

**Solution:**
```bash
git submodule update --init --recursive
```

**"Branch diverged from remote"**

**Cause:** Someone else pushed to your feature branch.

**Solution:**
```bash
# Pull and merge
git pull origin feature/my-feature

# Or pull and rebase
git pull --rebase origin feature/my-feature

# Or force push (dangerous!)
git push --force origin feature/my-feature
```

**"Cannot checkout main - uncommitted changes"**

**Cause:** Trying to cleanup with uncommitted work.

**Solution:**
```bash
# Commit or stash
git add .
git commit -m "feat: WIP"

# Then cleanup
/work-cleanup feature/my-feature
```

For more troubleshooting, see [WORKFLOW_TROUBLESHOOTING.md](WORKFLOW_TROUBLESHOOTING.md).

---

## Related Documentation

- **Core Principles:** [FEATURE_BRANCH_WORKFLOW.md](FEATURE_BRANCH_WORKFLOW.md)
- **CLI Workflow:** [CLI_WORKFLOW.md](CLI_WORKFLOW.md)
- **Troubleshooting:** [WORKFLOW_TROUBLESHOOTING.md](WORKFLOW_TROUBLESHOOTING.md)

---

**Last Updated:** 2025-12-31
**Maintained By:** Tuulbelt Core Team
