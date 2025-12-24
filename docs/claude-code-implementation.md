# Claude Code Workflow Implementation

## Overview

This document describes the Claude Code automation infrastructure implemented for the Tuulbelt project. This infrastructure provides standardized workflows, quality gates, and development automation that work across all tools regardless of language (TypeScript or Rust).

## Architecture

The Claude Code infrastructure consists of four main components:

1. **Slash Commands** - Reusable workflow commands (`.claude/commands/`)
2. **Specialized Agents** - Domain-specific AI assistants (`.claude/agents/`)
3. **Skills** - Cross-cutting best practices (`.claude/skills/`)
4. **Hooks** - Automated quality gates (`.claude/hooks/`)

All components are configured through `.claude/settings.json` and documented in `CLAUDE.md`.

## Directory Structure

```
.claude/
├── settings.json                 # Main configuration with hook definitions
├── settings.local.json.example   # Example local overrides
├── commands/                     # Slash commands
│   ├── test-all.md              # Run all tests (TS + Rust)
│   ├── security-scan.md         # Security analysis
│   ├── scaffold-tool.md         # Create new tool from template
│   └── git-commit.md            # Semantic commits
├── agents/                       # Specialized agents
│   ├── test-runner.md           # Testing expert
│   ├── security-reviewer.md     # Security expert
│   └── scaffold-assistant.md    # Tool creation expert
├── skills/                       # Best practices
│   ├── typescript-patterns/SKILL.md
│   ├── rust-idioms/SKILL.md
│   └── zero-deps-checker/SKILL.md
└── hooks/                        # Hook scripts
    ├── pre-write-check.sh       # Protect sensitive files
    ├── post-write-format.sh     # Auto-format code
    ├── post-bash-audit.sh       # Audit commands
    └── on-stop-cleanup.sh       # Session cleanup
```

## Component Details

### 1. Slash Commands

Slash commands are Markdown files that define reusable workflows. They can be invoked with `/command-name`.

#### `/test-all [filter]`

**Purpose**: Run comprehensive test suite for both TypeScript and Rust templates.

**Usage**:
```bash
/test-all                    # Run all tests
/test-all integration        # Run tests matching "integration"
```

**What it does**:
- Executes TypeScript tests in `templates/tool-repo-template/`
- Executes Rust tests in `templates/rust-tool-template/`
- Reports pass/fail counts and coverage

**File**: `.claude/commands/test-all.md`

#### `/security-scan`

**Purpose**: Comprehensive security analysis of the codebase.

**Usage**:
```bash
/security-scan
```

**What it does**:
- Dependency vulnerability scan (`npm audit`, `cargo audit`)
- Secret detection in code and staged files
- Protected file check (ensures no .env files committed)
- Zero-dependency validation
- Reports findings with severity levels

**File**: `.claude/commands/security-scan.md`

#### `/scaffold-tool <name> <language>`

**Purpose**: Create a new tool repository from template.

**Usage**:
```bash
/scaffold-tool path-normalizer typescript
/scaffold-tool json-validator rust
```

**What it does**:
- Validates tool name (must be kebab-case)
- Copies appropriate template (TypeScript or Rust)
- Customizes package.json/Cargo.toml with tool name
- Updates README.md
- Initializes git repository with first commit
- Runs initial tests to verify setup

**File**: `.claude/commands/scaffold-tool.md`

#### `/git-commit <type> <scope> <message>`

**Purpose**: Create semantic commit following Conventional Commits.

**Usage**:
```bash
/git-commit feat cli "add verbose logging option"
/git-commit fix core "handle empty input edge case"
/git-commit test parser "add coverage for malformed data"
```

**What it does**:
- Validates commit type (feat|fix|refactor|test|docs|chore|ci|perf)
- Shows git status and staged changes
- Quick secret scan
- Creates commit with format: `type(scope): message`
- Displays commit summary and branch status

**File**: `.claude/commands/git-commit.md`

### 2. Specialized Agents

Agents are specialized AI assistants with specific tools and expertise. They're automatically invoked when appropriate.

#### test-runner

**Domain**: Test execution, debugging, coverage analysis

**Capabilities**:
- Run TypeScript tests (Node.js native test runner)
- Run Rust tests (cargo test)
- Generate coverage reports
- Diagnose test failures
- Identify missing test coverage

**Tools**: Bash, Read, Grep, Glob

**Model**: claude-sonnet-4-5-20250929

**File**: `.claude/agents/test-runner.md`

**When used**: Automatically when dealing with test failures, coverage analysis, or test execution tasks.

