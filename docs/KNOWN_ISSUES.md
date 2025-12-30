# Known Issues

**Last Updated:** 2025-12-29

This document tracks known bugs, limitations, and cosmetic issues across the Tuulbelt project. Issues are categorized by severity and area.

---

## 🔴 Critical Issues

**None currently.**

---

## 🟡 Medium Priority Issues

### 2. Port Resolver Missing Asciinema URL

**Status:** Fixed (pending CI run to complete)
**Severity:** Medium
**Area:** CI/CD (create-demos.yml)
**Affects:** port-resolver and potentially all tool demos

**Description:**

The port-resolver tool has a placeholder `(#)` for its asciinema link instead of the actual recording URL.

**Root Cause (DISCOVERED 2025-12-30):**

The `create-demos.yml` workflow was looking for tools at the repo root (`$tool/`) but tools are now git submodules in `tools/` directory. This caused the workflow to skip all tools, never creating `demo-url.txt` files.

**Fix Applied (2025-12-30):**

1. **Fixed workflow paths** - Updated `create-demos.yml` to use `tools/$tool` paths:
   - Detection loop (lines 94-137): Now checks `tools/$tool/` instead of root
   - Recording loop (lines 143-171): Uses `tools/$tool/` for npm/cargo operations
   - Copy demos step: Uses `tools/*/` glob pattern
   - Update READMEs step: Uses `tools/*/` glob pattern
   - Update VitePress docs step: Uses `tools/*/` glob pattern
   - Git add command: Uses `tools/*/` patterns

2. **Renamed demo script** - From `record-test-port-resolver-demo.sh` to `record-port-resolver-demo.sh`

On next push to main, the workflow will:
1. Detect the script change and workflow change
2. Properly find tools in `tools/` directory
3. Re-record demos and upload to asciinema.org
4. Create `demo-url.txt` files and update VitePress docs

---

## 🟢 Low Priority Issues (Cosmetic)

### 1. StackBlitz Badge Vertical Alignment

**Status:** Open
**Severity:** Low (Cosmetic)
**Area:** Tool READMEs
**Affects:** All tool README.md files (Demo section)

**Description:**

The "Try it online:" label next to the StackBlitz badge image is not perfectly vertically centered with the badge. The text appears slightly lower than the center of the button image.

**Location:**
- Files: `test-flakiness-detector/README.md`, `cli-progress-reporting/README.md`
- Section: Demo (near end of file)
- Template: `templates/tool-repo-template/README.md`

**Current Implementation:**

```html
<div>
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="..." style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg"
         alt="Open in StackBlitz"
         style="vertical-align: middle;">
  </a>
</div>
```

**Attempted Fixes:**

1. Used `display: flex; align-items: center;` (failed - might not render well in GitHub markdown)
2. Used `display: inline-block; vertical-align: middle;` (current, better but not perfect)
3. Applied `vertical-align: middle` to all elements (still not perfect)

**Hypothesis:**

GitHub's markdown renderer may have limitations on CSS styling, or the StackBlitz badge SVG has internal padding/alignment that affects positioning. Markdown-to-HTML conversion might be stripping some styles.

**Next Steps to Try:**

1. Test rendering on actual GitHub to see if it looks different than local preview
2. Try `vertical-align: text-bottom` or `vertical-align: baseline` variations
3. Add explicit height to the span to match badge height
4. Use a table structure instead of divs
5. Consider using only markdown (no HTML) with creative formatting
6. Check if StackBlitz provides different badge sizes that might align better

**Workaround:**

None currently. Badge is functional and mostly aligned, just not pixel-perfect.

**References:**
- GitHub markdown spec: https://github.github.com/gfm/
- StackBlitz badge: https://developer.stackblitz.com/img/open_in_stackblitz.svg

---

## ✅ Resolved Issues

### Tag Newline Injection in file-based-semaphore ✅

**Resolved:** 2025-12-27
**Severity:** Low (Security)
**Area:** Core (file-based-semaphore)

**Problem:** Tags containing newline characters (`\n` or `\r`) could be used to inject fake key-value pairs into lock files, potentially causing the parser to read attacker-controlled values.

**Root Cause:** The `LockInfo::serialize()` method did not sanitize tag contents before writing to the lock file format.

**Solution:** Updated `src/lib.rs` to sanitize newlines in tags before serialization:
```rust
pub fn serialize(&self) -> String {
    let mut content = format!("pid={}\ntimestamp={}\n", self.pid, self.timestamp);
    if let Some(ref tag) = self.tag {
        // Sanitize newlines to prevent injection of fake keys
        let sanitized_tag = tag.replace('\n', " ").replace('\r', " ");
        content.push_str(&format!("tag={}\n", sanitized_tag));
    }
    content
}
```

**Security Tests Added:** 2 tests in `src/lib.rs` verifying injection prevention.

---

### VitePress Card Icons Not Theme-Adaptive ✅

**Resolved:** 2025-12-26
**Severity:** Low (Cosmetic)
**Area:** Documentation Site (VitePress)

**Problem:** The 6 feature card icons on the VitePress home page did not invert colors when switching between light and dark themes.

