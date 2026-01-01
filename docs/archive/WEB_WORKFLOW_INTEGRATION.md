# Web Workflow Integration

## Overview

The Web workflow is **automatically integrated** into Claude Code's session lifecycle. You don't need to manually run setup commands - everything happens automatically.

## Automatic Integration Points

### 1. Session Start Hook

**What:** `.claude/hooks/on-session-start.sh`

**When:** Runs automatically when a new Claude Code session starts

**What it does:**
- Detects if running in Claude Code Web (`CLAUDE_CODE_REMOTE=true`)
- If Web: Runs `/web-setup` automatically
  - Configures git credentials
  - Initializes tracking file
  - Creates/resumes session for current meta branch
  - Shows current status
- If CLI: Does nothing (CLI doesn't need this)

**Output:**
```
🌐 Claude Code Web detected - initializing Web workflow...

🔐 Setting up git credentials...
✓ Credential helper configured

📍 Current meta branch: claude/analyze-repo-structure-AtEoX

✓ Resuming existing session for: claude/analyze-repo-structure-AtEoX
Tracked submodules:
  - tools/file-based-semaphore
  - tools/cli-progress-reporting

Session ready! 🚀
```

### 2. Resume Work Integration

**What:** `/resume-work` command enhancement

**When:** User runs `/resume-work` to resume from previous session

**What it does:**
- Reads handoff documents (as before)
- **NEW:** If in Web environment, shows session status
- Includes submodule PR status in handoff summary
- Shows pending changes that need attention

**Enhanced Handoff Summary:**
```markdown
## Resuming from Previous Session

**Last Session:** Documentation Audit Pass 5
**Status:** In Progress

### What Was Accomplished
- Created 7 feature branches for submodules
- Migrated commits to feature branches
- 3 submodules ready for PR

### Web Session Status

Meta Branch: claude/analyze-repo-structure-AtEoX
Session ID: 4f7e70b1-0ab1-4117-94bd-1e51b9078b2f

┌────────────────────────────────────┬─────────┬─────────┬──────────┐
│ Submodule                          │ Changes │ Commits │ PR       │
├────────────────────────────────────┼─────────┼─────────┼──────────┤
│ config-file-merger                 │ Yes     │ 1       │ -        │
│ cross-platform-path-normalizer     │ Yes     │ 1       │ -        │
│ structured-error-handler           │ Yes     │ 1       │ -        │
└────────────────────────────────────┴─────────┴─────────┴──────────┘

### What's Next (Priority Order)

1. **Create PRs for 3 submodules with changes**
   - Use `gh pr create` or GitHub UI
   - PRs will be from feature branch to main
```

### 3. Before Editing Submodules

**What:** `.claude/hooks/before-submodule-edit.sh` (helper)

**When:** Can be sourced before working on submodule files

**What it does:**
- Detects if file being edited is in a submodule
- Ensures submodule has a feature branch
- Checks out the feature branch automatically
- Updates tracking file

**Usage:**
```bash
# In custom commands/agents, source this before editing
source .claude/hooks/before-submodule-edit.sh
ensure_submodule_branch "tools/file-based-semaphore/README.md"
# Now safe to edit - you're on the feature branch
```

## How It Works Together

### Scenario 1: Starting Fresh Session

```
1. User opens Claude Code Web
2. Session start hook runs automatically
   - Detects Web environment
   - Runs web-setup
   - Shows current status
3. User sees session is ready
4. User starts working (or runs /resume-work)
```

### Scenario 2: Resuming Work

```
1. User runs /resume-work
2. Command reads handoff documents
3. Command detects Web environment
4. Command runs /web-status
5. Shows combined handoff + session status
6. User sees what needs attention
```

### Scenario 3: Working on Submodule

```
1. User asks to edit submodule file
2. (Optional) before-submodule-edit hook ensures feature branch
3. Changes are made on feature branch
4. Tracking file updated automatically
5. User commits and pushes (credentials already configured)
```

### Scenario 4: Pausing and Resuming

```
Session 1 (Web):
- Work on feature branch A
- Tracking file committed to git
- Session ends

Session 2 (Web, hours later):
- Session start hook runs
- Loads tracking file from git
- Resumes session A
- User continues where they left off
- No duplicate branches created
```

## What's Automatic vs Manual

### ✅ Automatic (No Action Needed)

- **Environment detection** - Knows if Web or CLI
- **Credential setup** - Configured on session start
- **Session initialization** - Created/resumed automatically
- **Tracking file** - Loaded from git, updated as you work
- **Feature branch checkout** - Handled by scripts
- **Status display** - Shown in resume-work

### 🔧 Manual (You Control)

- **Making changes** - You decide what to edit
- **Committing changes** - You run git commands
- **Creating PRs** - You use `gh pr create` or GitHub UI
- **Merging PRs** - You approve/merge on GitHub
- **Switching meta branches** - You checkout different branches

## Key Files

| File | Purpose | When Created |
|------|---------|--------------|
| `.claude/hooks/on-session-start.sh` | Auto-runs web-setup | On session start |
| `.claude/hooks/before-submodule-edit.sh` | Ensures feature branch | Before edits (optional) |
| `.claude/web-session-tracking.json` | Session state | First time in Web |
| `scripts/web/*.sh` | Web workflow scripts | Pre-installed |

## Troubleshooting

### "Session start hook didn't run"

- Hook only runs if `.claude/hooks/on-session-start.sh` is executable
- Check: `ls -la .claude/hooks/on-session-start.sh`
- Fix: `chmod +x .claude/hooks/on-session-start.sh`

### "Tracking file not found"

- First time in Web? Run `/web-setup` manually
- Not on a feature branch? Checkout meta feature branch first

### "Submodule not on feature branch"

- Run: `./scripts/web/manage-submodule-branch.sh tools/<submodule>`
- This creates and checks out the feature branch

### "Can't push to GitHub"

- Credentials not set up? Run: `./scripts/web/setup-credentials.sh`
- Check: `git config --get credential.helper`
- Should show: `!f() { echo "username=${GITHUB_USERNAME}"; echo "password=${GITHUB_TOKEN}"; }; f`

## Best Practices

1. **Let the hooks run** - Don't interrupt session start
2. **Check status frequently** - Run `/web-status` to see current state
3. **Commit tracking file** - Always commit `.claude/web-session-tracking.json` changes
4. **One meta branch = one session** - Each meta branch gets its own submodule branches
5. **Create PRs when ready** - Don't let changes accumulate

## Summary

**You don't need to manually run setup!** The integration handles it automatically:

- Session start → Web setup runs
- Resume work → Session status shown
- Edit submodule → Feature branch ensured (optional)
- All tracking → Updated automatically
- Session ends → State saved in git

Just start working, and the system takes care of the Web-specific workflow.
