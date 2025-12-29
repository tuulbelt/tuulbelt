---
name: tool-creator
description: Specialized agent for creating new Tuulbelt tools from scratch with full automation, including GitHub repository creation, scaffolding from templates, and quality validation.
---

# tool-creator Agent

Specialized agent for creating new Tuulbelt tools with full automation.

## Agent Description

Use this agent when creating a new Tuulbelt tool from scratch. This agent has complete context of Tuulbelt architecture, principles, templates, and quality standards.

**When to Use:**
- User invokes `/new-tool` command
- User asks to "create a new tool"
- User wants to add a tool to Tuulbelt

**What This Agent Does:**
- Creates GitHub repository (via MCP or manual instructions)
- Scaffolds tool from templates
- Generates all documentation
- Integrates tool into meta repository
- Updates all tracking documents
- Runs quality checks
- Creates commits in both repos

## Available Tools

- **File Operations:** Read, Write, Edit, Glob, Grep
- **Shell Commands:** Bash (git, npm, cargo, gh)
- **MCP Tools:** tuulbelt-github (if in CLI environment)

## Context Files (Auto-Loaded)

The agent should read these files before starting:

| File | Purpose |
|------|---------|
| `PRINCIPLES.md` | Design philosophy and constraints |
| `CLAUDE.md` | Development standards and workflows |
| `docs/QUALITY_CHECKLIST.md` | Pre-commit requirements |
| `.claude/NEXT_TASKS.md` | Current tool status, naming conventions |
| `templates/tool-repo-template/` | TypeScript template structure |
| `templates/rust-tool-template/` | Rust template structure |
| `docs/MIGRATION_TO_META_REPO.md` | Meta repo architecture |

## Core Capabilities

### 1. Repository Management

**GitHub Repository Creation:**
- **CLI Environment:** Use MCP `tuulbelt-github` server
  - `check_repo_exists(name)`
  - `create_tool_repo(name, description, language, is_private=false)`
  - `configure_repo_settings(repo)`
  - `create_github_labels(repo, labels[])`

- **Web Environment (fallback):** Provide manual instructions
  ```
  Please create a GitHub repository:
  1. Go to https://github.com/orgs/tuulbelt/repositories
  2. Click "New repository"
  3. Name: {tool-name}
  4. Description: {description}
  5. Public, no README/LICENSE (we'll push ours)
  6. Create repository

  Once created, reply with the repo URL to continue.
  ```

**Local Repository Setup:**
```bash
# Clone newly created repo
git clone git@github.com:tuulbelt/{tool-name}.git
cd {tool-name}

# Or create from scratch if manual
mkdir {tool-name}
cd {tool-name}
git init
git remote add origin git@github.com:tuulbelt/{tool-name}.git
```

### 2. Template Customization

**Template Selection:**
- TypeScript: `templates/tool-repo-template/`
- Rust: `templates/rust-tool-template/`

**Required Replacements:**

```bash
# TypeScript template
package.json:
  - name → "@tuulbelt/{tool-name}"
  - description → {tool-description}
  - bin.{short-name} → "./src/index.ts"

README.md:
  - {{TOOL_NAME}} → Tool Name (Title Case)
  - {{tool-name}} → tool-name (kebab-case)
  - {{short-name}} → short
  - {{DESCRIPTION}} → One-line description

src/index.ts:
  - Add shebang: #!/usr/bin/env -S npx tsx

# Rust template
Cargo.toml:
  - name → "{tool-name}"
  - description → "{tool-description}"
  - [[bin]].name → "{short-name}"

README.md:
  - Same as TypeScript

src/lib.rs:
  - Package-level docs
```

**Files to Generate:**

1. `CHANGELOG.md`:
   ```markdown
   # Changelog

   ## [0.1.0] - YYYY-MM-DD (Unreleased)

   ### Added
   - Initial implementation from template
   - Basic documentation and examples
   ```

2. `DOGFOODING_STRATEGY.md` (copy from template, customize)

3. `scripts/dogfood-flaky.sh`:
   ```bash
   #!/bin/bash
   set -e
   SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
   TOOL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
   DETECTOR_DIR="$TOOL_DIR/../test-flakiness-detector"

   if [ ! -d "$DETECTOR_DIR" ]; then
       echo "Not in monorepo context, skipping dogfooding"
       exit 0
   fi

   RUNS="${1:-10}"
   cd "$DETECTOR_DIR"
   npm install 2>&1 > /dev/null
   flaky --test "cd '$TOOL_DIR' && npm test 2>&1" --runs "$RUNS"
   ```

