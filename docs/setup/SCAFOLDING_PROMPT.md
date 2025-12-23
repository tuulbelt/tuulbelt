# Claude Code Initialization Prompt for Tuulbelt Meta Repo

Use this prompt in Claude Code after cloning the tuulbelt meta repository:

```
Initialize the tuulbelt meta repository using these guides:

SETUP: Reference the documentation files in docs/setup/:
- docs/setup/META_REPO_SETUP.md
- docs/setup/TUULBELT_TRIAGE.md
- docs/setup/TUULBELT_TEMPLATES.md

1. From docs/setup/META_REPO_SETUP.md Part 4, create all files with exact content:
   - README.md, PRINCIPLES.md, CONTRIBUTING.md, ARCHITECTURE.md
   - docs/claude-code-workflow.md, docs/testing-standards.md
   - docs/tool-template.md (stub), docs/security-guidelines.md (stub)

2. Create directory structure:
   - templates/tool-repo-template/ with package.json, tsconfig.json, src/index.ts, test/index.test.ts, examples/basic.ts, SPEC.md, LICENSE, .github/workflows/test.yml
   - templates/rust-tool-template/ with Cargo.toml, src/lib.rs, tests/, examples/, README.md
   - .github/ISSUE_TEMPLATE/ with bug_report.md, feature_request.md, tool_proposal.md

3. Create supporting files:
   - LICENSE (MIT)
   - .gitignore (Node/TypeScript/Rust standard)

4. After creating all files:
   - git add .
   - git commit -m "Initialize tuulbelt meta repo with templates and documentation"
   - git push origin main

Verify all 50+ files are created correctly and pushed before confirming completion.
```

---

## Documentation Placement

Before using the prompt, add these files to the meta repo:

```
tuulbelt/
├── docs/
│   └── setup/
│       ├── META_REPO_SETUP.md           # Add here
│       ├── TUULBELT_TRIAGE.md           # Add here
│       └── TUULBELT_TEMPLATES.md        # Add here
```

**Steps:**
1. Clone tuulbelt repo: `git clone https://github.com/tuulbelt/tuulbelt.git`
2. Create directories: `mkdir -p docs/setup`
3. Copy the three setup files into `docs/setup/`
4. Commit and push:
   ```bash
   git add docs/setup/
   git commit -m "Add setup documentation"
   git push origin main
   ```
5. Now open the repo in Claude Code and use the prompt above

This way Claude Code can reference the setup guides while creating the structure.
