# /update-all-counts Command

Update tool counts across all documentation.

## Usage

```
/update-all-counts
```

No parameters required - automatically counts tools and updates all references.

## What This Command Does

Scans `tools/` directory for tool repositories and updates count references across the meta repository.

### 1. Count Tools

```bash
# Count subdirectories in tools/ (after migration)
# Or count subdirectories in root (current monorepo)
TOOL_COUNT=$(ls -d tools/*/ 2>/dev/null | wc -l)
# Or for monorepo: find . -maxdepth 1 -type d (excluding hidden, docs, etc.)
```

Calculate:
- Total tools implemented
- Total target tools (33)
- Percentage complete

### 2. Update README.md (root)

**Location:** Line ~50 (Status section)

```markdown
# Before:
**Progress:** 10 of 33 tools implemented (30%)

# After:
**Progress:** {TOOL_COUNT} of 33 tools implemented ({PERCENTAGE}%)
```

**Location:** Line ~150 (Recently Completed)

```markdown
# Before:
**Recently Completed:** Test Port Resolver v0.1.0 (2025-12-29)

# After:
**Recently Completed:** {LATEST_TOOL} v0.1.0 ({DATE})
```

### 3. Update docs/index.md (home page)

**Location:** Line ~20 (Hero section)

```markdown
# Before:
action:
  theme: alt
  text: View All 10 Tools →

# After:
action:
  theme: alt
  text: View All {TOOL_COUNT} Tools →
```

**Location:** Line ~100 (Progress section)

```markdown
# Before:
## Progress

Tuulbelt currently has 10 of 33 tools implemented (30%).

# After:
## Progress

Tuulbelt currently has {TOOL_COUNT} of 33 tools implemented ({PERCENTAGE}%).
```

### 4. Update docs/tools/index.md

**Location:** Line ~10 (header)

```markdown
# Before:
# 10 of 33 Tuulbelt Tools

## Tools ({PERCENTAGE}% Complete)

# After:
# {TOOL_COUNT} of 33 Tuulbelt Tools

## Tools ({PERCENTAGE}% Complete)
```

### 5. Update docs/guide/getting-started.md

**Location:** Line ~15 (intro)

```markdown
# Before:
Tuulbelt currently has 10 tools available, with 23 more planned.

# After:
Tuulbelt currently has {TOOL_COUNT} tools available, with {33 - TOOL_COUNT} more planned.
```

**Location:** Line ~50 (tool table)

Verify tool table has {TOOL_COUNT} rows (excluding header).

### 6. Update docs/guide/philosophy.md

**Location:** Line ~80 (progress)

```markdown
# Before:
With 10 of 33 tools (30%) implemented, Tuulbelt demonstrates...

# After:
With {TOOL_COUNT} of 33 tools ({PERCENTAGE}%) implemented, Tuulbelt demonstrates...
```

### 7. Update .claude/NEXT_TASKS.md

**Location:** Line ~120 (Completed section)

Update phase summaries:
```markdown
# Before:
### Completed (Phase 1: 5/5 = 100%) 🎉
### Completed (Phase 2: 5/28)

# After:
### Completed (Phase 1: 5/5 = 100%) 🎉
### Completed (Phase 2: {COUNT}/28)
```

### 8. Update ROADMAP.md (if exists)

**Location:** Line ~10 (Current Status)

```markdown
# Before:
## Current Status
- 10 of 33 tools implemented (30%)

# After:
## Current Status
- {TOOL_COUNT} of 33 tools implemented ({PERCENTAGE}%)
```

## Calculation Logic

```typescript
interface ToolCounts {
  total: number;           // Total tools (33)
  implemented: number;     // Completed tools
  remaining: number;       // 33 - implemented
  percentage: number;      // Math.round((implemented / total) * 100)
  phase1: number;          // Tools in Phase 1 (5)
  phase2: number;          // Tools in Phase 2 (calculated)
}

function calculateCounts(): ToolCounts {
  const tools = fs.readdirSync('tools/', { withFileTypes: true })
    .filter(d => d.isDirectory())
    .filter(d => !d.name.startsWith('.'))
    .map(d => d.name);

  const implemented = tools.length;
  const total = 33;
  const remaining = total - implemented;
  const percentage = Math.round((implemented / total) * 100);

  // Phase counts (from NEXT_TASKS.md)
  const phase1 = 5;  // Fixed
  const phase2 = implemented - phase1;

  return { total, implemented, remaining, percentage, phase1, phase2 };
}
```

## Files Updated