#### security-reviewer

**Domain**: Security analysis, vulnerability scanning, compliance

**Capabilities**:
- OWASP Top 10 vulnerability detection
- Credential and secret scanning
- Dependency auditing
- Input validation review
- Path traversal prevention
- Zero-dependency validation

**Tools**: Bash, Read, Grep, Glob

**Model**: claude-sonnet-4-5-20250929

**File**: `.claude/agents/security-reviewer.md`

**When used**: During security scans, code reviews, pre-release checks.

#### scaffold-assistant

**Domain**: Tool creation, template customization

**Capabilities**:
- Template selection and validation
- Tool name validation (kebab-case)
- File customization (package.json, Cargo.toml, README)
- Git initialization
- Initial testing

**Tools**: Bash, Read, Write, Edit, Glob

**Model**: claude-sonnet-4-5-20250929

**File**: `.claude/agents/scaffold-assistant.md`

**When used**: When creating new tools from templates.

### 3. Skills

Skills are Claude-invoked capabilities that enforce best practices automatically.

#### typescript-patterns

**Domain**: TypeScript best practices for zero-dependency tools

**Coverage**:
- Strict type safety (no `any`, explicit return types)
- Result pattern for error handling
- Async/await patterns (no callbacks)
- File I/O with Node.js built-ins
- Testing with native test runner
- CLI patterns without dependencies

**File**: `.claude/skills/typescript-patterns/SKILL.md`

**When used**: Automatically when writing TypeScript code.

#### rust-idioms

**Domain**: Rust best practices for zero-dependency tools

**Coverage**:
- Ownership and borrowing patterns
- Result types (no unwrap in production)
- Type safety with newtypes
- Iterator chains over loops
- Pattern matching
- Testing patterns
- CLI without dependencies

**File**: `.claude/skills/rust-idioms/SKILL.md`

**When used**: Automatically when writing Rust code.

#### zero-deps-checker

**Domain**: Validate zero-dependency principle

**Coverage**:
- Check package.json dependencies (must be empty)
- Check Cargo.toml dependencies (must be empty)
- Validate dev-dependencies are acceptable
- Suggest alternatives to external packages
- Explain zero-dependency principle

**File**: `.claude/skills/zero-deps-checker/SKILL.md`

**When used**: When examining package.json or Cargo.toml files.

### 4. Hooks

Hooks are bash scripts that run at specific lifecycle events. They enforce quality gates and automate repetitive tasks.

#### pre-write-check.sh

**Event**: Before any file write (`PreToolUse/Write`)

**Purpose**: Prevent modification of protected files

**Protected patterns**:
- Lock files: `package-lock.json`, `yarn.lock`, `Cargo.lock`
- Environment files: `.env*`, `*.env.*`
- Secrets: `*.secret.*`, `*.private.*`, `*.key`, `*.pem`
- Credentials: `credentials.json`, `secrets.json`

**Action**: Blocks write with exit code 2 and explains why.

**Timeout**: 5 seconds

**File**: `.claude/hooks/pre-write-check.sh`

#### post-write-format.sh

**Event**: After any file write (`PostToolUse/Write`)

**Purpose**: Automatically format code files

**Formatting rules**:
- TypeScript/JavaScript: prettier (if available)
- Rust: rustfmt
- JSON: jq or prettier
- Markdown: prettier
- TOML: taplo (if available)
- YAML: prettier

**Action**: Formats file in-place, continues on failure.

**Timeout**: 15 seconds

**File**: `.claude/hooks/post-write-format.sh`

#### post-bash-audit.sh

**Event**: After any bash command execution (`PostToolUse/Bash`)

**Purpose**: Audit and log all bash commands

**Actions**:
1. Log command with timestamp to `~/.claude/logs/bash-audit-YYYY-MM.log`
2. Warn about dangerous patterns:
   - `rm -rf /`, `rm -rf ~`, `rm -rf *`
   - Fork bombs, disk wipers
   - Database drops
3. Notice sensitive operations:
   - `git push --force`
   - `npm publish`, `cargo publish`
4. Rotate log if >10MB

**Timeout**: 5 seconds

**File**: `.claude/hooks/post-bash-audit.sh`

#### on-stop-cleanup.sh

**Event**: When Claude Code session stops (`Stop`)

**Purpose**: Clean up temporary files and archive logs

**Actions**:
1. Remove temporary files (*.tmp, .plan.md, *.swp, *~)
2. Archive session logs with timestamp
3. Compress archived logs
4. Delete old logs (>30 days for sessions, >90 days for bash audit)
5. Display git status if uncommitted changes exist

