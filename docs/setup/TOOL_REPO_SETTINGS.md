# Tool Repository GitHub Settings Template

This document provides the standard GitHub repository settings for all Tuulbelt tool repositories.

## Purpose

When creating a new tool repository (during meta repo migration or for new tools), these settings ensure consistency across all Tuulbelt tools.

---

## Repository Settings

### General

| Setting | Value | Notes |
|---------|-------|-------|
| **Repository name** | `tool-name` | Lowercase, hyphen-separated (e.g., `test-flakiness-detector`) |
| **Description** | Brief one-line description | e.g., "Detect unreliable tests by running them N times" |
| **Website** | https://tuulbelt.github.io/tuulbelt/tools/{tool-name}/ | Link to VitePress docs |
| **Topics** | `tuulbelt`, `zero-dependencies`, `cli-tool`, `{language}` | Add relevant tags |
| **Default branch** | `main` | Standard |
| **Visibility** | Public | All Tuulbelt tools are open source |

### Features

| Feature | Enabled? | Notes |
|---------|----------|-------|
| **Issues** | ✅ Yes | Enable for tool-specific bug reports |
| **Discussions** | ❌ No | Use meta repo for discussions |
| **Projects** | ❌ No | Use meta repo for project tracking |
| **Wiki** | ❌ No | Use VitePress docs instead |
| **Sponsorships** | ❌ No | Optional per maintainer preference |

### Pull Requests

| Setting | Value | Notes |
|---------|-------|-------|
| **Allow merge commits** | ✅ Yes | Default merge strategy |
| **Allow squash merging** | ✅ Yes | For cleaning up commit history |
| **Allow rebase merging** | ✅ Yes | For linear history |
| **Automatically delete head branches** | ✅ Yes | Keep repo clean |
| **Allow auto-merge** | ❌ No | Manual review required |

---

## Branch Protection (main)

### Protect matching branches

Pattern: `main`

### Branch protection rules

| Rule | Enabled? | Configuration |
|------|----------|---------------|
| **Require pull request before merging** | ⚠️ Optional | Recommended for multi-maintainer tools |
| **Require approvals** | ⚠️ Optional | If PR required: 1 approval minimum |
| **Dismiss stale reviews** | ❌ No | Allow flexibility |
| **Require review from Code Owners** | ❌ No | Single maintainer typically |
| **Require status checks to pass** | ✅ Yes | **Critical for quality** |
| **Require branches to be up to date** | ⚠️ Optional | Prevents conflicts but slower |
| **Require conversation resolution** | ❌ No | Optional |
| **Require signed commits** | ❌ No | Optional security enhancement |
| **Require linear history** | ⚠️ Optional | Cleaner git log but less flexible |
| **Include administrators** | ✅ Yes | Even admins must pass CI |
| **Allow force pushes** | ❌ No | **Critical: Protects history** |
| **Allow deletions** | ❌ No | **Critical: Protects main branch** |

### Required status checks

**TypeScript tools:**
- `test-all-tools / test-{tool-name}` (if using meta repo CI)
- Or: `Test / test` (if tool has own CI)

**Rust tools:**
- `test-all-tools / test-{tool-name}` (if using meta repo CI)
- Or: `Test Rust / test` (if tool has own CI)

---

## GitHub Actions

### Actions permissions

| Permission | Value | Notes |
|------------|-------|-------|
| **Allow all actions and reusable workflows** | ✅ Yes | Standard |
| **Allow GitHub Actions to create PRs** | ✅ Yes | For automated updates (demos, docs) |
| **Allow GitHub Actions to approve PRs** | ❌ No | Manual approval required |

### Workflow permissions

| Permission | Value | Notes |
|------------|-------|-------|
| **Read repository contents** | ✅ Required | For all workflows |
| **Write repository contents** | ✅ Required | For demo updates, releases |
| **Read and write permissions** | ✅ Yes | Default for most workflows |

---

## GitHub Pages

### Pages settings

| Setting | Value | Notes |
|---------|-------|-------|
| **Source** | GitHub Actions | Modern deployment method |
| **Build and deployment** | GitHub Actions | Uses VitePress workflow |
| **Enforce HTTPS** | ✅ Yes | Security best practice |
| **Custom domain** | ❌ No | Use tuulbelt.github.io/tuulbelt/tools/{tool-name}/ |

### Deployment

Pages are deployed from the **meta repository** (`tuulbelt/tuulbelt`), not individual tool repos.

Individual tool repos do NOT need Pages enabled unless they have standalone documentation.

---

## Secrets and Variables

### Repository secrets

No secrets required for basic tool repos. The meta repo handles deployment.

### Environment secrets (if needed)

