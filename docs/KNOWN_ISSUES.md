# Known Issues

**Last Updated:** 2025-12-29

This document tracks known bugs, limitations, and cosmetic issues across the Tuulbelt project. Issues are categorized by severity and area.

---

## 🔴 Critical Issues

**None currently.**

---

## 🟡 Medium Priority Issues

### 2. Demo GIFs Not Deployed to GitHub Pages After create-demos Workflow

**Status:** Open
**Severity:** Medium
**Area:** CI/CD (create-demos.yml, deploy-docs.yml)
**Affects:** All tool VitePress documentation pages

**Description:**

When the `create-demos.yml` workflow runs and updates demo recordings, the demo GIFs appear correctly in the GitHub README but NOT on the VitePress GitHub Pages site. The GIF is present in `docs/public/{tool}/demo.gif` but not visible on the live site.

**Root Cause:**

The `create-demos.yml` workflow commits with `[skip ci]` in the commit message:
```
docs: update demo recordings and embed in READMEs [skip ci]
```

This prevents `deploy-docs.yml` from running after demo updates, so the new GIFs are in the repository but never deployed to GitHub Pages.

**Location:**
- `.github/workflows/create-demos.yml` (line ~331)
- Affects: `docs/public/*/demo.gif` files

**Evidence:**
- GitHub README shows GIF correctly (uses relative path `docs/demo.gif`)
- GitHub README shows correct asciinema link
- VitePress page shows no GIF (references `/tool-name/demo.gif` which needs deployment)
- VitePress asciinema link still shows `(#)` placeholder (clicking reloads page)
- Repository has correct content, but deployed site has stale content

**Workaround:**

1. Manually trigger `deploy-docs` workflow after `create-demos` completes
2. Or make any trivial docs change to trigger deployment

**Proposed Fix:**

Option A: Remove `[skip ci]` from the demo commit message (may cause CI loops)

Option B: Have `create-demos.yml` trigger `deploy-docs.yml` via `workflow_call` or `repository_dispatch`

Option C: Add `docs/public/**` to `deploy-docs.yml` path triggers (current paths only include `docs/**` but not specific public subdirectories for tool demos)

**References:**
- `create-demos.yml` commit step (line ~331)
- `deploy-docs.yml` path triggers

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
**Resolved:** 2

**By Area:**
- Documentation: 1
- Core: 1 (resolved)
- Testing: 0
- CI/CD: 1

---

## Related Documents

- **HANDOFF.md** - Session handoff with current work status
- **NEXT_TASKS.md** - Task backlog (bugs → tasks)
- **QUALITY_CHECKLIST.md** - Quality standards and common pitfalls
- **CONTRIBUTING.md** - How to contribute fixes

---

**Last Review:** 2025-12-27
**Next Review:** When new issues discovered or existing issues resolved
