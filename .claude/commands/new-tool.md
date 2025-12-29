# /new-tool Command

Create a new Tuulbelt tool with full automation.

## Usage

```
/new-tool <tool-name> <language> [short-name] [description]
```

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| tool-name | Yes | Full kebab-case name | `component-prop-validator` |
| language | Yes | `typescript` or `rust` | `typescript` |
| short-name | No | CLI short name (auto-generated if omitted) | `propval` |
| description | No | One-line description | `"Runtime prop validation"` |

## Examples

```bash
# Full specification
/new-tool component-prop-validator typescript propval "Runtime prop validation for components"

# Minimal (auto-generates short name and description)
/new-tool json-schema-validator typescript

# Rust tool
/new-tool universal-log-normalizer rust lognorm "Structured log standardization"
```

## What This Command Does

This command automates the entire tool creation process with ~30 automated steps:

###  Phase 1: Repository Creation (4 steps)

1. Check if repo already exists (MCP: `check_repo_exists` or `gh repo view`)
2. Create GitHub repository (MCP: `create_tool_repo` or `gh repo create`)
3. Configure repo settings (MCP: `configure_repo_settings` or manual)
4. Clone to local workspace

### Phase 2: Scaffolding (7 steps)

5. Copy appropriate template (TypeScript or Rust)
6. Customize package.json / Cargo.toml with tool info
7. Generate README.md with all sections
8. Create CHANGELOG.md with initial entry
9. Generate DOGFOODING_STRATEGY.md
10. Create dogfood-*.sh scripts
11. Set up test boilerplate

### Phase 3: Documentation (4 steps)

12. Create docs/ directory structure
13. Generate 6 VitePress pages from templates
14. Create placeholder demo.gif
15. Create demo recording script

### Phase 4: Meta Repo Integration (7 steps)

16. Add as git submodule to tools/
17. Update docs/.vitepress/config.ts (sidebar)
18. Update docs/tools/index.md (count, card)
19. Update docs/index.md (home page)
20. Update README.md (tool list)
21. Update .claude/NEXT_TASKS.md
22. Update docs/guide/getting-started.md

### Phase 5: CI/CD Setup (3 steps)

23. Add path filter to create-demos.yml
24. Verify tool's test.yml workflow exists
25. Create GitHub labels (if centralized issues)

### Phase 6: Verification (4 steps)

26. Run `npm install` / `cargo build`
27. Run `npm test` / `cargo test`
28. Run `npm run docs:build` (VitePress)
29. Run `/quality-check`

### Phase 7: Commit (2 steps)

30. Commit tool repo: `feat: initialize {tool-name} from template`
31. Push tool repo to origin
32. Commit meta repo: `chore: add {tool-name} submodule`

## Environment Detection

The command automatically detects CLI vs Web environment:

**CLI Environment (with MCP):**
- Uses `tuulbelt-github` MCP server for GitHub operations
- Fully automated GitHub repo creation
- No manual steps required

**Web Environment (without MCP):**
- Provides manual instructions for GitHub repo creation
- User creates repo via GitHub UI or `gh` CLI
- Continues with scaffolding after user confirms repo creation

## How to Use This Command

1. **Ensure you're in the meta repository root**
   ```bash
   cd /path/to/tuulbelt
   ```

2. **Decide on tool name and short name**
   - Tool name: Full kebab-case (e.g., `component-prop-validator`)
   - Short name: Concise CLI alias (e.g., `propval`)
   - See `.claude/NEXT_TASKS.md` for naming conventions

3. **Run the command**
   ```
   /new-tool component-prop-validator typescript propval "Runtime prop validation"
   ```

4. **Follow prompts**
   - CLI: Automated via MCP server
   - Web: Follow manual GitHub repo creation instructions

5. **Verify completion**
   - Tool repo created at `tools/component-prop-validator/`
   - GitHub repo: `https://github.com/tuulbelt/component-prop-validator`
   - All tracking documents updated

## Quality Gates

The command enforces these quality standards:

- ✅ Build must succeed (`npm run build` or `cargo build`)
- ✅ Tests must pass (`npm test` or `cargo test`)
- ✅ VitePress must build (`npm run docs:build`)
- ✅ Zero runtime dependencies verified
- ✅ All required documentation generated

If any gate fails, the command will:
1. Report the failure with diagnostics
2. NOT create the final commits
3. Provide instructions to fix manually
4. Allow resuming once fixed

## Post-Creation Tasks

After the tool is created, you should:

1. **Implement the tool**
   ```bash
   cd tools/component-prop-validator
   npm install  # or cargo build
   npm test
   ```

2. **Add real implementation**
   - Replace placeholder code in `src/index.ts` or `src/lib.rs`
   - Add real tests
   - Update documentation

3. **Run dogfood scripts**
   ```bash
   ./scripts/dogfood-flaky.sh   # Validate test determinism
   ./scripts/dogfood-diff.sh    # Verify consistent outputs
   ```

4. **Create demo**
   ```bash
   ./scripts/record-*-demo.sh   # Record demo for GitHub Pages
   ```

5. **Create first release**
   ```
   /release-tool component-prop-validator 0.1.0 "Initial release"
   ```

## Troubleshooting

### "Repository already exists"
- Check GitHub: `gh repo view tuulbelt/{tool-name}`
- Delete if test repo: `gh repo delete tuulbelt/{tool-name} --confirm`
- Use different name

### "GITHUB_TOKEN not set"
- Set token: `export GITHUB_TOKEN=your_token`
- Or create repo manually and provide URL when prompted

### "MCP server not available"
- Running in Web environment - follow manual instructions
- Or restart Claude Code CLI to load `.mcp.json`

### "Tests fail after creation"
- This is expected - template has placeholder tests
- Implement real functionality and tests
- Don't commit until tests pass

### "VitePress build fails"
- Check for dead links in generated docs
- Verify all required pages exist
- Run `npm run docs:build` manually for details

## References

- Migration Plan: `docs/MIGRATION_TO_META_REPO.md` (Section C.2)
- Tool Creator Agent: `.claude/agents/tool-creator.md`
- MCP Server: `.claude/mcp/tuulbelt-github/README.md`
- Quality Checklist: `docs/QUALITY_CHECKLIST.md`
- Template Documentation: `templates/README.md`

## Related Commands

- `/release-tool` - Create new version release
- `/add-tool-dependency` - Add Tuulbelt tool dependency
- `/sync-tool-docs` - Synchronize README and VitePress docs
- `/update-all-counts` - Update tool counts across docs
- `/quality-check` - Run pre-commit quality checks
