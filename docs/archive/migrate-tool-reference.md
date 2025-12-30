# /migrate-tool - Migrate Monorepo Tool to Standalone Repository

> **⚠️ ARCHIVED:** This command completed its purpose. All 10 tools have been migrated to standalone repositories (Phase 2 complete, 2025-12-29). This documentation is preserved for reference only.

Automates the complete migration of a tool from the monorepo structure to a standalone GitHub repository with git submodule integration.

## Usage

```bash
/migrate-tool <tool-name>
```

**Example:**
```bash
/migrate-tool cli-progress-reporting
```

## What This Command Does

This command automates the entire migration workflow:

1. **Extract Git History**
   - Uses `git subtree split` to extract tool's commit history
   - Preserves all commits, authors, and dates
   - Creates temporary branch with extracted history

2. **Create GitHub Repository**
   - Creates new repo: `tuulbelt/{tool-name}` (public)
   - Sets repository description (short tool summary)
   - Adds GitHub topics (e.g., tuulbelt, typescript, zero-dependencies, tool-specific keywords)
   - Configures repository settings:
     - **Disables issues** (all issues go to meta repo)
     - **Disables wiki** (docs in README and VitePress)
     - **Disables projects** (tracking in meta repo)
   - Sets homepage URL to repository README

3. **Prepare Standalone Repository**
   - Clones new empty repo to `/tmp/{tool-name}`
   - Pulls extracted git history
   - Updates metadata for standalone use:
     - **package.json or Cargo.toml**: homepage, bugs, repository URLs
     - **CI workflow**: standalone paths, multi-version matrix, zero-dep check
     - **README.md**:
       - Update badge URLs to point to standalone repo (not monorepo workflows)
       - Convert relative links to absolute GitHub URLs where needed
       - Keep meta repo reference link (appropriate)
     - **CLAUDE.md**: Creates tool-specific development context file

4. **Commit and Release**
   - Commits all changes using `scripts/commit.sh`
   - Tags v0.1.0 in new repository
   - Pushes to GitHub with correct credentials

5. **Verify Standalone Functionality**
   - Fresh clone from GitHub
   - Runs tests to verify everything works
   - Checks TypeScript compilation (for TS tools)
   - Verifies build succeeds

6. **Add Git Submodule**
   - Adds tool as git submodule: `tools/{tool-name}`
   - Creates `.gitmodules` entry
   - Commits submodule addition to meta repo

7. **Update Tracking Documents**
   - **ALL FOUR documents must be updated** (critical for session continuity):
     - **`.claude/HANDOFF.md`**: Update session summary, mark tool complete, update Wave 1 progress
     - **`STATUS.md`**: Update current phase, tool count (e.g., 2/7 complete)
     - **`CHANGELOG.md`**: Add complete migration entry with commit count, test results
     - **`.claude/NEXT_TASKS.md`**: Move tool from pending to completed, update remaining count
   - Commits tracking updates using `scripts/commit.sh`
   - Pushes to meta repo using `scripts/push.sh`

## Prerequisites

Before running this command:

1. **Authentication configured:**
   ```bash
   source scripts/setup-github-auth.sh
   ```

2. **Tool exists in monorepo:**
   - Tool directory exists (e.g., `cli-progress-reporting/`)
   - Tool has tests passing
   - Tool has proper package.json or Cargo.toml

3. **GitHub CLI authenticated:**
   ```bash
   gh auth status
   ```

## Expected Output

The command will output progress for each step:

```
✓ Extracting git history (442 commits found)
✓ Creating GitHub repository: tuulbelt/cli-progress-reporting
✓ Cloning and preparing standalone repo
✓ Updating package.json with repository URLs
✓ Updating CI workflow for standalone
✓ Updating README.md with absolute URLs
✓ Creating CLAUDE.md
✓ Committing changes with correct author
✓ Tagging v0.1.0
✓ Pushing to GitHub
✓ Verifying standalone: 121/121 tests passing
✓ Adding git submodule to meta repo
✓ Updating tracking documents
✓ Committing and pushing tracking updates

Migration complete! Repository: https://github.com/tuulbelt/cli-progress-reporting
```

## What Gets Created

**In GitHub:**
- New repository: `https://github.com/tuulbelt/{tool-name}`
- Tagged release: v0.1.0
- CI workflow configured and running

**In Meta Repo:**
- Git submodule: `tools/{tool-name}/`
- Updated `.gitmodules`
- Updated tracking documents (HANDOFF.md, STATUS.md, CHANGELOG.md, NEXT_TASKS.md)

## Error Handling

The command will stop and report errors if:

- Tool directory doesn't exist
- Git history extraction fails
- GitHub repository creation fails
- Tests fail in standalone verification
- Submodule addition fails
- Tracking document updates fail

After fixing the issue, you can re-run the command. It's safe to retry.

## Manual Override

If you need to manually perform any step:

```bash
# Extract history only
git subtree split -P {tool-name} -b {tool-name}-history

# Create repo only
gh repo create tuulbelt/{tool-name} --public

# Add submodule only
git submodule add https://github.com/tuulbelt/{tool-name}.git tools/{tool-name}
```

## Post-Migration Verification

After migration completes, verify:

1. **GitHub Repository:**
   - Visit https://github.com/tuulbelt/{tool-name}
   - Check CI is passing
   - Verify README displays correctly

2. **Standalone Clone:**
   ```bash
   cd /tmp
   git clone https://github.com/tuulbelt/{tool-name}.git
   cd {tool-name}
   npm test  # or cargo test
   ```

3. **Git Submodule:**
   ```bash
   cd tools/{tool-name}
   git log --oneline -5
   ```

