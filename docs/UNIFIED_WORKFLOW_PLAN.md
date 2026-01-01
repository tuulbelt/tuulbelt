# Unified Workflow Implementation Plan

**Created:** 2025-12-31
**Status:** Implementation In Progress
**Goal:** Unified feature-branch workflow across Claude Code CLI and Web

**Testing Report:** See `docs/WORKFLOW_TEST_RESULTS.md` for comprehensive CLI and Web testing results across all phases.

---

## Implementation Progress

**Last Updated:** 2025-12-31

### Phase 1: Branch Protection (Universal) - ✅ COMPLETE
- [x] 1.1: Meta repo hook template ✅ `scripts/workflow/templates/meta-pre-commit-hook.sh`
- [x] 1.2: Submodule hook template ✅ `scripts/workflow/templates/submodule-pre-commit-hook.sh`
- [x] 1.3: Universal hook installer script ✅ `scripts/workflow/install-hooks.sh` (installed 11 hooks)
- [x] 1.4: Session start integration ✅ `.claude/hooks/on-session-start.sh` (lines 32-35)
- [x] Verification: All hooks tested ✅ (meta + all 10 submodules block commits to main)

### Phase 2: CLI Workspace Commands - ✅ COMPLETE
- [x] 2.1: Workspace tracking file structure ✅ `.claude/schemas/cli-workspace-tracking.json`, `.claude/schemas/web-session-tracking.json`
- [x] 2.2: /work-init command ✅ `.claude/commands/work-init.md`, `scripts/workflow/init-workspace.sh`
- [x] 2.3: CLI worktree creation script ✅ `scripts/cli/create-worktree.sh`, `scripts/cli/create-branch.sh`, `scripts/cli/update-cli-tracking.sh`
- [x] 2.4: /work-status command ✅ `.claude/commands/work-status.md`, `scripts/workflow/show-status.sh`, `scripts/cli/show-cli-status.sh`
- [x] 2.5: /work-pr command ✅ `.claude/commands/work-pr.md`, `scripts/workflow/create-prs.sh`, `scripts/cli/create-cli-prs.sh`
- [x] 2.6: /work-cleanup command ✅ `.claude/commands/work-cleanup.md`, `scripts/workflow/cleanup-workspace.sh`, `scripts/cli/cleanup-cli-workspace.sh`
- [x] 2.7: Tracking file regeneration script ✅ `scripts/workflow/regenerate-tracking.sh`

### Phase 3: Environment-Aware Commands - ✅ COMPLETE
- [x] 3.1: Unified command interface ✅ All wrapper scripts detect environment and delegate
  - Wrapper scripts in `scripts/workflow/` already implemented environment detection in Phase 2
  - Created missing Web scripts: `create-session-branches.sh`, `show-web-status.sh`, `create-web-prs.sh`, `cleanup-web-session.sh`
  - Web scripts use branch-based workflow (no worktrees)
  - All scripts use `$CLAUDE_CODE_REMOTE` environment variable for detection
- [x] 3.2: Update push.sh for feature branches ✅ Detects current branch instead of defaulting to main
  - Updated `scripts/push.sh` line 10: `BRANCH="${3:-$(git -C "${REPO_PATH}" rev-parse --abbrev-ref HEAD)}"`
  - Now automatically detects and pushes current branch of the repository
  - Falls back to "main" only if branch detection fails

### Phase 4: Session Lifecycle Hooks - ✅ COMPLETE
- [x] 4.1: Enhanced on-session-start.sh ✅ Updated with environment-specific setup
  - Web: Calls `resume-session.sh` to restore session state
  - CLI: Shows workspace status via `show-cli-status.sh`
- [x] 4.2: Enhanced on-session-end.sh ✅ Created session cleanup hook
  - Auto-commits tracking file changes
  - Web: Calls `save-session.sh` to persist state
  - CLI: No cleanup needed (worktrees persist)

### Phase 5: Documentation - ✅ COMPLETE
- [x] 5.1: FEATURE_BRANCH_WORKFLOW.md ✅ Created (core principles, mental model, best practices)
- [x] 5.2: CLI_WORKFLOW.md ✅ Created (worktree architecture, commands, examples)
- [x] 5.3: WEB_WORKFLOW.md ✅ Created (session architecture, commands, examples)
- [x] 5.4: WORKFLOW_TROUBLESHOOTING.md ✅ Created (edge cases, recovery procedures)
- [x] 5.5: WEB_WORKFLOW_INTEGRATION.md archived ✅ Moved to docs/archive/
- [x] 5.6: CLAUDE.md updated ✅ Added workflow documentation references

### Phase 6: Testing & Validation - ✅ COMPLETE
- [x] Test matrix execution ✅ All 6 CLI tests passed (100% pass rate)
- [x] Edge case validation ✅ Branch protection verified, worktree lifecycle validated
- **Note:** CLI testing complete. Web testing requires Web environment (documented in WORKFLOW_TEST_RESULTS.md)

---

## Implementation Best Practices

**Lessons learned during Phase 1 and Phase 2 implementation:**

### Best Practice 1: Directory Context in Loops (CRITICAL)

**Problem:** Bash pipes (`|`) create subshells where `cd` commands don't persist between iterations. This causes scripts to run commands from the wrong directory.

**Example Bug (Fixed in Phase 2):**
```bash
# WRONG - cd doesn't persist in pipe subshell
git submodule foreach --quiet 'echo $path' | while read -r submodule; do
  cd "$REPO_ROOT/$submodule"  # Works for first iteration only
  git rev-parse --abbrev-ref HEAD  # Subsequent iterations run from wrong directory
done
```

**Solution Pattern:**
```bash
# CORRECT - Use process substitution and explicit directory management
REPO_ROOT="$(git rev-parse --show-toplevel)"  # Define base directory
cd "$REPO_ROOT"  # Start from known location

while read -r submodule; do
  echo "Processing: $submodule"

  # Change to target directory
  cd "$REPO_ROOT/$submodule"

  # Run commands in target directory
  BRANCH=$(git rev-parse --abbrev-ref HEAD)

  # CRITICAL: Return to base directory for next iteration
  cd "$REPO_ROOT"

done < <(git submodule foreach --quiet 'echo $path')
```

**Key Principles:**
1. **Define base directory once** at script start: `REPO_ROOT="$(git rev-parse --show-toplevel)"`
2. **Use process substitution** instead of pipes: `< <(command)` not `command | while`
3. **Return to base directory** after each iteration: `cd "$REPO_ROOT"`
4. **Return in all code paths**: Add `cd "$REPO_ROOT"` before `continue`, after function calls, etc.

**Apply This Pattern To:**
- `create-cli-prs.sh` (Phase 2.5) ✅ Fixed
- `cleanup-cli-workspace.sh` (Phase 2.6) - if iterating submodules
- `show-cli-status.sh` (Phase 2.4) - when checking submodule state
- `regenerate-tracking.sh` (Phase 2.7) - when scanning worktrees
- Any future script that iterates through git repositories

### Best Practice 2: Resilient Error Handling

**Problem:** Scripts using `set -e` exit immediately on any error, potentially leaving state inconsistent.

**Solution Pattern:**
```bash
# Try primary approach, fall back on failure
if ! git worktree remove "$WORKTREE_DIR" --force; then
  echo "  ⚠ Primary method failed, attempting fallback..."
  rm -rf "$WORKTREE_DIR"
  git worktree prune
  echo "  ✓ Cleaned up via fallback"
fi
```

**Apply This Pattern To:**
- File operations that might fail (worktree removal, branch deletion)
- Network operations (git push, gh CLI commands)
- Cleanup operations (should always succeed, even if partially broken)

### Best Practice 3: Detect and Handle Stale State

**Problem:** Git state can become inconsistent (worktree deleted manually, tracking file references nonexistent paths).

**Solution Pattern:**
```bash
# Check if directory exists but git still tracks it (stale entry)
if [ ! -d "$WORKTREE_DIR" ] && git worktree list | grep -q "$WORKTREE_DIR"; then
  echo "Pruning stale worktree entry..."
  git worktree prune
fi
```

**Apply This Pattern To:**
- Worktree operations
- Branch reference checks
- Tracking file validation

---

## Table of Contents

