# /release-tool Command

Release a new version of a Tuulbelt tool.

## Usage

```
/release-tool <tool-name> <version> [changelog-entry]
```

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| tool-name | Yes | Tool to release | `test-flakiness-detector` |
| version | Yes | Semver version | `0.2.0` or `1.0.0` |
| changelog-entry | No | What changed | `"Add timeout support"` |

## Examples

```bash
# Minor version with changelog
/release-tool test-flakiness-detector 0.2.0 "Add --timeout flag for long-running tests"

# Patch version
/release-tool cli-progress-reporting 0.1.1 "Fix race condition in concurrent updates"

# Major version (breaking change)
/release-tool cross-platform-path-normalizer 1.0.0 "First stable release"
```

## What This Command Does

### 1. Validation
- Verify tool exists in `tools/` directory
- Validate version format (semver)
- Check version is greater than current version
- Ensure git status is clean

### 2. Update Version
**TypeScript:**
```bash
cd tools/{tool-name}
npm version {version} --no-git-tag-version
```

**Rust:**
```bash
cd tools/{tool-name}
# Update Cargo.toml version field
sed -i 's/^version = .*/version = "{version}"/' Cargo.toml
cargo build  # Update Cargo.lock
```

### 3. Update CHANGELOG.md
```markdown
## [{version}] - {YYYY-MM-DD}

### {Category}
- {changelog-entry or inferred from commits}
```

Categories: Added, Changed, Fixed, Removed, Security, Deprecated

### 4. Update README.md Badge (if present)
```markdown
🟢 v{old-version} → 🟢 v{version}
```

### 5. Create Commit
```bash
git add package.json CHANGELOG.md README.md  # or Cargo.toml
git commit -m "chore(tool): release v{version}

{changelog-entry}

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

### 6. Create Git Tag
```bash
git tag v{version}
```

### 7. Push
```bash
git push origin main
git push origin v{version}
```

### 8. Update Meta Repo (Optional)
If tool version referenced in meta repo docs:
```bash
cd ../../  # back to meta repo
# Update README.md or docs/ if version mentioned
git commit -m "docs: update {tool-name} to v{version}"
git push
```

## Semver Guidelines

Tuulbelt tools follow [Semantic Versioning](https://semver.org/):

| Version Type | When to Use | Example |
|--------------|-------------|---------|
| `0.x.y` | Pre-1.0 development (API may change) | 0.1.0 → 0.2.0 |
| `1.0.0` | First stable release | 0.9.0 → 1.0.0 |
| `x.Y.0` | Add backward-compatible functionality | 1.0.0 → 1.1.0 |
| `x.y.Z` | Backward-compatible bug fixes | 1.1.0 → 1.1.1 |
| `X.0.0` | Breaking changes | 1.5.0 → 2.0.0 |

**Examples:**

- `0.1.0 → 0.2.0`: Added new CLI flags (pre-stable)
- `0.9.0 → 1.0.0`: API is stable, production-ready
- `1.0.0 → 1.1.0`: Added new optional features
- `1.1.0 → 1.1.1`: Fixed bug in existing feature
- `1.5.0 → 2.0.0`: Removed deprecated API, renamed flags

## Quality Gates

Before releasing, the command verifies:

- [ ] Git status is clean (no uncommitted changes)
- [ ] All tests pass (`npm test` or `cargo test`)
- [ ] Build succeeds (`npm run build` or `cargo build`)
- [ ] Version is valid semver and > current version
- [ ] CHANGELOG.md exists and is valid
- [ ] No security vulnerabilities (`npm audit` or `cargo audit`)

If any gate fails, the release is aborted.

## Post-Release Tasks

After the command completes:

1. **Verify GitHub Release**
   - Check if GitHub Actions created a release automatically
   - Add release notes if needed: `gh release edit v{version}`

2. **Update Documentation**
   - Verify VitePress updated: https://tuulbelt.github.io/tuulbelt/tools/{tool-name}/
   - Check demo still works

3. **Announce (for major releases)**
   - Update README.md in meta repo
   - Add to ROADMAP.md if milestone

4. **Consider Publishing (future)**
   - npm: `npm publish` (when ready for registry)
   - crates.io: `cargo publish` (when ready)

## Changelog Inference

If no changelog entry is provided, the command infers from git commits since last tag:

```bash
git log v{last-version}..HEAD --pretty=format:"%s"
```

Example:
```
fix: handle empty input
feat: add --verbose flag
test: add edge case coverage
```

Inferred changelog:
```markdown
## [0.2.0] - 2025-12-29

### Added
- Verbose logging flag

### Fixed
- Handle empty input correctly

### Changed
- Improved edge case test coverage
```

## Troubleshooting

### "Version not greater than current"
```
ERROR: Version 0.1.5 is not greater than current version 0.2.0

Check current version:
- TypeScript: cat tools/{tool-name}/package.json | grep version
- Rust: cat tools/{tool-name}/Cargo.toml | grep ^version

Use a higher version number.
```

### "Git status not clean"
```
ERROR: Git working directory has uncommitted changes.

Fix:
1. Review changes: git status
2. Commit or stash: git commit -am "..." or git stash
3. Retry release command
```

### "Tests failing"
```
ERROR: Tests must pass before releasing.

Fix:
1. Run tests: npm test (or cargo test)
2. Fix failing tests
3. Commit fixes
4. Retry release command
```

### "Tag already exists"
```
ERROR: Tag v0.2.0 already exists.

Options:
1. Delete tag: git tag -d v0.2.0 && git push origin :refs/tags/v0.2.0
2. Use a different version number
```

## Related Commands

- `/new-tool` - Create new tool from template
- `/add-tool-dependency` - Add dependency to tool
- `/update-all-counts` - Update version references across docs
- `/quality-check` - Run pre-release quality checks

## References

- Semver Specification: https://semver.org/
- Conventional Commits: https://www.conventionalcommits.org/
- Keep a Changelog: https://keepachangelog.com/
- Tuulbelt Versioning: `CONTRIBUTING.md` (in tool repos)
