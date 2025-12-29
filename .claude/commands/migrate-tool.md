# /migrate-tool - Migrate Monorepo Tool to Standalone Repository

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
   - Creates new repo: `tuulbelt/{tool-name}`
   - Configures description, topics, homepage
   - Sets up repository settings (issues disabled, points to meta repo)

3. **Prepare Standalone Repository**
   - Clones new empty repo to `/tmp/{tool-name}`
   - Pulls extracted git history
   - Updates metadata for standalone use:
     - package.json or Cargo.toml (homepage, bugs, repository URLs)
     - CI workflow (standalone paths, multi-version matrix)
     - README.md (badges, absolute GitHub URLs)
     - Creates CLAUDE.md with tool-specific context

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
   - Updates `.claude/HANDOFF.md` with migration progress
   - Updates `STATUS.md` with current phase status
   - Updates `CHANGELOG.md` with migration entry
   - Updates `.claude/NEXT_TASKS.md` marking tool complete
   - Commits and pushes tracking updates

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
- `scripts/commit.sh` - Commit with correct author (koficodedat), no Claude attribution
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
cd /Users/kofi/_/tuulbelt
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
2. **Be idempotent** - Safe to re-run if it fails partway through
3. **Validate inputs** - Check tool exists, .env configured, gh authenticated
4. **Provide clear progress** - Show what's happening at each step
5. **Handle both languages** - Support TypeScript and Rust tools
6. **Atomic operations** - Use transactions where possible (git operations)
7. **Clean up on failure** - Remove temporary branches, partial repos

## Future Enhancements

Potential improvements:

- Dry-run mode: `--dry-run` flag to preview changes
- Parallel migration: Migrate multiple tools at once
- Rollback support: Undo migration if issues found
- Dependency resolution: Automatically detect and warn about tool dependencies
- CI integration: Trigger meta repo CI after migration

---

**Created:** 2025-12-29
**Last Updated:** 2025-12-29
**Status:** Active - Required for Phase 2 Wave 1 migrations
