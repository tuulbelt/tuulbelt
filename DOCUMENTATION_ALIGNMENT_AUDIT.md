# Documentation Alignment Audit Report

**Date:** 2025-12-31
**Auditor:** Claude (Systematic Documentation Review)
**Scope:** All 10 Tuulbelt tools - GitHub README (source of truth) vs VitePress documentation
**Root Cause:** Migration from monorepo to standalone repositories left VitePress with old monorepo references

---

## Executive Summary

**STATUS:** ❌ **ALL 10 TOOLS HAVE MIGRATION ARTIFACTS IN VITEPRESS**

All VitePress documentation files contain outdated monorepo references that need to be updated to standalone repository URLs. The GitHub READMEs (source of truth) have been correctly updated, but VitePress docs still point to the old `github.com/tuulbelt/tuulbelt/tree/main/<tool-name>` structure.

### Affected Tools (10/10)
1. test-flakiness-detector ❌
2. cli-progress-reporting ❌
3. cross-platform-path-normalizer ❌
4. file-based-semaphore ❌
5. output-diffing-utility ❌
6. structured-error-handler ❌
7. config-file-merger ❌
8. snapshot-comparison ❌
9. file-based-semaphore-ts ❌
10. port-resolver ❌

---

## Detailed Findings

### Pattern of Migration Artifacts

Each VitePress `docs/tools/<tool-name>/index.md` contains **3 types of outdated references**:

#### 1. Repository Link (Line ~13)
**Current (WRONG):**
```markdown
**Repository:** [tuulbelt/tuulbelt/<tool-name>](https://github.com/tuulbelt/tuulbelt/tree/main/<tool-name>)
```

**Should be:**
```markdown
**Repository:** [tuulbelt/<tool-name>](https://github.com/tuulbelt/<tool-name>)
```

#### 2. Clone Instructions (Quick Start section, Lines ~38-40)
**Current (WRONG):**
```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/<tool-name>
```

**Should be:**
```bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
```

#### 3. StackBlitz Links (Demo section, Line ~79-85)
**Current (WRONG):**
```html
<a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/<tool-name>">
```

**Should be:**
```html
<a href="https://stackblitz.com/github/tuulbelt/<tool-name>">
```

---

## Tool-by-Tool Breakdown

### TypeScript Tools (7 tools)

#### 1. test-flakiness-detector
- **Repository link:** Line 13 ❌ (points to monorepo)
- **Clone command:** Line 38-39 ❌ (clones monorepo)
- **StackBlitz link:** Line 79 ❌ (points to monorepo)
- **GitHub README:** ✅ Correct (standalone repo)

#### 2. cli-progress-reporting
- **Repository link:** Line 13 ❌
- **Clone command:** Line 39-40 ❌
- **StackBlitz link:** Line 85 ❌
- **GitHub README:** ✅ Correct

#### 3. cross-platform-path-normalizer
- **Repository link:** Line 13 ❌
- **Clone command:** Line 45-46 ❌
- **StackBlitz link:** Line 114 ❌
- **GitHub README:** ✅ Correct

#### 4. structured-error-handler
- **Repository link:** Line 13 ❌
- **Clone command:** Line 45-46 ❌ (appears twice: line 45 and line 130)
- **StackBlitz link:** Line 148 ❌
- **GitHub README:** ✅ Correct

#### 5. config-file-merger
- **Repository link:** Line 13 ❌
- **Clone command:** Line 41-42 ❌
- **StackBlitz link:** Line 83 ❌
- **GitHub README:** ✅ Correct

#### 6. file-based-semaphore-ts
- **Repository link:** Line 13 ❌
- **Clone command:** Line 43-44 ❌
- **StackBlitz link:** Line 89 ❌
- **GitHub README:** ✅ Correct

#### 7. port-resolver
- **Repository link:** Line 17 ❌
- **Clone command:** Line 49-50 ❌
- **StackBlitz link:** Not found (TypeScript tool without StackBlitz)
- **GitHub README:** ✅ Correct

### Rust Tools (3 tools)

#### 8. file-based-semaphore
- **Repository link:** Line 13 ❌
- **Clone command:** Line 45-46 ❌ (appears twice: line 45 and line 168)
- **StackBlitz link:** Line 185 ❌
- **GitHub README:** ✅ Correct

#### 9. output-diffing-utility
- **Repository link:** Line 13 ❌
- **Clone command:** Line 49-50 ❌ (appears twice: line 49 and line 163)
- **StackBlitz link:** Line 185 ❌
- **GitHub README:** ✅ Correct

#### 10. snapshot-comparison
- **Repository link:** Line 17 ❌
- **Clone command:** Line 48-49 ❌
- **StackBlitz link:** Line 76 ❌ (includes `?file=examples/basic.rs`)
- **GitHub README:** ✅ Correct

---

## Additional Findings

### Duplicate Clone Commands
Some tools have **duplicate clone commands** in different sections:
- **structured-error-handler:** Lines 45 and 130
- **file-based-semaphore:** Lines 45 and 168
- **output-diffing-utility:** Lines 49 and 163

**Action:** All occurrences must be updated to standalone repo URLs.

### StackBlitz Query Parameters
- **snapshot-comparison** uses: `?file=examples/basic.rs`
- This should be preserved in the corrected URL

### Port-Resolver Missing StackBlitz
- **port-resolver** VitePress docs don't have a StackBlitz link
- GitHub README also doesn't have one
- **No action needed** (consistent)

---

## Verification Against GitHub (Source of Truth)

