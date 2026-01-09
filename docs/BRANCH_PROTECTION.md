# Branch Protection for Tuulbelt Repositories

**Last Updated:** 2026-01-09

This document explains how branch protection is configured for all Tuulbelt repositories and how to apply it to new or existing repositories.

---

## Overview

All Tuulbelt repositories enforce branch protection rules on the `main` branch to ensure code quality and prevent accidental changes.

### Protection Rules

| Rule | Value | Purpose |
|------|-------|---------|
| **Require pull request** | Yes (1 approval) | All changes go through code review |
| **Dismiss stale reviews** | Yes | New commits invalidate previous approvals |
| **Require status checks** | Yes (`test` workflow) | Tests must pass before merge |
| **Require branches up to date** | Yes | Prevent merge conflicts |
| **Require linear history** | Yes | No merge commits (rebase only) |
| **Require conversation resolution** | Yes | All review comments must be resolved |
| **Block force pushes** | Yes | Protect against history rewriting |
| **Block deletions** | Yes | Prevent accidental branch deletion |
| **Enforce for admins** | No | Allow maintainers to bypass if needed |

---

## For New Repositories

When creating a new repository using `/new-tool`, branch protection is **NOT** applied automatically because the `main` branch doesn't exist yet (repos are created with `auto_init: false`).

### Automatic Protection (Recommended)

Branch protection will be applied automatically after:
1. First push to `main` branch
2. Running the post-creation workflow

**Manual trigger after first push:**
```bash
# After pushing to main for the first time:
./scripts/apply-branch-protection.sh my-new-tool
```

### MCP Tool (CLI Environment Only)

If using Claude Code CLI with the `tuulbelt-github` MCP server:

```
# After creating repo and pushing to main
use apply_branch_protection tool with:
  repo: "my-new-tool"
  branch: "main"
```

This will apply all protection rules automatically.

---

## For Existing Repositories

### Apply to Single Repository

```bash
./scripts/apply-branch-protection.sh repository-name
```

**Example:**
```bash
./scripts/apply-branch-protection.sh test-flakiness-detector
```

**Output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Applying branch protection to: tuulbelt/test-flakiness-detector
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 Applying protection rules...
✅ Branch protection applied successfully

Protection rules:
  ✓ Require pull request before merging
  ✓ Require 1 approval
  ✓ Dismiss stale reviews when new commits pushed
  ✓ Require status checks to pass (test workflow)
  ✓ Require branches to be up to date before merging
  ✓ Require linear history (no merge commits)
  ✓ Require conversation resolution before merging
  ✓ Block force pushes
  ✓ Block deletions
```

### Apply to All Repositories

```bash
./scripts/apply-branch-protection.sh
```

This will:
1. Discover all repositories in the `tuulbelt` organization
2. Apply protection rules to each repo with a `main` branch
3. Skip repos that don't have a `main` branch yet
4. Report summary of success/skipped/failed repos

**Example Output:**
```
🔍 Discovering all tuulbelt repositories...
Found 12 repositories

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Applying branch protection to: tuulbelt/test-flakiness-detector
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Branch protection applied successfully

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Applying branch protection to: tuulbelt/new-tool-no-commits
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⚠️  Branch 'main' does not exist in new-tool-no-commits
   Skipping protection (apply after first push)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Summary:
  ✅ Successfully protected: 11 repos
  ⚠️  Skipped (no main branch): 1 repos
  ❌ Failed: 0 repos
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Prerequisites

### Required Tools

- `gh` CLI (GitHub CLI) installed and authenticated
- GitHub Personal Access Token with `repo` scope

### Authentication

The script uses the credentials from `.env` file via `scripts/lib/load-credentials.sh`.

**Verify authentication:**
```bash
gh auth status

# Should show:
# ✓ Logged in to github.com account koficodedat (GH_TOKEN)
```

**If not authenticated:**
```bash
# Set up credentials
./scripts/setup-github-auth.sh
```

---

## Verification

### Check Protection Status

**Via GitHub Web UI:**
1. Visit `https://github.com/tuulbelt/{repo-name}/settings/branches`
2. Under "Branch protection rules", you should see `main`
3. Click "Edit" to view all applied rules

**Via `gh` CLI:**
```bash
gh api repos/tuulbelt/{repo-name}/branches/main/protection
```

