# /trim-docs - Prune Documentation Bloat

Automatically trim HANDOFF.md, NEXT_TASKS.md, and CHANGELOG.md to keep only relevant, recent context.

## Usage

```bash
/trim-docs
```

## What This Command Does

Enforces the documentation maintenance rules from `docs/DOCUMENTATION_MAINTENANCE.md`:

1. **HANDOFF.md** → Keep only current + next session (~80-100 lines)
   - Delete all previous session details
   - Keep current session summary
   - Keep next session instructions
   - Keep tool status table

2. **NEXT_TASKS.md** → Keep only active tasks (~150-200 lines)
   - Keep meta repo migration status (current phase only)
   - Keep short CLI names reference
   - Keep next 3-5 tools priority
   - Delete completed tasks older than 30 days
   - Delete old session notes

3. **CHANGELOG.md** → Keep only last 60 days (~100-150 lines)
   - Keep unreleased section (current work)
   - Keep entries from last 60 days only
   - Delete Phase 0, Phase 1 entries (in git history)
   - Delete individual tool completions (tools have own repos)

## When to Run

**Every 5-10 migrations:**
- Quick check after each migration
- Prevent gradual bloat

**Monthly:**
- Deep clean of all three files
- Audit against line count limits

**When documentation >500 total lines:**
- HANDOFF.md + NEXT_TASKS.md + CHANGELOG.md >500 lines
- Immediate trim needed

## Expected Output

```
Trimming documentation bloat...

HANDOFF.md:
  Before: 487 lines
  After: 92 lines
  Removed: 395 lines (previous sessions, old gap analyses)

NEXT_TASKS.md:
  Before: 612 lines
  After: 178 lines
  Removed: 434 lines (completed tasks, session notes)

CHANGELOG.md:
  Before: 358 lines
  After: 124 lines
  Removed: 234 lines (Phase 0, Phase 1, old tool entries)

Total:
  Before: 1,457 lines
  After: 394 lines
  Saved: 1,063 lines (73% reduction)

✓ Documentation trimmed successfully
```

## Implementation

### Step 1: Analyze Current State

```bash
# Count lines
wc -l .claude/HANDOFF.md .claude/NEXT_TASKS.md CHANGELOG.md

# Find date cutoff (60 days ago)
date -v-60d +%Y-%m-%d
```

### Step 2: Trim HANDOFF.md

**Keep:**
- Lines from "## ✅ THIS SESSION:" to end of current section
- Lines from "## 🎯 NEXT SESSION:" to end of next section
- Tool status table (10 tools)
- Migration progress (if Phase 2)

**Delete:**
- All "## ✅ PREVIOUS SESSION:" sections
- Old gap analyses
- Historical accomplishments before current session

### Step 3: Trim NEXT_TASKS.md

**Keep:**
- Meta repo migration section (current phase only)
- Short CLI names reference table
- New tools priority (next 3-5 tools)
- Active blockers section

**Delete:**
- Tool maintenance sections (completed work)
- Session notes older than 30 days
- Resolved bugs/issues
- Infrastructure status details

### Step 4: Trim CHANGELOG.md

**Keep:**
- [Unreleased] section
- Entries with dates from last 60 days
- Current phase work (Phase 2 Wave 1)

**Delete:**
- Phase 0 entries (setup, in git)
- Phase 1 entries (automation, in git)
- Individual tool completion entries (tools migrated to own repos)
- Entries older than 60 days

### Step 5: Verify Results

```bash
# Check new line counts
wc -l .claude/HANDOFF.md .claude/NEXT_TASKS.md CHANGELOG.md

# Verify limits:
# - HANDOFF.md: 80-150 lines
# - NEXT_TASKS.md: 150-250 lines
# - CHANGELOG.md: 100-200 lines
```

## Manual Override

If you need to manually trim:

```bash
# View current sizes
wc -l .claude/HANDOFF.md .claude/NEXT_TASKS.md CHANGELOG.md

# Edit each file to match limits
code .claude/HANDOFF.md
code .claude/NEXT_TASKS.md
code CHANGELOG.md
```

## Safety

**Nothing is lost:**
- All content remains in git history
- Use `git log -p <file>` to view historical versions
- Use `git show <commit>:<file>` to restore old versions

**Recovery:**
```bash
# See old HANDOFF.md content
git log -p .claude/HANDOFF.md | less

# Restore previous version
git show HEAD~5:.claude/HANDOFF.md > /tmp/old-handoff.md
```

## Integration with Quality Checks

Add to `docs/QUALITY_CHECKLIST.md`:

```markdown
### Documentation Size Check

- [ ] **Run before every commit:**
  ```bash
  # Check sizes
  wc -l .claude/HANDOFF.md .claude/NEXT_TASKS.md CHANGELOG.md

  # If total >500 lines, run:
  /trim-docs
  ```

- [ ] **Verify limits:**
  - HANDOFF.md: <150 lines
  - NEXT_TASKS.md: <250 lines
  - CHANGELOG.md: <200 lines
```

## Examples

### HANDOFF.md Before/After

**Before (487 lines):**
```markdown
## Previous Session 5: Authentication
[100 lines]

## Previous Session 4: Migration Docs
[120 lines]

## Previous Session 3: Path Normalizer
[90 lines]

## Previous Session 2: CLI Progress
[85 lines]

## Current Session: config-file-merger
[92 lines]
```

**After (92 lines):**
```markdown
## ✅ THIS SESSION: config-file-merger
[45 lines]

## 🎯 NEXT SESSION: structured-error-handler
[30 lines]

## Tool Status
[17 lines]
```

### CHANGELOG.md Before/After

**Before (358 lines):**
```markdown
## Phase 0 Complete (2025-12-29)
[80 lines about preparation]

## Phase 1 Complete (2025-12-29)
[120 lines about automation]

## Tool 1 Complete
## Tool 2 Complete
...
[158 lines of individual tools]
```

**After (124 lines):**
```markdown
## [Unreleased]

### Added - Phase 2 Wave 1: config-file-merger (2025-12-29)
[40 lines]

### Added - Authentication Workflow (2025-12-29)
[35 lines]

### Added - Migration Learnings (2025-12-29)
[49 lines]
```

---

**Created:** 2025-12-29
**Reference:** docs/DOCUMENTATION_MAINTENANCE.md
**Enforcement:** Run every 5-10 migrations or when docs >500 lines total