4. **Tracking Documents:**
   - Check `.claude/HANDOFF.md` shows tool complete
   - Check `STATUS.md` reflects migration progress
   - Check `CHANGELOG.md` has migration entry

## Migration Phases

This command is used in **Phase 2** of the meta repository migration:

- **Phase 2 Wave 1:** 7 independent tools (no dependencies)
- **Phase 2 Wave 2:** Tools with optional dependencies
- **Phase 2 Wave 3:** Tools with required dependencies

See `docs/MIGRATION_TO_META_REPO.md` for complete migration plan.

## Authentication Scripts

This command uses the following authentication scripts:

- `scripts/setup-github-auth.sh` - Configure session from .env
- `scripts/commit.sh` - Commit with correct author (from .env), no Claude attribution
- `scripts/push.sh` - Push with correct credentials

All scripts source `GITHUB_TOKEN`, `GITHUB_USERNAME`, `GITHUB_EMAIL` from meta repo `.env`.

## Workflow Integration

**Before migration:**
```bash
# 1. Ensure tool is complete
cd {tool-name}
npm test  # or cargo test
npm run build

# 2. Configure authentication
cd /path/to/tuulbelt
source scripts/setup-github-auth.sh

# 3. Run migration
/migrate-tool {tool-name}
```

**After migration:**
- Tool is standalone and independently cloneable
- Meta repo references tool via git submodule
- Ready to migrate next tool with fresh session

## Related Commands

- `/new-tool` - Create brand new tool from template
- `/release-tool` - Manage semantic versioning
- `/add-tool-dependency` - Add git URL dependencies
- `/sync-tool-docs` - Sync documentation changes

## Implementation Notes

The command implementation should:

1. **Use existing scripts** - Leverage `scripts/commit.sh` and `scripts/push.sh`
   - **Alternative commit method**: Using `git commit` with `GIT_AUTHOR_NAME`, `GIT_AUTHOR_EMAIL`, `GIT_COMMITTER_NAME`, and `GIT_COMMITTER_EMAIL` environment variables is acceptable and produces identical results
   - Example: `GIT_AUTHOR_NAME="$GITHUB_USERNAME" GIT_AUTHOR_EMAIL="$GITHUB_EMAIL" GIT_COMMITTER_NAME="$GITHUB_USERNAME" GIT_COMMITTER_EMAIL="$GITHUB_EMAIL" git commit -m "message"`
   - Both methods ensure correct author attribution; prefer scripts for consistency
2. **Be idempotent** - Safe to re-run if it fails partway through
3. **Validate inputs** - Check tool exists, .env configured, gh authenticated
4. **Provide clear progress** - Show what's happening at each step
5. **Handle both languages** - Support TypeScript and Rust tools
6. **Atomic operations** - Use transactions where possible (git operations)
7. **Clean up on failure** - Remove temporary branches, partial repos

## Lessons Learned (From cli-progress-reporting and cross-platform-path-normalizer Migrations)

**Authentication:**
- **CRITICAL: Chain source with gh commands**: Each `gh` command must be chained with `source scripts/setup-github-auth.sh &&` because Claude Code runs each Bash command in a separate shell
- **Pattern**: `source scripts/setup-github-auth.sh && gh repo create ...`
- **Why**: Environment variables don't persist between separate Bash tool calls
- **Alternative: Use MCP server**: The custom GitHub MCP server (`.claude/mcp/tuulbelt-github/`) reads from `.env` automatically
- **Verification**: `source scripts/setup-github-auth.sh && gh api user --jq '.login'` should show project username

**GitHub Configuration:**
- **Not automatic**: Repository settings (issues/wiki/projects), topics, and description must be explicitly configured after repo creation
- **Verification required**: Check GitHub web UI or use `gh repo view` to verify all settings applied correctly

**README Updates:**
- **Badge URLs critical**: Badges must point to standalone repo workflows, not monorepo `test-all-tools.yml`
- **Meta repo links OK**: Keeping "Part of Tuulbelt" link to meta repo is appropriate and expected

**Tracking Documents:**
- **All 4 must be updated**: HANDOFF.md, STATUS.md, CHANGELOG.md, NEXT_TASKS.md
- **Session continuity**: These documents preserve context across sessions after migrations
- **Gap analysis essential**: Always do comprehensive review against specification before considering migration complete

**Testing:**
- **Fresh clone required**: Don't trust local state - always verify with fresh clone from GitHub
- **Full test suite**: Run complete test suite (unit + integration + CLI) in standalone context
- **Build verification**: Verify TypeScript compilation and build succeed standalone

**Cleanup:**
- **Delete temporary branch**: After successful push, delete the temporary history branch (e.g., `git branch -D {tool-name}-history`)
- **Verify description consistency**: GitHub repository description should match package.json/Cargo.toml description field

**Rust-Specific CI:**
- **Zero-dep check template fixed**: Rust template now includes working awk-based pattern
- **TOML parsing fragile**: Avoid grep patterns on `[dependencies]`; use awk to properly handle TOML sections
- **False positives**: Early patterns incorrectly matched `[profile.release]` section lines as dependencies

## Future Enhancements

Potential improvements:

- Dry-run mode: `--dry-run` flag to preview changes
- Parallel migration: Migrate multiple tools at once
- Rollback support: Undo migration if issues found
- Dependency resolution: Automatically detect and warn about tool dependencies
- CI integration: Trigger meta repo CI after migration
- Automated gap analysis: Built-in checklist verification

---

**Created:** 2025-12-29
**Last Updated:** 2025-12-29 (Enhanced after cross-platform-path-normalizer migration)
**Status:** Active - Required for Phase 2 Wave 1 migrations
