---
name: zero-deps-checker
description: "Validate adherence to Tuulbelt's zero-dependency principle. Automatically checks package.json and Cargo.toml for runtime dependencies and flags violations."
---

# Zero Dependencies Checker

## Principle

**Every Tuulbelt tool must have ZERO runtime dependencies.**

This is a core architectural principle that ensures:
- No supply chain vulnerabilities
- Minimal attack surface
- Long-term maintainability
- Fast installation
- No dependency conflicts
- Tools work in restricted environments

## What's Allowed

### TypeScript/Node.js

**Allowed:**
- Node.js built-in modules (`fs`, `path`, `crypto`, `http`, etc.)
- TypeScript compiler (dev dependency)
- Test runners (dev dependency)
- Build tools (dev dependency)

**Not Allowed:**
- npm packages in `dependencies`
- Global npm packages that tools depend on
- Peer dependencies

**package.json structure:**

```json
{
  "name": "my-tool",
  "dependencies": {},  // MUST BE EMPTY
  "devDependencies": {
    "typescript": "^5.3.0",  // OK
    "tsx": "^4.7.0"           // OK
  }
}
```

### Rust

**Allowed:**
- Rust standard library (`std`)
- Core library (`core`)
- Test framework (built-in)
- Build dependencies (if absolutely necessary)
- Dev dependencies for testing

**Not Allowed:**
- External crates in `[dependencies]`
- proc-macro dependencies (unless for tests)

**Cargo.toml structure:**

```toml
[package]
name = "my-tool"
version = "0.1.0"
edition = "2021"

[dependencies]
# MUST BE EMPTY

[dev-dependencies]
# Test-only dependencies OK
tokio = { version = "1", features = ["test-util"] }  # OK for tests

[build-dependencies]
# Build-time only, use sparingly
```

## Validation Commands

### TypeScript Validation

```bash
# Check for external runtime dependencies (excludes @tuulbelt/* packages)
external_deps=$(jq -r '.dependencies // {} | keys | map(select(startswith("@tuulbelt/") | not)) | length' package.json)
if [ "$external_deps" -gt 0 ]; then
  echo "❌ VIOLATION: Found $external_deps external runtime dependencies"
  jq -r '.dependencies | to_entries | map(select(.key | startswith("@tuulbelt/") | not)) | from_entries' package.json
  exit 1
else
  echo "✅ Zero external dependencies (TypeScript)"
fi

# Show Tuulbelt dependencies (informational)
tuulbelt_deps=$(jq -r '.dependencies // {} | keys | map(select(startswith("@tuulbelt/"))) | length' package.json)
if [ "$tuulbelt_deps" -gt 0 ]; then
  echo "ℹ️  Found $tuulbelt_deps Tuulbelt dependencies (allowed per Exception 2)"
  jq -r '.dependencies | to_entries | map(select(.key | startswith("@tuulbelt/"))) | .[].key' package.json
fi

# Check for peer dependencies
peer_deps=$(jq -r '.peerDependencies // {} | length' package.json)
if [ "$peer_deps" -gt 0 ]; then
  echo "⚠️  WARNING: Found peer dependencies (should be avoided)"
  jq -r '.peerDependencies' package.json
fi
```

### Rust Validation

```bash
# Check for external runtime dependencies (excludes Tuulbelt git dependencies)
deps_section=$(awk '/^\[dependencies\]/,/^\[/' Cargo.toml | grep -v "^\[" | grep -v "^#" | grep -v "^$")
external_deps=$(echo "$deps_section" | grep -v "github.com/tuulbelt/" | grep -c "." || echo "0")

if [ "$external_deps" -gt 0 ]; then
  echo "❌ VIOLATION: Found $external_deps external runtime dependencies in Cargo.toml"
  echo "$deps_section" | grep -v "github.com/tuulbelt/"
  exit 1
else
  echo "✅ Zero external dependencies (Rust)"
fi

# Show Tuulbelt dependencies (informational)
tuulbelt_deps=$(echo "$deps_section" | grep -c "github.com/tuulbelt/" || echo "0")
if [ "$tuulbelt_deps" -gt 0 ]; then
  echo "ℹ️  Found $tuulbelt_deps Tuulbelt dependencies (allowed per Exception 2)"
  echo "$deps_section" | grep "github.com/tuulbelt/"
fi

# Check dev-dependencies (informational)
dev_deps=$(grep -A 100 "^\[dev-dependencies\]" Cargo.toml | grep -v "^\[" | grep -v "^#" | grep -v "^$" | wc -l)
if [ "$dev_deps" -gt 0 ]; then
  echo "ℹ️  Found $dev_deps dev dependencies (OK for testing)"
fi
```

