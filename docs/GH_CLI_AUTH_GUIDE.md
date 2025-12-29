# GitHub CLI Authentication Guide

## The Problem

The global `gh` CLI is authenticated as `kofirc`, but this project needs to use `koficodedat` with different permissions.

## The Solution

**Always prefix `gh` commands with `GH_TOKEN=$GITHUB_TOKEN`** to override global authentication.

## Correct Usage Patterns

### In Bash Tool Calls (Claude Code)

```bash
# WRONG - uses global auth (kofirc)
gh repo edit tuulbelt/tool-name --add-topic rust

# CORRECT - uses project auth (koficodedat)
source .env  # Load GITHUB_TOKEN from .env
export GH_TOKEN="$GITHUB_TOKEN"
GH_TOKEN=$GITHUB_TOKEN gh repo edit tuulbelt/tool-name --add-topic rust
```

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
cd /Users/kofi/_/tuulbelt

# GH_TOKEN is now set, gh commands will use koficodedat
gh repo list tuulbelt
```

## Verification

Check which account `gh` is using:

```bash
GH_TOKEN=$GITHUB_TOKEN gh api user --jq '.login'
# Should output: koficodedat
```

## Why This Works

1. `gh` CLI prioritizes authentication in this order:
   1. `GH_TOKEN` environment variable ✅ **We use this**
   2. Stored credentials in `~/.config/gh/hosts.yml` (kofirc - we avoid this)

2. Setting `GH_TOKEN` before EACH command ensures it overrides stored auth

3. `.envrc` (with direnv) automatically sets `GH_TOKEN` when entering the directory

## For Future Migrations

**Pattern to follow in all bash commands:**

```bash
# Step 1: Load credentials
source .env  # Loads GITHUB_TOKEN
export GH_TOKEN="$GITHUB_TOKEN"

# Step 2: Use gh with explicit GH_TOKEN
GH_TOKEN=$GITHUB_TOKEN gh repo edit tuulbelt/output-diffing-utility \
  --add-topic tuulbelt \
  --add-topic rust \
  --add-topic zero-dependencies

# Step 3: Verify
GH_TOKEN=$GITHUB_TOKEN gh repo view tuulbelt/output-diffing-utility --json repositoryTopics
```

## Troubleshooting

**If you see "Permission denied" or "Not Found" errors:**

1. Check which account gh is using:
   ```bash
   gh api user --jq '.login'
   ```

2. If it says `kofirc`, you forgot to set `GH_TOKEN`:
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
