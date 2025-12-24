# Create Session Handoff

Create or update the session handoff document (`.claude/HANDOFF.md`) to prepare for the next session.

## Your Task

You are helping the user create a comprehensive handoff for the next Claude Code session. This handoff ensures continuity between sessions and prevents loss of context.

## Steps

1. **Analyze Current Session Context**
   - What was accomplished in this session?
   - What is incomplete or in progress?
   - What decisions were made?
   - What blockers or issues were encountered?

2. **Review Current State**
   - Check git status and recent commits
   - Review NEXT_TASKS.md for pending work
   - Check KNOWN_ISSUES.md for tracked problems
   - Review recent file changes

3. **Update HANDOFF.md**

Update `.claude/HANDOFF.md` with:

**Required Sections:**
- **Session Summary**: What was accomplished, what's pending
- **Key Decisions Made**: Important choices that affect future work
- **Context for Next Session**: Project state, branch status, quality standards
- **Next Immediate Tasks**: Priority-ordered list of what to do next
- **Important References**: Links to key documents
- **Blockers/Issues**: Anything preventing progress
- **Notes for Next Session**: Tips, warnings, reminders

**Task Type Format:**
Categorize the primary work type:
- 🚀 **NEW_TOOL**: Creating a new tool from scratch
- 🔧 **UPDATE_TOOL**: Modifying existing tool functionality
- 🧪 **ADD_TESTS**: Adding or improving tests
- 🐛 **FIX_BUG**: Fixing bugs or issues
- 📚 **DOCS**: Documentation work
- ⚙️ **INFRA**: CI/CD, workflows, infrastructure
- 🎨 **DESIGN**: UI/UX improvements
- ♻️ **REFACTOR**: Code cleanup or restructuring

4. **Update NEXT_TASKS.md (if needed)**

If new tasks were discovered or priorities changed:
- Add new tasks to appropriate sections
- Update status of in-progress tasks
- Mark completed tasks as done
- Reorder priorities if needed

5. **Update KNOWN_ISSUES.md (if needed)**

If new issues were discovered or existing issues were resolved:
- Add new issues with full template
- Update status of in-progress fixes
- Move resolved issues to "Resolved" section
- Update attempted fixes for open issues

6. **Verify Handoff Quality**

Check that handoff includes:
- [ ] Clear summary of what was done
- [ ] Specific next steps (not vague)
- [ ] All context needed to resume work
- [ ] References to important documents
- [ ] No assumed knowledge
- [ ] Timestamp (Last Updated)

7. **Present Summary to User**

Show the user:
- What was recorded in the handoff
- What the next session should focus on
- Any important warnings or notes
- Link to HANDOFF.md for their review

## Output Format

After updating the files, provide this summary:

```markdown
## Session Handoff Created

**Type:** [Task Type Icon + Name]
**Status:** Ready for next session

### Accomplished This Session
- [List key achievements]

### Next Session Priorities
1. [Priority 1]
2. [Priority 2]
3. [Priority 3]

### Important Notes
- [Any critical information]
- [Warnings or gotchas]

**Handoff Document:** `.claude/HANDOFF.md`
**Task Backlog:** `.claude/NEXT_TASKS.md`
**Known Issues:** `docs/KNOWN_ISSUES.md`

✅ Ready for next session to run `/resume-work`
```

## Important Notes

- **Be Specific**: Vague handoffs waste time in the next session
- **Include Context**: Next session won't remember this conversation
- **Document Decisions**: Explain *why* choices were made
- **Link Everything**: Provide file paths and line numbers
- **Update Timestamps**: Mark when handoff was created
- **Think Continuity**: What would YOU need to resume this work?

## Example Usage

**User:** `/handoff`
**Claude:** [Analyzes session, updates HANDOFF.md, NEXT_TASKS.md, presents summary]

**User:** `/handoff We're switching to work on bug fixes instead`
**Claude:** [Updates handoff to reflect new direction, updates task priorities]

## Anti-Patterns (Don't Do This)

❌ "Worked on stuff, continue later"
❌ "Added some features, do more tests"
❌ Forgetting to document decisions
❌ Not linking to specific files/commits
❌ Assuming next session remembers context
❌ Skipping the "Why" behind decisions

✅ "Implemented Cross-Platform Path Normalizer (TypeScript chosen for Node.js path module integration). Completed core implementation with 85% test coverage. Pending: VitePress docs (follow verbose format from cli-progress-reporting), demo recording. Next: Create docs/tools/path-normalizer/index.md following template at templates/tool-repo-template/README.md"