**Timeout**: 30 seconds

**File**: `.claude/hooks/on-stop-cleanup.sh`

## Configuration

### settings.json

Main configuration file that defines permissions and hook mappings.

**Permissions**:
```json
{
  "permissions": {
    "allow": ["Skill", "Bash", "Read", "Write", "Edit", "Glob", "Grep"]
  }
}
```

**Hook mappings**:
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write",
        "hooks": [{"type": "command", "command": "bash .../pre-write-check.sh", "timeout": 5000}]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [{"type": "command", "command": "bash .../post-write-format.sh", "timeout": 15000}]
      },
      {
        "matcher": "Bash",
        "hooks": [{"type": "command", "command": "bash .../post-bash-audit.sh", "timeout": 5000}]
      }
    ],
    "Stop": [
      {
        "matcher": "",
        "hooks": [{"type": "command", "command": "bash .../on-stop-cleanup.sh", "timeout": 30000}]
      }
    ]
  }
}
```

### Local Overrides

Create `.claude/settings.local.json` (gitignored) to override project settings for personal preferences.

Example:
```json
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "hooks": {
    "PostToolUse": []  // Disable all hooks locally
  }
}
```

## Usage Workflows

### Creating a New Tool

1. Use scaffold command:
   ```bash
   /scaffold-tool my-awesome-tool typescript
   ```

2. Claude Code will:
   - Validate the name
   - Copy template
   - Customize files
   - Initialize git
   - Run initial tests

3. Develop the tool:
   - Edit `src/index.ts` (auto-formatted on save)
   - Add tests
   - Security scan automatically runs on commits

### Running Tests

1. Execute tests:
   ```bash
   /test-all
   ```

2. test-runner agent will:
   - Run all tests
   - Report failures
   - Show coverage
   - Diagnose issues

### Security Review

1. Run scan:
   ```bash
   /security-scan
   ```

2. security-reviewer agent will:
   - Check dependencies
   - Scan for secrets
   - Validate zero-dependency principle
   - Report findings with severity

### Making Commits

1. Create commit:
   ```bash
   /git-commit feat parser "add support for nested objects"
   ```

2. Claude Code will:
   - Show what's being committed
   - Quick secret scan
   - Create semantic commit
   - Display branch status

## Hook Behavior

### Pre-Write Protection Example

**Attempt to write to package-lock.json:**
```
🛑 BLOCKED: Cannot modify protected file: package-lock.json

This file is protected because:
  - Lock files should only be modified by package managers
  - Use: npm install, yarn install, or pnpm install