**Root Cause:** CSS was targeting `.vp-feature-icon` class, but VitePress actually uses `.VPImage` class within `.VPFeature` container.

**Solution:** Updated `docs/.vitepress/theme/custom.css` to target the correct selectors:
```css
.VPFeature .VPImage {
  filter: invert(1) !important; /* dark mode */
}
```

**Commit:** `fb646ed` - "fix(docs): home page feature icons now adapt to dark/light theme"

---

### Git Push Conflicts During Migration ✅

**Resolved:** 2025-12-29
**Severity:** Medium (Workflow Blocker)
**Area:** Scripts (push.sh)

**Problem:** Every git push attempt failed with "failed to push some refs" error, requiring manual `git pull --rebase origin main` before each push. This occurred repeatedly during Phase 2 Wave 1 migration.

**Root Cause:** GitHub Actions workflows (create-demos.yml, update-dashboard.yml, update-demos.yml) automatically commit demo recordings and dashboard updates to main branch while development work is in progress, causing local branch to fall behind remote.

**Solution:** Updated `scripts/push.sh` to automatically pull and rebase before pushing:
```bash
# Pull latest changes to avoid conflicts (GitHub Actions may have committed)
echo "Pulling latest changes from $REMOTE/$BRANCH..."
git pull --rebase "$REMOTE" "$BRANCH" || {
  echo "ERROR: Failed to pull and rebase. Please resolve conflicts manually."
  exit 1
}
echo ""

# Push
git push "$REMOTE" "$BRANCH"
```

**Benefits:**
- Eliminates manual pull step before every push
- Automatically syncs with CI workflow commits
- Maintains clean linear history with rebase
- Provides clear error message if conflicts occur

---

### On-Stop Cleanup Hook Path Failure ✅

**Resolved:** 2025-12-29
**Severity:** Low (Hook Failure)
**Area:** Hooks (.claude/hooks/on-stop-cleanup.sh)

**Problem:** The on-stop-cleanup.sh hook failed during every session cleanup with path errors. Hook could not find repository at hardcoded path `/home/user/tuulbelt`.

**Root Cause:** Line 11 had hardcoded path that only worked in specific environment:
```bash
TUULBELT_ROOT="/home/user/tuulbelt"  # Only works on original dev machine
```

**Solution:** Replaced hardcoded path with dynamic detection:
```bash
# Dynamically detect repository root (works in both CLI and Web)
TUULBELT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$HOME/tuulbelt")"
```

**How It Works:**
- Uses `git rev-parse --show-toplevel` to find repository root
- Falls back to `$HOME/tuulbelt` if not in a git repository
- Works in Claude Code CLI and Web environments
- Adapts to any directory structure

**Verified:** Hook now runs successfully, detecting repository dynamically and cleaning up temporary files correctly.

---

## 📋 Issue Template

When adding new issues, use this template:

```markdown
### N. Issue Title

**Status:** Open / In Progress / Resolved
**Severity:** Critical / High / Medium / Low
**Area:** Documentation / Core / Testing / CI/CD / etc.
**Affects:** What components/files are affected

**Description:**
Clear description of the issue.

**Location:**
- Files affected
- Line numbers if applicable
- Components involved

**Attempted Fixes:**
1. What was tried
2. What was tried
3. Results

**Hypothesis:**
Why this might be happening.

**Next Steps to Try:**
1. Potential solution 1
2. Potential solution 2

**Workaround:**
If there's a temporary workaround.

**References:**
- Links to relevant docs
- Related issues
- Stack Overflow threads
```

---

## How to Use This Document

### Reporting a New Issue

1. Add to appropriate severity section
2. Fill out complete template
3. Reference from relevant code/docs
4. Update NEXT_TASKS.md if it becomes a task

### Updating an Issue

1. Add attempted fixes to "Attempted Fixes" section
2. Update hypothesis if new information discovered
3. Change severity if impact changes
4. Move to "Resolved" when fixed

### Referencing Issues

**In Code Comments:**
```typescript
// TODO: Fix icon theming (see docs/KNOWN_ISSUES.md #1)
```

**In Documentation:**
```markdown
> **Known Issue:** Icons don't adapt to theme. See [KNOWN_ISSUES.md](../KNOWN_ISSUES.md#1-vitepress-card-icons-not-theme-adaptive) for details.
```

**In Commit Messages:**
```bash
git commit -m "fix: improve icon theming (partial fix for KNOWN_ISSUES.md #1)"
```

---

## Statistics

**Total Issues:** 2
**Critical:** 0
**High:** 0
**Medium:** 1
**Low (Cosmetic):** 1
**Resolved:** 4

**By Area:**
- Documentation: 2
- Core: 0
- Testing: 0
- CI/CD: 0

---

## Related Documents

- **HANDOFF.md** - Session handoff with current work status
- **NEXT_TASKS.md** - Task backlog (bugs → tasks)
- **QUALITY_CHECKLIST.md** - Quality standards and common pitfalls
- **CONTRIBUTING.md** - How to contribute fixes

---

**Last Review:** 2025-12-27
**Next Review:** When new issues discovered or existing issues resolved