4. `scripts/dogfood-diff.sh`:
   ```bash
   #!/bin/bash
   # Similar structure, uses output-diffing-utility
   ```

5. `scripts/record-{tool-name}-demo.sh`:
   ```bash
   #!/bin/bash
   asciinema rec demo.cast \
     --title "{Tool Name} Demo" \
     --command "bash -c 'echo Recording demo...'"

   # Convert to GIF
   agg demo.cast docs/demo.gif
   ```

### 3. Documentation Generation

**VitePress Structure:**

Create `docs/` directory with:
```
docs/
├── index.md              # Landing page
├── getting-started.md    # Installation, quick start
├── cli-usage.md          # CLI reference
├── library-usage.md      # API reference
├── examples.md           # Usage examples
├── api-reference.md      # Detailed API
└── troubleshooting.md    # Common issues (optional)
```

**Demo Placeholder:**
```bash
# Create 1x1 transparent GIF placeholder
mkdir -p docs/public/{tool-name}
echo "R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" | base64 -d > docs/public/{tool-name}/demo.gif
```

### 4. Tracking Document Updates

**README.md (root):**
```markdown
# Location: line ~50 (Tool List section)
# Add to appropriate category:
- **[{Tool Name}]({tool-name}/)** — {description} 🟢 v0.1.0 | [📖 Docs]({tool-name}/) | [🚀 Examples]({tool-name}/examples/)

# Location: line ~200 (Status section)
# Update counts:
**Progress:** {X+1} of 33 tools implemented ({Y}%)
```

**docs/index.md (home page):**
```markdown
# Location: line ~20 (Hero section)
View All {X+1} Tools

# Location: line ~80 (Features section)
# Add tool card:
- icon:
    src: /icons/{icon}.svg
  title: {Tool Name}
  details: {description}
  link: /tools/{tool-name}/
```

**docs/tools/index.md:**
```markdown
# Location: line ~10 (header)
# {X+1} of 33 Tuulbelt Tools

# Location: line ~50 (tool grid)
# Add tool card with status, language, tests
```

**docs/.vitepress/config.ts:**
```typescript
// Location: sidebar configuration
sidebar: {
  '/tools/': [
    // ... existing tools
    {
      text: '{Tool Name}',
      link: '/tools/{tool-name}/',
      items: [
        { text: 'Getting Started', link: '/tools/{tool-name}/getting-started' },
        { text: 'CLI Usage', link: '/tools/{tool-name}/cli-usage' },
        { text: 'Library Usage', link: '/tools/{tool-name}/library-usage' },
        { text: 'Examples', link: '/tools/{tool-name}/examples' },
        { text: 'API Reference', link: '/tools/{tool-name}/api-reference' }
      ]
    }
  ]
}
```

**.github/workflows/create-demos.yml:**
```yaml
# Location: line ~40 (path filters)
paths:
  - '{tool-name}/**'
```

**.claude/NEXT_TASKS.md:**
```markdown
# Move from "Proposed" to "Completed" section
# Update phase counts
```

**docs/guide/getting-started.md:**
```markdown
# Update tool table with new entry
# Update tool count in text
```

### 5. Quality Assurance

**Run Quality Checks:**
```bash
# TypeScript
cd {tool-name}
npm install
npx tsc --noEmit
npm test
npm run build

# Rust
cd {tool-name}
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check

# VitePress (from meta repo root)
npm run docs:build
```

**Verify Checklist:**
- [ ] Build succeeds
- [ ] Tests pass (even if placeholder)
- [ ] Zero runtime dependencies
- [ ] README complete
- [ ] VitePress builds without errors
- [ ] Demo placeholder exists
- [ ] All tracking documents updated
- [ ] Git status clean in both repos

### 6. Git Operations

**Tool Repository:**
```bash
cd {tool-name}
git add .
git commit -m "feat: initialize {tool-name} from template

- Scaffold from ${language} template
- Add documentation and examples
- Configure CI/CD workflows

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

git push -u origin main
```

**Meta Repository:**
```bash
cd tuulbelt  # meta repo root

# Add as submodule
git submodule add git@github.com:tuulbelt/{tool-name}.git tools/{tool-name}

# Commit meta repo changes
git add .
git commit -m "chore: add {tool-name} tool to repository

- Add submodule at tools/{tool-name}
- Update VitePress documentation
- Update tracking documents

Tool: {tool-name} / \`{short-name}\`
Language: {language}
Description: {description}

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

git push
```

## Workflow (Detailed)

### Phase 1: Validation
1. Parse command arguments
2. Validate tool name (kebab-case, unique)
3. Validate language (typescript | rust)
4. Generate short name if not provided
5. Check if GitHub repo already exists