```

**Result**: Write is blocked, file remains unchanged.

### Post-Write Formatting Example

**Write TypeScript file:**
```typescript
// Before formatting
function foo(x:number):number{return x*2;}
```

```typescript
// After automatic formatting (prettier)
function foo(x: number): number {
  return x * 2;
}
```

**Result**: File is auto-formatted according to language standards.

### Bash Audit Example

**Execute dangerous command:**
```bash
rm -rf /tmp/old-data/*
```

**Log output:**
```
WARNING: Potentially dangerous command detected!
Command: rm -rf /tmp/old-data/*

This command matches a dangerous pattern: rm -rf
Execution was allowed, but please review carefully.
```

**Audit log:**
```
=== 2025-12-23T15:30:45Z ===
Command: rm -rf /tmp/old-data/*
Working Directory: /home/user/tuulbelt
!!! DANGEROUS COMMAND WARNING !!!
Pattern: rm -rf
---
```

## Troubleshooting

### Hooks Not Running

**Check**:
1. Hook scripts are executable: `chmod +x .claude/hooks/*.sh`
2. settings.json is valid JSON
3. Hook paths are absolute paths
4. Check Claude Code logs for errors

### Formatting Not Working

**Check**:
1. Formatter is installed (prettier, rustfmt)
2. Running in correct directory (package.json exists for prettier)
3. Check hook timeout (might need to increase)

### False Positives in Security Scan

**Solutions**:
1. Test fixtures: Add `test` directory to exclusions
2. Documentation examples: Mark with comments
3. Environment variable names: Use UPPERCASE convention

### Hook Timeouts

**If hooks time out**:
1. Increase timeout in settings.json
2. Optimize hook scripts
3. Consider running expensive operations in background

## Best Practices

### For Command Development

1. **Clear descriptions**: Use frontmatter `description` field
2. **Argument hints**: Specify expected parameters
3. **Error handling**: Provide clear error messages
4. **Documentation**: Explain success criteria and outputs

### For Agent Development

1. **Single responsibility**: One domain per agent
2. **Tool restriction**: Only grant necessary tools
3. **Clear prompts**: Define responsibilities explicitly
4. **Model selection**: Use Haiku for speed, Sonnet for complexity

### For Skill Development

1. **Actionable guidelines**: Provide concrete examples
2. **Anti-patterns**: Show what NOT to do
3. **Justification**: Explain WHY, not just WHAT
4. **Context-aware**: Consider zero-dependency principle

### For Hook Development

1. **Fast execution**: Keep under timeout limits
2. **Fail gracefully**: Don't block on non-critical issues
3. **Logging**: Audit important actions
4. **Idempotent**: Safe to run multiple times

## Performance Considerations

**Hook execution adds latency**:
- Pre-write: ~100-500ms per write
- Post-write format: ~1-3s per file
- Post-bash audit: ~50-200ms per command

**Mitigation strategies**:
1. Keep hook scripts simple
2. Use caching where possible
3. Avoid expensive operations in hot paths
4. Consider async operations where appropriate

## Security Considerations

**Hooks run with user permissions**:
- Can read/write any file user can access
- Can execute any command user can run
- Can access environment variables and secrets

**Best practices**:
1. Review all hook scripts before use
2. Don't store credentials in hooks
3. Use absolute paths (prevent injection)
4. Validate all inputs from JSON
5. Audit hook changes in code review

## Extending the Infrastructure

### Adding a New Slash Command

1. Create `.claude/commands/my-command.md`:
   ```markdown
   ---
   description: Brief description
   argument-hint: "[arg1] [arg2]"
   ---

   Detailed instructions for Claude...
   ```

2. Test with `/my-command`

### Adding a New Agent

1. Create `.claude/agents/my-agent.md`:
   ```markdown
   ---
   name: my-agent
   description: "What this agent does"
   tools: [Bash, Read]
   model: claude-sonnet-4-5-20250929
   ---

   # Agent Name

   System prompt and instructions...
   ```

2. Agent is automatically available

### Adding a New Skill

1. Create `.claude/skills/my-skill/SKILL.md`:
   ```markdown
   ---
   name: my-skill
   description: "When to use this skill"
   ---

   # Skill Name

   Best practices and guidelines...
   ```

2. Skill is automatically invoked by Claude when relevant

### Adding a New Hook

1. Create hook script in `.claude/hooks/my-hook.sh`
2. Make executable: `chmod +x .claude/hooks/my-hook.sh`
3. Add to `.claude/settings.json`:
   ```json
   {
     "hooks": {
       "PreToolUse": [
         {
           "matcher": "ToolName",
           "hooks": [{
             "type": "command",
             "command": "bash /home/user/tuulbelt/.claude/hooks/my-hook.sh",
             "timeout": 10000
           }]
         }
       ]
     }
   }
   ```

## Maintenance

### Regular Tasks

**Weekly**:
- Review bash audit logs for anomalies
- Check disk usage of logs directory

**Monthly**:
- Review and update security patterns
- Update hook scripts for new edge cases
- Validate all slash commands still work

**Per Release**:
- Test all workflows with new templates
- Update documentation with new patterns
- Review agent prompts for accuracy

### Log Rotation

**Automatic** (via on-stop-cleanup.sh):
- Session logs: Archived on stop, deleted after 30 days
- Bash audit logs: Rotated at 10MB, deleted after 90 days

**Manual**:
```bash
# Clean all logs
rm -rf ~/.claude/logs/*

# Clean old logs only
find ~/.claude/logs -mtime +30 -delete
```

## References

- CLAUDE.md - Main project context for Claude Code
- docs/testing-standards.md - Testing requirements
- docs/security-guidelines.md - Security checklist
- docs/claude-code-workflow.md - Best practices for using Claude Code

## Changelog

### 2025-12-23 - Initial Implementation

- Created 4 slash commands (test-all, security-scan, scaffold-tool, git-commit)
- Created 3 specialized agents (test-runner, security-reviewer, scaffold-assistant)
- Created 3 skills (typescript-patterns, rust-idioms, zero-deps-checker)
- Implemented 4 hooks (pre-write, post-write, post-bash, on-stop)
- Configured settings.json with hook mappings
- Documented entire infrastructure

---

This infrastructure provides production-ready automation for all Tuulbelt development while maintaining flexibility for customization and extension.
