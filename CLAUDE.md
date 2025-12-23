# Tuulbelt Meta Repository

## Project Overview

Tuulbelt is a meta-repository for curating focused, zero-dependency tools and utilities for modern software development. Each tool solves one specific problem and is maintained as an independent repository.

This meta repository provides:
- Scaffolding templates for TypeScript and Rust tools
- Standardized development workflows
- Cross-cutting quality assurance (testing, security, linting)
- Claude Code automation for consistent development

See @README.md for detailed architecture and @PRINCIPLES.md for design philosophy.

## Tech Stack

**TypeScript Tools:**
- Runtime: Node.js 18+
- Language: TypeScript 5.3+ (strict mode)
- Test Framework: Node.js native test runner (`node:test`)
- Assertions: Node.js native (`node:assert/strict`)
- Dev Dependencies: TypeScript, tsx only
- **Zero Runtime Dependencies**

**Rust Tools:**
- Rust Edition: 2021+
- Test Framework: Built-in cargo test
- Linter: clippy (all warnings denied)
- Formatter: rustfmt
- **Zero Runtime Dependencies**

**Development Tools:**
- Git with conventional commits
- GitHub Actions for CI/CD
- npm/yarn for TypeScript package management
- cargo for Rust package management

## Project Structure

```
/home/user/tuulbelt/
├── .claude/                      # Claude Code workflows & automation
│   ├── commands/                 # Reusable slash commands
│   │   ├── test-all.md          # Run all tests (TS + Rust)
│   │   ├── security-scan.md     # Security analysis
│   │   ├── scaffold-tool.md     # Create new tool from template
│   │   └── git-commit.md        # Semantic commits
│   ├── agents/                  # Specialized AI agents
│   │   ├── test-runner.md       # Testing expert
│   │   ├── security-reviewer.md # Security expert
│   │   └── scaffold-assistant.md# Scaffolding expert
│   ├── skills/                  # Cross-cutting expertise
│   │   ├── typescript-patterns/ # TypeScript best practices
│   │   ├── rust-idioms/        # Rust best practices
│   │   └── zero-deps-checker/  # Dependency validation
│   ├── hooks/                  # Quality gates & automation
│   │   ├── pre-write-check.sh  # Protect sensitive files
│   │   ├── post-write-format.sh# Auto-format code
│   │   └── post-bash-audit.sh  # Command auditing
│   └── settings.json           # Hook & permission config
├── templates/                   # Tool scaffolding templates
│   ├── tool-repo-template/     # TypeScript/Node.js
│   └── rust-tool-template/     # Rust
├── docs/                       # Development documentation
├── CLAUDE.md                   # This file
├── README.md                   # Project architecture
├── PRINCIPLES.md               # Design philosophy
└── CONTRIBUTING.md             # Contribution workflow
```

## Quick Commands

### Development Workflows

```bash
# Create new tool from template
/scaffold-tool <tool-name> <typescript|rust>

# Run all tests across templates
/test-all [filter]

# Security analysis
/security-scan

# Semantic git commit
/git-commit <type> <scope> <message>
```

### TypeScript Development

```bash
# In any tool repository using TypeScript template
cd tool-repo

# Install dependencies
npm install

# Run tests
npm test
npm test -- --watch

# Type checking
npx tsc --noEmit

# Linting (manual - no linter in zero-dep tools)
# Code style enforced by conventions, not tools
```

### Rust Development

```bash
# In any tool repository using Rust template
cd tool-repo

# Run tests
cargo test
cargo test -- --nocapture

# Check without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Build release
cargo build --release
```

## Code Conventions

### TypeScript Style

**File Naming:**
- Source files: `kebab-case.ts`
- Test files: `kebab-case.test.ts`
- Types/Interfaces: Defined in same file or `types.ts`

**Code Style:**
```typescript
// Explicit return types
export function processData(input: string): Result {
  // Implementation
}

// Result pattern for errors
export type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: Error };

// Async/await (not callbacks or raw Promises)
export async function fetchData(): Promise<Result<Data>> {
  try {
    const response = await fetch(url);
    const data = await response.json();
    return { ok: true, value: data };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}
```

**Testing Pattern:**
```typescript
import { test } from 'node:test';
import assert from 'node:assert/strict';

test('should process valid input', () => {
  const result = processData('input');
  assert.strictEqual(result.ok, true);
});

test('should handle errors gracefully', () => {
  const result = processData('');
  assert.strictEqual(result.ok, false);
});
```

### Rust Style

**File Naming:**
- Library: `src/lib.rs`
- Binary: `src/main.rs`
- Modules: `src/module_name.rs` or `src/module_name/mod.rs`
- Tests: `tests/integration_test.rs`

**Code Style:**
```rust
// Explicit error types
pub enum ProcessError {
    InvalidInput(String),
    IoError(std::io::Error),
}

// Result pattern for all fallible operations
pub fn process_data(input: &str) -> Result<Data, ProcessError> {
    if input.is_empty() {
        return Err(ProcessError::InvalidInput("empty input".to_string()));
    }
    // Implementation
    Ok(data)
}

// Async with tokio (if needed)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = fetch_data().await?;
    Ok(())
}
```