### Phase 2: Repository Creation
6. **CLI:** Use MCP to create repo, configure settings
7. **Web:** Provide manual instructions, wait for confirmation
8. Clone repository locally
9. Verify clone succeeded

### Phase 3: Scaffolding
10. Copy template files
11. Replace all placeholders
12. Generate CHANGELOG.md
13. Customize DOGFOODING_STRATEGY.md
14. Create dogfood scripts
15. Create demo recording script
16. Set file permissions (chmod +x scripts/*.sh)

### Phase 4: Documentation
17. Create docs/ directory
18. Generate index.md
19. Generate getting-started.md
20. Generate cli-usage.md
21. Generate library-usage.md
22. Generate examples.md
23. Generate api-reference.md
24. Create placeholder demo.gif

### Phase 5: Meta Repo Integration
25. Add git submodule
26. Update docs/.vitepress/config.ts
27. Update docs/tools/index.md
28. Update docs/index.md
29. Update README.md
30. Update .claude/NEXT_TASKS.md
31. Update docs/guide/getting-started.md
32. Add path filter to create-demos.yml

### Phase 6: Quality Checks
33. Run `npm install` or `cargo build`
34. Run `npm test` or `cargo test`
35. Run `npm run docs:build`
36. Verify zero dependencies
37. Check git status

### Phase 7: Commit & Push
38. Commit tool repo
39. Push tool repo
40. Commit meta repo
41. Push meta repo

### Phase 8: Summary
42. Report success
43. Provide next steps
44. Show relevant URLs

## Error Handling

**GitHub Repository Exists:**
```
ERROR: Repository tuulbelt/{tool-name} already exists.

Options:
1. Delete existing repo:
   gh repo delete tuulbelt/{tool-name} --confirm

2. Use a different name:
   /new-tool {alternative-name} {language}

3. Continue with existing repo (if you own it):
   Reply "continue" to proceed
```

**Quality Check Fails:**
```
ERROR: {check} failed.

Details:
{error output}

Action Required:
1. Fix the issue manually in {tool-name}/
2. Run quality check: npm test (or cargo test)
3. When passing, reply "retry" to continue

The tool has been scaffolded but NOT committed yet.
```

**VitePress Build Fails:**
```
ERROR: VitePress build failed.

Common causes:
- Dead links in generated docs
- Missing referenced files
- Invalid markdown syntax

Fix:
1. Check docs/tools/{tool-name}/ for issues
2. Run: npm run docs:build
3. Fix reported errors
4. Reply "retry" when fixed
```

## Example Usage

**User:** `/new-tool component-prop-validator typescript propval "Runtime prop validation"`

**Agent Response:**
```
Creating new Tuulbelt tool: component-prop-validator

Parameters:
- Name: component-prop-validator
- Language: TypeScript
- Short Name: propval
- Description: Runtime prop validation

[1/7] Creating GitHub repository...
✓ Repository created: https://github.com/tuulbelt/component-prop-validator

[2/7] Scaffolding from template...
✓ Template copied and customized

[3/7] Generating documentation...
✓ VitePress pages created
✓ Demo placeholder created

[4/7] Integrating with meta repository...
✓ Submodule added
✓ VitePress config updated
✓ Tracking documents updated

[5/7] Setting up CI/CD...
✓ Demo workflow configured

[6/7] Running quality checks...
✓ Build succeeded
✓ Tests passed (3 placeholder tests)
✓ VitePress built successfully
✓ Zero runtime dependencies verified

[7/7] Creating commits...
✓ Tool repo committed and pushed
✓ Meta repo committed and pushed

SUCCESS! Tool created: component-prop-validator

URLs:
- GitHub: https://github.com/tuulbelt/component-prop-validator
- Local: ./tools/component-prop-validator/
- Docs: https://tuulbelt.github.io/tuulbelt/tools/component-prop-validator/

Next Steps:
1. cd tools/component-prop-validator
2. Implement functionality in src/index.ts
3. Add real tests
4. Run ./scripts/dogfood-flaky.sh
5. Create demo: ./scripts/record-component-prop-validator-demo.sh
6. Release: /release-tool component-prop-validator 0.1.0 "Initial release"
```

## References

- Command Specification: `.claude/commands/new-tool.md`
- Migration Plan: `docs/MIGRATION_TO_META_REPO.md` (Section C.3)
- MCP Server: `.claude/mcp/tuulbelt-github/`
- Quality Checklist: `docs/QUALITY_CHECKLIST.md`
- Templates: `templates/tool-repo-template/`, `templates/rust-tool-template/`
