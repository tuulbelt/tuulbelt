# Migration to Meta Repository: Strategic Decisions

**Status:** Phase 0 Complete
**Date:** 2025-12-29

This document records key strategic decisions made during the migration from monorepo to meta repository architecture.

---

## Decision 0.4: Issue Tracking Strategy

**Problem:** After migration to individual tool repositories, where should users file issues?

### Options Considered

1. **Centralized (in meta repo `tuulbelt/tuulbelt`)**
   - ✅ Single place to search for all issues
   - ✅ Cross-tool issues easy to track
   - ✅ Users don't need to figure out which repo
   - ❌ May get cluttered at scale
   - ❌ Requires GitHub labels for organization

2. **Per-Tool (in each tool's repo)**
   - ✅ Cleaner separation
   - ✅ Tool-specific context stays with tool
   - ❌ Cross-tool issues harder to track
   - ❌ Users must find correct repo
   - ❌ Harder to get overview of all issues

3. **Hybrid (general in meta, specific in tool repos)**
   - ✅ Most flexible
   - ❌ Most complex
   - ❌ Confusing for users (where to file?)

### Decision: ✅ Centralized (Option 1)

**Rationale:**
- Lower barrier for users (don't need to choose repo)
- Easier to track cross-tool issues (e.g., dependencies)
- Can migrate to per-tool later if volume increases
- Similar to how React, Vue, and Babel manage monorepos

**Implementation Plan:**

1. **All issues filed in:** `https://github.com/tuulbelt/tuulbelt/issues`

2. **GitHub Labels Created:**
   - `tool:test-flakiness-detector`
   - `tool:cli-progress-reporting`
   - `tool:cross-platform-path-normalizer`
   - `tool:config-file-merger`
   - `tool:structured-error-handler`
   - `tool:file-based-semaphore`
   - `tool:file-based-semaphore-ts`
   - `tool:output-diffing-utility`
   - `tool:snapshot-comparison`
   - `tool:test-port-resolver`

3. **Each Tool's README.md Updated:**
   ```markdown
   ## Issues & Support
   
   Found a bug? Have a suggestion?
   - [Open an issue](https://github.com/tuulbelt/tuulbelt/issues/new)
   - Use label: `tool:<tool-name>`
   ```

4. **Individual Tool Repos:**
   - Issues disabled in GitHub settings
   - README redirects to meta repo issues

**Migration Task:** Phase 2 (when creating tool repos)

---

## Decision 0.5: Git Tagging Strategy

**Problem:** Should we create git tags in the monorepo before migration, or create them in individual tool repos after migration?

### Options Considered

1. **Pre-Migration Tagging (in monorepo)**
   ```bash
   git tag test-flakiness-detector-v0.1.0 <commit-hash>
   git tag cli-progress-reporting-v0.1.0 <commit-hash>
   # etc.
   ```
   - ✅ Preserves exact commit history with tags
   - ✅ Can reference specific releases in migration
   - ❌ More complex (need to find correct commit hashes)
   - ❌ Tags don't transfer when using `git subtree split`

2. **Post-Migration Tagging (in new repos)**
   ```bash
   # After migration, in each tool repo:
   git tag v0.1.0
   git push origin v0.1.0
   ```
   - ✅ Simpler and cleaner
   - ✅ Tags start fresh in new repos
   - ✅ Easier to automate
   - ❌ Loses exact commit history reference
   - ❌ Can't reference pre-migration tags

### Decision: ✅ Post-Migration Tagging (Option 2)

**Rationale:**
- All tools are currently at v0.1.0 (unstable API)
- Exact historical commit references less critical at this stage
- Simpler workflow reduces migration complexity
- Tags in new repos will be cleaner and easier to navigate
- Can document monorepo commit references in CHANGELOG.md if needed

**Implementation Plan:**

1. **No Pre-Migration Actions**
   - Don't create tags in monorepo
   - Document current status in CHANGELOG.md

2. **During Migration (Phase 3):**
   - After copying content to tool repo
   - Before pushing to origin:
     ```bash
     cd /path/to/tool-repo
     git tag v0.1.0
     git push origin main
     git push origin v0.1.0
     ```

3. **CHANGELOG.md in Each Tool:**
   - Document migration date and monorepo reference:
     ```markdown
     ## [0.1.0] - 2025-12-29
     
     ### Migrated
     - Migrated from monorepo tuulbelt/tuulbelt
     - Original monorepo commit: abc1234
     - All previous development history preserved in monorepo
     ```

4. **Future Releases:**
   - Use semantic versioning normally (v0.2.0, v1.0.0, etc.)
   - Create tags in individual tool repos
   - Use GitHub Releases for visibility

**Migration Task:** Phase 3 (when migrating content to tool repos)

---

## Decision Record

| Decision | Status | Date | Phase |
|----------|--------|------|-------|
| 0.4: Issue Tracking Strategy | ✅ Centralized | 2025-12-29 | Phase 0 |
| 0.5: Git Tagging Strategy | ✅ Post-Migration | 2025-12-29 | Phase 0 |

---

## Reversal Criteria

These decisions can be revisited if:

### Issue Tracking (0.4)
- **Trigger:** Issue volume exceeds 100+ issues
- **Indicator:** Labels become unmanageable
- **Action:** Migrate to per-tool issues, keep meta repo for cross-tool issues

### Git Tagging (0.5)
- **Trigger:** Need to reference exact historical commits becomes critical
- **Indicator:** Users frequently asking "what changed in v0.1.0?"
- **Action:** Document monorepo commit references in tool README.md or CHANGELOG.md

---

## Related Documents

- **Migration Plan:** `docs/MIGRATION_TO_META_REPO.md`
- **Tool Repo Settings:** `docs/setup/TOOL_REPO_SETTINGS.md`
- **Principles:** `PRINCIPLES.md`
- **ROADMAP:** `ROADMAP.md`

---

**Last Updated:** 2025-12-29
**Next Review:** After Phase 3 completion (when tool repos are created)