The command updates these files:

| File | Lines Changed | Pattern |
|------|---------------|---------|
| `README.md` | 2-3 | Tool count, percentage, recently completed |
| `docs/index.md` | 2-3 | Hero button, progress text |
| `docs/tools/index.md` | 2 | Header count, percentage |
| `docs/guide/getting-started.md` | 2 | Intro text, tool table |
| `docs/guide/philosophy.md` | 1 | Progress text |
| `.claude/NEXT_TASKS.md` | 2 | Phase summaries |
| `ROADMAP.md` | 1 | Current status |

Total: 7 files, ~15 line changes

## Quality Checks

Before committing changes:

- [ ] Verify counts are accurate (manual count vs. script)
- [ ] Check percentage calculation (no rounding errors)
- [ ] Ensure "recently completed" matches latest commit
- [ ] Validate no broken references introduced
- [ ] Run VitePress build: `npm run docs:build`

## Example Output

```
Updating tool counts across documentation...

Scanned: tools/ directory
Found: 11 tools

Calculated:
- Total: 33 tools (target)
- Implemented: 11 tools
- Remaining: 22 tools
- Percentage: 33%

Updated files:
✓ README.md (2 changes)
  - Line 50: 10 → 11 tools
  - Line 53: 30% → 33%
✓ docs/index.md (2 changes)
  - Line 22: "View All 10 Tools" → "View All 11 Tools"
  - Line 105: "10 of 33 (30%)" → "11 of 33 (33%)"
✓ docs/tools/index.md (2 changes)
  - Line 10: "10 of 33" → "11 of 33"
  - Line 12: "30% Complete" → "33% Complete"
✓ docs/guide/getting-started.md (1 change)
  - Line 15: "10 tools available" → "11 tools available"
✓ docs/guide/philosophy.md (1 change)
  - Line 80: "10 of 33 (30%)" → "11 of 33 (33%)"
✓ .claude/NEXT_TASKS.md (1 change)
  - Line 128: "Phase 2: 5/28" → "Phase 2: 6/28"

Total changes: 7 files, 9 line edits

Ready to commit? (y/n): y

Creating commit...
✓ Committed: "docs: update tool counts (11/33 = 33%)"
✓ Pushed to origin/main

Done!
```

## When to Run

Run this command:

**After Creating New Tool:**
```bash
/new-tool component-prop-validator typescript propval
# Command automatically runs /update-all-counts
```

**After Completing Tool:**
```bash
# After marking tool as v0.1.0 in README.md
/update-all-counts
```

**After Removing Tool (rare):**
```bash
# If a tool is removed/archived
/update-all-counts
```

**Manual Audit:**
```bash
# Periodic verification of consistency
/update-all-counts
```

## Troubleshooting

### "Count mismatch detected"
```
WARNING: Manual count (10) differs from automated count (11).

Manual count sources:
- README.md says: 10 tools
- Detected in tools/: 11 directories

Fix:
1. Verify tools/ directory contents
2. Check if tool was recently added
3. Re-run: /update-all-counts
```

### "Percentage calculation error"
```
ERROR: Percentage calculation inconsistent.

Calculated: 33.33333...
Rounded: 33%

Ensure consistent rounding logic (Math.round).
```

### "Recently completed tool incorrect"
```
WARNING: "Recently Completed" shows old tool.

Current: Test Port Resolver v0.1.0 (2025-12-29)
Latest commit: Component Prop Validator v0.1.0 (2025-12-30)

The command does NOT auto-update "Recently Completed".
Manually edit README.md if needed.
```

## Automation

This command can be automated:

**Git Hook (post-commit):**
```bash
# .git/hooks/post-commit
#!/bin/bash
if git diff --name-only HEAD~1 | grep -q "^tools/"; then
  echo "Tool change detected, updating counts..."
  claude-code '/update-all-counts'
fi
```

**CI Workflow:**
```yaml
# .github/workflows/update-counts.yml
on:
  push:
    paths:
      - 'tools/**'

jobs:
  update-counts:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: claude-code '/update-all-counts'
      - run: git commit -am "docs: update tool counts"
      - run: git push
```

## Related Commands

- `/new-tool` - Automatically runs /update-all-counts
- `/sync-tool-docs` - Sync documentation (includes count updates)
- `/release-tool` - May need count update if tool status changes

## References

- Tool Tracking: `.claude/NEXT_TASKS.md`
- Progress Tracking: `ROADMAP.md`
- Quality Checklist: `docs/QUALITY_CHECKLIST.md`
