# Documentation Maintenance Rules

**Last Updated:** 2025-12-29
**Purpose:** Prevent documentation bloat while preserving essential context

---

## Core Principle

**If it's in git history, we don't need it in documentation.**

Git log is the source of truth for historical events. Documentation should focus on:
- **Current state** (what's happening now)
- **Next actions** (what to do next)
- **Reference data** (tool lists, test counts)

---

## File-Specific Rules

### HANDOFF.md (~80-100 lines max)

**Keep:**
- Current session summary (5-10 bullet points)
- Next session instructions (clear, actionable steps)
- Tool status table (10 tools with test counts)
- Migration progress tracker (if in Phase 2)

**Delete:**
- Previous session details (move to git commit messages)
- Historical accomplishments (in git log)
- Old gap analyses (not needed after fixed)
- Infrastructure setup details (one-time work, in git)

**Template Structure:**
```markdown
# Session Handoff
**Last Updated:** YYYY-MM-DD
**Session:** [Brief description]
**Status:** [Current state emoji + summary]

## ✅ THIS SESSION: [What was accomplished]
[5-10 key bullets]

## 🎯 NEXT SESSION: [What to do next]
[Clear instructions with commands]

## Tool Status (10 tools)
[Table with test counts]
```

---

### NEXT_TASKS.md (~150-200 lines max)

**Keep:**
- Meta repo migration status (current phase only)
- Short CLI names reference table
- New tools priority order (next 3-5 tools)
- Active blockers (if any)

**Delete:**
- Completed tool maintenance details (in git)
- Resolved bugs/issues (in git log, KNOWN_ISSUES.md)
- Old session notes (historical context)
- Detailed infrastructure status (already built)
- Tasks completed >30 days ago

**Template Structure:**
```markdown
# Next Tasks

## 🚨 CRITICAL: Meta Repository Migration
[Current phase status, wave progress]

## 🎯 Short CLI Names Reference
[Table of 10 implemented + 3-5 proposed]

## 🚀 New Tools (Priority Order)
[Next 3-5 tools to implement]

## Active Blockers
[If any - otherwise omit this section]
```

---

### CHANGELOG.md (~100-150 lines max)

**Keep:**
- Unreleased section (current work in progress)
- Last 60 days of significant changes
- Current phase entries (e.g., Phase 2 Wave 1 migrations)

**Delete:**
- Entries older than 60 days (check: `git log --since="60 days ago"`)
- Completed phases (Phase 0, Phase 1 - in git history)
- Individual tool completions (tools have own repos/CHANGELOGs now)
- Infrastructure setup (one-time work)

**Pruning Rule:**
```bash
# Keep only entries from last 60 days
# Run every ~5-10 migrations or monthly
grep -E "20[0-9]{2}-[0-9]{2}-[0-9]{2}" CHANGELOG.md | \
  awk -v cutoff="$(date -v-60d +%Y-%m-%d)" '$0 >= cutoff'
```

**Template Structure:**
```markdown
# Tuulbelt Meta Repository Changelog

## [Unreleased]

### Added - [Current work]
[Most recent entries]

### Fixed - [Recent fixes]

## [YYYY-MM-DD] - [Last 60 days only]
[Recent significant changes]
```

---

## Enforcement Mechanisms

### 1. End-of-Session Checklist

Add to HANDOFF.md creation workflow:
```bash
# Before updating HANDOFF.md:
1. Delete previous session details (keep only current + next)
2. Check HANDOFF.md line count: should be 80-100 lines
3. If >150 lines, trim historical content
```

### 2. Monthly Maintenance Command

Create `/trim-docs` command:
```bash
/trim-docs

# What it does:
1. Prune CHANGELOG.md to last 60 days
2. Remove completed tasks from NEXT_TASKS.md (>30 days)
3. Archive old HANDOFF.md sessions (keep only current)
4. Report: "Trimmed X lines, kept Y lines"
```

### 3. Pre-Migration Checklist

Add to `docs/QUALITY_CHECKLIST.md` under "Before Every Commit":
```markdown
- [ ] **Documentation size check**:
  - HANDOFF.md <150 lines
  - NEXT_TASKS.md <250 lines
  - CHANGELOG.md <200 lines
  - If over, run /trim-docs or manually prune
```

### 4. Git Pre-Commit Hook

Add to `.git/hooks/pre-commit` (optional):
```bash
#!/bin/bash
# Warn if docs are bloated

HANDOFF_LINES=$(wc -l < .claude/HANDOFF.md)
NEXT_LINES=$(wc -l < .claude/NEXT_TASKS.md)
CHANGELOG_LINES=$(wc -l < CHANGELOG.md)

if [ $HANDOFF_LINES -gt 150 ] || [ $NEXT_LINES -gt 250 ] || [ $CHANGELOG_LINES -gt 200 ]; then
  echo "⚠️  WARNING: Documentation bloat detected"
  echo "  HANDOFF.md: $HANDOFF_LINES lines (limit: 150)"
  echo "  NEXT_TASKS.md: $NEXT_LINES lines (limit: 250)"
  echo "  CHANGELOG.md: $CHANGELOG_LINES lines (limit: 200)"
  echo ""
  echo "Consider running: /trim-docs"
  echo "Or manually prune historical content"
  echo ""
  read -p "Continue anyway? (y/N) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi
```

---

## What Lives Where

| Information Type | Location | Retention |
|------------------|----------|-----------|
| Historical events | Git log | Forever |
| Current work | HANDOFF.md | Current session only |
| Next actions | HANDOFF.md | Next session only |
| Recent changes | CHANGELOG.md | Last 60 days |
| Tool status | HANDOFF.md | Always current |
| Active tasks | NEXT_TASKS.md | Until complete +30 days |
| Resolved bugs | Git log | Forever |
| Old session notes | Git commits | Forever |
| Migration phases | Git log (after complete) | Forever |

---

## Recovery: "Where did that info go?"

If you need historical context:

```bash
# Find when a tool was completed
git log --grep="config-file-merger" --oneline

# See old session notes
git log --grep="Session:" --oneline
git show <commit-hash>

# View old HANDOFF.md
git log -p .claude/HANDOFF.md | less

# Find old CHANGELOG entries
git log -p CHANGELOG.md | less
```

---

## Review Schedule

**Every 5-10 migrations**: Quick trim
- Remove completed tasks from NEXT_TASKS.md
- Check HANDOFF.md is current + next only

**Monthly**: Deep clean
- Prune CHANGELOG.md to 60 days
- Review all three files against line limits
- Consider creating /trim-docs command if manual

**Quarterly**: Audit
- Review if rules need adjustment
- Check git log is being used for historical queries
- Update this document if patterns change

---

## Examples of Good vs Bad

### HANDOFF.md

**❌ Bad (bloated):**
```markdown
## Previous Session 1: Test Flakiness Detector
[300 lines of historical details]

## Previous Session 2: CLI Progress
[200 lines of historical details]

## Previous Session 3: ...
[Repeating pattern for 10 sessions]

## Current Session:
[Actually relevant info]
```

**✅ Good (lean):**
```markdown
## ✅ THIS SESSION: config-file-merger Review
- Fixed Node 18 bug
- Verified GitHub settings
- Updated tracking docs

## 🎯 NEXT SESSION: structured-error-handler
Run: /migrate-tool structured-error-handler
```

### CHANGELOG.md

**❌ Bad (keeping everything):**
```markdown
## 2025-12-22 - Initial Setup
[Details about project creation]

## 2025-12-23 - Infrastructure
[300 lines about setup]

## 2025-12-24 - Tool 1
## 2025-12-25 - Tool 2
...
[Every single event]
```

**✅ Good (recent only):**
```markdown
## [Unreleased]

### Added - Phase 2 Wave 1: config-file-merger (2025-12-29)
- Migrated to standalone repo
- Fixed Node 18 compatibility
- All tests passing
```

---

## Decision Tree: Keep or Delete?

```
Is this information about current work?
├─ Yes → KEEP in HANDOFF.md (current session)
└─ No → Is it about next actions?
    ├─ Yes → KEEP in HANDOFF.md (next session)
    └─ No → Is it from last 60 days AND significant?
        ├─ Yes → KEEP in CHANGELOG.md
        └─ No → DELETE (it's in git history)
```

---

**Remember:** Documentation is for doing work, not recording history. Git is our historian.
