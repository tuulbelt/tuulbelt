---
description: Create a semantic commit following conventional commits
argument-hint: "<type> <scope> <message>"
---

Create a git commit following the Conventional Commits specification.

## Arguments

- **Type**: $1 (feat|fix|refactor|test|docs|chore|ci|perf)
- **Scope**: $2 (component or area of change)
- **Message**: $3+ (concise description in imperative mood)

## Validation

First, validate the commit type:

Valid types: feat, fix, refactor, test, docs, chore, ci, perf

## Pre-Commit Checks

Before committing, run these checks:

1. **Git status** - Review what's being committed:
   ```bash
   git status --short
   ```

2. **Diff review** - Check staged changes:
   ```bash
   git diff --cached --stat
   ```

3. **Security check** - Quick scan for secrets:
   ```bash
   git diff --cached | grep -iE '(password|api_key|secret|token)' && echo "⚠️  Warning: Potential secrets detected" || echo "✓ No obvious secrets"
   ```

## Commit Process

Create the commit with the conventional format:

```bash
git commit -m "$1($2): $3"
```

## Post-Commit

After committing, display:

1. **Commit summary**:
   ```bash
   git log -1 --oneline
   ```

2. **Branch status**:
   ```bash
   git status
   ```

3. **Reminder** about pushing:
   ```
   Remember to push when ready: git push origin <branch-name>
   ```

## Examples

```bash
/git-commit feat cli "add verbose logging option"
/git-commit fix core "handle empty input edge case"
/git-commit test core "add coverage for error conditions"
/git-commit docs readme "update installation instructions"
```
