# Workflow Troubleshooting Guide

**Last Updated:** 2025-12-31

This guide covers common issues, edge cases, and their solutions for the Tuulbelt feature branch workflow.

---

## Table of Contents

1. [Branch Protection Issues](#branch-protection-issues)
2. [Worktree Issues (CLI)](#worktree-issues-cli)
3. [Session Issues (Web)](#session-issues-web)
4. [PR Creation Issues](#pr-creation-issues)
5. [Cleanup Issues](#cleanup-issues)
6. [Git State Issues](#git-state-issues)
7. [Tracking File Issues](#tracking-file-issues)
8. [Submodule Issues](#submodule-issues)
9. [Collaboration Issues](#collaboration-issues)
10. [Emergency Recovery](#emergency-recovery)

---

## Branch Protection Issues

### Error: "Direct commits to main branch are not allowed"

**What happened:**
You tried to commit to the main branch, but hooks blocked it.

**Why it happened:**
Pre-commit hooks are installed to prevent accidental commits to main.

**Solution:**
```bash
# Option 1: Stash changes and create feature branch
git stash
git checkout -b feature/my-fix
git stash pop
git add .
git commit -m "fix: my fix"

# Option 2: Use work-init (recommended)
git stash
/work-init feature/my-fix
# In worktree (CLI) or on feature branch (Web):
git stash pop
git add .
git commit -m "fix: my fix"
```

**Prevention:**
- Always use `/work-init` before starting work
- Never `git checkout main` to make changes

---

## Worktree Issues (CLI)

### Error: "Worktree already exists"

**What happened:**
```bash
/work-init feature/my-feature
# ❌ Worktree already exists: .claude/worktrees/feature-my-feature
# Use /work-switch to resume or /work-cleanup to remove
```

**Why it happened:**
You already created a worktree for this feature.

**Solutions:**

**Option 1: Resume existing worktree**
```bash
cd .claude/worktrees/feature-my-feature
# Continue work
```

**Option 2: Cleanup and recreate**
```bash
/work-cleanup feature/my-feature --force
/work-init feature/my-feature
```

**Option 3: Use different branch name**
```bash
/work-init feature/my-feature-v2
```

---

### Error: "Fatal: invalid reference"

**What happened:**
```bash
/work-init feature/non-existent
# fatal: invalid reference: feature/non-existent
```

**Why it happened:**
Branch doesn't exist locally or remotely.

**Solution:**
```bash
# /work-init will create the branch automatically
# This error shouldn't occur in normal usage

# If it does, create branch manually:
git checkout -b feature/my-feature
cd ..
/work-init feature/my-feature
```

---

### Worktree Directory Manually Deleted

**What happened:**
You deleted `.claude/worktrees/feature-my-feature/` outside the workflow (e.g., `rm -rf`).

**Symptoms:**
- Tracking file references nonexistent path
- `git worktree list` shows path but directory is gone

**Solution:**
```bash
# Clean up stale worktree entry
git worktree remove .claude/worktrees/feature-my-feature --force
# or
git worktree prune

# Remove from tracking file
/work-status  # Will detect and auto-clean stale entries

# Or manually regenerate tracking file
./scripts/workflow/regenerate-tracking.sh
```

**Prevention:**
- Always use `/work-cleanup` to remove worktrees
- Never manually delete worktree directories

---

### Multiple Worktrees Slow Down System

**What happened:**
Too many worktrees consuming disk space or causing confusion.

**Solution:**
```bash
# List all worktrees
git worktree list

# Remove inactive worktrees
/work-cleanup feature/old-feature-1
/work-cleanup feature/old-feature-2
/work-cleanup feature/old-feature-3

# Or remove all at once (DANGEROUS):
git worktree list --porcelain | grep worktree | awk '{print $2}' | xargs -n1 git worktree remove --force
git worktree prune
```

**Prevention:**
- Clean up merged features promptly
- Keep max 2-3 active worktrees

---

## Session Issues (Web)

### Session Not Resumed on Startup

**What happened:**
Session starts, but you're on `main` branch instead of your feature branch.

**Why it happened:**
- Tracking file not committed to git
- Feature branch not pushed to remote
- Session tracking corrupted

**Solution:**
```bash
# Check tracking file
cat .claude/web-session-tracking.json

# If empty or missing:
# 1. Check if branch exists on remote
git fetch origin
git branch -r | grep feature/my-feature

# 2. If branch exists, check it out
git checkout feature/my-feature

# 3. Recreate session entry
/work-init feature/my-feature
```

**Prevention:**
- Always push after creating session: `/work-init` then `git push`
- Verify tracking file committed: `git log .claude/web-session-tracking.json`

---

### Uncommitted Changes Lost After Session End

**What happened:**
You made changes but didn't commit. Session ended. Changes are gone.

**Why it happened:**
Web filesystem is ephemeral - uncommitted changes don't persist.

**Recovery:**
```bash
# If you pushed recently, check remote
git checkout feature/my-feature
git log
# See last committed work

# Redo uncommitted changes (no other recovery possible)
```

**Prevention:**
- Commit every 15-30 minutes
- Push after every commit
- Set browser reminder/alarm
- Use `/work-status` frequently to check uncommitted changes

---

### Branch Diverged from Remote

**What happened:**
```bash
git push
# error: failed to push some refs
# hint: Updates were rejected because the tip of your current branch is behind
```

**Why it happened:**
Someone else pushed to your feature branch (or you worked from multiple machines).

**Solutions:**

**Option 1: Merge remote changes**
```bash
git pull origin feature/my-feature
# Resolve conflicts if any
git push origin feature/my-feature
```

**Option 2: Rebase on remote changes**
```bash
git pull --rebase origin feature/my-feature
# Resolve conflicts if any
git push origin feature/my-feature
```

**Option 3: Force push (DANGEROUS)**
```bash
# Only if you're SURE remote changes should be discarded
git push --force-with-lease origin feature/my-feature
```

**Prevention:**
- Don't work on same feature from multiple machines simultaneously
- Always `git pull` before making changes if resuming from different machine

---

## PR Creation Issues

### Error: "gh pr create failed"

**What happened:**
```bash
/work-pr
# ❌ Failed to create PR via gh CLI
```

**Why it happened:**
- GitHub CLI not installed
- Not authenticated with GitHub
- No internet connection
- Permissions issue
- API rate limit

**Solutions:**

**Check authentication:**
```bash
gh auth status

# If not authenticated:
gh auth login
```

**Check network:**
```bash
ping github.com
```

**Check rate limit:**
```bash
gh api rate_limit
```

**Manual PR creation:**
```bash
# If gh CLI fails completely, create PR manually:
# 1. Visit GitHub:
#    https://github.com/tuulbelt/tuulbelt/compare/main...feature/my-feature
# 2. Click "Create pull request"
# 3. Fill in title and description
# 4. Create PR
```

**Prevention:**
- Verify `gh auth status` before starting work
- Keep `gh` CLI updated: `gh --upgrade`

---

### Rate Limit Exceeded

**What happened:**
```bash
/work-pr
# ❌ GitHub API rate limit exceeded
```

**Why it happened:**
Too many API calls to GitHub in short time.

**Solution:**
```bash
# Wait 1 hour for rate limit reset

# Or authenticate with PAT for higher limits:
gh auth login
# Select "Paste an authentication token"
# Use GitHub PAT with repo scope

# Or create PR manually (doesn't count against API limit)
```

**Prevention:**
- Use authenticated `gh` CLI (60 requests/hour → 5000 requests/hour)
- Don't run `/work-pr` repeatedly

---

## Cleanup Issues

### Error: "PRs not merged yet"

**What happened:**
```bash
/work-cleanup feature/my-feature
# ⚠️  WARNING: PRs not merged yet
# Pending PRs:
#   - Meta: #42 (OPEN)
#   - Submodule: #123 (OPEN)
# Use --force to cleanup anyway (changes will be lost if PRs rejected)
```

**Why it happened:**
Safety check prevents deleting work before PR approval.

**Solutions:**

**Option 1: Wait for PR merge (recommended)**
```bash
# Wait for review and approval on GitHub
# Then retry cleanup
```

**Option 2: Force cleanup (DANGEROUS)**
```bash
/work-cleanup feature/my-feature --force
# Use only if PR was rejected or work should be abandoned
```

**Prevention:**
- Only cleanup after confirming PRs merged on GitHub
- Use GitHub notifications to track PR status

---

### Merge Conflict During Cleanup

**What happened:**
```bash
/work-cleanup feature/my-feature
# ❌ Merge conflict detected
# Cleanup aborted. Resolve conflicts manually.
```

**Why it happened:**
Main branch has changes that conflict with your feature branch.

**Solution:**
```bash
# Don't cleanup yet. First resolve conflicts:
git checkout main
git pull origin main
git checkout feature/my-feature
git merge main

# Resolve conflicts:
git status  # See conflicting files
vim conflicting-file.ts  # Edit and resolve
git add conflicting-file.ts
git commit -m "chore: merge main and resolve conflicts"

# Push resolved version
git push origin feature/my-feature

# Update PR (conflicts will be resolved in PR too)

# After PR merged, retry cleanup
/work-cleanup feature/my-feature
```

---

## Git State Issues

### Detached HEAD State

**What happened:**
```bash
git branch
# * (HEAD detached at abc1234)
```

**Why it happened:**
Checked out a commit SHA, tag, or remote branch without creating local branch.

**Solution:**

**In meta repo:**
```bash
# Create feature branch from current state
git checkout -b feature/recover-work
/work-init feature/recover-work
```

**In submodule:**
```bash
cd tools/my-submodule
git checkout -b feature/my-feature
cd ../..
```

**Prevention:**
- Always create branches: `git checkout -b feature/name`
- Don't checkout commit SHAs directly

---

### Force-Push Breaks Tracking

**What happened:**
```bash
git push --force origin feature/my-feature
# Now tracking file references old SHA
```

**Why it happened:**
Force push rewrites history, SHA references become stale.

**Solution:**
```bash
# Tracking file will auto-update on next status check
/work-status

# Or regenerate tracking file
./scripts/workflow/regenerate-tracking.sh
```

**Prevention:**
- Avoid `git push --force` on feature branches
- Use `git push --force-with-lease` (safer)
- Only force-push if absolutely necessary

---

### Branch Renamed After Tracking

**What happened:**
```bash
git branch -m feature/old-name feature/new-name
# Tracking file still references old name
```

**Why it happened:**
Renamed branch outside the workflow.

**Solution:**
```bash
# Update tracking file manually
vim .claude/cli-workspace-tracking.json
# Change "feature/old-name" to "feature/new-name"

# Or delete entry and recreate
./scripts/workflow/regenerate-tracking.sh
```

**Prevention:**
- Don't rename branches after creating workspace
- If you must rename, cleanup old workspace and create new one:
  ```bash
  /work-cleanup feature/old-name --force
  /work-init feature/new-name
  ```

---

## Tracking File Issues

### Tracking File Corrupted

**What happened:**
```bash
/work-status
# ❌ Tracking file corrupted
# error: JSON parse failed
```

**Why it happened:**
Manual edit introduced syntax error, or file was truncated.

**Solution:**
```bash
# Backup corrupted file
cp .claude/cli-workspace-tracking.json .claude/cli-workspace-tracking.json.backup

# Regenerate from git state
./scripts/workflow/regenerate-tracking.sh

# Review regenerated file
cat .claude/cli-workspace-tracking.json

# If correct, commit
git add .claude/cli-workspace-tracking.json
git commit -m "fix: regenerate corrupted tracking file"
```

**Prevention:**
- Don't manually edit tracking file
- Use commands to update state

---

### Tracking File Out of Sync

**What happened:**
Tracking file shows worktrees/sessions that don't exist, or doesn't show ones that do.

**Solution:**
```bash
# Regenerate from current git state
./scripts/workflow/regenerate-tracking.sh

# Verify
/work-status

# Commit if correct
git add .claude/cli-workspace-tracking.json
git commit -m "fix: sync tracking file with git state"
```

---

## Submodule Issues

### Submodule Detached HEAD

**What happened:**
```bash
cd tools/my-submodule
git branch
# * (HEAD detached at abc1234)
```

**Why it happened:**
Submodule not on a branch (common after `git submodule update`).

**Solution:**
```bash
# Create feature branch in submodule
cd tools/my-submodule
git checkout -b feature/my-feature

# Or if branch exists:
git checkout feature/my-feature

cd ../..
```

**Prevention:**
- After `/work-init`, submodules should auto-create branches
- If they don't, scripts will create branches on first commit

---

### Submodule Not Initialized

**What happened:**
```bash
ls tools/my-submodule
# Empty directory
```

**Why it happened:**
Submodules weren't initialized in worktree.

**Solution:**
```bash
# Initialize all submodules
git submodule update --init --recursive

# If that fails, in worktree:
cd .claude/worktrees/feature-my-feature
git submodule update --init --recursive
```

**Prevention:**
- `/work-init` should auto-initialize submodules
- If it doesn't, report bug

---

## Collaboration Issues

### Multiple Users, Same Feature

**What happened:**
Two developers working on same feature branch from different machines.

**Symptoms:**
- Merge conflicts
- Diverged history
- Tracking file conflicts

**Solutions:**

**Communication:**
- Coordinate who works on what
- Use separate feature branches if working in parallel

**Technical:**
```bash
# Before starting work, always pull latest
git fetch origin
git checkout feature/shared-feature
git pull origin feature/shared-feature

# After committing, push immediately
git push origin feature/shared-feature

# If conflicts:
git pull --rebase origin feature/shared-feature
# Resolve conflicts
git push origin feature/shared-feature
```

**Prevention:**
- One feature = one developer
- Or split work into separate features
- Or pair program in same session

---

### Modifying Another User's Worktree

**What happened (CLI):**
```bash
/work-status
# ⚠️  WARNING: This worktree was created by alice@laptop
# You are: bob@desktop
```

**Why it happened:**
Tracking file includes user attribution (future feature).

**Solution:**
```bash
# Continue anyway (with caution):
# Press 'y' when prompted

# Or create your own worktree:
/work-init feature/bob-version
```

**Prevention:**
- Each developer uses their own worktrees/branches
- Don't share worktrees across machines

---

## Emergency Recovery

### Lost All Work (Uncommitted)

**What happened:**
Session ended, worktree deleted, or branch deleted before committing.

**Recovery:**
```bash
# Check reflog (last 30 days of commits/checkouts)
git reflog

# Find lost commit
# Look for entry like: abc1234 HEAD@{1}: commit: feat: my work

# Create branch from that commit
git checkout -b feature/recovered abc1234

# Or cherry-pick specific commits
git cherry-pick abc1234
```

**If reflog doesn't help:**
- No recovery possible for uncommitted changes
- This is why we emphasize committing frequently!

---

### Accidentally Deleted Main Branch

**What happened:**
```bash
git branch -D main
# error: Cannot delete branch 'main'
# (Hooks should prevent this, but if it happens...)
```

**Recovery:**
```bash
# Restore from remote
git fetch origin
git checkout -b main origin/main

# Or reset to remote
git checkout main
git reset --hard origin/main
```

**Prevention:**
- Hooks prevent deleting main
- Don't use `git branch -D` (use `/work-cleanup`)

---

### Tracking File Deleted

**What happened:**
`.claude/cli-workspace-tracking.json` or `.claude/web-session-tracking.json` was deleted.

**Recovery:**
```bash
# Regenerate from git state
./scripts/workflow/regenerate-tracking.sh

# Review generated file
cat .claude/cli-workspace-tracking.json

# Commit if correct
git add .claude/cli-workspace-tracking.json
git commit -m "fix: regenerate tracking file"
```

---

### Entire .claude/ Directory Deleted

**What happened:**
Entire `.claude/` directory was deleted (including tracking files, hooks, etc.).

**Recovery:**
```bash
# Restore from git
git checkout HEAD -- .claude/

# If that doesn't work, restore from remote
git fetch origin
git checkout origin/main -- .claude/

# Verify hooks restored
ls -la .claude/hooks/
ls -la .git/hooks/

# Reinstall hooks if needed
./scripts/workflow/install-hooks.sh

# Regenerate tracking file
./scripts/workflow/regenerate-tracking.sh
```

---

### Complete Disaster Recovery

**What happened:**
Everything is broken, nothing works, need to start fresh.

**Nuclear Option:**
```bash
# 1. Save any uncommitted work (if possible)
git stash

# 2. Clean working directory
git clean -fdx

# 3. Reset to main
git checkout main
git reset --hard origin/main

# 4. Remove all worktrees
git worktree list --porcelain | grep worktree | awk '{print $2}' | xargs -n1 git worktree remove --force
git worktree prune

# 5. Reinstall hooks
./scripts/workflow/install-hooks.sh

# 6. Regenerate tracking file
./scripts/workflow/regenerate-tracking.sh

# 7. Start fresh
/work-init feature/new-start
```

---

## Getting Help

If this guide doesn't solve your problem:

1. **Check logs:**
   ```bash
   # Session start logs
   cat .claude/logs/session-start.log

   # Git logs
   git log --oneline -20
   git reflog -20
   ```

2. **Search existing issues:**
   - https://github.com/tuulbelt/tuulbelt/issues

3. **Create new issue:**
   - Use "Workflow Bug Report" template
   - Include:
     - Environment (CLI or Web)
     - Commands run
     - Error messages
     - Current git state: `git status`, `git branch`, `/work-status`

4. **Emergency contact:**
   - File issue with `urgent` label
   - Mention @tuulbelt/maintainers

---

## Preventive Maintenance

### Daily

- [ ] Run `/work-status` before starting work
- [ ] Commit every 15-30 minutes (Web) or end of logical change (CLI)
- [ ] Push after every commit (Web) or end of day (CLI)

### Weekly

- [ ] Clean up merged feature branches: `/work-cleanup`
- [ ] Remove unused worktrees: `git worktree list`
- [ ] Verify tracking file: `/work-status`

### Monthly

- [ ] Update `gh` CLI: `gh --upgrade`
- [ ] Review open PRs: `gh pr list`
- [ ] Archive old branches: `git branch | grep feature/old-*`

---

## Related Documentation

- **Core Workflow:** [FEATURE_BRANCH_WORKFLOW.md](FEATURE_BRANCH_WORKFLOW.md)
- **CLI Guide:** [CLI_WORKFLOW.md](CLI_WORKFLOW.md)
- **Web Guide:** [WEB_WORKFLOW.md](WEB_WORKFLOW.md)

---

**Last Updated:** 2025-12-31
**Maintained By:** Tuulbelt Core Team