| Secret | Purpose | Notes |
|--------|---------|-------|
| `GITHUB_TOKEN` | Automatic | Provided by GitHub Actions |

---

## Security

### Security settings

| Setting | Enabled? | Notes |
|---------|----------|-------|
| **Private vulnerability reporting** | ✅ Yes | Allow security researchers to report |
| **Dependabot alerts** | ⚠️ Optional | Only for devDependencies |
| **Dependabot security updates** | ⚠️ Optional | Auto-update vulnerable deps |
| **Dependabot version updates** | ❌ No | Manual updates preferred (zero deps) |

### Security policy

Each tool repo should have a `SECURITY.md` file:

```markdown
# Security Policy

## Reporting a Vulnerability

Please report security vulnerabilities to:
- Open an issue at https://github.com/tuulbelt/tuulbelt/issues (for non-sensitive issues)
- Or email [maintainer email] for sensitive disclosures

## Supported Versions

Only the latest version is supported with security updates.

## Security Standards

This tool follows Tuulbelt security standards:
- Zero runtime dependencies (eliminates supply chain attacks)
- Input validation at all boundaries
- No hardcoded secrets
- Safe file operations (path traversal prevention)

See docs/security-guidelines.md in the meta repo for details.
```

---

## Labels

### Recommended labels

| Label | Color | Description |
|-------|-------|-------------|
| `bug` | #d73a4a | Something isn't working |
| `documentation` | #0075ca | Improvements or additions to documentation |
| `enhancement` | #a2eeef | New feature or request |
| `good first issue` | #7057ff | Good for newcomers |
| `help wanted` | #008672 | Extra attention is needed |
| `question` | #d876e3 | Further information is requested |
| `wontfix` | #ffffff | This will not be worked on |
| `duplicate` | #cfd3d7 | This issue or pull request already exists |
| `invalid` | #e4e669 | This doesn't seem right |
| `security` | #ee0701 | Security-related issue |
| `performance` | #fbca04 | Performance improvement |
| `test` | #d4c5f9 | Related to tests |

---

## Automation with `gh` CLI

### Create repository with settings

```bash
# Create new tool repo
gh repo create tuulbelt/test-flakiness-detector \
  --public \
  --description "Detect unreliable tests by running them N times" \
  --homepage "https://tuulbelt.github.io/tuulbelt/tools/test-flakiness-detector/"

# Enable/disable features
gh repo edit tuulbelt/test-flakiness-detector \
  --enable-issues \
  --disable-wiki \
  --disable-projects

# Add topics
gh repo edit tuulbelt/test-flakiness-detector \
  --add-topic tuulbelt \
  --add-topic zero-dependencies \
  --add-topic cli-tool \
  --add-topic typescript
```

### Set up branch protection

```bash
# Enable branch protection with required status checks
gh api repos/tuulbelt/test-flakiness-detector/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":false,"contexts":["test-all-tools / test-test-flakiness-detector"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews=null \
  --field restrictions=null \
  --field allow_force_pushes=false \
  --field allow_deletions=false
```

---

## Checklist for New Tool Repo

Use this checklist when creating a new tool repository:

### Initial Setup
- [ ] Create repository with correct name (lowercase, hyphenated)
- [ ] Add description and website URL
- [ ] Add topics (tuulbelt, zero-dependencies, cli-tool, language)
- [ ] Set default branch to `main`

### Features
- [ ] Enable Issues
- [ ] Disable Discussions, Projects, Wiki

### Branch Protection
- [ ] Protect `main` branch
- [ ] Require status checks to pass
- [ ] Add required status check: `test-all-tools / test-{tool-name}`
- [ ] Enable "Include administrators"
- [ ] Disable force pushes
- [ ] Disable deletions

### Actions
- [ ] Allow all actions
- [ ] Allow GitHub Actions to create PRs
- [ ] Set workflow permissions to read/write

### Security
- [ ] Enable private vulnerability reporting
- [ ] Add SECURITY.md file
- [ ] (Optional) Enable Dependabot alerts for devDependencies

### Documentation
- [ ] Add README.md with description, installation, usage
- [ ] Add LICENSE (MIT)
- [ ] Add SECURITY.md
- [ ] Add CONTRIBUTING.md with versioning section

### Meta Repo Integration
- [ ] Add tool to meta repo README.md
- [ ] Add tool to docs/tools/index.md (VitePress)
- [ ] Create VitePress docs pages
- [ ] Add to ROADMAP.md
- [ ] Update NEXT_TASKS.md

---

## References

- **GitHub Docs:** https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features
- **gh CLI Docs:** https://cli.github.com/manual/
- **Branch Protection:** https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches
- **GitHub Actions:** https://docs.github.com/en/actions

---

**Last Updated:** 2025-12-29
**Next Review:** After first tool migration