**Testing Pattern:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let result = process_data("input");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_input() {
        let result = process_data("");
        assert!(result.is_err());
    }
}
```

## Git Conventions

All commits follow **Conventional Commits** specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code restructuring without behavior change
- `test`: Test additions or modifications
- `docs`: Documentation updates
- `chore`: Tooling, configuration, dependencies
- `ci`: CI/CD pipeline changes
- `perf`: Performance improvements

**Scopes:** (examples for tool repositories)
- `cli`: Command-line interface
- `core`: Core functionality
- `types`: Type definitions
- `test`: Test infrastructure
- `build`: Build system

**Examples:**
```bash
feat(cli): add verbose logging option
fix(core): handle edge case in path normalization
test(core): add coverage for error conditions
docs(readme): update installation instructions
chore(deps): update TypeScript to 5.3.3
```

## Testing Standards

### Coverage Requirements

- **Minimum 80% line coverage** for all tools
- **90% coverage for critical paths** (data processing, error handling)
- Edge cases must be tested (empty input, malformed data, large inputs)

### Test Organization

**TypeScript:**
```
src/
├── index.ts
└── index.test.ts    # Colocated tests
```

**Rust:**
```
src/
├── lib.rs           # Unit tests in #[cfg(test)] modules
└── main.rs

tests/
└── integration.rs   # Integration tests
```

### Running Tests

```bash
# TypeScript
npm test                    # Run all tests
npm test -- --watch        # Watch mode

# Rust
cargo test                 # Run all tests
cargo test -- --nocapture  # Show output
cargo tarpaulin --out Html # Coverage report
```

## Security Guidelines

### Protected Patterns

Claude Code hooks prevent modification of:
- `package-lock.json` / `yarn.lock` (use package manager commands)
- `Cargo.lock` (use cargo commands)
- `.env*` files (environment configuration)
- `*.secret.*` / `*.private.*` (security-sensitive files)

### Security Checklist (enforced by /security-scan)

1. **No hardcoded credentials** - API keys, passwords, tokens
2. **Input validation** - All external input sanitized
3. **Path traversal prevention** - Validate file paths
4. **Dependency auditing** - `npm audit` / `cargo audit` clean
5. **Error messages** - No sensitive data in error output
6. **TLS/HTTPS** - All external API calls use HTTPS
7. **No eval/exec** - No dynamic code execution with user input

## Zero Dependencies Principle

**Every Tuulbelt tool must have ZERO runtime dependencies.**

**Allowed:**
- Standard library (Node.js built-ins, Rust std)
- Language runtime (Node.js, Rust compiler)

**Not Allowed:**
- npm packages as dependencies (devDependencies OK)
- External crates as dependencies (dev-dependencies OK)
- System libraries beyond standard runtime

**Validation:**
```bash
# TypeScript - package.json should have:
"dependencies": {}

# Rust - Cargo.toml should have:
[dependencies]
# (empty)
```

The `zero-deps-checker` skill automatically validates this.

## Available Workflows

### Slash Commands

Invoke with `/command-name`:

- `/test-all [filter]` - Run TypeScript and Rust tests
- `/security-scan` - Comprehensive security analysis
- `/scaffold-tool <name> <lang>` - Create new tool repository
- `/git-commit <type> <scope> <msg>` - Semantic commit with validation

### Specialized Agents

Automatically invoked when needed:

- `test-runner` - Expert in test execution, coverage, debugging
- `security-reviewer` - Security analysis, vulnerability scanning
- `scaffold-assistant` - Tool creation, template customization

### Skills

LLM-invoked capabilities (automatic):

- `typescript-patterns` - TypeScript best practices enforcement
- `rust-idioms` - Rust best practices enforcement
- `zero-deps-checker` - Validate zero-dependency principle

## Quality Gates (Hooks)

Automated enforcement via Claude Code hooks:

**Pre-Write:**
- Prevent modification of protected files

**Post-Write:**
- Auto-format with prettier (TypeScript)
- Auto-format with rustfmt (Rust)

**Post-Bash:**
- Audit dangerous commands (rm -rf, etc.)
- Log all command executions

**On Stop:**
- Cleanup temporary files
- Archive session logs

## Common Workflows

### Creating a New Tool

1. Use scaffolding command:
   ```bash
   /scaffold-tool my-awesome-tool typescript
   ```

2. Customize implementation in new repository

3. Run tests and validate:
   ```bash
   /test-all
   /security-scan
   ```

4. Commit with conventions:
   ```bash
   /git-commit feat core "implement main functionality"
   ```

### Adding Tests

1. Write test colocated with source (TypeScript) or in `tests/` (Rust)

2. Run tests:
   ```bash
   npm test        # TypeScript
   cargo test      # Rust
   ```

3. Verify coverage meets 80% threshold

4. Commit:
   ```bash
   /git-commit test core "add edge case coverage"
   ```

### Security Review Before Release

1. Run comprehensive scan:
   ```bash
   /security-scan
   ```

2. Review findings and address issues

3. Re-run until clean

4. Proceed with release

## References

- @README.md - Project architecture
- @PRINCIPLES.md - Design philosophy
- @CONTRIBUTING.md - Contribution workflow
- @docs/testing-standards.md - Testing requirements
- @docs/security-guidelines.md - Security checklist
- @templates/tool-repo-template/ - TypeScript template
- @templates/rust-tool-template/ - Rust template

## Known Limitations

- Templates assume Unix-like environment (Linux, macOS)
- Windows support requires WSL or Git Bash
- Cross-platform path handling must be explicit in tools
- CI/CD assumes GitHub Actions (adaptable to other platforms)

---

*This CLAUDE.md file is automatically included in Claude Code's context for all development in this repository.*