## Common Violations and Fixes

### TypeScript

#### Violation: Using lodash

**BAD:**
```json
{
  "dependencies": {
    "lodash": "^4.17.21"
  }
}
```

```typescript
import _ from 'lodash';
const unique = _.uniq(array);
```

**GOOD:**
```json
{
  "dependencies": {}
}
```

```typescript
// Use native JavaScript
const unique = [...new Set(array)];
```

#### Violation: Using commander for CLI parsing

**BAD:**
```json
{
  "dependencies": {
    "commander": "^11.0.0"
  }
}
```

**GOOD:**
```typescript
// Parse arguments manually (see typescript-patterns skill)
function parseArgs(args: string[]): Args {
  // Custom parsing logic using process.argv
}
```

#### Violation: Using axios for HTTP

**BAD:**
```json
{
  "dependencies": {
    "axios": "^1.0.0"
  }
}
```

**GOOD:**
```typescript
// Use native fetch (Node.js 18+) or http/https modules
const response = await fetch(url);
const data = await response.json();
```

### Rust

#### Violation: Using serde for JSON

**BAD:**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**GOOD:**
```rust
// Parse JSON manually using std::str or custom parser
// For simple cases, use string manipulation
// For complex cases, implement a minimal parser
```

#### Violation: Using clap for CLI parsing

**BAD:**
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

**GOOD:**
```rust
// Parse arguments manually (see rust-idioms skill)
use std::env;

fn parse_args() -> Result<Args, String> {
  let args: Vec<String> = env::args().collect();
  // Custom parsing logic
}
```

#### Violation: Using reqwest for HTTP

**BAD:**
```toml
[dependencies]
reqwest = "0.11"
```

**GOOD:**
```rust
// Use std::net for TCP/HTTP or implement minimal HTTP client
// Or use std::process::Command to call curl if acceptable
use std::process::Command;

let output = Command::new("curl")
    .arg(url)
    .output()?;
```

## Acceptable Workarounds

### When Functionality Requires External Code

**Option 1: Implement it yourself**
- Write minimal implementation using std library
- Focus on specific use case (not general purpose)
- Example: Simple JSON parser for specific schema

**Option 2: Vendor/inline code**
- Copy minimal implementation into your tool
- Properly attribute original source (license compliance)
- Example: Small SHA-256 implementation

**Option 3: Use system tools**
- Shell out to system utilities
- Example: Use `curl` for HTTP instead of library
- Ensure tool availability or graceful degradation

**Option 4: Generate code at build time**
- Use build.rs to generate code
- Put generator in `[build-dependencies]`
- Generated code has no runtime dependencies

### Build Dependencies

**Acceptable use cases:**
```toml
[build-dependencies]
bindgen = "0.69"  # Generate FFI bindings at build time
```

Only use build dependencies when:
- Generated code has no runtime deps
- Cannot be pre-generated and checked in
- Significantly improves maintainability

### Dev Dependencies

**Always acceptable:**
```toml
[dev-dependencies]
tokio = { version = "1", features = ["test-util"] }
criterion = "0.5"  # Benchmarking
proptest = "1.0"   # Property testing
```

Dev dependencies don't affect runtime, so they're fine.

## CI/CD Enforcement

### GitHub Actions Workflow

Add to `.github/workflows/test.yml`:

```yaml
jobs:
  check-dependencies:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Check TypeScript dependencies
        if: hashFiles('package.json')
        run: |
          # Filter out @tuulbelt/* packages (allowed per Exception 2)
          external=$(node -e "
            const p = require('./package.json');
            const deps = Object.keys(p.dependencies || {}).filter(d => !d.startsWith('@tuulbelt/'));
            if (deps.length > 0) {
              console.error('Found external dependencies:', deps);
              process.exit(1);
            }
            console.log('✅ Zero external dependencies');
          ")

      - name: Check Rust dependencies
        if: hashFiles('Cargo.toml')
        run: |
          # Filter out Tuulbelt git dependencies (allowed per Exception 2)
          deps=$(awk '/^\[dependencies\]/,/^\[/' Cargo.toml | grep -v "^\[" | grep -v "^#" | grep -v "^$" | grep -v "github.com/tuulbelt/")
          if [ -n "$deps" ]; then
            echo "Error: Found external runtime dependencies"
            echo "$deps"
            exit 1
          fi
          echo "✅ Zero external dependencies"
```

## Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash

# TypeScript check (excludes @tuulbelt/* packages per Exception 2)
if [ -f package.json ]; then
  external_deps=$(jq -r '.dependencies // {} | keys | map(select(startswith("@tuulbelt/") | not)) | length' package.json)
  if [ "$external_deps" -gt 0 ]; then
    echo "❌ Cannot commit: External runtime dependencies found in package.json"
    jq -r '.dependencies | to_entries | map(select(.key | startswith("@tuulbelt/") | not)) | from_entries' package.json
    exit 1
  fi
fi

# Rust check (excludes Tuulbelt git dependencies per Exception 2)
if [ -f Cargo.toml ]; then
  deps=$(awk '/^\[dependencies\]/,/^\[/' Cargo.toml | grep -v "^\[" | grep -v "^#" | grep -v "^$" | grep -v "github.com/tuulbelt/")
  if [ -n "$deps" ]; then
    echo "❌ Cannot commit: External runtime dependencies found in Cargo.toml"
    echo "$deps"
    exit 1
  fi
fi

echo "✅ Zero external dependency check passed"
```

## Exceptions

### Exception 2: Tuulbelt-to-Tuulbelt Composition

Per [PRINCIPLES.md](../../../PRINCIPLES.md), Tuulbelt tools **MAY** depend on other Tuulbelt tools via git URL dependencies.

**Why this is allowed:**
- All Tuulbelt tools are themselves zero-dependency
- Composition preserves the zero-dep guarantee transitively
- Enables richer functionality without external dependency chains
- No supply chain risk (we control all dependencies)

**Allowed patterns (TypeScript):**
```json
{
  "dependencies": {
    "@tuulbelt/cli-progress-reporting": "git+https://github.com/tuulbelt/cli-progress-reporting.git"
  }
}
```

**Allowed patterns (Rust):**
```toml
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility.git" }
```

**Examples in production:**
- `test-flakiness-detector` → depends on `@tuulbelt/cli-progress-reporting`
- `port-resolver` → depends on `@tuulbelt/file-based-semaphore-ts`
- `snapshot-comparison` → depends on `output_diffing_utility`

**Validation:** When checking for violations, filter out:
- TypeScript: packages starting with `@tuulbelt/`
- Rust: git dependencies pointing to `github.com/tuulbelt/`

### External Dependencies

**External dependencies remain PROHIBITED.**

If a tool requires non-Tuulbelt external dependencies:
1. It doesn't belong in Tuulbelt
2. Consider splitting: core tool (zero deps) + optional wrapper
3. Re-evaluate if the problem is narrow enough

### Splitting Approach

**Example: Tool needs JSON schema validation**

**Option A:** Implement minimal validator for specific schema
```
tool-name/           # Zero deps, validates known schema
```

**Option B:** Core + wrapper pattern
```
tool-name-core/      # Zero deps, basic validation
tool-name-full/      # With deps, comprehensive validation (separate repo)
```

Choose Option A whenever possible.

## When to Run This Check

**Automatically:**
- In Claude Code hooks (PreToolUse/Write on package.json or Cargo.toml)
- In CI/CD pipeline
- In pre-commit hooks

**Manually:**
- Before every commit (`/security-scan` includes this)
- Before creating PR
- During code review

## Educational Notes

**Why Zero Dependencies?**

1. **Security**: Each dependency is a potential vulnerability
   - Example: left-pad incident (2016)
   - Example: event-stream malware (2018)
   - Example: ua-parser-js malware (2021)

2. **Maintenance**: Dependencies require updates
   - Breaking changes in dependencies
   - Abandoned dependencies
   - License changes

3. **Simplicity**: Easier to understand and audit
   - No need to learn dependency APIs
   - No version conflicts
   - Faster installation

4. **Longevity**: Code works for years without changes
   - No dependency rot
   - No ecosystem churn
   - Works in future Node.js/Rust versions

## Remember

**Zero external dependencies is non-negotiable for Tuulbelt tools.**

Tuulbelt-to-Tuulbelt composition is allowed (Exception 2), but external dependencies are prohibited.

If you find yourself reaching for an external dependency:
1. Check if std library has the functionality
2. Check if a Tuulbelt tool already solves this (use it!)
3. Implement minimal version yourself
4. Use system tools as fallback
5. Re-evaluate if tool scope is too broad

This skill should be invoked automatically when examining package.json or Cargo.toml files.