✅ **All GitHub READMEs are correct** - they use standalone repository URLs:
- Clone: `git clone https://github.com/tuulbelt/<tool-name>.git`
- StackBlitz: `https://stackblitz.com/github/tuulbelt/<tool-name>`
- Repository references point to standalone repos

---

## Impact Assessment

### User Impact
- **High:** Users clicking VitePress links are sent to broken/404 monorepo paths
- **High:** Copy-pasting clone commands from VitePress fails (path doesn't exist in monorepo anymore)
- **Medium:** StackBlitz links may open wrong context or fail

### SEO/Discoverability Impact
- **Medium:** Incorrect canonical URLs in VitePress
- **Low:** GitHub Pages serving outdated references

### Documentation Consistency
- **Critical:** GitHub and VitePress are out of sync
- **Critical:** Users see different instructions depending on entry point

---

## Alignment Plan

### Phase 1: Automated Batch Update (Recommended)

**Strategy:** Create a script to systematically replace all monorepo references with standalone repo URLs.

**Script Logic:**
```bash
for tool in test-flakiness-detector cli-progress-reporting ... (all 10); do
  # Replace repository link
  sed -i "s|tuulbelt/tuulbelt/$tool\](https://github.com/tuulbelt/tuulbelt/tree/main/$tool)|tuulbelt/$tool](https://github.com/tuulbelt/$tool)|g" \
    docs/tools/$tool/index.md

  # Replace clone command (monorepo → standalone)
  sed -i "s|https://github.com/tuulbelt/tuulbelt.git|https://github.com/tuulbelt/$tool.git|g" \
    docs/tools/$tool/index.md

  sed -i "s|cd tuulbelt/$tool|cd $tool|g" \
    docs/tools/$tool/index.md

  # Replace StackBlitz links
  sed -i "s|stackblitz.com/github/tuulbelt/tuulbelt/tree/main/$tool|stackblitz.com/github/tuulbelt/$tool|g" \
    docs/tools/$tool/index.md
done
```

**Advantages:**
- Fast: Updates all 10 tools in seconds
- Consistent: Same pattern applied everywhere
- Auditable: Script can be reviewed before running

**Risks:**
- May miss edge cases (e.g., snapshot-comparison's `?file=` parameter)
- Requires careful testing after execution

### Phase 2: Manual Verification Per Tool (Alternative)

**Strategy:** Manually update each tool's VitePress index.md by comparing with GitHub README.

**Process per tool:**
1. Open GitHub README (source of truth): `tools/<tool-name>/README.md`
2. Open VitePress index: `docs/tools/<tool-name>/index.md`
3. Find and replace:
   - Repository link
   - Clone command(s) - check for duplicates
   - StackBlitz link
4. Commit changes
5. Move to next tool

**Advantages:**
- Careful review of each tool
- Opportunity to spot other inconsistencies
- Less risk of automated errors

**Disadvantages:**
- Time-consuming: ~10-15 min per tool × 10 = 1.5-2.5 hours
- Human error risk (copy-paste mistakes)
- Tedious and repetitive

### Phase 3: Hybrid Approach (Recommended)

**Strategy:** Automated batch update + manual verification of edge cases.

**Steps:**
1. Run automated script for bulk replacements
2. Manually verify tools with known edge cases:
   - snapshot-comparison (StackBlitz query params)
   - structured-error-handler (duplicate clone commands)
   - file-based-semaphore (duplicate clone commands)
   - output-diffing-utility (duplicate clone commands)
3. Test 2-3 VitePress pages in browser
4. Commit all changes in single commit

**Advantages:**
- Speed of automation
- Safety of manual review
- Best of both worlds

---

## Testing Plan

After alignment, verify:

### 1. VitePress Build
```bash
npm run docs:build
# Should complete without dead link errors
```

### 2. Link Validation (Sample 3 tools)
- **test-flakiness-detector:**
  - Click "Repository" link → Should open `https://github.com/tuulbelt/test-flakiness-detector`
  - Click StackBlitz → Should open standalone repo in StackBlitz
- **cli-progress-reporting:**
  - Copy clone command → Should clone standalone repo
- **snapshot-comparison:**
  - StackBlitz link → Should preserve `?file=examples/basic.rs` parameter

### 3. Visual Inspection
- Preview VitePress site: `npm run docs:preview`
- Navigate to 3 random tools
- Verify all links work

---

## Recommended Next Steps

1. **Review this audit** - Confirm findings and approach
2. **Choose alignment strategy** - Automated, manual, or hybrid
3. **Create alignment script** - If using automated/hybrid approach
4. **Execute alignment** - Update all VitePress docs
5. **Test thoroughly** - Verify links, build, visual check
6. **Commit changes** - Single commit: `docs: align VitePress with standalone repo URLs (migration cleanup)`
7. **Deploy and verify** - Check live GitHub Pages site

---

## Appendix: Full List of Affected Files

All files need updates:
```
docs/tools/test-flakiness-detector/index.md
docs/tools/cli-progress-reporting/index.md
docs/tools/cross-platform-path-normalizer/index.md
docs/tools/file-based-semaphore/index.md
docs/tools/output-diffing-utility/index.md
docs/tools/structured-error-handler/index.md
docs/tools/config-file-merger/index.md
docs/tools/snapshot-comparison/index.md
docs/tools/file-based-semaphore-ts/index.md
docs/tools/port-resolver/index.md
```

**Total:** 10 files, ~30 individual replacements required

---

**End of Audit Report**
