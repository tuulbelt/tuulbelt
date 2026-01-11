# /new-research Command

Create a new Tuulbelt research project for exploration.

## Usage

```
/new-research <research-name> [language] [description]
```

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| research-name | Yes | Full kebab-case name | `effect-system` |
| language | No | `typescript` or `rust` (default: typescript) | `rust` |
| description | No | One-line description | `"Effect system for TypeScript"` |

## Examples

```bash
# Full specification
/new-research effect-system typescript "Effect system for TypeScript"

# Minimal (defaults to TypeScript)
/new-research zero-copy-parsing

# Rust research
/new-research incremental-compute rust
```

## What This Command Does

This command automates research project creation with the following steps:

### Phase 1: Repository Creation

1. Check if repo already exists
2. Create GitHub repository: `tuulbelt/<research-name>`
3. Configure repo settings (description, topics: "research", "experimental")
4. Clone to local workspace

### Phase 2: Scaffolding

5. Copy `templates/research-template/`
6. Customize package.json/Cargo.toml with project info
7. Generate README.md with research focus
8. Create HYPOTHESIS.md template (REQUIRED)
9. Create FINDINGS.md template
10. Create STATUS.md template
11. Set up experiment scripts

### Phase 3: Meta Repo Integration

12. Add as git submodule to `research/`
13. Update `docs/.vitepress/config.ts` (sidebar)
14. Update `docs/research/index.md` (count, card)
15. Update `README.md` (research list)

### Phase 4: Verification (Minimal)

16. Run `npm install` / `cargo build`
17. Run experiments (failures are acceptable)
18. Verify HYPOTHESIS.md exists

### Phase 5: Commit

19. Commit research repo: `feat: initialize {research-name} exploration`
20. Push research repo to origin
21. Commit meta repo: `chore: add {research-name} submodule`

## Research Template Structure

```
research-name/
├── HYPOTHESIS.md         # Research hypothesis (REQUIRED)
├── FINDINGS.md           # Documented discoveries
├── STATUS.md             # Current status and progress
├── src/
│   ├── index.ts          # TypeScript implementation
│   └── lib.rs            # Rust implementation
├── experiments/
│   ├── main.ts           # TypeScript experiment runner
│   └── main.rs           # Rust experiment runner
├── package.json          # TypeScript config
├── Cargo.toml            # Rust config
├── README.md
├── CLAUDE.md
├── LICENSE
└── .github/workflows/test.yml
```

## HYPOTHESIS.md Requirements

The hypothesis document must include:

1. **Hypothesis Statement** - Clear, testable claim
2. **Specific Claims** - Individual testable assertions
3. **Background** - Problem statement, prior art
4. **Approach** - Core idea, technical approach
5. **Success Criteria** - Metrics and validation plan
6. **Risks** - Technical risks and assumptions
7. **Graduation Path** - Where this could go if successful

## Relaxed Principles

Research projects have **relaxed governance**:
- Dependencies are allowed (pragmatism)
- Test failures are acceptable
- API stability not required
- Exploration is the goal

| Aspect | Research | Production |
|--------|----------|------------|
| Dependencies | Allowed | Zero/Minimal |
| Test Failures | Acceptable | Not allowed |
| API Stability | Not required | Required |
| Documentation | HYPOTHESIS.md | README, API docs |
| Governance | Low | Medium-High |

## Status Labels

Research projects use these status labels:

- **Active** — Currently being explored
- **Paused** — Temporarily on hold
- **Concluded** — Exploration complete (success or failure)
- **Graduated** — Moved to production category

## Post-Creation Tasks

After the research project is created:

1. **Define hypothesis**
   ```bash
   cd research/effect-system
   # Edit HYPOTHESIS.md with clear hypothesis
   ```

2. **Run initial experiments**
   ```bash
   npm run experiment  # or cargo run --bin experiment
   ```

3. **Document findings**
   ```bash
   # Edit FINDINGS.md with results
   # Update STATUS.md with progress
   ```

4. **Evaluate graduation**
   - Does it work?
   - Does it meet a production category's principles?
   - Is it worth stabilizing?

## Graduation Process

When research succeeds:

1. Validate hypothesis (FINDINGS.md shows success)
2. Determine target category (libraries/, tools/, etc.)
3. Create production project with `/new-<category>`
4. Port implementation to meet category principles
5. Archive or reference original research

## Related Commands

- `/new-tool` - Create CLI tools
- `/new-library` - Create programmatic APIs
- `/new-protocol` - Create wire format specifications
