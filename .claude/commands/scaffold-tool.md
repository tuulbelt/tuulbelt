---
description: Create a new tool repository from template
argument-hint: "<tool-name> <typescript|rust>"
---

Create a new tool repository using the appropriate template.

## Arguments

- **Tool Name**: $1 (kebab-case, e.g., "path-normalizer")
- **Language**: $2 (either "typescript" or "rust")

## Validation

First, validate the arguments:

1. Tool name must be kebab-case (lowercase with hyphens)
2. Language must be either "typescript" or "rust"
3. Tool name must not already exist in the current directory

## Scaffolding Process

Based on the language choice, copy the appropriate template and customize it:

### For TypeScript Tools

1. Copy template:
   ```bash
   cp -r templates/tool-repo-template "$1"
   ```

2. Customize package.json:
   - Update name, description, repository URL
   - **Update `bin` entry with short CLI name and full tool name:**
     ```json
     "bin": {
       "short-name": "./src/index.ts",
       "tool-name": "./src/index.ts"
     }
     ```
   - Choose short name: See `.claude/NEXT_TASKS.md` for proposed names

3. Verify src/index.ts has the shebang (required for `npm link`):
   ```typescript
   #!/usr/bin/env -S npx tsx
   ```

4. Update README.md:
   - Replace `tool-name` placeholder with actual tool name
   - Replace `short-name` placeholder with chosen short name
   - Both forms should be documented with short form recommended

4. Customize STATUS.md:
   - Replace {{TOOL_NAME}} with the actual tool name
   - Replace {{DATE}} with current date (YYYY-MM-DD format)

5. Customize CHANGELOG.md:
   - Replace {{TOOL_NAME}} with the actual tool name

6. Initialize git repository:
   ```bash
   cd "$1"
   git init
   git add .
   git commit -m "feat: initialize $1 tool from template"
   ```

### For Rust Tools

1. Copy template:
   ```bash
   cp -r templates/rust-tool-template "$1"
   ```

2. Customize Cargo.toml:
   - Update name, description, repository URL
   - **Update `[[bin]]` entries with short and long names:**
     ```toml
     [[bin]]
     name = "short-name"
     path = "src/main.rs"

     [[bin]]
     name = "tool-name"
     path = "src/main.rs"
     ```
   - Choose short name: See `.claude/NEXT_TASKS.md` for proposed names

3. Update README.md:
   - Replace `tool-name` placeholder with actual tool name
   - Replace `short-name` placeholder with chosen short name
   - Both forms should be documented with short form recommended

4. Customize STATUS.md:
   - Replace {{TOOL_NAME}} with the actual tool name
   - Replace {{DATE}} with current date (YYYY-MM-DD format)

5. Customize CHANGELOG.md:
   - Replace {{TOOL_NAME}} with the actual tool name

6. Initialize git repository:
   ```bash
   cd "$1"
   git init
   git add .
   git commit -m "feat: initialize $1 tool from template"
   ```

## Post-Scaffolding

After creating the tool:

1. Verify tests run:
   - TypeScript: `cd $1 && npm install && npm test`
   - Rust: `cd $1 && cargo test`

2. **Customize dogfood scripts:**
   The template includes `scripts/dogfood-flaky.sh` and `scripts/dogfood-diff.sh`.
   Update these scripts to replace `[TOOL_NAME]` placeholder with actual tool name.

   Also customize `DOGFOODING_STRATEGY.md`:
   - Replace `[Tool Name]` with actual tool name
   - Document why each composition provides value
   - Add tool-specific dogfood scripts if needed

   **Verify dogfood scripts work:**
   ```bash
   cd $1
   ./scripts/dogfood-flaky.sh 5  # Quick validation
   ./scripts/dogfood-diff.sh     # Output determinism
   ```

   **CI Integration:**
   Dogfood scripts are automatically discovered and run by `dogfood-validation.yml`.
   No additional CI configuration needed.

3. Create demo recording script:
   ```bash
   # Create demo script in scripts/record-$1-demo.sh
   # This enables automatic demo recording via GitHub Actions workflow
   # Pattern: scripts/record-{tool-name}-demo.sh
   # The workflow auto-discovers and runs all demo scripts
   ```

   Use existing scripts as template:
   - `scripts/record-test-flakiness-detector-demo.sh`
   - `scripts/record-cli-progress-reporting-demo.sh`

   Key elements:
   - Set clean terminal environment
   - Record with asciinema (demo.cast file)
   - Upload to asciinema.org (if token available)
   - Convert to GIF (if agg available)
   - Include realistic usage scenario (~20-30 seconds)

   **Automatic Embedding:**
   When the workflow runs (on push to main when demo scripts change):
   - Generates `$1/demo.cast` (asciinema recording)
   - Generates `$1/docs/demo.gif` (animated GIF)
   - Generates `$1/demo-url.txt` (asciinema.org link)
   - **Automatically updates `$1/README.md` Demo section** with:
     - Embedded GIF: `![Demo](docs/demo.gif)`
     - Live asciinema.org link
   - **GIF is served on GitHub Pages** at `/toolname/demo.gif`
   - No manual README editing needed!

   **CI/CD Auto-Discovery:**
   All tools are automatically discovered and tested by CI workflows:
   - **TypeScript tools:** Any directory with `package.json` (excluding root, docs, templates, .github, .claude, scripts)
   - **Rust tools:** Any directory with `Cargo.toml` (excluding templates)
   - No workflow configuration needed - just create the tool directory!
   - Workflows auto-run on push to main and pull requests
   - Tests run via `npm test` or `cargo test`
   - Both `test-all-tools.yml` and `update-dashboard.yml` discover tools automatically

3. Create multi-tool documentation structure:
   ```bash
   mkdir -p docs/tools/$1
   ```

   Create these documentation pages in `docs/tools/$1/`:
   - `index.md` - Overview and features
   - `getting-started.md` - Installation and quick start
   - `cli-usage.md` - Command-line reference (if applicable)
   - `library-usage.md` - API usage (if applicable)
   - `examples.md` - Real-world usage patterns
   - `api-reference.md` - Complete API documentation

   See `docs/tools/cli-progress-reporting/` as the reference pattern.

3. Update VitePress configuration:
   - Add tool to `docs/.vitepress/config.ts` sidebar
   - Add tool card to `docs/index.md` landing page
   - Add tool to `docs/tools/index.md` tools list

5. Display next steps to the user:
   - Update README.md with tool description (include ## Demo section)
   - Implement core functionality in src/
   - Add comprehensive tests (80%+ coverage)
   - **Customize dogfood scripts** (replace [TOOL_NAME] placeholder)
   - **Update DOGFOODING_STRATEGY.md** with tool-specific composition strategy
   - **Create documentation in docs/tools/$1/**
   - **Create demo recording script in scripts/record-$1-demo.sh**
   - Update STATUS.md as you progress
   - Update CHANGELOG.md when releasing versions
   - Update root README.md and ROADMAP.md (add 🐕 badge if dogfooded)
   - Run `/security-scan` before first commit
   - **Demo will auto-appear in workflows** (no config needed)
   - **Dogfood scripts run automatically** after tests pass (dogfood-validation.yml)

## Output

Provide:
- Path to the new tool directory
- Language and template used
- Next steps for development
- **Documentation structure:** Remind to create docs/tools/$1/ pages
- **VitePress integration:** Update config.ts, index.md, tools/index.md
- Reminder to update README.md, SPEC.md, STATUS.md, and CHANGELOG.md
- Explain that STATUS.md enables session handoffs and progress tracking
- **Reference:** Point to docs/tools/cli-progress-reporting/ as documentation example
