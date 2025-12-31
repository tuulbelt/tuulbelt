# /web-setup Command

Initialize or resume Claude Code Web session workflow.

This command:
- Detects if running in Claude Code Web environment
- Sets up git credential helper for GitHub authentication
- Initializes web-session-tracking.json
- Creates session entry for current meta branch
- Ensures all submodules are initialized

## Usage

```
/web-setup
```

## What It Does

1. **Environment Detection**: Checks `CLAUDE_CODE_REMOTE=true`
2. **Credential Setup**: Configures git to use `GITHUB_TOKEN` for HTTPS pushes
3. **Session Initialization**: Creates tracking file at `.claude/web-session-tracking.json`
4. **Session Resume**: If session exists for current branch, resumes it
5. **Submodule Init**: Runs `git submodule update --init --recursive`

## When to Use

- At the start of every Web session
- After switching to a different meta branch
- When tracking file is missing or corrupted

## Output

Shows:
- Current meta branch
- Session ID
- List of tracked submodules (if any)

## Implementation

Runs: `./scripts/web/init-session.sh`

## See Also

- `/web-status` - Show current session status
- `./scripts/web/manage-submodule-branch.sh` - Create submodule feature branch