1. [Overview](#overview)
2. [Core Principles](#core-principles)
3. [Architecture](#architecture)
4. [Implementation Phases](#implementation-phases)
5. [Implementation Best Practices](#implementation-best-practices)
6. [Session Lifecycle](#session-lifecycle)
7. [Edge Cases](#edge-cases)
8. [Success Criteria](#success-criteria)
9. [Migration Path](#migration-path)

---

## Overview

### Problem Statement

**Current State:**
- Web has session-based feature branch workflow (documented in WEB_WORKFLOW_INTEGRATION.md)
- CLI has no structured workflow (users can commit directly to main)
- No branch protection on main (meta or submodules)
- Inconsistent mental models between environments
- Risk of accidental commits to main

**Desired State:**
- Unified feature-branch workflow (same principles, different mechanics)
- Protected main branches (meta + all submodules)
- All changes flow through feature branches → PRs
- Clear session lifecycle with automatic state management
- Environment-aware commands (detect CLI vs Web, adapt behavior)

### Goals

1. **Safety:** Prevent accidental commits to main
2. **Consistency:** Same mental model across CLI and Web
3. **Efficiency:** Leverage each environment's strengths (worktrees for CLI, branches for Web)
4. **Traceability:** All changes reviewed via PRs
5. **Resumability:** State persists across session boundaries

---

## Core Principles

These principles apply to **both** CLI and Web environments:

### 1. Never Commit to Main

**Rule:** `main` branch is read-only. All changes via feature branches.

**Enforcement:**
- Git hooks reject commits to main
- Commands detect main branch and error
- Documentation emphasizes PR workflow

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
```

### 3. Track Workspace State

**Rule:** System knows what you're working on, what's changed, what needs PR.

**Tracking Mechanisms:**
- CLI: `.claude/cli-workspace-tracking.json` + worktree directories
- Web: `.claude/web-session-tracking.json` + branch refs

### 4. PR-Based Workflow

**Rule:** All merges to main via GitHub PRs (reviewable, auditable).

**Process:**
1. Create feature branch
2. Make changes, commit
3. Push feature branch
4. Create PR (meta repo and/or submodules)
5. Review, approve, merge
6. Cleanup feature branch

### 5. Environment Detection

**Rule:** Commands adapt behavior based on environment.

**Detection:**
```bash
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment
else
  # CLI environment
fi
```

---

## Architecture

### Environment Comparison

| Aspect | CLI | Web |
|--------|-----|-----|
| **Filesystem** | Persistent across sessions | Ephemeral (destroyed each session) |
| **Worktrees** | Available and persistent | Not practical (would need recreation) |
| **Best Approach** | Worktree-based (parallel work) | Branch-based (lightweight) |
| **State Storage** | Worktree paths on disk + tracking file | Branches in git + tracking file |
| **Resume Cost** | Free (worktrees exist) | Free (checkout branch from git) |
| **Parallel Features** | Multiple worktrees simultaneously | Serial (checkout different branches) |

### Unified Mental Model

**Same Outcome, Different Mechanics:**

```
Mental Model (Universal):
┌─────────────────────────────────────┐
│ Feature Isolation via Branches     │
│ ↓                                   │
│ Changes Committed to Feature Branch │
│ ↓                                   │
│ PR Created (Review Required)        │
│ ↓                                   │
│ Merge to Main (Protected)           │
└─────────────────────────────────────┘

CLI Implementation:
  Workspace = Worktree directory at .claude/worktrees/<feature>/
  Isolation = Separate working directory per feature
  Parallel = Multiple worktrees active simultaneously
  Resume = Worktrees persist on disk

Web Implementation:
  Workspace = Checked out feature branch
  Isolation = Different branch, same working directory
  Serial = Checkout different branch to switch features
  Resume = Branches persist in git (checkout on session start)
```

### File Structure

```
tuulbelt/                                    # Meta repository
├── .git/                                    # Git metadata
│   └── hooks/
│       ├── pre-commit                       # Reject commits to main (universal)
│       └── prepare-commit-msg               # Add Co-authored-by
│
├── .claude/
│   ├── hooks/
│   │   ├── on-session-start.sh              # Environment detection, setup
│   │   ├── on-session-end.sh                # Cleanup, state save
│   │   ├── pre-work-init.sh                 # Validate before workspace creation
│   │   └── post-work-cleanup.sh             # Validate after workspace removal
│   │
│   ├── commands/
│   │   ├── work-init.md                     # Create workspace (CLI worktree or Web branch)
│   │   ├── work-status.md                   # Show current workspace state
│   │   ├── work-pr.md                       # Create PRs for changed repos
│   │   ├── work-cleanup.md                  # Remove workspace after merge
│   │   └── work-switch.md                   # Switch between workspaces
│   │
│   ├── cli-workspace-tracking.json          # CLI: Worktree tracking
│   ├── web-session-tracking.json            # Web: Session tracking
│   │
│   └── worktrees/                           # CLI only: Worktree directories
│       ├── feature-component-prop-validator/
│       └── fix-docs-typo/
│
├── tools/                                   # Submodules (git)
│   ├── test-flakiness-detector/
│   │   └── .git/hooks/
│   │       └── pre-commit                   # Same protection (installed automatically)
│   └── ...
│
└── scripts/
    ├── workflow/
    │   ├── init-workspace.sh                # Create workspace (environment-aware)
    │   ├── show-status.sh                   # Show state (environment-aware)
    │   ├── create-prs.sh                    # Create PRs (batch)
    │   ├── cleanup-workspace.sh             # Remove workspace
    │   └── install-submodule-hooks.sh       # Install hooks in all submodules
    │
    ├── cli/
    │   ├── create-worktree.sh               # CLI-specific: Create worktree
    │   ├── remove-worktree.sh               # CLI-specific: Remove worktree
    │   └── update-cli-tracking.sh           # CLI-specific: Update tracking file
    │
    └── web/
        ├── create-session-branches.sh       # Web-specific: Create session branches
        ├── cleanup-session-branches.sh      # Web-specific: Cleanup after merge
        └── update-web-tracking.sh           # Web-specific: Update tracking file
```

---

## Implementation Phases

### Phase 0: Prerequisites ✅ (Already Complete)

**Goal:** Verify current state before implementing workflow.

**Tasks:**
- [x] Web workflow documented (WEB_WORKFLOW_INTEGRATION.md exists)
- [x] Web session tracking file structure defined
- [x] Scripts directory structure in place
- [x] Git submodules working correctly

**Verification:**
```bash
ls -la .claude/web-session-tracking.json  # Exists
ls -la scripts/web/*.sh                   # Multiple scripts
git submodule status                      # All submodules initialized
```

---

### Phase 1: Branch Protection (Universal)

**Goal:** Prevent commits to main in meta repo and all submodules.

**Duration:** ~1 hour
**Priority:** HIGH (Safety feature)

#### 1.1: Meta Repo Hook Template

**File:** `scripts/workflow/templates/meta-pre-commit-hook.sh`

**Template:**
```bash
#!/bin/bash
# Reject commits to main branch (meta repo protection)

BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$BRANCH" = "main" ]; then
  echo ""
  echo "❌ ERROR: Direct commits to main branch are not allowed"
  echo ""
  echo "Please create a feature workspace:"
  echo "  CLI: /work-init <feature-name>"
  echo "  Web: Automatically handled by session hooks"
  echo ""
  echo "Example:"
  echo "  /work-init feature/my-feature"
  echo ""
  exit 1
fi

exit 0
```

**Note:** Template stored in version control, installed on session start (not in .git/)

#### 1.2: Submodule Hook Template

**File:** `scripts/workflow/templates/submodule-pre-commit-hook.sh`

**Template:**
```bash
#!/bin/bash
# Reject commits to main branch (submodule protection)

BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$BRANCH" = "main" ]; then
  echo ""
  echo "❌ ERROR: Direct commits to main branch are not allowed in submodule"
  echo ""
  echo "This submodule is managed via feature branches."
  echo "The meta repository workflow handles submodule branching automatically."
  echo ""
  exit 1
fi

exit 0
```

#### 1.3: Hook Installer Script (Universal)

**File:** `scripts/workflow/install-hooks.sh`

**Implementation:**
```bash
#!/bin/bash
# Install pre-commit hooks in meta repo and all submodules

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel)"

META_HOOK_TEMPLATE="$SCRIPT_DIR/templates/meta-pre-commit-hook.sh"
SUBMODULE_HOOK_TEMPLATE="$SCRIPT_DIR/templates/submodule-pre-commit-hook.sh"

# Install meta repo hook
echo "Installing pre-commit hook in meta repo..."
META_HOOK_PATH="$REPO_ROOT/.git/hooks/pre-commit"

if [ ! -d "$REPO_ROOT/.git/hooks" ]; then
  mkdir -p "$REPO_ROOT/.git/hooks"
fi

cp "$META_HOOK_TEMPLATE" "$META_HOOK_PATH"
chmod +x "$META_HOOK_PATH"
echo "✓ Installed hook in meta repo"
echo ""

# Install submodule hooks
SUBMODULES=$(git submodule status | awk '{print $2}')

if [ -z "$SUBMODULES" ]; then
  echo "No submodules found"
  exit 0
fi

echo "Installing pre-commit hooks in submodules..."
echo ""

for submodule in $SUBMODULES; do
  HOOK_PATH="$submodule/.git/hooks/pre-commit"

  if [ ! -d "$submodule/.git/hooks" ]; then
    mkdir -p "$submodule/.git/hooks"
  fi

  cp "$SUBMODULE_HOOK_TEMPLATE" "$HOOK_PATH"
  chmod +x "$HOOK_PATH"

  echo "✓ Installed hook in $submodule"
done

echo ""
echo "✓ All hooks installed (meta + submodules)"
```

**Usage:**
```bash
./scripts/workflow/install-hooks.sh
```

#### 1.4: Session Start Integration

**File:** `.claude/hooks/on-session-start.sh` (update)

**Add to existing hook:**
```bash
# Install hooks in meta repo and all submodules (runs on every session start)
if [ -f "./scripts/workflow/install-hooks.sh" ]; then
  ./scripts/workflow/install-hooks.sh > /dev/null 2>&1
fi
```

**Verification:**
```bash
# Test in meta repo
git checkout main
echo "test" > test.txt
git add test.txt
git commit -m "test"  # Should be rejected

# Test in submodule
cd tools/test-flakiness-detector
git checkout main
echo "test" > test2.txt
git add test2.txt
git commit -m "test"  # Should be rejected

# Cleanup test files
rm ../test.txt tools/test-flakiness-detector/test2.txt
```

---

### Phase 2: CLI Workspace Commands

**Goal:** Create CLI workflow commands for worktree-based development.

**Duration:** ~4 hours
**Priority:** HIGH (Core CLI functionality)

#### 2.1: Workspace Tracking File Structure

**File:** `.claude/cli-workspace-tracking.json` (create on first use)

**Schema:**
```json
{
  "version": "1.0",
  "environment": "cli",
  "worktrees": {
    ".claude/worktrees/feature-component-prop-validator": {
      "meta_branch": "feature/component-prop-validator",
      "created_at": "2025-12-31T18:00:00Z",
      "updated_at": "2025-12-31T19:30:00Z",
      "status": "active",
      "submodules": {
        "tools/test-flakiness-detector": {
          "branch": "feature/component-prop-validator",
          "created_at": "2025-12-31T18:05:00Z",
          "has_changes": true,
          "commits_count": 3,
          "last_commit_sha": "abc123...",
          "pr_url": null,
          "pr_number": null,
          "pr_state": null,
          "pr_merged": false
        }
      }
    }
  }
}
```

#### 2.2: /work-init Command

**File:** `.claude/commands/work-init.md`

**Command Spec:**
```markdown
# /work-init Command

Create a new workspace for feature development.

## Usage

/work-init <feature-name> [--no-worktree]

## Parameters

- feature-name: Branch name (e.g., feature/my-feature)
- --no-worktree: (Optional) Use branch instead of worktree (CLI only)

## Behavior

**CLI (default):**
- Creates worktree at .claude/worktrees/<sanitized-name>/
- Creates feature branch in worktree
- Initializes tracking file entry
- Outputs workspace path

**CLI (--no-worktree):**
- Creates feature branch in main working directory
- Checks out branch
- Initializes tracking file entry

**Web:**
- Creates feature branch
- Checks out branch
- Initializes session tracking entry

## Examples

/work-init feature/component-prop-validator
/work-init fix/docs-typo --no-worktree
```

**Implementation:** `scripts/workflow/init-workspace.sh`

```bash
#!/bin/bash
# Initialize workspace (environment-aware)

set -e

# Show help
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
  cat <<EOF
/work-init - Create a new workspace for feature development

Usage:
  /work-init <feature-name> [--no-worktree]
  /work-init --help

Parameters:
  feature-name       Branch name (e.g., feature/my-feature)
  --no-worktree      (Optional, CLI only) Use branch instead of worktree
  --help, -h         Show this help message

Examples:
  /work-init feature/component-prop-validator
  /work-init fix/docs-typo --no-worktree

Branch Naming:
  Format: <type>/<description>
  Types: feature, fix, chore, refactor

Environment Behavior:
  CLI (default):    Creates worktree at .claude/worktrees/
  CLI (--no-worktree): Creates branch in main working directory
  Web:              Creates feature branch (no worktrees)

EOF
  exit 0
fi

FEATURE_NAME="$1"
NO_WORKTREE="${2:-}"

if [ -z "$FEATURE_NAME" ]; then
  echo "Usage: /work-init <feature-name> [--no-worktree]"
  echo "Run '/work-init --help' for more information"
  exit 1
fi

# Validate branch name format
if ! echo "$FEATURE_NAME" | grep -qE '^(feature|fix|chore|refactor)/[a-z0-9-]+$'; then
  echo "❌ Invalid branch name format"
  echo "Expected: <type>/<description>"
  echo "Types: feature, fix, chore, refactor"
  echo "Example: feature/component-prop-validator"
  exit 1
fi

# Detect environment
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment: Always use branch-based
  echo "🌐 Web environment detected - creating feature branch..."
  ./scripts/web/create-session-branches.sh "$FEATURE_NAME"
else
  # CLI environment: Worktree or branch
  if [ "$NO_WORKTREE" = "--no-worktree" ]; then
    echo "💻 CLI environment - creating feature branch (no worktree)..."
    ./scripts/cli/create-branch.sh "$FEATURE_NAME"
  else
    echo "💻 CLI environment - creating worktree..."
    ./scripts/cli/create-worktree.sh "$FEATURE_NAME"
  fi
fi
```

#### 2.3: CLI Worktree Creation Script

**File:** `scripts/cli/create-worktree.sh`

**Implementation:**
```bash
#!/bin/bash
# Create worktree for feature development (CLI only)

set -e

FEATURE_NAME="$1"
REPO_ROOT="$(git rev-parse --show-toplevel)"
SANITIZED_NAME=$(echo "$FEATURE_NAME" | tr '/' '-')
WORKTREE_DIR="$REPO_ROOT/.claude/worktrees/$SANITIZED_NAME"

# Check if worktree already exists
if [ -d "$WORKTREE_DIR" ]; then
  echo "❌ Worktree already exists: $WORKTREE_DIR"
  echo "Use /work-switch to resume or /work-cleanup to remove"
  exit 1
fi

# Create worktree directory
mkdir -p "$REPO_ROOT/.claude/worktrees"

# Check if branch already exists (local or remote)
echo "Checking branch status..."
if git show-ref --verify --quiet "refs/heads/$FEATURE_NAME"; then
  # Branch exists locally - use it
  echo "Branch exists locally, checking out in worktree..."
  git worktree add "$WORKTREE_DIR" "$FEATURE_NAME"
elif git show-ref --verify --quiet "refs/remotes/origin/$FEATURE_NAME"; then
  # Branch exists remotely - fetch and checkout
  echo "Branch exists remotely, fetching and checking out..."
  git fetch origin "$FEATURE_NAME:$FEATURE_NAME"
  git worktree add "$WORKTREE_DIR" "$FEATURE_NAME"
else
  # New branch - create it
  echo "Creating new branch in worktree..."
  git worktree add -b "$FEATURE_NAME" "$WORKTREE_DIR"
fi

# Initialize submodules in worktree
echo "Initializing submodules..."
cd "$WORKTREE_DIR"
git submodule update --init --recursive

# Install hooks (meta + submodules)
echo "Installing hooks..."
"$REPO_ROOT/scripts/workflow/install-hooks.sh" > /dev/null 2>&1

# Update tracking file
echo "Updating tracking file..."
"$REPO_ROOT/scripts/cli/update-cli-tracking.sh" add "$WORKTREE_DIR" "$FEATURE_NAME"

echo ""
echo "✓ Worktree created successfully!"
echo ""
echo "📁 Worktree path: $WORKTREE_DIR"
echo "🌿 Branch: $FEATURE_NAME"
echo ""
echo "Next steps:"
echo "  cd $WORKTREE_DIR"
echo "  # Make your changes"
echo "  git add ."
echo "  git commit -m 'feat: implement feature'"
echo "  /work-pr  # Create PRs"
echo ""
```

#### 2.4: /work-status Command

**File:** `.claude/commands/work-status.md`

**Command Spec:**
```markdown
# /work-status Command

Display current workspace state.

## Usage

/work-status

## Output

Shows:
- Current environment (CLI or Web)
- Active workspaces/sessions
- Branch names
- Changed files count
- Commits count
- PR status

## Examples

CLI Output:
┌──────────────────────────────────────────────────────────────┐
│ CLI Workspace Status                                         │
└──────────────────────────────────────────────────────────────┘

Worktree: .claude/worktrees/feature-component-prop-validator
Meta Branch: feature/component-prop-validator (3 commits)
Status: active

┌────────────────────────────────┬─────────┬─────────┬──────┐
│ Submodule                      │ Changes │ Commits │ PR   │
├────────────────────────────────┼─────────┼─────────┼──────┤
│ test-flakiness-detector        │ Yes     │ 2       │ #123 │
│ cli-progress-reporting         │ No      │ 0       │ -    │
└────────────────────────────────┴─────────┴─────────┴──────┘

Web Output:
(Same format as CLI, adapted for Web session tracking)
```

**Implementation:** `scripts/workflow/show-status.sh`

```bash
#!/bin/bash
# Show workspace status (environment-aware)

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
  cat <<EOF
/work-status - Display current workspace state

Usage:
  /work-status
  /work-status --help

Shows:
  - Current environment (CLI or Web)
  - Active workspaces/sessions
  - Branch names
  - Changed files count
  - Commits count
  - PR status

EOF
  exit 0
fi

if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web environment
  ./scripts/web/show-status.sh
else
  # CLI environment
  ./scripts/cli/show-cli-status.sh
fi
```

#### 2.5: /work-pr Command

**File:** `.claude/commands/work-pr.md`

**Command Spec:**
```markdown
# /work-pr Command

Create pull requests for changed repositories.

## Usage

/work-pr [--meta] [--submodules]

## Flags

- --meta: Create PR for meta repo only
- --submodules: Create PRs for submodules only
- (no flags): Create PRs for both meta and submodules

## Behavior

1. Detects current workspace (worktree or branch)
2. Finds repos with uncommitted/unpushed changes
3. Pushes branches to remote
4. Creates PRs via gh CLI
5. Updates tracking file with PR URLs

## Examples

/work-pr                  # Create PRs for all changed repos
/work-pr --meta           # Meta repo only
/work-pr --submodules     # Submodules only
```

**Implementation:** `scripts/workflow/create-prs.sh`

```bash
#!/bin/bash
# Create pull requests for changed repositories

set -e

# Show help
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
  cat <<EOF
/work-pr - Create pull requests for changed repositories

Usage:
  /work-pr [--meta] [--submodules]
  /work-pr --help

Flags:
  --meta         Create PR for meta repo only
  --submodules   Create PRs for submodules only
  (no flags)     Create PRs for both meta and submodules

Behavior:
  1. Detects current workspace (worktree or branch)
  2. Finds repos with uncommitted/unpushed changes
  3. Pushes branches to remote
  4. Creates PRs via gh CLI
  5. Updates tracking file with PR URLs

Examples:
  /work-pr                  # Create PRs for all changed repos
  /work-pr --meta           # Meta repo only
  /work-pr --submodules     # Submodules only

Requirements:
  - gh CLI installed and authenticated
  - Changes committed to feature branch
  - Remote repository exists

EOF
  exit 0
fi

# Implementation continues...
# (Full implementation in actual script file)
```

#### 2.6: /work-cleanup Command

**File:** `.claude/commands/work-cleanup.md`

**Command Spec:**
```markdown
# /work-cleanup Command

Remove workspace after PR merge.

## Usage

/work-cleanup <feature-name>

## Behavior

**CLI (worktree):**
1. Verify PRs merged (or --force flag)
2. Remove worktree directory
3. Delete local branches
4. Delete remote branches
5. Update tracking file

**CLI (branch):**
1. Verify PRs merged (or --force flag)
2. Checkout main
3. Delete local branch
4. Delete remote branch
5. Update tracking file

**Web:**
1. Verify PRs merged (or --force flag)
2. Checkout main
3. Delete local branches (meta + submodules)
4. Delete remote branches
5. Update tracking file

## Examples

/work-cleanup feature/component-prop-validator
/work-cleanup fix/docs-typo --force
```

**Implementation:** `scripts/workflow/cleanup-workspace.sh`

```bash
#!/bin/bash
# Remove workspace after PR merge

set -e

# Show help
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
  cat <<EOF
/work-cleanup - Remove workspace after PR merge

Usage:
  /work-cleanup <feature-name> [--force]
  /work-cleanup --help

Parameters:
  feature-name   Branch name to cleanup (e.g., feature/my-feature)
  --force        Skip PR merge verification (dangerous)

Behavior:
  CLI (worktree):
    1. Verify PRs merged (or --force flag)
    2. Remove worktree directory
    3. Delete local branches
    4. Delete remote branches
    5. Update tracking file

  CLI (branch):
    1. Verify PRs merged (or --force flag)
    2. Checkout main
    3. Delete local branch
    4. Delete remote branch
    5. Update tracking file

  Web:
    1. Verify PRs merged (or --force flag)
    2. Checkout main
    3. Delete local branches (meta + submodules)
    4. Delete remote branches
    5. Update tracking file

Examples:
  /work-cleanup feature/component-prop-validator
  /work-cleanup fix/docs-typo --force

Warnings:
  - Using --force may lose work if PRs are rejected
  - Always verify PRs are merged before cleanup
  - Backup important changes before using --force

EOF
  exit 0
fi

# Implementation continues...
# (Full implementation in actual script file)
```

#### 2.7: Tracking File Regeneration Script

**File:** `scripts/workflow/regenerate-tracking.sh`

**Purpose:** Regenerate tracking file from current git state (recovery from corruption)

**Implementation:**
```bash
#!/bin/bash
# Regenerate tracking file from current git state

set -e

REPO_ROOT="$(git rev-parse --show-toplevel)"

# Detect environment
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  TRACKING_FILE="$REPO_ROOT/.claude/web-session-tracking.json"
  ENVIRONMENT="web"
else
  TRACKING_FILE="$REPO_ROOT/.claude/cli-workspace-tracking.json"
  ENVIRONMENT="cli"
fi

echo "Regenerating $ENVIRONMENT tracking file..."

# Create backup
if [ -f "$TRACKING_FILE" ]; then
  BACKUP="$TRACKING_FILE.backup.$(date +%Y%m%d-%H%M%S)"
  cp "$TRACKING_FILE" "$BACKUP"
  echo "✓ Backed up existing file to: $BACKUP"
fi

if [ "$ENVIRONMENT" = "cli" ]; then
  # CLI: Scan for worktrees
  WORKTREES_DIR="$REPO_ROOT/.claude/worktrees"

  cat > "$TRACKING_FILE" <<EOF
{
  "version": "1.0",
  "environment": "cli",
  "worktrees": {
EOF

  FIRST=true
  if [ -d "$WORKTREES_DIR" ]; then
    for worktree_path in "$WORKTREES_DIR"/*; do
      if [ -d "$worktree_path" ]; then
        cd "$worktree_path"
        BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

        if [ "$FIRST" = false ]; then
          echo "," >> "$TRACKING_FILE"
        fi
        FIRST=false

        cat >> "$TRACKING_FILE" <<EOF
    "$worktree_path": {
      "meta_branch": "$BRANCH",
      "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
      "updated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
      "status": "active",
      "submodules": {}
    }
EOF
      fi
    done
  fi

  cat >> "$TRACKING_FILE" <<EOF

  }
}
EOF

else
  # Web: Use current branch as session
  CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
  SESSION_ID=$(uuidgen 2>/dev/null || echo "$(date +%s)-$$")

  cat > "$TRACKING_FILE" <<EOF
{
  "version": "1.0",
  "sessions": {
    "$CURRENT_BRANCH": {
      "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
      "updated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
      "session_id": "$SESSION_ID",
      "environment": "web",
      "status": "active",
      "submodules": {}
    }
  }
}
EOF

fi

echo "✓ Tracking file regenerated"
echo "  File: $TRACKING_FILE"
echo "  Review the file and commit if correct"
```

**Usage:**
```bash
# If tracking file is corrupted
./scripts/workflow/regenerate-tracking.sh

# Review generated file
cat .claude/cli-workspace-tracking.json  # or web-session-tracking.json

# Commit if correct
git add .claude/*.json
git commit -m "chore: regenerate tracking file"
```

---

### Phase 3: Environment-Aware Commands (Unified)

**Goal:** Create unified commands that work in both CLI and Web.

**Duration:** ~2 hours
**Priority:** MEDIUM (Improves UX consistency)

#### 3.1: Unified Command Interface

**Commands to update:**
- `/work-init` → Detects environment, delegates
- `/work-status` → Detects environment, delegates
- `/work-pr` → Works in both (same gh CLI)
- `/work-cleanup` → Detects environment, delegates

**Pattern:**
```bash
#!/bin/bash
# Unified command entry point

if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web-specific implementation
  ./scripts/web/web-<command>.sh "$@"
else
  # CLI-specific implementation
  ./scripts/cli/cli-<command>.sh "$@"
fi
```

#### 3.2: Update push.sh for Feature Branches

**File:** `scripts/push.sh` (update)

**Current Issue:** Hardcoded to push `main` branch

**Fix:**
```bash
# Detect current branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Push current branch (not hardcoded main)
git push -u origin "$BRANCH"
```

---

### Phase 4: Session Lifecycle Hooks

**Goal:** Automate workspace setup/teardown on session boundaries.

**Duration:** ~2 hours
**Priority:** MEDIUM (Improves automation)

#### 4.1: Enhanced on-session-start.sh

**File:** `.claude/hooks/on-session-start.sh` (update)

**Additions:**
```bash
# Install submodule hooks (universal)
./scripts/workflow/install-submodule-hooks.sh > /dev/null 2>&1

# Environment-specific setup
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web: Resume session
  ./scripts/web/resume-session.sh
else
  # CLI: Show workspace status
  echo "💻 CLI environment detected"
  if [ -f ".claude/cli-workspace-tracking.json" ]; then
    echo "Active workspaces:"
    ./scripts/cli/show-cli-status.sh
  fi
fi
```

#### 4.2: Enhanced on-session-end.sh

**File:** `.claude/hooks/on-session-end.sh` (create)

**Implementation:**
```bash
#!/bin/bash
# Session end cleanup

# Commit tracking file if changed
if [ -f ".claude/cli-workspace-tracking.json" ]; then
  git add .claude/cli-workspace-tracking.json
  git commit -m "chore: update CLI workspace tracking" || true
fi

if [ -f ".claude/web-session-tracking.json" ]; then
  git add .claude/web-session-tracking.json
  git commit -m "chore: update Web session tracking" || true
fi

# Environment-specific cleanup
if [ "$CLAUDE_CODE_REMOTE" = "true" ]; then
  # Web: Save session state
  ./scripts/web/save-session.sh
else
  # CLI: No cleanup needed (worktrees persist)
  echo "✓ Session ended (worktrees preserved)"
fi
```

---

### Phase 5: Documentation

**Goal:** Comprehensive, streamlined documentation.

**Duration:** ~3 hours
**Priority:** HIGH (Critical for adoption)

#### 5.1: Document Structure

**New Files:**
```
docs/
├── FEATURE_BRANCH_WORKFLOW.md      # Core principles (universal)
├── CLI_WORKFLOW.md                 # CLI implementation guide
├── WEB_WORKFLOW.md                 # Web implementation guide (simplified)
└── WORKFLOW_TROUBLESHOOTING.md     # Common issues, solutions
```

**Archive:**
```
docs/archive/
└── WEB_WORKFLOW_INTEGRATION.md     # Original (superseded)
```

#### 5.2: FEATURE_BRANCH_WORKFLOW.md

**Sections:**
1. Overview (why feature branches?)
2. Core Principles (universal rules)
3. Mental Model (unified approach)
4. Environment Comparison (CLI vs Web)
5. Common Commands (cross-reference)
6. Best Practices
7. Troubleshooting (link to dedicated doc)

#### 5.3: CLI_WORKFLOW.md

**Sections:**
1. Overview (CLI-specific benefits)
2. Worktree Architecture (why worktrees?)
3. Command Reference (/work-init, /work-status, etc.)
4. Session Lifecycle (start, work, end)
5. Parallel Development (multiple worktrees)
6. Examples (complete workflows)

#### 5.4: WEB_WORKFLOW.md

**Sections:**
1. Overview (Web constraints, why branches?)
2. Session Architecture (ephemeral filesystem)
3. Command Reference (same commands, different behavior)
4. Session Lifecycle (start, work, end, resume)
5. Serial Development (branch switching)
6. Examples (complete workflows)

#### 5.5: Update CLAUDE.md

**Add to Essential References:**
```markdown
**Development Workflow:**
- @docs/FEATURE_BRANCH_WORKFLOW.md - **Feature branch principles (universal)**
- @docs/CLI_WORKFLOW.md - **CLI worktree-based development**
- @docs/WEB_WORKFLOW.md - **Web session-based development**
- @docs/WORKFLOW_TROUBLESHOOTING.md - **Common issues and solutions**
```

---

### Phase 6: Testing & Validation

**Goal:** Verify all components work correctly.

**Duration:** ~2 hours
**Priority:** HIGH (Quality assurance)

#### 6.1: Test Matrix

| Scenario | CLI | Web | Expected Outcome |
|----------|-----|-----|------------------|
| Commit to main (meta) | ❌ Hook blocks | ❌ Hook blocks | Error message shown |
| Commit to main (submodule) | ❌ Hook blocks | ❌ Hook blocks | Error message shown |
| /work-init creates worktree | ✅ Creates | N/A (branches) | Worktree directory exists |
| /work-init creates branch | N/A (worktrees) | ✅ Creates | Branch created, checked out |
| /work-status shows active workspace | ✅ Shows worktree | ✅ Shows session | Table displayed |
| /work-pr creates PRs | ✅ Creates | ✅ Creates | PRs created, tracking updated |
| /work-cleanup removes workspace | ✅ Removes worktree | ✅ Deletes branches | Cleanup successful |
| Session resume (after restart) | ✅ Status shown | ✅ Branches restored | State preserved |

#### 6.2: Manual Test Scenarios

**Test 1: CLI Worktree Workflow**
```bash
# Start
/work-init feature/test-workflow

# Verify worktree created
ls .claude/worktrees/feature-test-workflow

# Make changes
cd .claude/worktrees/feature-test-workflow
echo "test" > test.txt
git add test.txt
git commit -m "test: add test file"

# Check status
/work-status

# Create PR
/work-pr

# Cleanup
/work-cleanup feature/test-workflow
```

**Test 2: Web Session Workflow**
```bash
# (In Web environment)

# Session auto-starts
# Verify session tracking file exists
cat .claude/web-session-tracking.json

# Create feature branch
/work-init feature/test-web-workflow

# Make changes
echo "test" > test.txt
git add test.txt
git commit -m "test: add test file"

# Check status
/work-status

# End session, restart, resume
# Verify session restored
```

**Test 3: Branch Protection**
```bash
# Should fail
git checkout main
echo "test" > test.txt
git add test.txt
git commit -m "test"  # ❌ Hook should reject

# Should succeed
git checkout -b feature/test-protection
git commit -m "test"  # ✅ Allowed on feature branch
```

---

### Phase 7: Advanced Features (Future Enhancements)

**Goal:** Add advanced workflow capabilities discovered during usage.

**Duration:** ~3 hours
**Priority:** LOW (Nice-to-have, implement as needed)

#### 7.1: CLI ↔ Web Mid-Work Switching

**Scenario:** User starts work in CLI, needs to continue in Web (or vice versa)

**File:** `scripts/workflow/export-to-web.sh` and `scripts/workflow/import-from-cli.sh`

**CLI → Web Migration:**
```bash
#!/bin/bash
# Export CLI workspace to Web-compatible format

WORKTREE_PATH="$1"

if [ ! -d "$WORKTREE_PATH" ]; then
  echo "❌ Worktree not found: $WORKTREE_PATH"
  exit 1
fi

cd "$WORKTREE_PATH"
BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Push branch to remote
echo "Pushing branch to remote..."
git push -u origin "$BRANCH"

# Push all submodule branches
git submodule foreach "git push -u origin \$(git rev-parse --abbrev-ref HEAD)"

# Create Web session tracking entry
REPO_ROOT="$(git rev-parse --show-toplevel)"
WEB_TRACKING="$REPO_ROOT/.claude/web-session-tracking.json"

# Add session to Web tracking
jq ".sessions[\"$BRANCH\"] = {
  created_at: \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",
  updated_at: \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",
  session_id: \"$(uuidgen)\",
  environment: \"web\",
  status: \"active\",
  submodules: {}
}" "$WEB_TRACKING" > "$WEB_TRACKING.tmp"

mv "$WEB_TRACKING.tmp" "$WEB_TRACKING"

echo "✓ Exported to Web session: $BRANCH"
echo ""
echo "In Web environment, run:"
echo "  git checkout $BRANCH"
echo "  /work-status"
```

**Web → CLI Migration:**
```bash
#!/bin/bash
# Import Web session to CLI worktree

BRANCH="$1"

if [ -z "$BRANCH" ]; then
  echo "Usage: import-from-web.sh <branch-name>"
  exit 1
fi

# Fetch from remote
git fetch origin "$BRANCH"

# Create worktree
REPO_ROOT="$(git rev-parse --show-toplevel)"
./scripts/cli/create-worktree.sh "$BRANCH"

echo "✓ Imported to CLI worktree"
echo ""
echo "Continue work:"
echo "  cd .claude/worktrees/$(echo "$BRANCH" | tr '/' '-')"
```

**Usage:**
```bash
# Switching from CLI to Web
cd .claude/worktrees/feature-my-feature
./scripts/workflow/export-to-web.sh "$(pwd)"

# Later, in Web
git checkout feature/my-feature

# Switching from Web to CLI
./scripts/workflow/import-from-cli.sh feature/my-feature
```

#### 7.2: /work-switch Command (CLI Multiple Worktrees)

**Purpose:** Quickly switch between active worktrees

**File:** `.claude/commands/work-switch.md`

**Implementation:**
```bash
#!/bin/bash
# Switch to a different worktree

TRACKING_FILE=".claude/cli-workspace-tracking.json"

# List active worktrees
echo "Active worktrees:"
echo ""

jq -r '.worktrees | keys[]' "$TRACKING_FILE" | nl

echo ""
read -p "Select worktree (number): " CHOICE

WORKTREE_PATH=$(jq -r ".worktrees | keys[$((CHOICE-1))]" "$TRACKING_FILE")

if [ -z "$WORKTREE_PATH" ] || [ "$WORKTREE_PATH" = "null" ]; then
  echo "❌ Invalid selection"
  exit 1
fi

cd "$WORKTREE_PATH"
echo "✓ Switched to: $WORKTREE_PATH"
echo ""
exec $SHELL
```

#### 7.3: Workspace Templates

**Purpose:** Pre-configure common workspace setups

**File:** `scripts/workflow/templates/workspace-template.json`

**Schema:**
```json
{
  "templates": {
    "frontend-feature": {
      "branches": ["feature/ui", "feature/api-integration"],
      "submodules": ["ui-components", "api-client"],
      "scripts": ["npm install", "npm run setup"]
    },
    "bug-fix": {
      "branches": ["fix/issue-123"],
      "submodules": [],
      "scripts": ["npm test"]
    }
  }
}
```

**Usage:**
```bash
/work-init --template frontend-feature
# Creates workspace with pre-configured setup
```

#### 7.4: Workspace Snapshots

**Purpose:** Save and restore workspace state

**Implementation:**
```bash
# Save snapshot
/work-snapshot save "before-refactor"

# Restore snapshot
/work-snapshot restore "before-refactor"

# List snapshots
/work-snapshot list
```

**Use Case:** Before risky refactoring, save workspace state for quick rollback.

---

## Session Lifecycle

### CLI Session Lifecycle

```
┌─────────────────────────────────────────────────────────────┐
│ Session Start                                               │
└─────────────────────────────────────────────────────────────┘
│
├─ Hook: on-session-start.sh
│  ├─ Detect CLI environment
│  ├─ Install submodule hooks
│  └─ Show active workspaces (if any)
│
├─ User Action: /work-init feature/my-feature
│  ├─ Create worktree at .claude/worktrees/feature-my-feature/
│  ├─ Create feature branch in worktree
│  ├─ Initialize submodules in worktree
│  ├─ Install hooks in worktree submodules
│  └─ Update tracking file
│
├─ User Action: Make changes
│  ├─ cd .claude/worktrees/feature-my-feature/
│  ├─ Edit files (meta or submodule)
│  ├─ git add, git commit
│  └─ Tracking file auto-updated (via hook)
│
├─ User Action: /work-status
│  ├─ Read tracking file
│  ├─ Count commits, detect changes
│  └─ Display table
│
├─ User Action: /work-pr
│  ├─ Detect changed repos (meta + submodules)
│  ├─ Push branches to remote
│  ├─ Create PRs via gh CLI
│  └─ Update tracking file with PR URLs
│
├─ User Action: Merge PRs on GitHub
│  └─ (External: GitHub UI or API)
│
├─ User Action: /work-cleanup feature/my-feature
│  ├─ Verify PRs merged
│  ├─ Remove worktree directory
│  ├─ Delete local branches
│  ├─ Delete remote branches
│  └─ Update tracking file (remove entry)
│
└─────────────────────────────────────────────────────────────┘
│ Session End
└─ Hook: on-session-end.sh
   ├─ Commit tracking file if changed
   └─ Exit (worktrees persist on disk)
```

### Web Session Lifecycle

```
┌─────────────────────────────────────────────────────────────┐
│ Session Start                                               │
└─────────────────────────────────────────────────────────────┘
│
├─ Hook: on-session-start.sh
│  ├─ Detect Web environment
│  ├─ Install submodule hooks
│  ├─ Run /web-setup
│  │  ├─ Configure git credentials
│  │  ├─ Load tracking file from git
│  │  ├─ Resume existing session (or create new)
│  │  └─ Show session status
│  └─ Display active session
│
├─ User Action: /work-init feature/my-feature (optional, if new work)
│  ├─ Create meta branch: feature/my-feature
│  ├─ Checkout branch
│  └─ Update tracking file
│
├─ User Action: Make changes
│  ├─ Edit files (meta or submodule)
│  ├─ Submodule auto-creates feature branch (via hook)
│  ├─ git add, git commit
│  └─ Tracking file auto-updated
│
├─ User Action: /work-status
│  ├─ Read tracking file
│  ├─ Count commits, detect changes
│  └─ Display table
│
├─ User Action: /work-pr
│  ├─ Detect changed repos (meta + submodules)
│  ├─ Push branches to remote
│  ├─ Create PRs via gh CLI
│  └─ Update tracking file with PR URLs
│
├─ Session End
│  └─ Hook: on-session-end.sh
│     ├─ Commit tracking file to git
│     └─ Exit (filesystem destroyed)
│
├─ Session Resume (hours/days later)
│  ├─ Clone fresh from git
│  ├─ Hook: on-session-start.sh
│  ├─ Load tracking file from git
│  ├─ Checkout meta branch (from tracking)
│  ├─ Checkout submodule branches (from tracking)
│  └─ Display session status (resume exactly where left off)
│
├─ User Action: Merge PRs on GitHub
│  └─ (External: GitHub UI or API)
│
└─ User Action: /work-cleanup feature/my-feature
   ├─ Verify PRs merged
   ├─ Checkout main (meta + submodules)
   ├─ Delete local branches
   ├─ Delete remote branches
   └─ Update tracking file (remove entry)
```

---

## Edge Cases

### Edge Case 1: Commit to Main Attempted

**Scenario:** User tries to commit to main branch (meta or submodule)

**Detection:** Pre-commit hook

**Handling:**
```bash
# Hook rejects commit
echo "❌ Direct commits to main not allowed"
exit 1
```

**User Recovery:**
```bash
# Create feature branch from current changes
git stash
git checkout -b feature/my-fix
git stash pop
git add .
git commit -m "fix: my fix"
```

### Edge Case 2: Worktree Already Exists

**Scenario:** User runs `/work-init feature/existing` but worktree already exists

**Detection:** Script checks for existing worktree directory

**Handling:**
```bash
if [ -d "$WORKTREE_DIR" ]; then
  echo "❌ Worktree already exists: $WORKTREE_DIR"
  echo "Use /work-switch to resume or /work-cleanup to remove"
  exit 1
fi
```

**User Recovery:**
```bash
# Option 1: Resume existing worktree
cd .claude/worktrees/feature-existing

# Option 2: Cleanup and recreate
/work-cleanup feature/existing
/work-init feature/existing
```

### Edge Case 3: PR Creation Fails (No GitHub Access)

**Scenario:** `gh pr create` fails (no token, network issue, permissions)

**Detection:** Script checks gh CLI exit code

**Handling:**
```bash
if ! gh pr create ...; then
  echo "❌ Failed to create PR via gh CLI"
  echo ""
  echo "Manual PR creation:"
  echo "  1. Visit: https://github.com/tuulbelt/tuulbelt/compare/main...$BRANCH"
  echo "  2. Create PR manually"
  echo "  3. Update tracking file with PR URL (optional)"
  echo ""
  exit 1
fi
```

**User Recovery:**
- Create PR manually via GitHub UI
- Use PR link in future status checks

### Edge Case 4: Cleanup Before PRs Merged

**Scenario:** User runs `/work-cleanup` but PRs still open

**Detection:** Script checks PR state via gh API

**Handling:**
```bash
if [ "$PR_STATE" != "MERGED" ]; then
  echo "⚠️  WARNING: PRs not merged yet"
  echo ""
  echo "Pending PRs:"
  echo "  - Meta: $META_PR_URL ($META_PR_STATE)"
  echo "  - Submodule: $SUB_PR_URL ($SUB_PR_STATE)"
  echo ""
  echo "Use --force to cleanup anyway (changes will be lost if PRs rejected)"
  exit 1
fi
```

**User Recovery:**
```bash
# Option 1: Wait for PRs to merge
# (do nothing)

# Option 2: Force cleanup (dangerous)
/work-cleanup feature/my-feature --force
```

### Edge Case 5: Session Resume with Conflicting Changes

**Scenario:** Web session resumes, but remote branch has new commits (other user pushed)

**Detection:** Script checks if local branch diverged from remote

**Handling:**
```bash
if ! git diff --quiet HEAD origin/$BRANCH; then
  echo "⚠️  WARNING: Remote branch has new commits"
  echo ""
  echo "Options:"
  echo "  1. Pull and merge: git pull origin $BRANCH"
  echo "  2. Pull and rebase: git pull --rebase origin $BRANCH"
  echo "  3. Force push (dangerous): git push --force origin $BRANCH"
  echo ""
  exit 1
fi
```

**User Recovery:**
```bash
# Option 1: Merge remote changes
git pull origin feature/my-feature

# Option 2: Rebase on remote changes
git pull --rebase origin feature/my-feature
```

### Edge Case 6: Submodule Detached HEAD

**Scenario:** Submodule ends up in detached HEAD state

**Detection:** Script checks submodule state

**Handling:**
```bash
cd tools/my-submodule
BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$BRANCH" = "HEAD" ]; then
  echo "⚠️  Submodule in detached HEAD state"
  echo "Creating feature branch from current state..."
  git checkout -b "$FEATURE_NAME"
fi
```

**User Recovery:**
- Automatically handled by script
- Creates feature branch from current state

### Edge Case 7: Tracking File Corrupted

**Scenario:** Tracking file has invalid JSON or missing required fields

**Detection:** Script validates JSON structure

**Handling:**
```bash
if ! jq empty "$TRACKING_FILE" 2>/dev/null; then
  echo "❌ Tracking file corrupted"
  echo "Backing up to: $TRACKING_FILE.backup"
  cp "$TRACKING_FILE" "$TRACKING_FILE.backup"
  echo "Regenerating tracking file from current git state..."
  ./scripts/workflow/regenerate-tracking.sh
fi
```

**User Recovery:**
- Script auto-regenerates from git state
- Backup preserved for manual recovery if needed

### Edge Case 8: Parallel Features in Web

**Scenario:** User wants to work on multiple features simultaneously in Web (not supported)

**Detection:** User runs `/work-init` while already on feature branch

**Handling:**
```bash
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$CURRENT_BRANCH" != "main" ]; then
  echo "⚠️  Already on feature branch: $CURRENT_BRANCH"
  echo ""
  echo "Web environment supports serial development only."
  echo ""
  echo "Options:"
  echo "  1. Finish current work: /work-pr, merge, /work-cleanup"
  echo "  2. Stash and switch: git stash, /work-init new-feature"
  echo "  3. Use CLI for parallel work (worktrees)"
  echo ""
  exit 1
fi
```

**User Recovery:**
- Complete current work first
- Or switch to CLI for parallel development

### Edge Case 9: GitHub Rate Limit Exceeded

**Scenario:** gh CLI hits GitHub API rate limit during PR creation

**Detection:** gh CLI returns rate limit error

**Handling:**
```bash
if gh pr list 2>&1 | grep -q "rate limit"; then
  echo "❌ GitHub API rate limit exceeded"
  echo ""
  echo "Try again in 1 hour, or:"
  echo "  1. Authenticate with PAT: gh auth login"
  echo "  2. Create PR manually via GitHub UI"
  echo ""
  exit 1
fi
```

**User Recovery:**
- Wait for rate limit reset
- Use manual PR creation via GitHub UI

### Edge Case 10: Merge Conflict During Cleanup

**Scenario:** `/work-cleanup` tries to merge main, but conflicts exist

**Detection:** Git merge command fails with conflict status

**Handling:**
```bash
if ! git merge origin/main --no-ff; then
  echo "❌ Merge conflict detected"
  echo ""
  echo "Cleanup aborted. Resolve conflicts manually:"
  echo "  1. git status  # See conflicting files"
  echo "  2. Edit files to resolve conflicts"
  echo "  3. git add ."
  echo "  4. git commit"
  echo "  5. Re-run /work-cleanup"
  echo ""
  exit 1
fi
```

**User Recovery:**
- Manually resolve conflicts
- Complete merge
- Re-run cleanup command

### Edge Case 11: Detached HEAD in Meta Repo

**Scenario:** Meta repo ends up in detached HEAD state (checkout of commit SHA, tag, etc.)

**Detection:** Script checks if current branch is HEAD

**Handling:**
```bash
REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"
BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [ "$BRANCH" = "HEAD" ]; then
  echo "⚠️  Meta repo in detached HEAD state"
  echo "Creating feature branch from current state..."

  # Prompt for branch name
  read -p "Enter feature branch name (e.g., feature/recover-work): " FEATURE_NAME

  if ! echo "$FEATURE_NAME" | grep -qE '^(feature|fix|chore|refactor)/'; then
    echo "❌ Invalid branch name format"
    exit 1
  fi

  git checkout -b "$FEATURE_NAME"
  echo "✓ Created branch: $FEATURE_NAME"
fi
```

**User Recovery:**
- Script automatically creates feature branch from detached HEAD
- User provides branch name when prompted

### Edge Case 12: Manually Deleted Worktree Directory

**Scenario:** User deletes `.claude/worktrees/<name>/` directory outside workflow (rm -rf)

**Detection:** Tracking file references worktree path that doesn't exist

**Handling:**
```bash
# In work-status script
for worktree_path in $(jq -r '.worktrees | keys[]' "$TRACKING_FILE"); do
  if [ ! -d "$worktree_path" ]; then
    echo "⚠️  Worktree directory missing: $worktree_path"
    echo "  Removing from tracking file..."

    # Remove from git worktree list
    git worktree remove "$worktree_path" --force 2>/dev/null || true

    # Remove from tracking file
    jq "del(.worktrees[\"$worktree_path\"])" "$TRACKING_FILE" > "$TRACKING_FILE.tmp"
    mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"

    echo "  ✓ Cleaned up orphaned entry"
  fi
done
```

**User Recovery:**
- Automatic cleanup on next `/work-status` call
- Orphaned entries removed from tracking

### Edge Case 13: Force-Pushing Tracked Branches

**Scenario:** User force-pushes feature branch, breaking tracking SHA references

**Detection:** Local commits don't match remote (diverged history)

**Handling:**
```bash
# In work-pr script, before creating PR
git fetch origin "$FEATURE_NAME" 2>/dev/null || true

LOCAL_SHA=$(git rev-parse "$FEATURE_NAME")
REMOTE_SHA=$(git rev-parse "origin/$FEATURE_NAME" 2>/dev/null || echo "")

if [ -n "$REMOTE_SHA" ] && [ "$LOCAL_SHA" != "$REMOTE_SHA" ]; then
  # Check if diverged (force push detected)
  if ! git merge-base --is-ancestor "$REMOTE_SHA" "$LOCAL_SHA"; then
    echo "⚠️  WARNING: Local branch diverged from remote (force push detected)"
    echo ""
    echo "Local:  $LOCAL_SHA"
    echo "Remote: $REMOTE_SHA"
    echo ""
    echo "Options:"
    echo "  1. Push with --force-with-lease (safer force push)"
    echo "  2. Reset local to remote: git reset --hard origin/$FEATURE_NAME"
    echo "  3. Cancel and investigate"
    echo ""
    read -p "Continue with force push? (y/N): " CONFIRM

    if [ "$CONFIRM" != "y" ]; then
      echo "Aborted"
      exit 1
    fi

    git push --force-with-lease origin "$FEATURE_NAME"
  fi
fi
```

**User Recovery:**
- Warned before force push
- Can choose safer `--force-with-lease` option
- Can abort and investigate

### Edge Case 14: Renaming Branches After Tracking

**Scenario:** User renames feature branch after it's tracked (git branch -m old-name new-name)

**Detection:** Tracking file references branch name that doesn't exist

**Handling:**
```bash
# In work-status script
for tracked_branch in $(jq -r '.worktrees[].meta_branch' "$TRACKING_FILE"); do
  if ! git show-ref --verify --quiet "refs/heads/$tracked_branch"; then
    echo "⚠️  Tracked branch not found: $tracked_branch"
    echo ""
    echo "Possible causes:"
    echo "  - Branch was renamed"
    echo "  - Branch was deleted"
    echo ""
    echo "Options:"
    echo "  1. Update tracking file with new branch name"
    echo "  2. Remove entry from tracking file"
    echo ""
    read -p "Enter new branch name (or 'delete' to remove): " NEW_NAME

    if [ "$NEW_NAME" = "delete" ]; then
      # Remove from tracking
      jq "del(.worktrees[] | select(.meta_branch == \"$tracked_branch\"))" \
         "$TRACKING_FILE" > "$TRACKING_FILE.tmp"
      mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"
      echo "✓ Removed from tracking"
    else
      # Update tracking
      jq "(.worktrees[] | select(.meta_branch == \"$tracked_branch\").meta_branch) = \"$NEW_NAME\"" \
         "$TRACKING_FILE" > "$TRACKING_FILE.tmp"
      mv "$TRACKING_FILE.tmp" "$TRACKING_FILE"
      echo "✓ Updated to: $NEW_NAME"
    fi
  fi
done
```

**User Recovery:**
- Prompted to provide new branch name or remove entry
- Tracking file updated accordingly

### Edge Case 15: Multiple Users Collaborating

**Scenario:** Two users working on same meta repo, creating conflicting worktrees/branches

**Detection:** Tracking file shows different users' work

**Handling:**
```bash
# Add user identifier to tracking file
USER_ID="${USER:-unknown}@${HOSTNAME:-unknown}"

# In tracking file schema, add:
{
  "worktrees": {
    "path": {
      "meta_branch": "feature/my-feature",
      "created_by": "alice@laptop",  # New field
      "created_at": "...",
      "updated_at": "...",
      "status": "active"
    }
  }
}

# Warn if user tries to modify another user's worktree
CURRENT_USER="${USER:-unknown}@${HOSTNAME:-unknown}"
WORKTREE_USER=$(jq -r ".worktrees[\"$WORKTREE_PATH\"].created_by" "$TRACKING_FILE")

if [ "$WORKTREE_USER" != "$CURRENT_USER" ]; then
  echo "⚠️  WARNING: This worktree was created by $WORKTREE_USER"
  echo "You are: $CURRENT_USER"
  echo ""
  read -p "Continue anyway? (y/N): " CONFIRM

  if [ "$CONFIRM" != "y" ]; then
    echo "Aborted"
    exit 1
  fi
fi
```

**User Recovery:**
- Tracking file includes user attribution
- Warned before modifying other user's work
- Can proceed with caution or create separate worktree

---

## Success Criteria

### Phase 1 Success (Branch Protection)

- [ ] Pre-commit hook installed in meta repo
- [ ] Pre-commit hook installed in all submodules
- [ ] Hook blocks commits to main (meta)
- [ ] Hook blocks commits to main (submodules)
- [ ] Error message guides user to feature branch workflow
- [ ] Hooks auto-install on session start

### Phase 2 Success (CLI Commands)

- [ ] `/work-init` creates worktree successfully
- [ ] `/work-init` handles existing branches (local and remote)
- [ ] `/work-init --help` displays usage information
- [ ] `/work-status` shows accurate workspace state
- [ ] `/work-status --help` displays usage information
- [ ] `/work-pr` creates PRs for changed repos
- [ ] `/work-pr --help` displays usage information
- [ ] `/work-cleanup` removes worktree and deletes branches
- [ ] `/work-cleanup --help` displays usage information
- [ ] Tracking file accurately reflects workspace state
- [ ] Tracking file regeneration script works correctly
- [ ] Multiple worktrees can coexist
- [ ] Branch naming validation enforces conventions

### Phase 3 Success (Unified Commands)

- [ ] Commands detect environment (CLI vs Web) automatically
- [ ] Same command names work in both environments
- [ ] Behavior adapts appropriately (worktrees vs branches)
- [ ] `push.sh` detects and pushes current branch

### Phase 4 Success (Session Lifecycle)

- [ ] `on-session-start.sh` sets up environment correctly
- [ ] `on-session-end.sh` saves state correctly
- [ ] Web sessions resume seamlessly
- [ ] CLI workspaces persist across sessions
- [ ] Tracking files committed to git

### Phase 5 Success (Documentation)

- [ ] `FEATURE_BRANCH_WORKFLOW.md` explains core principles
- [ ] `CLI_WORKFLOW.md` documents CLI-specific workflow
- [ ] `WEB_WORKFLOW.md` documents Web-specific workflow
- [ ] `WORKFLOW_TROUBLESHOOTING.md` covers all edge cases
- [ ] CLAUDE.md references all workflow docs
- [ ] Examples demonstrate complete workflows

### Phase 6 Success (Testing)

- [ ] All test scenarios pass
- [ ] Edge cases handled gracefully
- [ ] Error messages guide user to recovery
- [ ] No data loss scenarios
- [ ] Clean state after cleanup commands

### Edge Cases Validation (15 Scenarios)

**Core Workflow Edge Cases (1-10):**
- [ ] Edge Case 1: Commit to main blocked with helpful message
- [ ] Edge Case 2: Existing worktree detected, user guided to resume/cleanup
- [ ] Edge Case 3: PR creation failure provides manual instructions
- [ ] Edge Case 4: Cleanup blocked if PRs not merged (unless --force)
- [ ] Edge Case 5: Session resume handles remote branch changes
- [ ] Edge Case 6: Submodule detached HEAD auto-creates feature branch
- [ ] Edge Case 7: Corrupted tracking file regenerates with backup
- [ ] Edge Case 8: Web parallel features rejected with clear explanation
- [ ] Edge Case 9: GitHub rate limit provides wait time and alternatives
- [ ] Edge Case 10: Merge conflicts during cleanup abort with instructions

**Advanced Edge Cases (11-15):**
- [ ] Edge Case 11: Meta repo detached HEAD creates feature branch
- [ ] Edge Case 12: Manually deleted worktree auto-cleans tracking
- [ ] Edge Case 13: Force-push detection warns and uses --force-with-lease
- [ ] Edge Case 14: Renamed branch updates tracking or removes entry
- [ ] Edge Case 15: Multiple users warned before modifying others' work

### Phase 7 Success (Future Enhancements - Optional)

- [ ] CLI ↔ Web switching scripts work correctly
- [ ] `/work-switch` command enables quick worktree switching
- [ ] Workspace templates reduce setup time
- [ ] Workspace snapshots enable safe rollback

---

## Migration Path

### Step 0: Pre-Migration Cleanup (CRITICAL)

**Goal:** Save uncommitted work before installing branch protection hooks

**Why This Matters:**
Once hooks are installed, commits to main will be blocked. Any uncommitted changes on main will prevent workflow adoption.

**Actions:**
1. **Check for uncommitted changes on main:**
   ```bash
   git status --porcelain
   ```

2. **If changes exist, save them to a feature branch:**
   ```bash
   # Stash changes
   git stash push -m "work in progress before workflow migration"

   # Create feature branch
   git checkout -b feature/pre-migration-work

   # Apply stashed changes
   git stash pop

   # Review and commit
   git add .
   git commit -m "feat: work in progress from main"

   # Push to remote
   git push -u origin feature/pre-migration-work

   # Return to main
   git checkout main
   ```

3. **Verify main is clean:**
   ```bash
   git status  # Should show "working tree clean"
   ```

4. **Check all submodules:**
   ```bash
   git submodule foreach 'git status --porcelain'
   ```

5. **If submodules have changes, repeat for each:**
   ```bash
   cd tools/my-submodule
   git stash push -m "WIP before workflow migration"
   git checkout -b feature/pre-migration-submodule-work
   git stash pop
   git add .
   git commit -m "feat: submodule WIP from main"
   git push -u origin feature/pre-migration-submodule-work
   git checkout main
   cd ../..
   ```

**Verification:**
```bash
# Meta repo clean
git status

# All submodules clean
git submodule foreach 'git status'

# Should see "working tree clean" for all repos
```

**Rollback Plan:**
- If issues arise during migration, feature branches preserve all work
- Can cherry-pick commits back to main if needed
- No data loss risk

### Step 1: Pilot on Meta Repo Only

**Goal:** Test workflow in meta repo before rolling out to submodules

**Actions:**
1. Install branch protection in meta repo only
2. Create CLI commands (worktree-based)
3. Test with small feature (e.g., documentation update)
4. Verify workflow end-to-end
5. Fix issues before submodule rollout

**Rollback Plan:**
- Remove pre-commit hook
- Delete tracking file
- Document lessons learned

### Step 2: Gradual Submodule Rollout

**Goal:** Install hooks in submodules incrementally

**Actions:**
1. Install in 1-2 submodules first
2. Test cross-repo PR workflow
3. Monitor for issues
4. If successful, roll out to remaining submodules
5. Use hook installer script for automation

**Rollback Plan:**
- Remove hooks from affected submodules
- Document which submodules opted out (if needed)

### Step 3: Documentation and Training

**Goal:** Ensure all documentation is complete and accessible

**Actions:**
1. Write all Phase 5 documentation
2. Update CLAUDE.md with workflow references
3. Add examples to each command doc
4. Test documentation by following it exactly
5. Archive old WEB_WORKFLOW_INTEGRATION.md

**Rollback Plan:**
- Keep old docs accessible during transition
- Restore old workflow if adoption fails

### Step 4: Enforce Workflow

**Goal:** Make feature branch workflow mandatory

**Actions:**
1. Update CONTRIBUTING.md to mandate workflow
2. Add workflow check to `/quality-check`
3. Train contributors (if team grows)
4. Monitor PR patterns to ensure compliance

**Rollback Plan:**
- Make hooks non-blocking (warn instead of error)
- Gradually phase in enforcement

---

## Next Steps

1. **Review this plan** - Validate approach, identify gaps
2. **Prioritize phases** - Decide which phases to implement first
3. **Create detailed tasks** - Break each phase into micro-tasks
4. **Implement Phase 1** - Start with branch protection (safest, highest value)
5. **Test and iterate** - Validate each phase before moving to next

---

**End of Plan**