**Expected output (abbreviated):**
```json
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["test"]
  },
  "required_pull_request_reviews": {
    "dismiss_stale_reviews": true,
    "required_approving_review_count": 1
  },
  "required_linear_history": {
    "enabled": true
  },
  "allow_force_pushes": {
    "enabled": false
  },
  "allow_deletions": {
    "enabled": false
  }
}
```

---

## Troubleshooting

### "Branch 'main' does not exist"

**Cause:** Repository has no commits yet.

**Solution:**
```bash
# Push at least one commit first
cd tools/my-new-tool
echo "# My Tool" > README.md
git add .
git commit -m "feat: initial commit"
git push -u origin main

# Now apply protection
cd ../..
./scripts/apply-branch-protection.sh my-new-tool
```

### "Failed to apply branch protection"

**Cause:** Insufficient permissions or API rate limit.

**Solution 1: Check permissions**
```bash
gh auth status
# Verify token has 'repo' scope

# If not:
gh auth login
# Select token with repo scope
```

**Solution 2: Rate limit**
```bash
gh api rate_limit

# If exhausted, wait 1 hour or use authenticated token (higher limits)
```

### "Resource not accessible by integration"

**Cause:** Using `GITHUB_TOKEN` from Actions (limited permissions).

**Solution:** Use Personal Access Token (PAT):
```bash
# In .env file:
GITHUB_TOKEN=ghp_your_personal_access_token_here

# Not:
GITHUB_TOKEN=${{ secrets.GITHUB_TOKEN }}  # Actions token (limited)
```

---

## Modifying Protection Rules

### Via Script

Edit `scripts/apply-branch-protection.sh` and modify the API call:

```bash
gh api \
  --method PUT \
  "/repos/$ORG/$repo/branches/$PROTECTED_BRANCH/protection" \
  -f required_status_checks='{"strict":true,"contexts":["test"]}' \
  -f required_pull_request_reviews='{"required_approving_review_count":2}' \  # Changed from 1 to 2
  ...
```

### Via MCP Server

Edit `.claude/mcp/tuulbelt-github/index.js` in the `applyBranchProtection()` function:

```javascript
required_pull_request_reviews: {
  dismissal_restrictions: {},
  dismiss_stale_reviews: true,
  require_code_owner_reviews: false,
  required_approving_review_count: 2,  // Changed from 1
  ...
}
```

### Via GitHub Web UI

1. Go to `https://github.com/tuulbelt/{repo-name}/settings/branches`
2. Click "Edit" on the `main` branch rule
3. Modify settings
4. Click "Save changes"

**Note:** Changes via Web UI only affect that specific repository. To apply to all repos, update the script or MCP server.

---

## GitHub API Reference

- **Branch Protection API:** https://docs.github.com/en/rest/branches/branch-protection
- **Update Branch Protection:** https://docs.github.com/en/rest/branches/branch-protection#update-branch-protection

**Example API Call:**
```bash
gh api \
  --method PUT \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  "/repos/tuulbelt/test-flakiness-detector/branches/main/protection" \
  -f required_status_checks='{"strict":true,"contexts":["test"]}' \
  -f enforce_admins=false \
  -f required_pull_request_reviews='{"dismiss_stale_reviews":true,"required_approving_review_count":1}' \
  -f restrictions=null \
  -F required_linear_history=true \
  -F allow_force_pushes=false \
  -F allow_deletions=false
```

---

## Best Practices

### For New Tools

1. Create repository via `/new-tool`
2. Scaffold and commit initial code
3. Push to `main` branch
4. **Immediately apply branch protection:** `./scripts/apply-branch-protection.sh {tool-name}`
5. Create feature branch for future work
6. All changes now require PR + approval

### For Existing Tools

1. Audit current protection: `gh api repos/tuulbelt/{repo}/branches/main/protection`
2. Apply standard rules: `./scripts/apply-branch-protection.sh {repo}`
3. Verify via GitHub Web UI
4. Test by attempting direct push (should fail)

### Periodic Audits

Run this monthly to ensure all repos are protected:

```bash
./scripts/apply-branch-protection.sh > protection-audit-$(date +%Y-%m-%d).log
```

Review the log for any skipped or failed repos.

---

## Related Documentation

- **New Tool Creation:** `.claude/commands/new-tool.md`
- **MCP Server:** `.claude/mcp/tuulbelt-github/README.md`
- **Quality Checklist:** `docs/QUALITY_CHECKLIST.md`

---

**Last Updated:** 2026-01-09
**Maintained By:** Tuulbelt Core Team
