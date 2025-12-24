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

2. Customize package.json with the tool name

3. Update README.md with the tool name and description

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

2. Customize Cargo.toml with the tool name

3. Update README.md with the tool name and description

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

2. Create multi-tool documentation structure:
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

4. Display next steps to the user:
   - Update README.md with tool description
   - Implement core functionality in src/
   - Add comprehensive tests (80%+ coverage)
   - **Create documentation in docs/tools/$1/**
   - Update STATUS.md as you progress
   - Update CHANGELOG.md when releasing versions
   - Update root README.md and ROADMAP.md
   - Run `/security-scan` before first commit

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
