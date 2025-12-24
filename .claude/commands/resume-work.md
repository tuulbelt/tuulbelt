# Resume Work from Previous Session

Load and present the handoff from the previous session to understand what needs to be done.

## Your Task

You are helping the user resume work from a previous Claude Code session. Read the handoff documents and present a clear action plan.

## Steps

1. **Read Handoff Documents**

```bash
# Read in this order:
.claude/HANDOFF.md          # What the previous session handed off
.claude/NEXT_TASKS.md       # Task backlog
docs/KNOWN_ISSUES.md        # Known bugs/issues
```

2. **Verify Project State**

Check that the project state matches the handoff:
- Run `git status` to see current branch and uncommitted changes
- Check recent commits: `git log --oneline -5`
- Verify no unexpected changes

3. **Present Handoff Summary**

Show the user a clear, actionable summary:

```markdown
## Resuming from Previous Session

**Last Session:** [Session name/date from HANDOFF.md]
**Task Type:** [Task type from handoff]
**Status:** [Status from handoff]

### What Was Accomplished

[Summary from HANDOFF.md]

### What's Next (Priority Order)

1. **[Priority 1 Task]**
   - Description
   - Files to work on
   - Expected outcome

2. **[Priority 2 Task]**
   - Description
   - Files to work on
   - Expected outcome

3. **[Priority 3 Task]**
   - Description
   - Files to work on
   - Expected outcome

### Important Context

[Key decisions, constraints, or context from handoff]

### Known Issues to Avoid

[Any issues from KNOWN_ISSUES.md that might affect current work]

### Quality Reminders

- Run `/quality-check` before committing
- Follow [specific standard mentioned in handoff]
- Remember [specific gotcha mentioned in handoff]

### Quick Reference

- **Handoff**: `.claude/HANDOFF.md`
- **Tasks**: `.claude/NEXT_TASKS.md`
- **Issues**: `docs/KNOWN_ISSUES.md`
- **Principles**: `PRINCIPLES.md`
- **Quality**: `docs/QUALITY_CHECKLIST.md`

---

**Ready to start?** I can begin with [Priority 1 Task] or wait for your direction.
```

4. **Offer to Start First Task**

After presenting the summary, offer to start the highest priority task:

```markdown
I'm ready to start the first task. Would you like me to:

A) Begin [Priority 1 Task] immediately
B) Review the plan and adjust priorities
C) Address a different task
D) Update the handoff before starting

Let me know how you'd like to proceed!
```

5. **Handle Different Scenarios**

**Scenario: No Handoff Found**
```markdown
## No Previous Handoff Found

I don't see a previous session handoff at `.claude/HANDOFF.md`.

This could mean:
- This is a fresh start
- The handoff wasn't created in the last session
- You want to start something new

What would you like to work on?

1. Create a new tool (I can help select from NEXT_TASKS.md)
2. Update existing tools
3. Fix bugs or known issues
4. Documentation work
5. Something else

Use `/handoff` at the end of this session to create a handoff for next time.
```

**Scenario: Handoff is Stale (>7 days old)**
```markdown
## Handoff Found (⚠️ Stale)

The last handoff is from [date] ([X] days ago).

**Last Session Summary:**
[Summary]

**Recommended Actions:**
1. Review if this is still the priority
2. Check if main branch has updates: `git fetch origin main`
3. Update NEXT_TASKS.md if priorities changed
4. Refresh KNOWN_ISSUES.md if issues were fixed

Would you like to:
A) Continue with the stale handoff anyway
B) Update the handoff first
C) Start fresh on a new task
```

**Scenario: Git State Doesn't Match Handoff**
```markdown
## ⚠️ Git State Mismatch

**Handoff says:** Branch `[branch-name]`, [status]
**Current state:** [actual git status]

This might mean:
- Work was committed/pushed after handoff
- Branch was switched
- Changes were made outside Claude Code

**Recommended Actions:**
1. Review git log to understand what changed
2. Update HANDOFF.md if work was completed
3. Verify project still builds: `/quality-check`

Should I:
A) Investigate the mismatch
B) Proceed assuming handoff is outdated
C) Wait for you to clarify
```

## Important Notes

- **Trust the Handoff**: The previous session put effort into documenting context
- **Verify Before Acting**: Check git state, build status, tests before diving in
- **Update if Stale**: If handoff is old, ask user if priorities changed
- **Present Options**: Don't assume what user wants to do next
- **Surface Issues**: Highlight any known issues that affect current work

## Example Usage

**User:** `/resume-work`
**Claude:** [Reads handoff, presents summary, offers to start first task]

**User:** `/resume-work` (no handoff exists)
**Claude:** [Notes no handoff found, asks what user wants to work on]

**User:** `/resume-work` (handoff is stale)
**Claude:** [Notes staleness, asks if user wants to refresh or proceed]

## Integration with Other Commands

**After `/resume-work`:**
- Use `/quality-check` to verify project state
- Use `/test-all` to verify tests still pass
- Use tools/agents as needed for the resumed work

**Before ending session:**
- Use `/handoff` to create handoff for next session

## Anti-Patterns (Don't Do This)

❌ Start working without presenting the summary
❌ Assume handoff is current without checking
❌ Ignore git state mismatches
❌ Skip known issues that might affect work
❌ Dive into code before verifying project builds

✅ Read all handoff documents
✅ Verify git state matches expectations
✅ Present clear, actionable summary
✅ Offer options before starting work
✅ Surface any concerns or mismatches
