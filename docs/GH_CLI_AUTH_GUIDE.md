# GitHub CLI Authentication Guide

## The Problem

The global `gh` CLI is authenticated as one user account, but this project needs to use a different account with different permissions.

## The Solution

**Chain `source scripts/setup-github-auth.sh` with `gh` commands using `&&`** to ensure correct credentials.

## Correct Usage Patterns

### In Bash Tool Calls (Claude Code)

**CRITICAL: Each Bash command in Claude Code runs in a separate shell instance, so environment variables don't persist between commands.**

```bash
# WRONG - uses global auth (wrong account)
gh repo edit tuulbelt/tool-name --add-topic rust

# CORRECT - chain source with gh command in SAME command
source scripts/setup-github-auth.sh && gh repo edit tuulbelt/tool-name --add-topic rust
source scripts/setup-github-auth.sh && gh repo create tuulbelt/tool-name --public
source scripts/setup-github-auth.sh && gh api user --jq '.login'
```

**Why this works:**
- `source scripts/setup-github-auth.sh` loads credentials and exports `GH_TOKEN`
- Chaining with `&&` keeps the environment in the same shell session
- `gh` CLI respects `GH_TOKEN` and uses project credentials

### In Scripts

```bash
#!/bin/bash
# Always source auth first
source scripts/setup-github-auth.sh

# Then use gh commands (GH_TOKEN is already exported)
gh repo edit tuulbelt/tool-name --enable-issues=false
```

### In Interactive Shell (with direnv)

```bash
# direnv automatically loads .envrc when you cd into the project
cd /path/to/tuulbelt

# GH_TOKEN is now set, gh commands will use project account
gh repo list tuulbelt
```

## Verification

Check which account `gh` is using:

```bash
GH_TOKEN=$GITHUB_TOKEN gh api user --jq '.login'
# Should output: your-project-username
```

## Why This Works

1. `gh` CLI prioritizes authentication in this order:
   1. `GH_TOKEN` environment variable ✅ **We use this**
   2. Stored credentials in `~/.config/gh/hosts.yml` (global account - we avoid this)

2. Setting `GH_TOKEN` before EACH command ensures it overrides stored auth

3. `.envrc` (with direnv) automatically sets `GH_TOKEN` when entering the directory

## For Future Migrations

**Pattern to follow in all bash commands (Claude Code):**

```bash
# EVERY gh command must be chained with source in the SAME command
source scripts/setup-github-auth.sh && gh repo create tuulbelt/tool-name --public

source scripts/setup-github-auth.sh && gh repo edit tuulbelt/tool-name \
  --add-topic tuulbelt \
  --add-topic rust \
  --add-topic zero-dependencies

source scripts/setup-github-auth.sh && gh repo edit tuulbelt/tool-name --disable-issues

source scripts/setup-github-auth.sh && gh repo view tuulbelt/tool-name --json repositoryTopics
```

**Key points:**
- Each `gh` command needs `source scripts/setup-github-auth.sh &&` prefix
- Don't try to source once and run multiple gh commands separately - won't work
- The `&&` ensures gh runs in the same shell with exported `GH_TOKEN`

## Troubleshooting

**If you see "Permission denied" or "Not Found" errors:**

1. Check which account gh is using:
   ```bash
   gh api user --jq '.login'
   ```

2. If it shows the wrong account, you forgot to set `GH_TOKEN`:
   ```bash
   export GH_TOKEN="$GITHUB_TOKEN"
   ```

3. Verify the token is set:
   ```bash
   echo "GH_TOKEN set: ${GH_TOKEN:+YES}"
   ```

## Alternative: gh-project Helper Function

If you have direnv installed and `.envrc` loaded:

```bash
# Use the helper function
gh-project repo edit tuulbelt/tool-name --enable-issues=false
```

This automatically uses the project token.

---

**Last Updated:** 2025-12-29
**Status:** Verified working
